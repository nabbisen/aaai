# RFC 064 — GUI `suggest_patterns` unit tests

**Status.** Implemented (v0.26.0 — Phase 18)
**Touches.** `crates/aaai-gui/src/app.rs` (5 new unit tests).

Unit tests for the RFC 055 glob-suggestion algorithm:
depth-2, depth-3, no-extension, single-component, empty string.
aaai-gui tests: 15 → 20.
