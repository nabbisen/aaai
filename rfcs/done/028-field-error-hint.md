# RFC 028 — FieldError native `hint` field

**Status.** Implemented (v0.21.0 — Phase 13)
**Tracks.** GUI error UX, inspector validation surface
**Touches.** `crates/aaai-gui/src/app.rs` (FieldError struct + 1 site),
`crates/aaai-gui/src/views/inspector.rs` (render path),
`crates/aaai-gui/src/style.rs` (muted hint style), tests.

## Summary

The `FieldError` struct currently carries a single `message: String`
field. RFC 026 needed to surface a `message + hint` pair on the
inspector's regex validation site, and worked around the missing
field by composing both lines into `message` with a `💡` separator:

```rust
message: format!("{} ({}) 💡 {}", err.message, e, err.hint),
```

This RFC adds a native `hint: Option<String>` field to `FieldError`
so the hint can render in its own muted style beneath the message
— matching the visual language already established for `UserError`
banners (RFC 020) and now toasts (RFC 026).

Once the field exists, the `💡` composition from RFC 026 §4 goes
away in favour of structural separation.

## Why now

RFC 020 set the architectural pattern: errors have two parts —
*what happened* and *what to do next* — and the UI should render
them as separate visual layers. `UserError` carries that pair
directly. Toasts carry it via `push_toast_with_hint`'s body
formatting plus the `💡` marker (RFC 026). The inspector's
inline `FieldError` is the only error surface still without
structural hint support.

The composition workaround in RFC 026 works but leaks a
display-layer concern (the `💡` marker, the parenthesis around the
concrete error) into the model layer (the `FieldError.message`
string). Untangling this now keeps the model clean as more
validation sites grow hint support.

The change is also small — six construction sites total, five of
which pass `hint: None` and need no further work.

## External design

### Updated `FieldError`

```rust
#[derive(Debug, Clone)]
pub struct FieldError {
    pub field: String,
    pub message: String,
    /// Optional next-action hint. Rendered beneath `message` in a
    /// muted style. `None` for errors where the message is
    /// self-explanatory (e.g. "cannot be empty").
    pub hint: Option<String>,
}
```

The `#[allow(dead_code)]` annotation currently on `FieldError` is
preserved as-is; nothing about its read-side surface changes
(the existing `field` and `message` fields stay public).

### Render contract

When `hint` is `Some(s)`:
- Message renders on the first line in the error color
- Hint renders beneath, indented slightly, in a muted grey
- No `💡` prefix on the hint — the structural separation is the
  signal, not an emoji

When `hint` is `None`:
- Render is identical to today — single-line message

### Migration of the RFC 026 composition

Before (RFC 026):
```rust
let err = crate::error::UserError::from_i18n("error.inspector.invalid_regex");
v.strategy_errors.push(FieldError {
    field: "pattern".into(),
    message: format!("{} ({}) 💡 {}", err.message, e, err.hint),
});
```

After (RFC 028):
```rust
let err = crate::error::UserError::from_i18n("error.inspector.invalid_regex");
v.strategy_errors.push(FieldError {
    field: "pattern".into(),
    message: format!("{} ({})", err.message, e),
    hint: Some(err.hint),
});
```

The concrete regex parser error `e` stays appended to the message
(it's part of "what happened" — the specific syntactic problem).
The actionable hint moves into its own field.

## Internal design

### Other five construction sites

All five existing `FieldError { … }` literals pick up `hint: None`:

| File | Line | Site | hint |
|---|---|---|---|
| `app.rs` | ~723 | expires_at parse error | `None` |
| `app.rs` | ~1449 | LineMatch validation | `None` |
| `app.rs` | ~1457 | LineMatch validation | `None` |
| `app.rs` | ~1464 | Checksum validation | `None` |
| `app.rs` | ~1488 | Exact validation | `None` |

These are all short "cannot be empty" / "invalid format" messages
where a hint would just repeat the message. Adding hints to them
is out of scope for this RFC — it would require designing
"what does the user actually do?" copy for each, and that's
better paired with i18n migration (a separate RFC entirely).

The structural rule of thumb is: **add a hint when the cause is
non-obvious or the corrective action isn't trivially inferable
from the message.**

### Render path location

`crates/aaai-gui/src/views/inspector.rs` renders the
`InspectorValidation` errors. The change is local to the
function that formats a `FieldError` for display — one match arm
or one `if let Some(hint)` branch.

### Style

A new helper in `crates/aaai-gui/src/style.rs`:

```rust
pub fn field_error_hint_color(theme: &Theme) -> Color {
    // ~60% opacity of the standard text color, matching the
    // muted style used for banner hints and toast body's second
    // line. Distinguishable from the message above (which uses
    // the error color) without competing visually for attention.
    ...
}
```

The exact value follows the existing palette — RFC 020's banner
hint already established the convention.

## Testing

- Unit test on `FieldError` construction with hint = `Some(...)`
  verifying both fields are populated
- Unit test on construction with hint = `None` verifying it doesn't
  panic and behaves identically to today
- `cargo test -p aaai-gui` continues to pass (9 → 11 tests after
  this RFC's additions)

No visual test — the render-side change is checked manually during
operator verification.

## Acceptance criteria

- [ ] `FieldError` gains `hint: Option<String>` field
- [ ] All 6 construction sites updated (1 with `Some`, 5 with `None`)
- [ ] The RFC 026 `💡 {hint}` composition removed from the regex
      validation site
- [ ] Inspector render path renders hint when `Some`, identical to
      today when `None`
- [ ] 2 new unit tests
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] `scripts/check-i18n-keys.py` continues to return 0/0/0
- [ ] CHANGELOG entry under `[Unreleased]`

## Open questions

None at acceptance. Future work that this RFC enables but doesn't
include:

- **i18n migration of the other five FieldError messages.** They're
  currently hardcoded English. A separate RFC could resolve them
  through `t!()` keys; this RFC just makes the structural slot
  available for when that happens.
- **Hint authoring sweep.** Once the slot exists, a future RFC
  could fill in hints for the four `None` sites where the
  corrective action is non-obvious (e.g. the Checksum format
  validation could hint at "use SHA-256 hex, 64 lowercase chars").
