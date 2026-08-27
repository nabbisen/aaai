use super::super::*;

impl App {
    pub(in crate::app) fn save_definition(&mut self) -> Task<Message> {
        let path = PathBuf::from(&self.definition_path);
        if path.as_os_str().is_empty() {
            // RFC 046 — open save-as dialog instead of showing a dead-end error.
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
                    // RFC 021 §3.2 — stamp save time so toolbar
                    // can show "Saved Nm ago" until next mutation.
                    self.last_saved_at = Some(chrono::Utc::now());
                    self.push_toast(
                        ToastIntent::Success,
                        t!("toast.saved").as_ref(),
                        t!("toast.saved_to_path", path = path.display().to_string()).as_ref(),
                    );
                }
                Err(e) => {
                    // RFC 026 — use message+hint pattern. The
                    // raw `e.to_string()` is appended to the
                    // localized message so the user sees both
                    // a user-friendly description and the
                    // concrete OS error.
                    let user_err = crate::error::UserError::from_i18n("error.save.failed");
                    let full_message = format!("{}\n({})", user_err.message, e);
                    self.push_toast_with_hint(
                        ToastIntent::Error,
                        t!("toast.save_failed").as_ref(),
                        &full_message,
                        &user_err.hint,
                    );
                }
            }
        }
        Task::none()
    }

    #[allow(clippy::needless_return)]
    pub(in crate::app) fn export_report(&mut self) -> Task<Message> {
        // RFC 040 — open native save-file dialog; format derived from extension.
        if self.audit_result.is_none() {
            self.push_toast(
                ToastIntent::Info,
                t!("toast.export_failed").as_ref(),
                t!("toast.no_audit_result").as_ref(),
            );
            return Task::none();
        }
        let title = t!("dialog.save_report").to_string();
        return Task::perform(
            async move {
                rfd::AsyncFileDialog::new()
                    .set_title(title)
                    .set_file_name("aaai-report.md")
                    .add_filter("Markdown", &["md"])
                    .add_filter("JSON", &["json"])
                    .save_file()
                    .await
                    .map(|h| h.path().to_path_buf())
            },
            Message::ReportPathPicked,
        );
    }

    pub(in crate::app) fn definition_save_path_cancelled(&mut self) {
        // User cancelled the dialog — clear any pending-leave flag, no toast.
        self.pending_leave_to_opening = false;
    }

    pub(in crate::app) fn definition_save_path_picked(&mut self, chosen: std::path::PathBuf) {
        self.definition_path = chosen.display().to_string();
        // RFC 047 — make the newly-saved path visible in Optional settings.
        self.optional_settings_expanded = true;
        if let Some(def) = &self.definition {
            match config_io::save(def, &chosen, true) {
                Ok(()) => {
                    self.dirty = false;
                    self.last_saved_at = Some(chrono::Utc::now());
                    self.push_toast(
                        ToastIntent::Success,
                        t!("toast.saved").as_ref(),
                        t!("toast.saved_to_path", path = chosen.display().to_string()).as_ref(),
                    );
                    if self.pending_leave_to_opening {
                        self.pending_leave_to_opening = false;
                        self.do_leave_to_opening();
                    }
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
                    self.pending_leave_to_opening = false;
                }
            }
        }
    }

    pub(in crate::app) fn report_path_cancelled(&mut self) {
        /* user cancelled */
    }

    pub(in crate::app) fn report_path_picked(&mut self, out: std::path::PathBuf) {
        if let Some(result) = &self.audit_result {
            let before = PathBuf::from(&self.before_path);
            let after = PathBuf::from(&self.after_path);
            let def_path = if self.definition_path.is_empty() {
                None
            } else {
                Some(PathBuf::from(&self.definition_path))
            };

            // Derive format from chosen extension.
            let use_json = out
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.eq_ignore_ascii_case("json"))
                .unwrap_or(false);

            // RFC 103 §5.1a — GUI report export is out of RFC 103
            // scope; carries the same gap `cmd/report.rs` had,
            // tracked as follow-up.
            let res = if use_json {
                aaai::report::generator::ReportGenerator::write_json(
                    result,
                    &before,
                    &after,
                    def_path.as_deref(),
                    &out,
                    aaai::Masking::Disabled,
                )
            } else {
                aaai::report::generator::ReportGenerator::write_markdown(
                    result,
                    &before,
                    &after,
                    def_path.as_deref(),
                    &out,
                    aaai::Masking::Disabled,
                )
            };

            match res {
                Ok(()) => {
                    self.last_reported_at = Some(chrono::Utc::now());
                    self.push_toast(
                        ToastIntent::Success,
                        t!("toast.export_ok").as_ref(),
                        t!("toast.saved_to_path", path = out.display().to_string()).as_ref(),
                    );
                }
                Err(e) => self.push_toast(
                    ToastIntent::Error,
                    t!("toast.export_failed").as_ref(),
                    &e.to_string(),
                ),
            }
        }
    }

    pub(in crate::app) fn ignore_path_changed(&mut self, s: String) {
        self.ignore_path = s;
    }

    pub(in crate::app) fn profile_name_changed(&mut self, s: String) {
        self.profile_name_input = s;
    }

    pub(in crate::app) fn save_profile(&mut self) -> Task<Message> {
        let name = self.profile_name_input.trim().to_string();
        if name.is_empty() {
            self.push_toast(
                ToastIntent::Error,
                t!("toast.profile").as_ref(),
                t!("toast.profile_name_empty").as_ref(),
            );
            return Task::none();
        }
        let profile = AuditProfile {
            name: name.clone(),
            before: self.before_path.clone(),
            after: self.after_path.clone(),
            definition: if self.definition_path.is_empty() {
                None
            } else {
                Some(self.definition_path.clone())
            },
            ignore_file: if self.ignore_path.is_empty() {
                None
            } else {
                Some(self.ignore_path.clone())
            },
            // RFC 023 §3.2: new profiles start un-touched. The
            // first LoadProfile or explicit re-save will stamp this.
            last_used_at: None,
        };
        self.profiles.add(profile);
        if let Err(e) = self.profiles.save() {
            // RFC 026 — message+hint pattern. The same
            // "couldn't write" hint applies: the user's
            // recourse is the same.
            let user_err = crate::error::UserError::from_i18n("error.save.failed");
            let full_message = format!("{}\n({})", user_err.message, e);
            self.push_toast_with_hint(
                ToastIntent::Error,
                t!("toast.save_failed").as_ref(),
                &full_message,
                &user_err.hint,
            );
        } else {
            self.push_toast(ToastIntent::Success, t!("profile.saved").as_ref(), &name);
            self.profile_name_input.clear();
        }
        Task::none()
    }

    pub(in crate::app) fn load_profile(&mut self, idx: usize) {
        if let Some(p) = self.profiles.profiles.get(idx).cloned() {
            self.before_path = p.before;
            self.after_path = p.after;
            self.definition_path = p.definition.unwrap_or_default();
            self.ignore_path = p.ignore_file.unwrap_or_default();
            // RFC 047 — auto-expand so the user can see which approvals file loaded.
            if !self.definition_path.is_empty() {
                self.optional_settings_expanded = true;
            }
            // RFC 023 FR-6: stamp last_used_at when loading so the
            // Recent list re-orders on next view. We swallow the
            // I/O error: failing to persist the timestamp must not
            // block the user from continuing into the audit.
            let _ = self.profiles.touch(&p.name);
            self.push_toast(
                ToastIntent::Info,
                t!("toast.profile").as_ref(),
                t!("toast.profile_loaded").as_ref(),
            );
        }
    }

    pub(in crate::app) fn delete_profile(&mut self, idx: usize) {
        if let Some(p) = self.profiles.profiles.get(idx).cloned() {
            self.profiles.remove(&p.name);
            let _ = self.profiles.save();
            self.push_toast(
                ToastIntent::Success,
                t!("profile.deleted").as_ref(),
                &p.name,
            );
        }
    }
}
