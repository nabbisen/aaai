//! Next-action hint generator shared between `audit` and `dashboard`.
//!
//! RFC 024 §3.2 extracts the Zone 4 hint logic out of `audit.rs` so any CLI
//! surface that displays an [`AuditSummary`] can emit the same advice. The
//! function is pure: it returns the hint string (if any) without printing,
//! so the caller is responsible for
//!
//!   - suppressing it under `--quiet` / `--json-output` (audit does this by
//!     not calling the human-output path at all);
//!   - applying terminal colour (callers wrap the return value in
//!     `colored::Colorize::dimmed()` as appropriate).
//!
//! The hint is intentionally English-only for v1.0; CLI i18n is out of scope
//! per RFC 024 NFR-3.

use aaai::AuditSummary;

/// Suggested next user action given an audit summary.
///
/// Decision order:
///
///   1. If only Pending entries (no Failed) → ask for reasons.
///   2. Else if any Failed → ask to review and fix rules.
///   3. Else if any Error → ask to check file access.
///   4. Else (all OK / Ignored) → suggest generating a report.
///
/// Returns `None` only when the summary is empty (`total == 0`) — there is
/// no useful advice to give before any diff has run.
pub fn next_action_hint(s: &AuditSummary) -> Option<String> {
    if s.total == 0 {
        return None;
    }
    if s.pending > 0 && s.failed == 0 {
        return Some(format!(
            "Next: open your audit.yaml and fill in 'reason' for {} Pending {},\n      then re-run `aaai audit`.",
            s.pending,
            entries_word(s.pending),
        ));
    }
    if s.failed > 0 {
        return Some(format!(
            "Next: review {} Failed {} in the diff viewer,\n      update rules or reason, then re-run `aaai audit`.",
            s.failed,
            entries_word(s.failed),
        ));
    }
    if s.error > 0 {
        return Some(format!(
            "Next: check {} Error{} for file access issues.",
            s.error,
            s_word(s.error),
        ));
    }
    Some("Next: run `aaai report -o report.md` to generate a report.".to_string())
}

fn entries_word(n: usize) -> &'static str {
    if n == 1 { "entry" } else { "entries" }
}

fn s_word(n: usize) -> &'static str {
    if n == 1 { "" } else { "s" }
}

#[cfg(test)]
mod tests;
