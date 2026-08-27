use super::super::*;

impl App {
    pub(in crate::app) fn before_path_changed(&mut self, s: String) {
        self.before_path = s;
        self.validate_opening();
    }

    pub(in crate::app) fn after_path_changed(&mut self, s: String) {
        self.after_path = s;
        self.validate_opening();
    }

    pub(in crate::app) fn definition_path_changed(&mut self, s: String) {
        self.definition_path = s;
    }

    #[allow(clippy::needless_return, clippy::collapsible_if)]
    pub(in crate::app) fn start_audit(&mut self) -> Task<Message> {
        self.open_error = None;
        let before = PathBuf::from(&self.before_path);
        let after = PathBuf::from(&self.after_path);
        let def_path = PathBuf::from(&self.definition_path);

        if !before.is_dir() {
            self.open_error = Some(crate::error::UserError::new(
                t!(
                    "error.opening.before_not_found.message",
                    path = before.display().to_string()
                ),
                t!("error.opening.before_not_found.hint"),
            ));
            return Task::none();
        }
        if !after.is_dir() {
            self.open_error = Some(crate::error::UserError::new(
                t!(
                    "error.opening.after_not_found.message",
                    path = after.display().to_string()
                ),
                t!("error.opening.after_not_found.hint"),
            ));
            return Task::none();
        }

        let definition = if def_path.exists() {
            match config_io::load(&def_path) {
                Ok(d) => d,
                Err(e) => {
                    self.open_error = Some(crate::error::UserError::new(
                        t!(
                            "error.opening.definition_load_failed.message",
                            reason = e.to_string()
                        ),
                        t!("error.opening.definition_load_failed.hint"),
                    ));
                    return Task::none();
                }
            }
        } else {
            AuditDefinition::new_empty()
        };

        // RFC 036 — Build merged ignore rules:
        // 1. Global directory ignores from app settings (always applied)
        // 2. Per-project .aaaiignore rules appended after
        let mut ignore_text = String::new();
        for dir in &self.prefs.global_ignored_dirs {
            let dir = dir.trim();
            if !dir.is_empty() {
                ignore_text.push_str(&format!("{}/**\n", dir));
            }
        }
        let ignore_path_str = self.ignore_path.trim().to_string();
        let project_file = if ignore_path_str.is_empty() {
            before.join(".aaaiignore")
        } else {
            std::path::PathBuf::from(&ignore_path_str)
        };
        if project_file.exists() {
            if let Ok(project_text) = std::fs::read_to_string(&project_file) {
                ignore_text.push('\n');
                ignore_text.push_str(&project_text);
            }
        }
        let ignore = IgnoreRules::from_str(&ignore_text).unwrap_or_default();

        // RFC 042 — auto-save a profile for this session so Recent
        // Projects is always current without requiring an explicit
        // "Save Profile" action.
        self.auto_save_profile();

        // Phase 8: run diff on a background thread to keep the GUI responsive.
        self.is_loading = true;
        // RFC 031 — i18n'd.
        self.load_progress = Some(t!("progress.comparing_folders").to_string());

        let ignore_for_msg = ignore.clone();
        return Task::perform(
            async move {
                tokio::task::spawn_blocking(move || {
                    DiffEngine::compare_with_ignore(&before, &after, &ignore)
                        .map(|diffs| (diffs, definition))
                })
                .await
                .map_err(|e| e.to_string())
                .and_then(|r| r.map_err(|e| e.to_string()))
            },
            |result| match result {
                Ok((diffs, def)) => Message::DiffReady(diffs, def, ignore_for_msg),
                Err(e) => Message::DiffFailed(e),
            },
        );
    }

    #[allow(clippy::needless_return)]
    pub(in crate::app) fn pick_before_folder(&mut self) -> Task<Message> {
        let title = t!("dialog.pick_before").to_string();
        return Task::perform(
            async move {
                rfd::AsyncFileDialog::new()
                    .set_title(title)
                    .pick_folder()
                    .await
                    .map(|h| h.path().to_path_buf())
            },
            Message::BeforeFolderPicked,
        );
    }

    #[allow(clippy::needless_return)]
    pub(in crate::app) fn pick_after_folder(&mut self) -> Task<Message> {
        let title = t!("dialog.pick_after").to_string();
        return Task::perform(
            async move {
                rfd::AsyncFileDialog::new()
                    .set_title(title)
                    .pick_folder()
                    .await
                    .map(|h| h.path().to_path_buf())
            },
            Message::AfterFolderPicked,
        );
    }

    #[allow(clippy::needless_return)]
    pub(in crate::app) fn pick_definition_file(&mut self) -> Task<Message> {
        let title = t!("dialog.pick_audit_yaml").to_string();
        return Task::perform(
            async move {
                rfd::AsyncFileDialog::new()
                    .set_title(title)
                    .add_filter("YAML", &["yaml", "yml"])
                    .pick_file()
                    .await
                    .map(|h| h.path().to_path_buf())
            },
            Message::DefinitionFilePicked,
        );
    }

    #[allow(clippy::needless_return)]
    pub(in crate::app) fn pick_ignore_file(&mut self) -> Task<Message> {
        let title = t!("dialog.pick_aaaiignore").to_string();
        return Task::perform(
            async move {
                rfd::AsyncFileDialog::new()
                    .set_title(title)
                    .pick_file()
                    .await
                    .map(|h| h.path().to_path_buf())
            },
            Message::IgnoreFilePicked,
        );
    }

    pub(in crate::app) fn before_folder_picked(&mut self, opt: Option<std::path::PathBuf>) {
        if let Some(path) = opt {
            self.before_path = path.display().to_string();
            self.validate_opening();
        }
    }

    pub(in crate::app) fn after_folder_picked(&mut self, opt: Option<std::path::PathBuf>) {
        if let Some(path) = opt {
            self.after_path = path.display().to_string();
            self.validate_opening();
        }
    }

    pub(in crate::app) fn definition_file_picked(&mut self, opt: Option<std::path::PathBuf>) {
        if let Some(path) = opt {
            self.definition_path = path.display().to_string();
            self.validate_opening();
        }
    }

    pub(in crate::app) fn ignore_file_picked(&mut self, opt: Option<std::path::PathBuf>) {
        if let Some(path) = opt {
            self.ignore_path = path.display().to_string();
            self.validate_opening();
        }
    }

    pub(in crate::app) fn toggle_optional_settings(&mut self) {
        self.optional_settings_expanded = !self.optional_settings_expanded;
    }

    pub(in crate::app) fn file_hover_enter(&mut self) {
        // Only meaningful while the user is on Opening. We don't
        // restrict by `self.screen` here because the iced event
        // arrives globally; the Opening view itself ignores the
        // `file_hovering` flag on other screens.
        self.file_hovering = true;
    }

    pub(in crate::app) fn file_hover_leave(&mut self) {
        self.file_hovering = false;
    }

    pub(in crate::app) fn file_dropped(&mut self, path: std::path::PathBuf) -> Task<Message> {
        self.file_hovering = false;
        // Only act on Opening — on the Main screen the drop is
        // ignored to avoid surprising the user mid-audit.
        if self.screen != Screen::Opening {
            return Task::none();
        }
        if !path.is_dir() {
            // RFC 023 FR-3: non-folder drops surface as inline
            // error via the open_error banner (RFC 020 pattern).
            self.open_error = Some(crate::error::UserError::new(
                t!(
                    "error.opening.drop_invalid_kind.message",
                    path = path.display().to_string()
                ),
                t!("error.opening.drop_invalid_kind.hint"),
            ));
            return Task::none();
        }
        // Route to the first empty card; if both filled, route to
        // Before (the user can re-drag for After). This is the
        // simplest mapping that satisfies RFC 023 FR-1 without
        // needing layout-coordinate hit-testing.
        let target = path.display().to_string();
        if self.before_path.trim().is_empty() {
            self.before_path = target;
        } else if self.after_path.trim().is_empty() {
            self.after_path = target;
        } else {
            // Both are set — overwrite Before by convention.
            self.before_path = target;
        }
        self.validate_opening();
        Task::none()
    }
}
