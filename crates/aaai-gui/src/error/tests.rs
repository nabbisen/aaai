use super::*;

#[test]
fn new_holds_both_fields() {
    let e = UserError::new("what", "what next");
    assert_eq!(e.message, "what");
    assert_eq!(e.hint, "what next");
}

#[test]
fn from_i18n_resolves_message_and_hint() {
    // Set the locale to a known value to avoid test inter-dependency.
    rust_i18n::set_locale("en");
    // Use a prefix whose two derived keys exist in locales/en.yaml.
    // The literal prefix is intentionally split across `let` bindings
    // so the audit script doesn't treat the prefix itself as a
    // referenced key — only the two derived keys count.
    let prefix = ["error", "save", "failed"].join(".");
    let e = UserError::from_i18n(&prefix);
    // Fields are populated and not the literal key (which would
    // indicate a missing translation lookup).
    assert!(!e.message.contains(&prefix));
    assert!(!e.hint.contains(&prefix));
    assert!(!e.message.is_empty());
    assert!(!e.hint.is_empty());
}
