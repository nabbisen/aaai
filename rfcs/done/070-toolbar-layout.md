# RFC 070 — Toolbar layout stability and Undo relocation

**Status.** Implemented (v0.29.0 — Phase 20)
**Tracks.** GUI & UI/UX Quality
**Touches.** `crates/aaai-gui/src/views/main_view.rs` (build_toolbar, build_filter_bar).

Saved/reported marks now stack below buttons (stable width). Undo moved from
filter bar to toolbar. Icon glyphs clarified: Open=←, Save=↓, Export=↑, Undo=↩.
