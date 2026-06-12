# RFC 030 — FieldError hint authoring (selective)

**Status.** Implemented (v0.21.0 — Phase 13)
**Tracks.** GUI error UX, content design
**Touches.** `crates/aaai-gui/src/app.rs` (2 of the 4 RFC 029
sites get hint upgrades), `crates/aaai-gui/locales/{en,ja}.yaml`
(2 new `.hint` keys × 2 locales).

## Summary

RFC 028 introduced `FieldError.hint: Option<String>`. RFC 029
migrated 4 hardcoded validation messages to i18n keys but kept
all four with `hint: None` to stay focused on the structural /
i18n work.

This RFC closes the message-plus-hint loop by filling hints on
the **subset of sites where the message alone doesn't tell the
user what to do**. Two of the four sites qualify:

- **Checksum hex format** (`invalid_sha256`) — "Must be 64 hex
  characters" tells the user *what's wrong* but not *how to fix
  it*. Users unfamiliar with SHA-256 need to know where the
  value comes from.
- **LineMatch empty rules** (`empty_rules`) — "At least one rule
  is required" tells what's wrong but a new user may not realise
  there's an `+ Add rule` button to click.

The other two sites (`empty_rule_line`, `empty_expected`) keep
`hint: None`: their messages already point at the action ("type
something in this field").

## Why selective, not comprehensive

Phase 12 / 13 established the `message + hint` pattern. The
temptation now is to fill every hint slot. Resist:

- A hint that just rephrases the message adds visual noise
  without adding information.
- The "two-line error" shape becomes diluted when half the hints
  are trivial. Users learn to skip the second line.
- Maintenance cost: every line of UI copy needs translation,
  reviewing, and updating when behaviour shifts.

The rule of thumb (also documented in RFC 028 §3): **add a hint
when the cause is non-obvious or the corrective action isn't
trivially inferable from the message.**

By that rule:

| Site | Message states the action? | Hint? |
|---|---|---|
| `invalid_sha256` | No (assumes SHA-256 knowledge) | **Yes** |
| `empty_rules` | Implicitly (the "rule" word may be opaque) | **Yes** |
| `empty_rule_line` | Yes ("line cannot be empty" → type a line) | No |
| `empty_expected` | Yes ("content cannot be empty" → type content) | No |

## External design

### New i18n keys (2 × 2 locales = 4 entries)

```yaml
# en.yaml
error:
  inspector:
    invalid_sha256:
      hint: "Generate one with `sha256sum filename`, or copy the value from an earlier audit run."
    empty_rules:
      hint: "Click the `+ Add rule` button below to define what line changes are allowed."

# ja.yaml
error:
  inspector:
    invalid_sha256:
      hint: "`sha256sum filename` で生成するか、過去の監査実行結果から値をコピーしてください。"
    empty_rules:
      hint: "下の `+ ルール追加` ボタンをクリックして、許容する行変更を定義してください。"
```

The hint text refers to **actual UI labels** that exist in the
locale (`inspector.add_rule`), so it stays consistent if the
button label is changed (the hint and the button would migrate
together). Both locales reference the same UI element by its
displayed label, in matching font.

### Call-site updates (2)

The two upgraded sites switch from direct `t!()` to
`UserError::from_i18n("prefix")`, since both `.message` and
`.hint` keys now exist:

```rust
// Before (RFC 029):
v.strategy_errors.push(FieldError {
    field: "expected_sha256".into(),
    message: t!("error.inspector.invalid_sha256.message").to_string(),
    hint: None,
});

// After (RFC 030):
let err = crate::error::UserError::from_i18n("error.inspector.invalid_sha256");
v.strategy_errors.push(FieldError {
    field: "expected_sha256".into(),
    message: err.message,
    hint: Some(err.hint),
});
```

Same pattern as the existing `invalid_regex` site (RFC 028 §3).

### Unchanged sites (2)

- `empty_rule_line` — continues using
  `t!("error.inspector.empty_rule_line.message")`, `hint: None`.
- `empty_expected` — same shape.

The `from_i18n` audit-script recognition (RFC 026) is unaffected:
each upgraded site adds explicit `from_i18n("prefix")` calls and
the script automatically counts both `.message` and `.hint` as
referenced.

## Internal design

### YAML add only (no key restructure)

The 4 existing `.message` keys from RFC 029 stay untouched. The 2
new `.hint` siblings are added under the same paths. The script
`scripts/check-i18n-keys.py` continues to handle this naturally:
seeing `UserError::from_i18n("error.inspector.invalid_sha256")`
means both `error.inspector.invalid_sha256.message` and
`error.inspector.invalid_sha256.hint` are referenced; the YAML
now provides both.

### Locale counts

| Pre-RFC | Post-RFC |
|---|---|
| 127 / 127 / 127 | **129 / 129 / 129** (+2 keys × 1 dimension each) |

## Testing

No new unit tests. The structural shape of `FieldError` is
unchanged from RFC 028; the only addition is two new YAML entries
and a call-site syntax change. Existing tests (97 / 70 / 11) all
continue to pass.

Audit script must report 0/0/0 after the change. mdbook smoke
test continues to pass.

## Acceptance criteria

- [ ] 2 new `.hint` i18n keys added in en.yaml (and parallel ja.yaml)
- [ ] `invalid_sha256` call-site refactored to use `from_i18n`
      and pass `hint: Some(err.hint)`
- [ ] `empty_rules` call-site likewise
- [ ] `empty_rule_line` and `empty_expected` sites **unchanged**
      (no hint creep)
- [ ] `scripts/check-i18n-keys.py --quiet` returns 0/0/0
      (129/129/129)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] CHANGELOG entry under `[Unreleased]`

## Open questions

None at acceptance. Future scope:

- **Drift maintenance.** If `inspector.add_rule`'s displayed
  label changes in a future RFC, the `empty_rules.hint` text
  must be reviewed in lockstep. Adding a CI lint that scans for
  "consistency between hint text and referenced UI labels" is a
  possible follow-up but probably over-engineering for v1.0.
- **Other hint sites.** As future error surfaces are added
  (DiffFailed, export failure, profile delete, etc.), each will
  need its own selective hint judgment using the rule of thumb
  from RFC 028 §3.
