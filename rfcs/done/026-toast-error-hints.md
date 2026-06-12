# RFC 026 — Toast Error Hints & i18n Key Re-introduction

**Status.** Implemented (v0.21.0 — first of Phase 13)
**Tracks.** GUI error UX, i18n surface
**Touches.** `crates/aaai-gui/src/app.rs` (push_toast helpers, error
sites), `crates/aaai-gui/src/error.rs` (UserError integration),
`crates/aaai-gui/locales/{en,ja}.yaml` (4 new keys), tests.

## Summary

RFC 020 introduced the `message + hint` two-line error pattern and
applied it to the Opening screen's inline error banner. The same
pattern should apply to **toast notifications** — short-lived popups
shown for actions like save, export, profile load, regex validation.

Currently toast errors carry only a single body line (`message`) with
no hint. During the v0.20.0 dead-key sweep, this lack of a hint
surface forced the deletion of 4 i18n keys that were planned for the
toast path:

- `error.inspector.invalid_regex.message` / `.hint`
- `error.save.failed.message` / `.hint`

This RFC re-introduces those keys and codifies the message + hint
pattern for toasts via a small helper method on `App`. No changes to
the upstream `snora::Toast` type are needed — the existing `message`
field can carry both lines separated by a blank line.

## Why this matters

Today's toast errors are dead-ends:

> ❌ Save failed
> File system error: Permission denied (os error 13)

The user sees a raw `std::io::Error` debug string and is left to
figure out what to do. RFC 020's pattern would make it:

> ❌ Save failed
> Permission denied writing audit.yaml
>
> 💡 Check the file isn't open in another application, or pick a
> different output location.

The first line names *what* happened in user-facing terms; the
second names *what to do next*. Same pattern as the Opening banner.

For regex validation:

> Before: `Invalid regex: error parsing regex near position 3: unmatched`
> After:
>   Invalid pattern
>   The regular expression couldn't be parsed.
>
>   💡 Test your pattern at regex101.com to find the issue.

## External design

### New helper on `App`

```rust
impl App {
    /// Push a toast with a two-line body following RFC 020's
    /// message + hint pattern. The first line is the user-facing
    /// description of what happened; the second line is the
    /// suggested next action.
    pub fn push_toast_with_hint(
        &mut self,
        intent: ToastIntent,
        title: &str,
        message: &str,
        hint: &str,
    ) {
        let body = format!("{message}\n\n💡 {hint}");
        self.push_toast(intent, title, &body);
    }
}
```

The emoji marker `💡` distinguishes the hint visually without
requiring snora-level field separation. It degrades gracefully in
terminals (a fallback `Tip:` prefix is not needed — toasts only
render in the GUI).

### Existing `push_toast` retained for cases without a hint

Toasts for success/info events (`profile.saved`, `audit.rerun`)
keep using `push_toast` with no hint. Hints add cognitive load
when the message is purely informational.

### Integration with `UserError` (RFC 020)

```rust
impl App {
    pub fn push_user_error_toast(
        &mut self,
        intent: ToastIntent,
        title: &str,
        err: &UserError,
    ) {
        self.push_toast_with_hint(intent, title, &err.message, &err.hint);
    }
}
```

Lets call-sites that already constructed a `UserError` (e.g. when
building the Opening banner) reuse it for toast surfaces too.

## Internal design

### i18n keys to re-introduce

```yaml
# en.yaml
error:
  inspector:
    invalid_regex:
      message: "The regular expression couldn't be parsed."
      hint: "Test your pattern at regex101.com to find the issue."
  save:
    failed:
      message: "Couldn't write the audit definition file."
      hint: "Check the file isn't open in another application, or pick a different output location."

# ja.yaml
error:
  inspector:
    invalid_regex:
      message: "正規表現を解析できませんでした。"
      hint: "regex101.com でパターンをテストして問題を特定してください。"
  save:
    failed:
      message: "監査定義ファイルを書き込めませんでした。"
      hint: "別のアプリで開いていないか、または別の出力先を指定してください。"
```

These 4 keys (8 locale entries total) bring the locale file totals
from 119 → 123 per locale. The `check-i18n-keys.py` audit script
should report 0 missing / 0 divergent / 0 unused after the
refactor lands.

### Call-site changes

| Site | Before | After |
|---|---|---|
| `app.rs:1207` save_failed | `push_toast(Error, t!("toast.save_failed"), &e.to_string())` | `push_toast_with_hint(Error, t!("toast.save_failed"), t!("error.save.failed.message"), t!("error.save.failed.hint"))` |
| `app.rs:867` Err(e) save | same pattern as 1207 | same |
| `app.rs:861` profile save error | same | same |
| Inspector regex validation | currently emits inline FieldError with hint string baked into message | route through new key pair |

### What stays the same

- `push_toast` keeps its 4-arg signature unchanged
- Success / info / warning toasts don't change
- `snora::Toast<Message>` is untouched (it's an external crate at
  pinned version 0.8)
- The UserError struct (from RFC 020) is untouched

### Testing

- One unit test per new helper verifying the formatted body
  contains both lines and the `💡` marker
- `scripts/check-i18n-keys.py` continues to return 0/0/0
- `cargo test -p aaai-gui` builds & runs (test count unchanged
  since these are pure helpers exposed for internal use)

## Open questions

None at acceptance time. Possible future refinements:

- If snora 0.9+ adds a native `subtitle` field, drop the in-band
  `💡` marker and use the structural field instead. That's a
  follow-up RFC, not blocking.
- Other error sites (DiffFailed, profile delete failure, export
  failure) could adopt the same pattern. This RFC leaves them
  as-is to keep scope tight; a follow-up sweep can route them
  through `push_toast_with_hint` once we see the pattern shake
  out in real usage.

## Acceptance criteria

- [ ] `App::push_toast_with_hint` lands with unit test
- [ ] `App::push_user_error_toast` lands with unit test
- [ ] 4 i18n keys (8 locale entries) added with structurally-parallel
      en/ja translations
- [ ] At least 2 call-sites in `app.rs` use the new helper
      (the inspector regex one and at least one save_failed site)
- [ ] `scripts/check-i18n-keys.py --quiet` returns 0/0/0
- [ ] `cargo test --workspace` green
- [ ] CHANGELOG entry under `[Unreleased]` describes the addition
