# RFC 059 — `aaai lint` activation

**Status.** Implemented (v0.26.0 — Phase 18)
**Touches.** `crates/aaai-cli/src/cmd/lint.rs` (stub, no changes needed),
`crates/aaai-cli/src/tests.rs` (3 tests).

Lints a definition file for best-practice issues: duplicate paths, short
reasons, missing ticket/approver (opt-in), expired entries, empty LineMatch
rules, strategy/diff_type mismatches, disabled entries. `--json-output`
for CI; exits 1 only on errors, 0 on warnings.
