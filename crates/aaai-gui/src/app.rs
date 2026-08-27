//! Top-level application state and message dispatcher (Phase 2).
//!
//! Changes from Phase 1:
//! * Toast subscription properly wired (`App::subscription`).
//! * `FilterMode` for file-tree filtering.
//! * `locale` field + `SwitchLocale` message.
//! * `Instant` passed to `sweep_expired` correctly.

use std::path::PathBuf;
use std::time::Instant;

use iced::widget::pane_grid;
use iced::{Element, Subscription, Task};
use snora::{AppLayout, Toast, ToastIntent, ToastPosition, render};

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
    DiffViewMode, FieldError, FilterMode, FocusTarget, InspectorState, InspectorValidation,
    OpeningValidation, PaneKind, Screen,
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

mod subscription;
mod toast;
mod update;
mod view;

#[cfg(test)]
mod tests;
