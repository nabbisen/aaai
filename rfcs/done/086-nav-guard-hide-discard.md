# RFC 086 — Navigation guard: hide "Discard and leave"

**Status.** Implemented (v0.32.0 — Phase 24)
**Tracks.** Plain-Language GUI (review §9)
**Touches.** `crates/aaai-gui/src/views/nav_guard.rs`,
`crates/aaai-gui/src/app.rs` (state + message), `locales/{en,ja}.yaml`.

The data-losing "Discard and leave" action is now hidden behind a
"More choices" link. Default dialog shows only safe choices: Stay here |
Save and leave. Cancel renamed to "Stay here". The safest action is the
easiest action.
