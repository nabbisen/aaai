use super::*;

fn rfc098_path_issue_diff() -> aaai::DiffEntry {
    aaai::DiffEntry {
        path: "linked".into(),
        diff_type: aaai::DiffType::Incomparable,
        is_dir: false,
        before_text: None,
        after_text: None,
        is_binary: false,
        before_size: None,
        after_size: None,
        before_sha256: None,
        after_sha256: None,
        stats: None,
        error_detail: Some("[AAAI-PATH-LINK] Link-like entries are not followed.".into()),
    }
}

#[test]
fn rfc098_initial_failure_is_actionable_without_replacing_prior_data() {
    let mut app = App::default();
    app.diffs = vec![rfc098_path_issue_diff()];
    app.is_loading = true;
    let _ = app.update(Message::DiffFailed(
        "[AAAI-ROOT-UNAVAILABLE] Select a physical directory".into(),
    ));
    assert!(!app.is_loading);
    assert_eq!(app.diffs.len(), 1);
    let error = app
        .open_error
        .as_ref()
        .expect("root failure must be presented");
    assert!(error.message.contains("AAAI-ROOT-UNAVAILABLE"));
    assert!(!error.hint.is_empty());
}

#[test]
fn rfc098_rerun_failure_retains_prior_result_as_stale() {
    let mut app = App::default();
    app.diffs = vec![rfc098_path_issue_diff()];
    app.audit_dirty = true;
    app.is_loading = true;
    let _ = app.update(Message::RerunDiffReady(Err(
        "[AAAI-ROOT-UNAVAILABLE] Select a physical directory".into(),
    )));
    assert!(!app.is_loading);
    assert!(
        app.audit_dirty,
        "a failed rerun must not mark stale data current"
    );
    assert_eq!(app.diffs.len(), 1);
    assert!(
        app.toasts
            .iter()
            .any(|toast| toast.message.contains("AAAI-ROOT-UNAVAILABLE"))
    );
}

/// RFC 028 — verify the new `hint` field is populated when the
/// construction site passes `Some(...)`, alongside the existing
/// `field` and `message` fields.
#[test]
fn field_error_with_hint_holds_all_three_fields() {
    let fe = FieldError {
        field: "pattern".into(),
        message: "Pattern parse failed".into(),
        hint: Some("Test at regex101.com".into()),
    };
    assert_eq!(fe.field, "pattern");
    assert_eq!(fe.message, "Pattern parse failed");
    assert_eq!(fe.hint.as_deref(), Some("Test at regex101.com"));
}

/// RFC 028 — verify `hint: None` is a valid construction and
/// behaves identically to the pre-RFC-028 `FieldError` for
/// errors where a hint would just repeat the message
/// (e.g. "cannot be empty" validations).
#[test]
fn field_error_without_hint_remains_valid() {
    let fe = FieldError {
        field: "expected_content".into(),
        message: "Expected content cannot be empty.".into(),
        hint: None,
    };
    assert_eq!(fe.field, "expected_content");
    assert!(fe.hint.is_none());
}

// ── RFC 064 — suggest_patterns unit tests ────────────────────────

#[test]
fn rfc064_suggest_patterns_depth2() {
    let s = App::suggest_patterns("src/main.rs");
    assert!(
        s.contains(&"src/**".to_string()),
        "should suggest parent/**"
    );
    assert!(
        s.contains(&"**/*.rs".to_string()),
        "should suggest **/*.ext"
    );
}

#[test]
fn rfc064_suggest_patterns_depth3() {
    let s = App::suggest_patterns("node_modules/lodash/README.md");
    assert!(s.contains(&"node_modules/**".to_string()));
    assert!(s.contains(&"node_modules/**/*.md".to_string()));
    assert!(s.contains(&"**/*.md".to_string()));
    assert!(s.len() <= 3, "at most 3 suggestions");
}

#[test]
fn rfc064_suggest_patterns_no_extension() {
    let s = App::suggest_patterns("dist/output");
    // depth 2, no extension: only parent/**
    assert!(s.contains(&"dist/**".to_string()));
    assert!(
        !s.iter().any(|p| p.contains("**/*.")),
        "no ext-based chip when file has no extension"
    );
}

#[test]
fn rfc064_suggest_patterns_single_component() {
    let s = App::suggest_patterns("README.md");
    // single component (no /): no parent chip, only ext
    assert!(
        !s.iter().any(|p| p.contains("/**")),
        "no parent/** for single-component path"
    );
    assert!(s.contains(&"**/*.md".to_string()));
}

#[test]
fn rfc064_suggest_patterns_empty() {
    let s = App::suggest_patterns("");
    assert!(s.is_empty(), "empty path → no suggestions");
}
