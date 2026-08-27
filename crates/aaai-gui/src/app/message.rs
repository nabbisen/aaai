use iced::widget::pane_grid;

use aaai::{
    IgnoreRules,
    config::definition::{LineAction, RegexTarget},
    profile::prefs::Theme as AppTheme,
};

use crate::util::StrategyKind;

use super::{DiffViewMode, FilterMode};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Message {
    // Opening
    BeforePathChanged(String),
    AfterPathChanged(String),
    DefinitionPathChanged(String),
    StartAudit,

    // File tree
    SelectEntry(usize),
    SetFilter(FilterMode),

    // Inspector
    ReasonChanged(String),
    ReasonAction(iced::widget::text_editor::Action), // RFC 009
    NoteChanged(String),
    /// RFC 035 — payload changed from `String` to `StrategyKind`
    /// to support pick_list display/value separation.
    StrategySelected(StrategyKind),
    ChecksumChanged(String),
    RegexPatternChanged(String),
    /// RFC 033 — payload changed from `String` to `RegexTarget` to
    /// support pick_list display/value separation.
    RegexTargetChanged(RegexTarget),
    AddLineRule,
    EditRule(usize), // RFC 012: toggle rule edit mode
    RemoveLineRule(usize),
    /// RFC 033 — payload changed from `String` to `LineAction`.
    LineRuleActionChanged(usize, LineAction),
    LineRuleLineChanged(usize, String),
    ExactContentChanged(String),

    // RFC 036: Settings dialog
    OpenSettings,
    CloseSettings,
    SaveSettings,
    SettingsLanguageChanged(String),
    /// RFC 093 — live-preview theme change from the settings picker.
    SettingsThemeChanged(AppTheme),
    SettingsIgnoreDirAdd,
    SettingsIgnoreDirEdit(usize, String),
    SettingsIgnoreDirRemove(usize),

    /// RFC 037 — carries the diff result from a background rerun started
    /// by `start_async_rerun()`. On Ok: re-evaluates audit + clears dirty.
    RerunDiffReady(Result<Vec<aaai::DiffEntry>, String>),

    // RFC 038: keyboard help overlay
    ToggleHelp,
    CloseHelp,
    /// RFC 038 — routes Escape: closes open overlays before falling through to deselect.
    EscapeKey,

    /// RFC 076 — toggle the status-legend popover.
    ToggleStatusLegend,
    /// RFC 077 — dismiss the first-audit coach line.
    DismissCoach,
    /// RFC 048 — toggle the expert fields section in the Inspector.
    ToggleAdvancedInspector,
    /// RFC 054 — toggle glob-pattern override in the Inspector.
    ToggleUsePattern,
    /// RFC 054 — user edited the pattern text input.
    PatternChanged(String),
    /// RFC 055 — user clicked a suggestion chip.
    ApplyPatternSuggestion(String),

    /// RFC 069 — diff pane scroll synchronisation.
    /// Fired when the user scrolls either side of the side-by-side diff.
    DiffBeforeScrolled(iced::widget::scrollable::Viewport),
    DiffAfterScrolled(iced::widget::scrollable::Viewport),

    /// RFC 039 — removes the currently-selected OK entry from the definition,
    /// reverting it to Pending status. Triggers an async rerun.
    RevertSelectedEntry,

    // RFC 041: navigation guard messages
    NavGuardSaveAndLeave,
    NavGuardDiscardAndLeave,
    /// RFC 086 — reveal the hidden "Discard and leave" action.
    NavGuardRevealDiscard,
    NavGuardCancel,

    // Actions
    /// Internal approval step used by [`Message::ApproveAndSave`].
    /// Prefer `ApproveAndSave` for direct user actions.
    ApproveEntry,
    ApproveAndSave, // RFC 002: approve + save in one action
    RerunAudit,
    SaveDefinition,
    /// RFC 040 — opens the native save-file dialog; format derived from extension.
    ExportReport,
    ReportPathPicked(Option<std::path::PathBuf>),

    /// RFC 046 — result of the save-file dialog opened when `definition_path` is empty.
    DefinitionSavePathPicked(Option<std::path::PathBuf>),

    // Phase 5: search
    SearchQueryChanged(String),

    // Phase 10: directory collapse
    ToggleDir(String),

    // Phase 8: async diff loading
    DiffLoading(String), // progress message (reserved for future channel-based progress)
    DiffReady(Vec<aaai::DiffEntry>, aaai::AuditDefinition, IgnoreRules),
    DiffFailed(String),

    // Phase 6: undo + keyboard navigation
    UndoApproval,
    SelectNext,
    SelectPrev,

    // Phase 3: inspector fields
    TicketChanged(String),
    ApprovedByChanged(String),
    ExpiresAtChanged(String),
    ApplyTemplate(String),

    // Phase 3: profiles
    IgnorePathChanged(String),
    ProfileNameChanged(String),
    SaveProfile,
    LoadProfile(usize),
    DeleteProfile(usize),

    // Phase 10: theme
    SetTheme(AppTheme),

    // Phase 10: pane resize
    PaneResized(pane_grid::ResizeEvent),
    PaneFocused(pane_grid::Pane),

    // RFC 015: Opening screen folder/file picker messages
    PickBeforeFolder,
    PickAfterFolder,
    PickDefinitionFile,
    PickIgnoreFile,
    BeforeFolderPicked(Option<std::path::PathBuf>),
    AfterFolderPicked(Option<std::path::PathBuf>),
    DefinitionFilePicked(Option<std::path::PathBuf>),
    IgnoreFilePicked(Option<std::path::PathBuf>),
    ToggleOptionalSettings,

    // RFC 023: drag-and-drop folder onto the Opening screen
    /// A file/folder is currently being hovered over the window. Used to
    /// switch the Opening view into "drop hint" mode.
    FileHoverEnter,
    /// The drag left the window without dropping. Clear hover state.
    FileHoverLeave,
    /// A file or folder was dropped on the window. The path may be a
    /// folder (routed to the first empty card) or a file (rejected with
    /// an inline error).
    FileDropped(std::path::PathBuf),

    // RFC 007: navigation
    BackToOpening,

    // RFC 011: diff view tab
    SetDiffViewMode(DiffViewMode),

    // RFC 005: keyboard focus messages
    DeselectEntry,
    FocusNext,
    FocusPrev,
    FocusSearch,
    FocusInspectorReason,
    Noop,

    // Locale
    SwitchLocale(String),

    // Overlays
    CloseModals,
    /// Fired by the snora ToastLayer when an outside click should close open overlays.
    /// Kept as a distinct variant (rather than aliasing `Noop`) so that
    /// snora's `on_close_menus()` callback type is self-documenting.
    CloseMenus,

    // Toasts
    DismissToast(u64),
    ToastTick,

    /// RFC 021 §3.5 — 30-second wall-clock tick. Used to refresh
    /// "Saved Nm ago" / "Reported Nm ago" relative-time labels. A
    /// no-op at the state level (it just causes a re-render).
    RelativeTimeTick,
}
