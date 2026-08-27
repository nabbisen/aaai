use super::*;
use chrono::TimeZone;

// rust_i18n is initialised by main.rs; tests run inside the same
// crate so the macro registry is available. We test the bucket
// dispatch logic by asserting which i18n key fires (presence of
// expected words in the output), not the exact wording — that's
// a translation concern.

fn t(year: i32, month: u32, day: u32, h: u32, m: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(year, month, day, h, m, 0).unwrap()
}

#[test]
fn within_a_minute_is_just_now() {
    let now = t(2026, 5, 13, 12, 0);
    let earlier = t(2026, 5, 13, 11, 59); // 60 s ago — boundary
    // Move 1 s into the "just now" side of the boundary (59 s ago,
    // not 61 s ago — the original `-1s` was a Phase 12 typo).
    let out = humanize_since_at(earlier + chrono::Duration::seconds(1), now);
    // Either translation; we just check the key resolved (no "min", no "h", no "d")
    assert!(!out.contains(" min "));
    assert!(!out.contains(" h "));
    assert!(!out.contains(" d "));
}

#[test]
fn minutes_bucket() {
    let now = t(2026, 5, 13, 12, 0);
    let earlier = t(2026, 5, 13, 11, 55); // 5 min ago
    let out = humanize_since_at(earlier, now);
    assert!(out.contains("5"), "expected '5' in output: {out}");
    // contains the "min" or Japanese-equivalent fragment via key resolution
}

#[test]
fn hours_bucket() {
    let now = t(2026, 5, 13, 12, 0);
    let earlier = t(2026, 5, 13, 9, 0); // 3 h ago
    let out = humanize_since_at(earlier, now);
    assert!(out.contains("3"), "expected '3' in output: {out}");
}

#[test]
fn days_bucket() {
    let now = t(2026, 5, 13, 12, 0);
    let earlier = t(2026, 5, 10, 12, 0); // 3 d ago
    let out = humanize_since_at(earlier, now);
    assert!(out.contains("3"), "expected '3' in output: {out}");
}

#[test]
fn beyond_a_week_falls_back_to_absolute_date() {
    let now = t(2026, 5, 13, 12, 0);
    let earlier = t(2026, 4, 1, 12, 0); // 42 d ago — well past a week
    let out = humanize_since_at(earlier, now);
    assert_eq!(
        out, "2026-04-01",
        "beyond 7 days should be an ISO date: {out}"
    );
}

#[test]
fn exactly_seven_days_is_already_absolute() {
    let now = t(2026, 5, 13, 12, 0);
    let earlier = t(2026, 5, 6, 12, 0); // 7 d ago exactly
    let out = humanize_since_at(earlier, now);
    assert_eq!(out, "2026-05-06");
}

#[test]
fn future_timestamp_does_not_panic() {
    let now = t(2026, 5, 13, 12, 0);
    let later = t(2026, 5, 13, 12, 5); // 5 min in the future
    let out = humanize_since_at(later, now);
    // Just confirm we get some string back rather than panic.
    assert!(!out.is_empty());
}

// ── LocalizedOption (RFC 033) ────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TestAction {
    A,
    B,
}

/// RFC 033 — verify equality compares by `value` alone, not by
/// `label`. This is the property that makes pick_list selection
/// work correctly when the locale changes: the picker's
/// "currently selected" identity is preserved across translations.
#[test]
fn localized_option_equality_ignores_label() {
    let a = LocalizedOption {
        value: TestAction::A,
        label: "Added".into(),
    };
    let b = LocalizedOption {
        value: TestAction::A,
        label: "追加".into(),
    };
    assert_eq!(a, b); // same value, different label
}

/// RFC 033 — verify inequality when values differ even if labels
/// happen to match. This is unlikely in practice (labels are
/// usually distinct) but the contract is: value determines identity.
#[test]
fn localized_option_inequality_by_value() {
    let a = LocalizedOption {
        value: TestAction::A,
        label: "X".into(),
    };
    let b = LocalizedOption {
        value: TestAction::B,
        label: "X".into(),
    };
    assert_ne!(a, b); // different value, same label
}

// ── StrategyKind (RFC 035) ───────────────────────────────────────

/// RFC 035 — verify the discriminator round-trips through
/// `to_default_strategy()` and back via `from_strategy()`.
/// This is the key contract for the picker: selecting a kind
/// produces a strategy whose kind matches.
#[test]
fn strategy_kind_roundtrips_through_strategy() {
    for kind in [
        StrategyKind::None,
        StrategyKind::Checksum,
        StrategyKind::LineMatch,
        StrategyKind::Regex,
        StrategyKind::Exact,
    ] {
        let strategy = kind.to_default_strategy();
        assert_eq!(
            StrategyKind::from_strategy(&strategy),
            kind,
            "round-trip failed for {kind:?}"
        );
    }
}

/// RFC 035 — `AuditStrategy::default()` is `None`; verify our
/// discriminator agrees with that.
#[test]
fn strategy_kind_default_is_none() {
    let default_strategy = AuditStrategy::default();
    assert_eq!(
        StrategyKind::from_strategy(&default_strategy),
        StrategyKind::None
    );
}
