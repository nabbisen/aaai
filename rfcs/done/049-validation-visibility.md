# RFC 049 — Inspector validation visibility + Approvals file placeholder

**Status.** Implemented (v0.24.0)
**Tracks.** RFC 048 follow-up, progressive disclosure correctness
**Touches.** `crates/aaai-gui/src/views/inspector.rs` (auto-expand on error),
`crates/aaai-gui/src/views/opening.rs` (placeholder),
`crates/aaai-gui/locales/{en,ja}.yaml` (1 new key × 2).

## Bugs and gaps addressed

### A — Validation error orphaned behind collapsed toggle

RFC 048 hides `expires_at` in the `▸ More options` section. If a
loaded entry has a malformed `expires_at` (e.g. a hand-edited audit.yaml
with `expires_at: "oops"`), the validation block renders
*outside* the advanced section while the field is hidden inside it.
The user sees:

```
Reason  [—]
Strategy  [None ▼]
▸ More options
✕ Invalid date format.   ← error is orphaned — field invisible
```

**Fix:** derive `effective_advanced_expanded` from the logical OR of
the user-toggle and the presence of any advanced-field error:

```rust
let effective_advanced_expanded =
    app.advanced_inspector_expanded
    || ins.validation.expires_at_error.is_some();
```

This ensures the field is always visible when it has an error.

### B — Approvals file text input has no placeholder

`file_picker_row` is called with an empty placeholder string `""`.
When the user expands Optional settings and sees the Approvals file
field, they see a blank input box with no guidance. They may not know:
- What format the path should be
- Whether the file must already exist
- What happens if they leave it empty

**Fix:** pass a localised placeholder through `file_picker_row`
and set a helpful hint for the Approvals file specifically:

```
"Leave empty to be prompted on first Save"
```

This is shown only while the field is empty (standard placeholder
behaviour). When a path is already loaded, the path is shown instead.

## i18n (1 new key × 2 locales)

```yaml
# en.yaml
opening:
  definition_placeholder: "Leave empty to be prompted on first Save"

# ja.yaml
opening:
  definition_placeholder: "空欄の場合は初回保存時に保存先を選択します"
```

## Code changes

### `inspector.rs`

```rust
// One-line fix:
let effective_advanced_expanded = app.advanced_inspector_expanded
    || ins.validation.expires_at_error.is_some();
// Replace all `advanced_expanded` uses with `effective_advanced_expanded`
```

### `opening.rs`

```rust
// file_picker_row now accepts a placeholder:
fn file_picker_row<'a, F>(
    label: String,
    placeholder: &'a str,   // ← new
    current: &'a str,
    pick_msg: Message,
    on_text_change: F,
)

// At call site for definition_path:
file_picker_row(
    t!("opening.definition_label").to_string(),
    &t!("opening.definition_placeholder"),   // ← new
    &app.definition_path,
    Message::PickDefinitionFile,
    Message::DefinitionPathChanged,
)
```

## Acceptance criteria

- [ ] `effective_advanced_expanded = toggle || expires_at_error.is_some()`
- [ ] Advanced section auto-opens when there is an expires_at error
- [ ] `file_picker_row` accepts a `placeholder` parameter
- [ ] Approvals file shows placeholder text when empty
- [ ] 1 new i18n key in en.yaml + ja.yaml
- [ ] `check-i18n-keys.py --quiet` → 0/0/0 (217/217/217)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All tests pass (101 / 70 / 15)
