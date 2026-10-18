use super::*;

fn summary(ok: usize, pending: usize, failed: usize, error: usize) -> AuditSummary {
    AuditSummary {
        total: ok + pending + failed + error,
        ok,
        pending,
        failed,
        ignored: 0,
        error,
        warning_count: 0,
    }
}

#[test]
fn empty_summary_returns_none() {
    assert!(next_action_hint(&summary(0, 0, 0, 0)).is_none());
}

#[test]
fn pending_only_suggests_filling_reasons() {
    let h = next_action_hint(&summary(5, 3, 0, 0)).unwrap();
    assert!(h.contains("3 Pending entries"));
    assert!(h.contains("re-run"));
    assert!(!h.contains("Failed"));
}

#[test]
fn one_pending_uses_singular_entry() {
    let h = next_action_hint(&summary(5, 1, 0, 0)).unwrap();
    assert!(h.contains("1 Pending entry"), "got: {h}");
    assert!(!h.contains("entries"));
}

#[test]
fn failed_takes_priority_over_pending() {
    let h = next_action_hint(&summary(5, 3, 2, 0)).unwrap();
    assert!(h.contains("2 Failed entries"));
    assert!(!h.contains("Pending"));
}

#[test]
fn one_failed_uses_singular_entry() {
    let h = next_action_hint(&summary(0, 0, 1, 0)).unwrap();
    assert!(h.contains("1 Failed entry"));
}

#[test]
fn error_only_suggests_checking_access() {
    let h = next_action_hint(&summary(5, 0, 0, 2)).unwrap();
    assert!(h.contains("2 Errors"));
    assert!(h.contains("file access"));
}

#[test]
fn one_error_uses_singular() {
    let h = next_action_hint(&summary(0, 0, 0, 1)).unwrap();
    assert!(h.contains("1 Error "), "got: {h}");
}

#[test]
fn all_clean_suggests_report() {
    let h = next_action_hint(&summary(10, 0, 0, 0)).unwrap();
    assert!(h.contains("aaai report"));
}
