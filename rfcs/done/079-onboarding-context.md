# RFC 079 — Opening onboarding: WHY context before HOW steps

**Status.** Implemented (v0.31.0 — Phase 22)
**Touches.** `crates/aaai-gui/src/views/opening.rs`,
`locales/{en,ja}.yaml` (+1 key: `empty_state.onboarding_context`).

Adds a two-sentence context paragraph before the numbered steps in
the first-run onboarding panel, explaining what the user will get
at the end of the process.
