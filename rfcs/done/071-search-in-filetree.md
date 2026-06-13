# RFC 071 — Search bar moved inside file tree pane

**Status.** Implemented (v0.29.0 — Phase 20)
**Tracks.** GUI & UI/UX Quality
**Touches.** `crates/aaai-gui/src/views/main_view.rs` (build_file_tree, view).

Search bar is now the header of the file tree pane instead of a full-width
row above the entire grid. Scoped to the pane it operates on.
