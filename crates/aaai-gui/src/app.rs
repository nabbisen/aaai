//! Top-level application state and message dispatcher (Phase 2).
//!
//! Changes from Phase 1:
//! * Toast subscription properly wired (`App::subscription`).
//! * `FilterMode` for file-tree filtering.
//! * `BatchApproveState` for bulk approval.
//! * `locale` field + `SwitchLocale` message.
//! * `Instant` passed to `sweep_expired` correctly.

use std::path::PathBuf;
use std::time::Instant;

use iced::widget::pane_grid;
use iced::{Element, Subscription, Task};
use snora::{AppLayout, Sheet, SheetEdge, SheetSize, Toast, ToastIntent, ToastPosition, render};

use aaai::{
    AuditDefinition, AuditEngine, AuditResult, AuditStatus, DiffEngine, DiffType, IgnoreRules,
    config::{
        definition::{AuditEntry, AuditStrategy, LineAction, LineRule, RegexTarget},
        io as config_io,
    },
    history::{record::HistoryRecord, store as history_store},
    profile::prefs::{Theme as AppTheme, UserPrefs},
    profile::store::{AuditProfile, ProfileStore},
};
use regex::Regex as RegexCheck;

use crate::style::panel_style;
use crate::util::StrategyKind;
use crate::views::{main_view, opening};
use rust_i18n::t;

mod state;
pub use state::{
    BatchApproveState, DiffViewMode, FieldError, FilterMode, FocusTarget, InspectorState,
    InspectorValidation, OpeningValidation, PaneKind, Screen,
};

// ── App state ─────────────────────────────────────────────────────────────

pub struct App {
    pub screen: Screen,

    // Phase 8: async diff state
    pub is_loading: bool,
    pub load_progress: Option<String>,

    // 最後に使用した IgnoreRules（rerun 時に再利用）
    pub active_ignore: IgnoreRules,

    // Opening
    /// RFC 015: optional settings (audit.yaml / .aaaiignore) section expansion
    pub optional_settings_expanded: bool,
    /// RFC 023: true while a drag is active over the window — flips
    /// `opening` into "drop here" hint mode.
    pub file_hovering: bool,
    pub before_path: String,
    pub after_path: String,
    pub definition_path: String,
    pub open_error: Option<crate::error::UserError>,
    pub opening_validation: OpeningValidation,

    // Main
    pub diffs: Vec<aaai::DiffEntry>,
    pub audit_result: Option<AuditResult>,
    pub definition: Option<AuditDefinition>,
    pub selected_index: Option<usize>,
    pub filter_mode: FilterMode,

    // Inspector
    pub inspector: InspectorState,

    // Batch
    pub batch: BatchApproveState,
    pub batch_sheet_open: bool,

    // Unsaved
    pub dirty: bool,

    // RFC 021 — screen navigation continuity
    /// True when the in-memory definition has changed since the last
    /// successful audit run, so the displayed `audit_result` may be
    /// stale. Set by handlers that mutate `self.definition` (approve,
    /// undo, inspector edits); cleared by `rerun_audit`.
    pub audit_dirty: bool,
    /// Wall-clock time of the last successful definition save. `None`
    /// until the first save. Used for the toolbar "Saved Nm ago" mark.
    pub last_saved_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Wall-clock time of the last successful report export. `None`
    /// until the first export. Used for the toolbar "Reported Nm ago" mark.
    pub last_reported_at: Option<chrono::DateTime<chrono::Utc>>,

    // Toasts
    pub toasts: Vec<Toast<Message>>,
    pub toast_id: u64,

    // Locale
    pub locale: String,

    // Phase 10: theme
    pub theme: AppTheme,
    /// RFC 092 — resolved design tokens for the active theme.
    /// Updated on SetTheme; read by every view for token-driven styling.
    pub design_tokens: snora::design::Tokens,

    // RFC 011: diff view tab selection
    pub diff_view_mode: DiffViewMode,

    // RFC 005: keyboard focus
    pub focus_target: FocusTarget,
    pub prefs: UserPrefs,

    // RFC 036: Settings dialog
    pub settings_open: bool,
    pub settings_draft: UserPrefs,

    // RFC 038: keyboard help overlay
    pub help_open: bool,

    // RFC 041: navigation guard (unsaved-changes confirmation)
    pub nav_guard_open: bool,
    /// RFC 086 — whether the secondary "Discard and leave" action is revealed.
    /// Hidden by default so the safest choice (Stay / Save) is easiest; the
    /// user must click "More choices" to expose the data-losing option.
    pub nav_guard_show_discard: bool,

    /// RFC 046 — set when a save-as dialog is opened from NavGuardSaveAndLeave.
    /// Tells `DefinitionSavePathPicked` to call `do_leave_to_opening()` after saving.
    pub pending_leave_to_opening: bool,

    /// RFC 048 — progressive disclosure in the Inspector.
    /// `false` = show Reason + Strategy only (default for new users).
    /// `true`  = show all fields (expert mode, global across entries).
    pub advanced_inspector_expanded: bool,

    /// RFC 069 — scroll-sync guard for side-by-side diff panes.
    /// Prevents the programmatic scroll that syncs the peer pane from
    /// bouncing back through on_scroll and creating an infinite loop.
    pub diff_scroll_syncing: bool,

    /// RFC 077 — first-audit coach line: hidden once the user dismisses it.
    pub coach_dismissed: bool,

    /// RFC 076 — status legend popover is open.
    pub status_legend_open: bool,

    // Phase 10: resizable pane layout
    pub panes: pane_grid::State<PaneKind>,
    pub focus: Option<pane_grid::Pane>,

    // Phase 3: profiles
    pub profiles: ProfileStore,
    pub profile_name_input: String,

    // Phase 3: ignore rules (loaded at audit start)
    pub ignore_path: String,

    // Phase 5: file tree search
    pub search_query: String,

    // Phase 10: directory collapse state
    pub collapsed_dirs: std::collections::HashSet<String>,

    // Phase 6: undo stack (stores path of last upserted entry)
    pub undo_stack: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        App {
            screen: Screen::Opening,
            optional_settings_expanded: false,
            file_hovering: false,
            is_loading: false,
            load_progress: None,
            active_ignore: IgnoreRules::default(),
            before_path: String::new(),
            after_path: String::new(),
            definition_path: String::new(),
            open_error: None,
            opening_validation: OpeningValidation::default(),
            diffs: Vec::new(),
            audit_result: None,
            definition: None,
            selected_index: None,
            filter_mode: FilterMode::ChangedOnly,
            inspector: InspectorState::default(),
            batch: BatchApproveState::default(),
            batch_sheet_open: false,
            dirty: false,
            audit_dirty: false,
            last_saved_at: None,
            last_reported_at: None,
            toasts: Vec::new(),
            toast_id: 0,
            prefs: {
                // RFC 036 — load persisted settings; apply stored language immediately.
                let p = UserPrefs::load();
                if !p.language.is_empty() {
                    rust_i18n::set_locale(&p.language);
                }
                p
            },
            locale: rust_i18n::locale().to_string(),
            theme: UserPrefs::load().theme,
            design_tokens: crate::design_tokens::tokens_for(&UserPrefs::load().theme),
            settings_open: false,
            settings_draft: UserPrefs::default(),
            help_open: false,
            nav_guard_open: false,
            nav_guard_show_discard: false,
            pending_leave_to_opening: false,
            advanced_inspector_expanded: false,
            diff_scroll_syncing: false,
            coach_dismissed: false,
            status_legend_open: false,
            diff_view_mode: DiffViewMode::default(),
            focus_target: FocusTarget::default(),
            profiles: ProfileStore::load().unwrap_or_default(),
            profile_name_input: String::new(),
            ignore_path: String::new(),
            search_query: String::new(),
            collapsed_dirs: std::collections::HashSet::new(),
            undo_stack: Vec::new(),
            panes: {
                let (tree, _) = pane_grid::State::new(PaneKind::FileTree);
                // We'll rebuild panes in rerun_audit/DiffReady; use placeholder here.
                tree
            },
            focus: None,
        }
    }
}

mod message;
pub use message::Message;

// ── Update ────────────────────────────────────────────────────────────────

impl App {
    pub fn update(&mut self, msg: Message) -> Task<Message> {
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

    // ── Subscription ─────────────────────────────────────────────────────

    pub fn subscription(&self) -> Subscription<Message> {
        let toast_sub = snora::toast::subscription(&self.toasts, || Message::ToastTick);
        let kb_sub = iced::keyboard::listen().map(|event| {
            use iced::keyboard::{Event as KbEvent, Key, Modifiers};
            match event {
                KbEvent::KeyPressed { key, modifiers, .. } => {
                    match (key.as_ref(), modifiers) {
                        (Key::Character("s"), m) if m.contains(Modifiers::CTRL) => {
                            Message::SaveDefinition
                        }
                        (Key::Character("r"), m) if m.contains(Modifiers::CTRL) => {
                            Message::RerunAudit
                        }
                        (Key::Character("z"), m)
                            if m.contains(Modifiers::CTRL) && m.contains(Modifiers::SHIFT) =>
                        {
                            Message::RevertSelectedEntry
                        }
                        (Key::Character("z"), m) if m.contains(Modifiers::CTRL) => {
                            Message::UndoApproval
                        }
                        // RFC 005: Ctrl+E → export report
                        (Key::Character("e"), m) if m.contains(Modifiers::CTRL) => {
                            Message::ExportReport
                        }
                        (Key::Named(iced::keyboard::key::Named::ArrowDown), _) => {
                            Message::SelectNext
                        }
                        (Key::Named(iced::keyboard::key::Named::ArrowUp), _) => Message::SelectPrev,
                        // RFC 005: Tab / Shift+Tab for pane focus cycling
                        (Key::Named(iced::keyboard::key::Named::Tab), m)
                            if m.contains(Modifiers::SHIFT) =>
                        {
                            Message::FocusPrev
                        }
                        (Key::Named(iced::keyboard::key::Named::Tab), _) => Message::FocusNext,
                        // RFC 005: / key → focus search
                        (Key::Character("/"), m)
                            if !m.contains(Modifiers::CTRL) && !m.contains(Modifiers::ALT) =>
                        {
                            Message::FocusSearch
                        }
                        // RFC 051 — Ctrl+Enter submits approval (the reason text is
                        // trimmed in the handler, so an accidental trailing newline
                        // from the text_editor is harmless).
                        (Key::Named(iced::keyboard::key::Named::Enter), m)
                            if m.contains(Modifiers::CTRL) =>
                        {
                            Message::ApproveAndSave
                        }
                        // RFC 005: Enter → focus inspector reason
                        (Key::Named(iced::keyboard::key::Named::Enter), _) => {
                            Message::FocusInspectorReason
                        }
                        // RFC 038: ? key → toggle keyboard help overlay
                        (Key::Character("?"), _) => Message::ToggleHelp,
                        // Escape — handled via EscapeKey to avoid capturing self in the closure
                        (Key::Named(iced::keyboard::key::Named::Escape), _) => Message::EscapeKey,
                        _ => Message::Noop,
                    }
                }
                _ => Message::Noop,
            }
        });
        // RFC 021 §3.5 — 30-second wall-clock tick. Only enabled when at
        // least one timestamp is present, so we don't burn CPU re-rendering
        // until the user has actually saved or exported once.
        let needs_tick = self.last_saved_at.is_some() || self.last_reported_at.is_some();
        let time_sub: Subscription<Message> = if needs_tick {
            iced::time::every(std::time::Duration::from_secs(30)).map(|_| Message::RelativeTimeTick)
        } else {
            Subscription::none()
        };

        Subscription::batch([toast_sub, kb_sub, dnd_sub(), time_sub])
    }

    // ── View ─────────────────────────────────────────────────────────────

    pub fn view(&self) -> Element<'_, Message> {
        let body = match self.screen {
            Screen::Opening => opening::view(self),
            Screen::Main => main_view::view(self),
        };

        let footer = self.view_footer();

        let mut layout = AppLayout::new(body)
            .footer(footer)
            .toasts(self.toasts.clone())
            .toast_position(ToastPosition::BottomEnd)
            .on_close_modals(Message::CloseModals)
            .on_close_menus(Message::CloseMenus);

        // Batch approve sheet
        if self.batch_sheet_open {
            let sheet_content = crate::views::batch::view(self);
            layout = layout.sheet(
                Sheet::new(sheet_content)
                    .at(SheetEdge::End)
                    .with_size(SheetSize::Pixels(380.0)),
            );
        }

        let base: Element<'_, Message> = render(layout);

        // RFC 036 — Settings dialog modal overlay
        if self.settings_open {
            use iced::widget::{container, mouse_area, stack};
            use iced::{Color, Length};

            let backdrop = mouse_area(
                container(
                    iced::widget::space()
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.35,
                    })),
                    ..Default::default()
                }),
            )
            .on_press(Message::CloseSettings);

            let dialog = iced::widget::center(crate::views::settings_dialog::view(
                &self.settings_draft,
                &self.locale,
                &self.design_tokens,
            ));

            stack![base, backdrop, dialog].into()

        // RFC 038 — Keyboard help overlay (only on Main screen)
        } else if self.help_open && matches!(self.screen, Screen::Main) {
            use iced::widget::{container, mouse_area, stack};
            use iced::{Color, Length};

            let backdrop = mouse_area(
                container(
                    iced::widget::space()
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.35,
                    })),
                    ..Default::default()
                }),
            )
            .on_press(Message::CloseHelp);

            let dialog =
                iced::widget::center(crate::views::help_overlay::view(&self.design_tokens));

            stack![base, backdrop, dialog].into()

        // RFC 041 — Navigation guard (only on Main screen)
        } else if self.nav_guard_open && matches!(self.screen, Screen::Main) {
            use iced::widget::{container, mouse_area, stack};
            use iced::{Color, Length};

            let backdrop = mouse_area(
                container(
                    iced::widget::space()
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.50,
                    })),
                    ..Default::default()
                }),
            )
            .on_press(Message::NavGuardCancel);

            let dialog = iced::widget::center(crate::views::nav_guard::view(
                self.nav_guard_show_discard,
                &self.design_tokens,
            ));

            stack![base, backdrop, dialog].into()
        } else {
            base
        }
    }

    fn view_footer(&self) -> Element<'_, Message> {
        use iced::widget::tooltip::Position;
        use iced::{
            Alignment::Center,
            Length,
            widget::{button, container, row, space, text, tooltip},
        };

        // RFC 036 — language picker moved to Settings dialog.
        // RFC 038 — ? button (help overlay) + ⚙ settings button.
        let help_btn = tooltip(
            button(
                text("?")
                    .size(self.design_tokens.typography.label.size)
                    .line_height(self.design_tokens.typography.label.line_height),
            )
            .on_press(Message::ToggleHelp)
            .padding(iced::Padding::from([
                self.design_tokens.spacing.xs,
                self.design_tokens.spacing.sm,
            ]))
            .style({
                let t = self.design_tokens.clone();
                move |_th, s| crate::style::btn_ghost(&t, s)
            }),
            text(t!("help.title").to_string())
                .size(self.design_tokens.typography.body_small.size)
                .line_height(self.design_tokens.typography.body_small.line_height),
            Position::Top,
        );

        let settings_btn = tooltip(
            button(
                text("⚙")
                    .size(self.design_tokens.typography.label.size)
                    .line_height(self.design_tokens.typography.label.line_height),
            )
            .on_press(Message::OpenSettings)
            .padding(iced::Padding::from([
                self.design_tokens.spacing.xs,
                self.design_tokens.spacing.sm,
            ]))
            .style({
                let t = self.design_tokens.clone();
                move |_th, s| crate::style::btn_ghost(&t, s)
            }),
            text(t!("settings.button_tooltip").to_string())
                .size(self.design_tokens.typography.body_small.size)
                .line_height(self.design_tokens.typography.body_small.line_height),
            Position::Top,
        );

        let left: Element<'_, Message> = if self.dirty {
            text(t!("footer.unsaved"))
                .size(self.design_tokens.typography.body_small.size)
                .line_height(self.design_tokens.typography.body_small.line_height)
                .color(crate::style::to_iced(self.design_tokens.palette.warning))
                .into()
        } else {
            text("")
                .size(self.design_tokens.typography.body_small.size)
                .line_height(self.design_tokens.typography.body_small.line_height)
                .into()
        };

        container(
            row![
                left,
                space().width(Length::Fill),
                help_btn,
                settings_btn,
                text(format!("v{}", env!("CARGO_PKG_VERSION")))
                    .size(self.design_tokens.typography.body_small.size)
                    .line_height(self.design_tokens.typography.body_small.line_height),
            ]
            .align_y(Center)
            .spacing(self.design_tokens.spacing.md),
        )
        .width(Length::Fill)
        .padding(iced::Padding::from([
            self.design_tokens.spacing.xs,
            self.design_tokens.spacing.lg,
        ]))
        .style(panel_style(self.design_tokens.clone()))
        .into()
    }

    // ── Helpers ───────────────────────────────────────────────────────────

    /// RFC 002: per-field real-time validation for the inspector.
    fn validate_inspector(&mut self) {
        use aaai::config::definition::AuditStrategy;
        let ins = &self.inspector;
        let mut v = InspectorValidation::default();

        // Reason (required)
        if ins.reason.trim().is_empty() {
            // RFC 031 — i18n'd.
            v.reason_error = Some(t!("error.inspector.reason_required.message").to_string());
        }

        // ExpiresAt format
        if !ins.expires_at_str.trim().is_empty() {
            if chrono::NaiveDate::parse_from_str(&ins.expires_at_str, "%Y-%m-%d").is_err() {
                // RFC 031 — i18n-migrated; this was the last
                // hardcoded user-facing string in app.rs.
                v.expires_at_error =
                    Some(t!("error.inspector.expires_at_format.message").to_string());
            }
        }

        // Strategy-specific validation
        match &ins.strategy {
            AuditStrategy::Checksum { expected_sha256 } => {
                let s = expected_sha256.trim();
                if s.len() != 64 || !s.chars().all(|c| c.is_ascii_hexdigit()) {
                    // RFC 030 — message + actionable hint. The
                    // raw "64 hex chars" message assumes SHA-256
                    // familiarity; the hint says where the value
                    // comes from for users new to the tool.
                    let err = crate::error::UserError::from_i18n("error.inspector.invalid_sha256");
                    v.strategy_errors.push(FieldError {
                        field: "expected_sha256".into(),
                        message: err.message,
                        hint: Some(err.hint),
                    });
                }
            }
            AuditStrategy::LineMatch { rules } => {
                if rules.is_empty() {
                    // RFC 030 — message + actionable hint. New
                    // users may not realise the `+ Add rule`
                    // button below the empty rules list is where
                    // they go next.
                    let err = crate::error::UserError::from_i18n("error.inspector.empty_rules");
                    v.strategy_errors.push(FieldError {
                        field: "rules".into(),
                        message: err.message,
                        hint: Some(err.hint),
                    });
                }
                for (i, rule) in rules.iter().enumerate() {
                    if rule.line.trim().is_empty() {
                        // RFC 029 — hint stays None: the message ("rule line
                        // cannot be empty") already points at the action.
                        v.strategy_errors.push(FieldError {
                            field: format!("rule[{}].line", i),
                            message: t!("error.inspector.empty_rule_line.message").to_string(),
                            hint: None,
                        });
                    }
                }
            }
            AuditStrategy::Regex { pattern, .. } => {
                if let Err(e) = RegexCheck::new(pattern) {
                    // RFC 028 — hint is now a structural field, not
                    // composed into the message. The message line
                    // carries the localized description + the concrete
                    // syntax error from the regex compiler; the hint
                    // line carries the actionable next step
                    // (i.e. "test at regex101.com").
                    let err = crate::error::UserError::from_i18n("error.inspector.invalid_regex");
                    v.strategy_errors.push(FieldError {
                        field: "pattern".into(),
                        message: format!("{} ({})", err.message, e),
                        hint: Some(err.hint),
                    });
                }
            }
            AuditStrategy::Exact { expected_content } => {
                if expected_content.trim().is_empty() {
                    // RFC 029.
                    v.strategy_errors.push(FieldError {
                        field: "expected_content".into(),
                        message: t!("error.inspector.empty_expected.message").to_string(),
                        hint: None,
                    });
                }
            }
            AuditStrategy::None => {}
        }

        self.inspector.validation = v;
    }

    pub fn validate_opening(&mut self) {
        let mut v = OpeningValidation::default();
        let before_s = self.before_path.trim().to_string();
        let after_s = self.after_path.trim().to_string();

        // RFC 031 — all 6 inline validation messages migrated to t!().
        // Distinct from the RFC 020 banner path's
        // error.opening.{before,after}_not_found.* keys, which carry
        // path interpolation. Inline versions are terse.
        if before_s.is_empty() {
            v.before_error = Some(t!("error.opening.before_required.message").to_string());
        } else {
            let p = std::path::Path::new(&before_s);
            if !p.exists() {
                v.before_error = Some(t!("error.opening.folder_missing.message").to_string());
            } else if !p.is_dir() {
                v.before_error = Some(t!("error.opening.not_a_directory.message").to_string());
            }
        }

        if after_s.is_empty() {
            v.after_error = Some(t!("error.opening.after_required.message").to_string());
        } else {
            let p = std::path::Path::new(&after_s);
            if !p.exists() {
                v.after_error = Some(t!("error.opening.folder_missing.message").to_string());
            } else if !p.is_dir() {
                v.after_error = Some(t!("error.opening.not_a_directory.message").to_string());
            }
        }
        self.opening_validation = v;
    }

    /// RFC 042 — silently upsert a profile for the current paths.
    ///
    /// Called at audit start so the Recent Projects list stays current
    /// without requiring explicit "Save Profile" actions. Profile name is
    /// derived from the definition file stem or before-folder name.
    /// I/O errors are swallowed — a failing auto-save must never block the audit.
    fn auto_save_profile(&mut self) {
        let name = {
            let from_def = (!self.definition_path.is_empty())
                .then(|| {
                    std::path::Path::new(&self.definition_path)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_string())
                })
                .flatten();

            let from_before = (!self.before_path.is_empty())
                .then(|| {
                    std::path::Path::new(&self.before_path)
                        .file_name()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_string())
                })
                .flatten();

            from_def
                .or(from_before)
                .unwrap_or_else(|| "untitled".to_string())
        };

        let profile = aaai::profile::store::AuditProfile {
            name,
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
            last_used_at: Some(chrono::Utc::now()),
        };

        self.profiles.add(profile);
        let _ = self.profiles.save();
    }

    /// RFC 075 — pre-select the most helpful strategy for a given diff type.
    /// Applied only to new (unapproved) entries so it doesn't override
    /// existing user choices. `pub` so the inspector view can read it.
    pub fn recommended_strategy(diff_type: DiffType) -> StrategyKind {
        match diff_type {
            DiffType::Modified => StrategyKind::LineMatch,
            _ => StrategyKind::None,
        }
    }

    /// RFC 055 — derive up to 3 glob suggestions from a concrete path.
    fn suggest_patterns(path: &str) -> Vec<String> {
        let parts: Vec<&str> = path.split('/').collect();
        let mut seen = std::collections::HashSet::new();
        let mut out: Vec<String> = Vec::new();

        let mut push = |s: String| {
            if seen.insert(s.clone()) && out.len() < 3 {
                out.push(s);
            }
        };

        // Extension (last component after last '.', not in a dir name)
        let ext: Option<&str> = path
            .rsplit('.')
            .next()
            .filter(|e| !e.contains('/') && !e.is_empty() && e.len() <= 6);

        if parts.len() >= 2 {
            push(format!("{}/**", parts[0]));
            if let Some(e) = ext {
                push(format!("{}/**/*.{}", parts[0], e));
            }
        }
        if let Some(e) = ext {
            push(format!("**/*.{}", e));
        }
        out
    }

    /// RFC 054 — validate the glob pattern in the Inspector.
    fn validate_pattern(&mut self) {
        if !self.inspector.use_pattern {
            self.inspector.validation.pattern_error = None;
            return;
        }
        let pat = self.inspector.pattern_path.trim();
        if pat.is_empty() {
            self.inspector.validation.pattern_error =
                Some(t!("inspector.pattern_empty").to_string());
        } else if glob::Pattern::new(pat).is_err() {
            self.inspector.validation.pattern_error =
                Some(t!("inspector.pattern_invalid").to_string());
        } else {
            self.inspector.validation.pattern_error = None;
        }
    }

    /// RFC 041 — centralised navigation to the Opening screen.
    /// Clears all main-screen state and closes any open overlays.
    fn do_leave_to_opening(&mut self) {
        self.screen = Screen::Opening;
        self.audit_result = None;
        self.diffs.clear();
        self.definition = None;
        self.selected_index = None;
        self.inspector = InspectorState::default();
        self.audit_dirty = false;
        self.help_open = false;
        self.nav_guard_open = false;
    }

    /// RFC 037 — Non-blocking rerun helper.
    /// Sets `is_loading = true`, `audit_dirty` stays true until the background
    /// diff completes and `RerunDiffReady` fires.  Callers should push any
    /// "what just happened" toast *before* returning this task so that the
    /// toast is immediately visible while the diff runs.
    fn start_async_rerun(&mut self) -> Task<Message> {
        let before = std::path::PathBuf::from(&self.before_path);
        let after = std::path::PathBuf::from(&self.after_path);
        let ignore = self.active_ignore.clone();

        self.is_loading = true;
        self.load_progress = Some(t!("progress.rerunning").to_string());

        Task::perform(
            async move {
                tokio::task::spawn_blocking(move || {
                    DiffEngine::compare_with_ignore(&before, &after, &ignore)
                        .map_err(|e| e.to_string())
                })
                .await
                .map_err(|e| e.to_string())
                .and_then(|r| r)
            },
            Message::RerunDiffReady,
        )
    }

    pub fn push_toast(&mut self, intent: ToastIntent, title: &str, body: &str) {
        let id = self.toast_id;
        self.toast_id += 1;
        self.toasts.push(Toast::new(
            id,
            intent,
            title.to_string(),
            body.to_string(),
            Message::DismissToast(id),
        ));
    }

    /// Push a toast with a two-line body following RFC 020's
    /// message + hint pattern (RFC 026 §3). The first line names what
    /// happened in user-facing terms; the second line — prefixed with
    /// a 💡 marker for visual distinction — names what to do next.
    ///
    /// Use this for *actionable* errors. For purely informational
    /// success / info toasts, use [`Self::push_toast`] directly.
    pub fn push_toast_with_hint(
        &mut self,
        intent: ToastIntent,
        title: &str,
        message: &str,
        hint: &str,
    ) {
        let body = format!("{message}\n\n💡 {hint}");
        self.push_toast(intent, title, &body);
    }

    /// Push a toast carrying an already-built [`crate::error::UserError`].
    /// Convenience over [`Self::push_toast_with_hint`] for call-sites
    /// that constructed the error elsewhere (e.g. via
    /// `UserError::from_i18n("error.save.failed")`).
    ///
    /// Currently no internal site uses this — the two existing save_failed
    /// sites and the inspector regex site each build the toast inline. This
    /// method is part of RFC 026's public surface for future error sites
    /// (e.g. when DiffFailed, profile delete failure, or export failure
    /// gain proper UserError plumbing).
    #[allow(dead_code)]
    pub fn push_user_error_toast(
        &mut self,
        intent: ToastIntent,
        title: &str,
        err: &crate::error::UserError,
    ) {
        self.push_toast_with_hint(intent, title, &err.message, &err.hint);
    }
}

mod subscription;
use subscription::dnd_sub;

mod update;

#[cfg(test)]
mod tests;
