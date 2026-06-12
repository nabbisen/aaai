# RFC 029 — FieldError i18n migration

**Status.** Implemented (v0.21.0 — Phase 13)
**Tracks.** GUI i18n parity, inspector validation surface
**Touches.** `crates/aaai-gui/src/app.rs` (4 validation sites),
`crates/aaai-gui/locales/{en,ja}.yaml` (4 new keys × 2 locales),
no test additions.

## Summary

Four `FieldError` construction sites in `app.rs`'s inspector
validation logic still carry **hardcoded English strings**:

```rust
message: "Must be exactly 64 hex characters.".into(),
message: "At least one rule is required.".into(),
message: "Rule line cannot be empty.".into(),
message: "Expected content cannot be empty.".into(),
```

This RFC migrates each to a `t!()` lookup against new i18n keys under
the established `error.inspector.*` namespace, bringing the inspector
to full i18n parity with the rest of the GUI.

## Why now

The hardcoded English strings are a hold-over from before the Phase 12
i18n hygiene work. They're the **only remaining user-facing strings in
`aaai-gui/src/app.rs`** that aren't routed through `t!()`. Japanese-locale
users currently see English error text when validation fires on
Checksum / LineMatch / Exact strategies, breaking the otherwise-complete
bilingual experience.

The change is mechanical:
- 4 string replacements at known line numbers
- 4 new keys × 2 locales = 8 YAML entries
- No struct changes, no render changes, no test changes
- `scripts/check-i18n-keys.py` audit goes from 0/0/0 to 0/0/0
  (4 new keys both referenced and defined)

## What this RFC does *not* do

- **Hint authoring.** RFC 028 introduced the `hint: Option<String>` slot
  on `FieldError`. This RFC keeps `hint: None` at all 4 migrated sites.
  Filling in actionable hints (e.g. "Generate a SHA-256 with `sha256sum`")
  is a content-design concern that benefits from UX review and
  belongs to a separate RFC.
- **`expires_at` parse error.** This site reads `message: e` where `e`
  is a `chrono::ParseError`-derived string. Routing chrono errors through
  i18n requires either error-code mapping or per-locale parsing of the
  variant — neither in scope. The user-visible text is the only
  remaining non-i18n string in `app.rs` after this RFC.
- **Regex compile error.** Already i18n'd via
  `UserError::from_i18n("error.inspector.invalid_regex")` (RFC 026).

## External design

### Four new i18n keys

Following the established `error.<surface>.<short_id>.message`
convention from RFC 020 / 026:

```yaml
# en.yaml
error:
  inspector:
    invalid_sha256:
      message: "Must be exactly 64 hex characters."
    empty_rules:
      message: "At least one rule is required."
    empty_rule_line:
      message: "Rule line cannot be empty."
    empty_expected:
      message: "Expected content cannot be empty."

# ja.yaml
error:
  inspector:
    invalid_sha256:
      message: "ちょうど 64 文字の 16 進数である必要があります。"
    empty_rules:
      message: "少なくとも 1 つのルールが必要です。"
    empty_rule_line:
      message: "ルールの行を空にできません。"
    empty_expected:
      message: "期待する内容を空にできません。"
```

These follow the same shape (`.message` leaf only — no `.hint`) as
hint-less errors deserve. Adding `.hint` keys later is non-breaking;
the audit script's `from_i18n` recognition is unaffected because
these sites use direct `t!()` lookup, not `from_i18n`.

### No `from_i18n` indirection

The migrated sites use direct `t!()` calls because they don't have
hints:

```rust
message: t!("error.inspector.invalid_sha256.message").to_string(),
```

Not:

```rust
let err = UserError::from_i18n("error.inspector.invalid_sha256");
// would expect prefix.message AND prefix.hint
```

This keeps the audit script's static-analysis honest — it sees the
direct key reference and doesn't have to infer a `.hint` companion
that doesn't exist.

## Internal design

### Call-site updates (4)

| File | Line | Strategy | New key |
|---|---|---|---|
| `app.rs` | ~1457 | Checksum hex format | `error.inspector.invalid_sha256.message` |
| `app.rs` | ~1465 | LineMatch empty rules | `error.inspector.empty_rules.message` |
| `app.rs` | ~1473 | LineMatch empty rule line | `error.inspector.empty_rule_line.message` |
| `app.rs` | ~1503 | Exact empty content | `error.inspector.empty_expected.message` |

Each replaces `"<string>".into()` with `t!("<key>").to_string()`.

### Locale entries (8)

Both `en.yaml` and `ja.yaml` gain 4 new entries under
`error.inspector.*`. The total locale key count rises from 123 to
**127** per locale.

### Audit script

No changes needed. The `t!("error.inspector.X.message")` calls are
static literals and get picked up by the existing dotted-key regex.

## Testing

No new unit tests. The struct shape doesn't change, no new helpers
are added, and the existing render path tests (`field_error_*` from
RFC 028) continue to pass unchanged.

`scripts/check-i18n-keys.py --quiet` is the primary acceptance check
— it must remain `0 missing / 0 divergent / 0 unused` with the new
counts `127 / 127 / 127`.

## Acceptance criteria

- [ ] 4 hardcoded English strings in `app.rs` replaced with `t!()` calls
- [ ] 4 new keys defined in `en.yaml` under `error.inspector.*`
- [ ] 4 parallel keys defined in `ja.yaml` with structurally-equivalent
      Japanese translations
- [ ] `scripts/check-i18n-keys.py --quiet` returns 0/0/0 (127/127/127)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All existing tests pass (97 / 70 / 11)
- [ ] CHANGELOG entry under `[Unreleased]`

## Open questions

None. Possible follow-up work this RFC enables but doesn't include:

- **`expires_at` chrono parse error i18n.** Requires a translation
  table keyed by `chrono::format::ParseErrorKind`. Out of scope.
- **Hint authoring** for the 4 migrated sites (and the regex one
  already done). Content-design concern; future RFC.
- **Other crates' user-facing strings.** `aaai-core` and `aaai-cli`
  emit some user-visible strings; whether to i18n those is a
  separate architectural decision (CLI tools often stay
  English-only since they target CI/CD and shell pipelines).
