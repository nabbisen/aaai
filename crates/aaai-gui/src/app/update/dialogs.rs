use super::super::*;

impl App {
    pub(in crate::app) fn set_theme(&mut self, t: AppTheme) {
        self.theme = t;
        self.design_tokens = crate::design_tokens::tokens_for(&t);
        self.prefs.theme = t;
        self.prefs.save();
    }

    pub(in crate::app) fn switch_locale(&mut self, code: String) {
        rust_i18n::set_locale(&code);
        self.locale = code;
    }

    pub(in crate::app) fn nav_guard_cancel(&mut self) {
        self.nav_guard_open = false;
        self.nav_guard_show_discard = false;
    }

    pub(in crate::app) fn nav_guard_reveal_discard(&mut self) {
        self.nav_guard_show_discard = true;
    }

    pub(in crate::app) fn nav_guard_discard_and_leave(&mut self) {
        self.nav_guard_open = false;
        self.dirty = false;
        self.do_leave_to_opening();
    }

    pub(in crate::app) fn nav_guard_save_and_leave(&mut self) -> Task<Message> {
        // Inline save; navigate on success, show error and stay on failure.
        let path = PathBuf::from(&self.definition_path);
        if path.as_os_str().is_empty() {
            // RFC 046 — open save-as dialog; navigate after a successful pick+save.
            self.nav_guard_open = false;
            self.pending_leave_to_opening = true;
            let title = t!("dialog.save_approvals_file").to_string();
            return Task::perform(
                async move {
                    rfd::AsyncFileDialog::new()
                        .set_title(title)
                        .set_file_name("audit.yaml")
                        .add_filter("YAML", &["yaml", "yml"])
                        .save_file()
                        .await
                        .map(|h| h.path().to_path_buf())
                },
                Message::DefinitionSavePathPicked,
            );
        }
        if let Some(def) = &self.definition {
            match config_io::save(def, &path, true) {
                Ok(()) => {
                    self.dirty = false;
                    self.last_saved_at = Some(chrono::Utc::now());
                    self.nav_guard_open = false;
                    self.do_leave_to_opening();
                }
                Err(e) => {
                    let user_err = crate::error::UserError::from_i18n("error.save.failed");
                    let full_message = format!("{}\n({})", user_err.message, e);
                    self.push_toast_with_hint(
                        ToastIntent::Error,
                        t!("toast.save_failed").as_ref(),
                        &full_message,
                        &user_err.hint,
                    );
                    self.nav_guard_open = false;
                    // Do NOT navigate — user must resolve the save error.
                }
            }
        }
        Task::none()
    }

    pub(in crate::app) fn toggle_help(&mut self) {
        self.help_open = !self.help_open;
    }

    pub(in crate::app) fn close_help(&mut self) {
        self.help_open = false;
    }

    pub(in crate::app) fn escape_key(&mut self) {
        // Prioritise overlay-close before falling through to deselect.
        if self.help_open {
            self.help_open = false;
        } else if self.nav_guard_open {
            self.nav_guard_open = false;
        } else if self.settings_open {
            self.settings_open = false;
        } else {
            self.selected_index = None;
        }
    }

    pub(in crate::app) fn toggle_status_legend(&mut self) {
        self.status_legend_open = !self.status_legend_open;
    }

    pub(in crate::app) fn dismiss_coach(&mut self) {
        self.coach_dismissed = true;
    }

    pub(in crate::app) fn open_settings(&mut self) {
        self.settings_draft = self.prefs.clone();
        self.settings_open = true;
    }

    pub(in crate::app) fn close_settings(&mut self) {
        // RFC 093 — revert live-preview theme change on cancel.
        let original = self.prefs.theme;
        self.theme = original;
        self.design_tokens = crate::design_tokens::tokens_for(&original);
        self.settings_open = false;
        // draft is abandoned; prefs remain unchanged
    }

    pub(in crate::app) fn save_settings(&mut self) {
        self.prefs = self.settings_draft.clone();
        // Trim empty entries before saving
        self.prefs
            .global_ignored_dirs
            .retain(|d| !d.trim().is_empty());
        // Apply language change immediately
        if !self.prefs.language.is_empty() {
            rust_i18n::set_locale(&self.prefs.language);
            self.locale = self.prefs.language.clone();
        }
        // RFC 093 — commit live-preview theme; tokens already applied.
        self.theme = self.prefs.theme;
        self.design_tokens = crate::design_tokens::tokens_for(&self.prefs.theme);
        self.prefs.save();
        self.settings_open = false;
    }

    pub(in crate::app) fn settings_language_changed(&mut self, code: String) {
        self.settings_draft.language = code;
    }

    pub(in crate::app) fn settings_theme_changed(&mut self, theme: AppTheme) {
        self.settings_draft.theme = theme;
        self.theme = theme;
        self.design_tokens = crate::design_tokens::tokens_for(&theme);
    }

    pub(in crate::app) fn settings_ignore_dir_add(&mut self) {
        self.settings_draft.global_ignored_dirs.push(String::new());
    }

    pub(in crate::app) fn settings_ignore_dir_edit(&mut self, i: usize, s: String) {
        if let Some(entry) = self.settings_draft.global_ignored_dirs.get_mut(i) {
            *entry = s;
        }
    }

    pub(in crate::app) fn settings_ignore_dir_remove(&mut self, i: usize) {
        let dirs = &mut self.settings_draft.global_ignored_dirs;
        if i < dirs.len() {
            dirs.remove(i);
        }
    }

    pub(in crate::app) fn close_modals(&mut self) {
        // RFC 111 removed the only state this closed (the multi-select
        // approval sheet). RFC 110 §4.1 makes this the single dismissal
        // path for the three real overlays instead.
    }

    pub(in crate::app) fn close_menus(&mut self) {
        /* snora overlay close — no state change needed */
    }

    pub(in crate::app) fn dismiss_toast(&mut self, id: u64) {
        self.toasts.retain(|t| t.id != id);
    }

    pub(in crate::app) fn toast_tick(&mut self) {
        snora::toast::sweep_expired(&mut self.toasts, Instant::now());
    }

    pub(in crate::app) fn relative_time_tick(&mut self) {
        // No-op at the state level — receiving this message
        // causes iced to re-render, which is enough to refresh
        // the "Saved Nm ago" labels through humanize_since.
    }
}
