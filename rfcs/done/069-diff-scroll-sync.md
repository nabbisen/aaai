# RFC 069 — Diff pane scroll synchronisation

**Status.** Implemented (v0.29.0 — Phase 20)
**Tracks.** GUI & UI/UX Quality
**Touches.** `crates/aaai-gui/src/views/diff_view.rs` (IDs + on_scroll),
`crates/aaai-gui/src/app.rs` (2 messages, handler, state field).

Side-by-side panes now scroll in lock-step via `iced::widget::operation::scroll_to`
and a `diff_scroll_syncing` guard that prevents infinite feedback.
