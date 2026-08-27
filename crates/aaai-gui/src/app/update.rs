use super::*;

mod approve;
mod audit;
mod dialogs;
mod inspector;
mod navigation;
mod opening;
mod save;

impl App {
    pub(crate) fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            // ── Opening ───────────────────────────────────────────────
            Message::BeforePathChanged(s) => self.before_path_changed(s),
            Message::AfterPathChanged(s) => self.after_path_changed(s),
            Message::DefinitionPathChanged(s) => self.definition_path_changed(s),

            Message::StartAudit => return self.start_audit(),

            // ── File tree ──────────────────────────────────────────────
            Message::SelectEntry(idx) => self.select_entry(idx),

            Message::SetFilter(f) => self.set_filter(f),

            // ── Inspector ──────────────────────────────────────────────
            Message::ReasonChanged(s) => self.reason_changed(s),

            Message::ReasonAction(action) => self.reason_action(action),
            Message::NoteChanged(s) => self.note_changed(s),

            Message::StrategySelected(kind) => self.strategy_selected(kind),
            Message::ChecksumChanged(s) => self.checksum_changed(s),
            Message::RegexPatternChanged(s) => self.regex_pattern_changed(s),
            Message::RegexTargetChanged(new_target) => self.regex_target_changed(new_target),
            Message::AddLineRule => self.add_line_rule(),
            Message::EditRule(idx) => self.edit_rule(idx),

            Message::RemoveLineRule(i) => self.remove_line_rule(i),
            Message::LineRuleActionChanged(i, new_action) => {
                self.line_rule_action_changed(i, new_action)
            }
            Message::LineRuleLineChanged(i, s) => self.line_rule_line_changed(i, s),
            Message::ExactContentChanged(s) => self.exact_content_changed(s),

            // ── Approve ───────────────────────────────────────────────
            Message::ApproveAndSave => return self.approve_and_save(),

            Message::ApproveEntry => return self.approve_entry(),

            // ── Batch ─────────────────────────────────────────────────
            Message::ToggleBatchSelect(idx) => self.toggle_batch_select(idx),
            Message::BatchReasonChanged(s) => self.batch_reason_changed(s),
            Message::BatchStrategySelected(kind) => self.batch_strategy_selected(kind),
            Message::OpenBatchSheet => self.open_batch_sheet(),
            Message::CloseBatchSheet => self.close_batch_sheet(),
            Message::CommitBatchApprove => return self.commit_batch_approve(),

            // ── Re-run / save / report ────────────────────────────────
            Message::RerunAudit => return self.rerun_audit(),

            Message::SaveDefinition => return self.save_definition(),

            Message::ExportReport => return self.export_report(),

            // RFC 046 — save-as dialog result ──────────────────────────
            Message::DefinitionSavePathPicked(None) => self.definition_save_path_cancelled(),

            Message::DefinitionSavePathPicked(Some(chosen)) => {
                self.definition_save_path_picked(chosen)
            }

            Message::ReportPathPicked(None) => self.report_path_cancelled(),

            Message::ReportPathPicked(Some(out)) => self.report_path_picked(out),

            // ── Phase 5: search ───────────────────────────────────────
            Message::SearchQueryChanged(s) => self.search_query_changed(s),

            // ── Phase 10: directory collapse ──────────────────────────────
            Message::ToggleDir(dir) => self.toggle_dir(dir),

            // ── Phase 8: async diff results ───────────────────────────────
            Message::DiffLoading(msg) => self.diff_loading(msg),
            Message::DiffReady(diffs, definition, ignore) => {
                return self.diff_ready(diffs, definition, ignore);
            }
            Message::DiffFailed(err) => self.diff_failed(err),

            // ── Phase 6: undo + navigation ───────────────────────────
            Message::UndoApproval => return self.undo_approval(),
            Message::SelectNext => return self.select_next(),
            Message::SelectPrev => return self.select_prev(),

            // ── Phase 10: theme ───────────────────────────────────────
            Message::SetTheme(t) => self.set_theme(t),

            // ── Phase 10: pane resize ─────────────────────────────────
            Message::PaneResized(e) => self.pane_resized(e),
            Message::PaneFocused(p) => self.pane_focused(p),

            // ── RFC 005: keyboard focus ───────────────────────────────────
            Message::Noop => self.noop(),
            Message::DeselectEntry => self.deselect_entry(),

            // ── RFC 011: diff view mode ───────────────────────────────
            Message::SetDiffViewMode(mode) => self.set_diff_view_mode(mode),

            // ── RFC 015: Opening picker handlers ─────────────────────
            Message::PickBeforeFolder => return self.pick_before_folder(),
            Message::PickAfterFolder => return self.pick_after_folder(),
            Message::PickDefinitionFile => return self.pick_definition_file(),
            Message::PickIgnoreFile => return self.pick_ignore_file(),
            Message::BeforeFolderPicked(opt) => self.before_folder_picked(opt),
            Message::AfterFolderPicked(opt) => self.after_folder_picked(opt),
            Message::DefinitionFilePicked(opt) => self.definition_file_picked(opt),
            Message::IgnoreFilePicked(opt) => self.ignore_file_picked(opt),
            Message::ToggleOptionalSettings => self.toggle_optional_settings(),

            // ── RFC 023: drag-and-drop on the Opening screen ──────────
            Message::FileHoverEnter => self.file_hover_enter(),
            Message::FileHoverLeave => self.file_hover_leave(),
            Message::FileDropped(path) => return self.file_dropped(path),

            // ── RFC 007: navigation ───────────────────────────────────
            Message::BackToOpening => self.back_to_opening(),
            Message::FocusNext => self.focus_next(),
            Message::FocusPrev => self.focus_prev(),
            Message::FocusSearch => self.focus_search(),
            Message::FocusInspectorReason => self.focus_inspector_reason(),

            // ── Phase 3: inspector fields ─────────────────────────────
            Message::TicketChanged(s) => self.ticket_changed(s),
            Message::ApprovedByChanged(s) => self.approved_by_changed(s),
            Message::ExpiresAtChanged(s) => self.expires_at_changed(s),
            Message::ApplyTemplate(id) => self.apply_template(id),

            // ── Phase 3: profiles ─────────────────────────────────────
            Message::IgnorePathChanged(s) => self.ignore_path_changed(s),
            Message::ProfileNameChanged(s) => self.profile_name_changed(s),
            Message::SaveProfile => return self.save_profile(),
            Message::LoadProfile(idx) => self.load_profile(idx),
            Message::DeleteProfile(idx) => self.delete_profile(idx),

            // ── Locale ────────────────────────────────────────────────
            Message::SwitchLocale(code) => self.switch_locale(code),

            // RFC 037 — async rerun completion ────────────────────────
            Message::RerunDiffReady(result) => self.rerun_diff_ready(result),

            // ── RFC 041: navigation guard ─────────────────────────────
            Message::NavGuardCancel => self.nav_guard_cancel(),
            // RFC 086 — second-step reveal of the data-losing action.
            Message::NavGuardRevealDiscard => self.nav_guard_reveal_discard(),

            Message::NavGuardDiscardAndLeave => self.nav_guard_discard_and_leave(),

            Message::NavGuardSaveAndLeave => return self.nav_guard_save_and_leave(),

            // ── RFC 038: keyboard help overlay ────────────────────────
            Message::ToggleHelp => self.toggle_help(),
            Message::CloseHelp => self.close_help(),
            Message::EscapeKey => self.escape_key(),

            // RFC 076 — status legend popover ─────────────────────────
            Message::ToggleStatusLegend => self.toggle_status_legend(),
            // RFC 077 — first-audit coach line ────────────────────────
            Message::DismissCoach => self.dismiss_coach(),

            // RFC 048 — Inspector progressive disclosure ───────────────
            Message::ToggleAdvancedInspector => self.toggle_advanced_inspector(),

            // RFC 054 — glob pattern toggle ───────────────────────────
            Message::ToggleUsePattern => self.toggle_use_pattern(),
            Message::PatternChanged(s) => self.pattern_changed(s),
            // RFC 055 — suggestion chip clicked
            Message::ApplyPatternSuggestion(s) => self.apply_pattern_suggestion(s),

            // RFC 069 — diff pane scroll sync ──────────────────────────
            Message::DiffBeforeScrolled(vp) => return self.diff_before_scrolled(vp),
            Message::DiffAfterScrolled(vp) => return self.diff_after_scrolled(vp),

            // RFC 039 — Revert selected OK entry to Pending ───────────
            Message::RevertSelectedEntry => return self.revert_selected_entry(),

            // ── RFC 036: Settings dialog ──────────────────────────────
            Message::OpenSettings => self.open_settings(),
            Message::CloseSettings => self.close_settings(),
            Message::SaveSettings => self.save_settings(),
            Message::SettingsLanguageChanged(code) => self.settings_language_changed(code),
            // RFC 093 — live preview: apply immediately; Cancel will revert.
            Message::SettingsThemeChanged(theme) => self.settings_theme_changed(theme),
            Message::SettingsIgnoreDirAdd => self.settings_ignore_dir_add(),
            Message::SettingsIgnoreDirEdit(i, s) => self.settings_ignore_dir_edit(i, s),
            Message::SettingsIgnoreDirRemove(i) => self.settings_ignore_dir_remove(i),

            // ── Overlays ──────────────────────────────────────────────
            Message::CloseModals => self.close_modals(),
            Message::CloseMenus => self.close_menus(),

            // ── Toasts ────────────────────────────────────────────────
            Message::DismissToast(id) => self.dismiss_toast(id),
            Message::ToastTick => self.toast_tick(),
            Message::RelativeTimeTick => self.relative_time_tick(),
        }
        Task::none()
    }
}
