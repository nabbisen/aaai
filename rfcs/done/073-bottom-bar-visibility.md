# RFC 073 — Bottom bar hidden when no file selected

**Status.** Implemented (v0.29.0 — Phase 20)
**Tracks.** GUI & UI/UX Quality
**Touches.** `crates/aaai-gui/src/views/main_view.rs` (build_bottom_bar).

Bottom bar hidden when selected_index is None. Only appears when there
is an actionable file selection.
