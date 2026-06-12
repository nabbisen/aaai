# RFC 066 — AuditDefinition direct unit tests

**Status.** Implemented (v0.27.0 — Phase 19)
**Touches.** `crates/aaai-core/src/config/definition.rs` (10 new unit tests).

Direct tests for the methods underlying the glob feature (RFC 054) and
expiry enforcement (RFC 044): `find_entry` exact/glob, `is_glob`,
`glob_matches` depth/extension patterns, `upsert_entry` updates-in-place,
`expired_entries`, `expiring_soon`, `is_approvable` reason validation.
aaai-core tests: 101 → 111.
