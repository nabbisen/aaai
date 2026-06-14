# RFC 078 — Fix stale □ Open icon reference

**Status.** Implemented (v0.31.0 — Phase 22)
**Touches.** `locales/{en,ja}.yaml` — `empty_state.diff_no_audit_step2`.

`□ Open` → `← Open` in the diff empty-state hint (RFC 070 changed the
icon but the i18n string was not updated).
