//! Small utility helpers for view layers.

use chrono::{DateTime, Utc};
use rust_i18n::t;

/// Render a UTC timestamp as a short human-readable "time ago" string,
/// suitable for the Recent-projects list per RFC 023 §3.4.
///
/// Buckets:
///   - `< 60 seconds`  → "Just now"
///   - `< 60 minutes`  → "N min ago"
///   - `< 24 hours`    → "N h ago"
///   - `< 7 days`      → "N d ago"
///   - older           → absolute date `YYYY-MM-DD`
///
/// The relative buckets resolve through i18n (`relative.*`); the absolute
/// date is locale-independent (ISO-style is unambiguous and stays inside
/// the chrono `format!` macro, so no i18n key is needed).
///
/// `now` is taken from the system clock at call time. For unit-testing
/// without a clock dependency, use [`humanize_since_at`].
pub fn humanize_since(t: DateTime<Utc>) -> String {
    humanize_since_at(t, Utc::now())
}

/// Inner form of [`humanize_since`] that takes an explicit "now" so tests
/// don't depend on the real clock.
pub fn humanize_since_at(t: DateTime<Utc>, now: DateTime<Utc>) -> String {
    let delta = now - t;

    // Future timestamps (clock skew or test data) fall through to "Just now"
    // rather than emitting a confusing negative count.
    if delta.num_seconds() < 60 {
        return t!("relative.just_now").to_string();
    }
    if delta.num_minutes() < 60 {
        return t!("relative.minutes_ago", n = delta.num_minutes().to_string()).to_string();
    }
    if delta.num_hours() < 24 {
        return t!("relative.hours_ago", n = delta.num_hours().to_string()).to_string();
    }
    if delta.num_days() < 7 {
        return t!("relative.days_ago", n = delta.num_days().to_string()).to_string();
    }
    t.format("%Y-%m-%d").to_string()
}

// ── LocalizedOption (RFC 033) ─────────────────────────────────────────────

/// Pairs a Rust enum variant with its localized display label for use as a
/// `pick_list` option. The variant is the canonical identity; the label is
/// the human-readable form rendered for the current locale.
///
/// The `PartialEq` implementation compares **by `value` only**, not by
/// label. This is the key trick that makes pick_list selection work
/// across locales: the picker uses equality to identify "the currently
/// selected option," and selecting by enum value rather than label
/// means changing the locale (or the label text) doesn't break selection
/// identity.
///
/// Use this in two places per picker:
/// 1. Build a `Vec<LocalizedOption<T>>` of options with localized labels
/// 2. Send the `LocalizedOption<T>` to `pick_list`; in the callback,
///    extract `o.value` and dispatch to a Message variant carrying `T`.
#[derive(Debug, Clone)]
pub struct LocalizedOption<T: Clone + PartialEq> {
    pub value: T,
    pub label: String,
}

impl<T: Clone + PartialEq> std::fmt::Display for LocalizedOption<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.label)
    }
}

impl<T: Clone + PartialEq> PartialEq for LocalizedOption<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Clone + PartialEq> Eq for LocalizedOption<T> {}

// ── StrategyKind (RFC 035) ────────────────────────────────────────────────

use aaai::config::definition::{AuditStrategy, RegexTarget};

/// Discriminator for `AuditStrategy` variants without their associated data.
/// Used as the value type for the strategy picker via
/// `LocalizedOption<StrategyKind>`.
///
/// This is a GUI-layer concern (display + Message protocol identity).
/// `aaai-core` continues to expose only `AuditStrategy`; this discriminator
/// stays inside the GUI's display layer.
///
/// The variants mirror `AuditStrategy`'s variants one-for-one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrategyKind {
    None,
    Checksum,
    LineMatch,
    Regex,
    Exact,
}

impl StrategyKind {
    /// Construct a zero-value `AuditStrategy` for this kind.
    /// Used when the picker selects a new kind — the inspector
    /// state's strategy is replaced with a fresh default of
    /// that variant (e.g. an empty `expected_sha256`, no rules).
    pub fn to_default_strategy(self) -> AuditStrategy {
        match self {
            StrategyKind::None => AuditStrategy::None,
            StrategyKind::Checksum => AuditStrategy::Checksum {
                expected_sha256: String::new(),
            },
            StrategyKind::LineMatch => AuditStrategy::LineMatch { rules: Vec::new() },
            StrategyKind::Regex => AuditStrategy::Regex {
                pattern: String::new(),
                target: RegexTarget::AddedLines,
            },
            StrategyKind::Exact => AuditStrategy::Exact {
                expected_content: String::new(),
            },
        }
    }

    /// Read the kind from an existing strategy.
    pub fn from_strategy(s: &AuditStrategy) -> StrategyKind {
        match s {
            AuditStrategy::None => StrategyKind::None,
            AuditStrategy::Checksum { .. } => StrategyKind::Checksum,
            AuditStrategy::LineMatch { .. } => StrategyKind::LineMatch,
            AuditStrategy::Regex { .. } => StrategyKind::Regex,
            AuditStrategy::Exact { .. } => StrategyKind::Exact,
        }
    }

    /// Localised label for the picker.
    /// Resolves through `inspector.strategy_{none,checksum,linematch,regex,exact}`.
    pub fn label(self) -> String {
        match self {
            StrategyKind::None => t!("inspector.strategy_none"),
            StrategyKind::Checksum => t!("inspector.strategy_checksum"),
            StrategyKind::LineMatch => t!("inspector.strategy_linematch"),
            StrategyKind::Regex => t!("inspector.strategy_regex"),
            StrategyKind::Exact => t!("inspector.strategy_exact"),
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests;
