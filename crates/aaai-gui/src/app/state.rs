use aaai::{AuditStatus, AuditStrategy, DiffType, FileAuditResult};

use crate::util::StrategyKind;

// ── Pane identifiers ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneKind {
    FileTree,
    Diff,
    Inspector,
}

// ── Diff view mode (RFC 011) ─────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiffViewMode {
    #[default]
    SideBySide, // 左右差分
    Unified,     // 統合
    ChangedOnly, // 変更のみ
}

// ── Keyboard focus (RFC 005) ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusTarget {
    #[default]
    FileTree,
    Search,
    Inspector,
}

// ── Opening screen validation ─────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct OpeningValidation {
    pub before_error: Option<String>,
    pub after_error: Option<String>,
}

impl OpeningValidation {
    pub fn can_start(&self) -> bool {
        self.before_error.is_none() && self.after_error.is_none()
    }
}

// ── Screens ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Opening,
    Main,
}

// ── File-tree filter ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
    All,
    ChangedOnly,
    PendingOnly,
    FailedAndError,
}

impl FilterMode {
    pub fn passes(self, far: &FileAuditResult) -> bool {
        match self {
            FilterMode::All => true,
            FilterMode::ChangedOnly => far.diff.diff_type != DiffType::Unchanged,
            FilterMode::PendingOnly => far.status == AuditStatus::Pending,
            FilterMode::FailedAndError => {
                matches!(far.status, AuditStatus::Failed | AuditStatus::Error)
            }
        }
    }
}

// ── Inspector validation (RFC 002) ────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct FieldError {
    /// Which inspector field triggered the error. Currently stored for
    /// future use in field-level focus/highlight; not yet read by view code.
    #[allow(dead_code)]
    pub field: String,
    pub message: String,
    /// RFC 028 — optional next-action hint. Rendered beneath
    /// `message` in a muted style. `None` for errors where the
    /// message is self-explanatory (e.g. "cannot be empty");
    /// `Some` when the corrective action isn't trivially inferable
    /// from the message text itself.
    pub hint: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct InspectorValidation {
    pub reason_error: Option<String>,
    pub strategy_errors: Vec<FieldError>,
    pub expires_at_error: Option<String>,
    pub pattern_error: Option<String>, // RFC 054: invalid/empty glob
}

impl InspectorValidation {
    pub fn can_approve(&self) -> bool {
        self.reason_error.is_none()
            && self.strategy_errors.is_empty()
            && self.expires_at_error.is_none()
            && self.pattern_error.is_none()
    }
}

// ── Inspector state ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct InspectorState {
    pub reason: String,
    /// RFC 012: index of the LineMatch rule currently in edit mode (None = all display mode)
    pub editing_rule: Option<usize>,
    /// RFC 009: multi-line text editor content backing the reason field.
    /// `reason` is kept in sync via `ReasonAction` handler.
    pub reason_content: iced::widget::text_editor::Content,
    pub strategy_kind: StrategyKind,
    pub strategy: AuditStrategy,
    pub note: String,
    pub validation: InspectorValidation, // RFC 002: replaces validation_error
    // Phase 3
    pub ticket: String,
    pub approved_by: String,
    pub expires_at_str: String,
    // RFC 054 — glob pattern override
    /// When true, `pattern_path` is used as the entry path instead of the diff path.
    pub use_pattern: bool,
    /// Editable path / glob; initialised to `far.diff.path` on selection.
    pub pattern_path: String,
    /// RFC 055 — auto-suggested glob chips derived from the diff path.
    pub pattern_suggestions: Vec<String>,
}

impl Default for InspectorState {
    fn default() -> Self {
        InspectorState {
            reason: String::new(),
            editing_rule: None,
            reason_content: iced::widget::text_editor::Content::new(),
            strategy_kind: StrategyKind::None,
            strategy: AuditStrategy::None,
            note: String::new(),
            validation: InspectorValidation::default(),
            ticket: String::new(),
            approved_by: String::new(),
            expires_at_str: String::new(),
            use_pattern: false,
            pattern_path: String::new(),
            pattern_suggestions: Vec::new(),
        }
    }
}
