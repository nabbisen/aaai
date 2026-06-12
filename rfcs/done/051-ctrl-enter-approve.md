# RFC 051 — Ctrl+Enter keyboard shortcut for approval

**Status.** Implemented (v0.24.0)
**Tracks.** Keyboard-driven workflow, RFC 050 follow-up
**Touches.** `crates/aaai-gui/src/app.rs` (keyboard handler),
`crates/aaai-gui/src/views/help_overlay.rs` (shortcuts table),
`crates/aaai-gui/locales/{en,ja}.yaml` (1 new key × 2).

## Summary

With RFC 050's auto-advance in place, the approval loop became:

```
type reason → [click Approve & Save] → auto-advances → type reason → …
```

The mouse click to submit is the remaining friction. `Ctrl+Enter` (standard
"submit form" convention in editors and IDEs) removes it. The Reason textarea
already has focus after each auto-advance, so the full keyboard loop is:

```
type reason → Ctrl+Enter → type reason → Ctrl+Enter → …
```

The keyboard handler inserts the new case *before* the plain `Enter` case,
so `Ctrl+Enter` triggers approval while plain `Enter` still focuses the
Reason field. The reason text is trimmed on approval, so any accidental
newline from the text_editor's own Enter processing is harmless.

## Changes

- `Ctrl+Enter` → `Message::ApproveAndSave` in the main-screen keyboard handler
- New shortcut row in the `?` help overlay
- 1 new i18n key: `help.approve_and_save`
- Total: 218/218/218
