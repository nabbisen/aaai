# RFC 061 — `aaai check` activation

**Status.** Implemented (v0.26.0 — Phase 18)
**Touches.** `crates/aaai-cli/src/cmd/check.rs` (stub, no changes needed),
`crates/aaai-cli/src/tests.rs` (2 tests).

Validates a definition file: YAML structure, entry approvability,
expired/expiring-soon detection. `--all` shows every entry. Exits 4
on YAML parse failure; exits 1 on invalid entries.
