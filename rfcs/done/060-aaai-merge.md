# RFC 060 — `aaai merge` activation

**Status.** Implemented (v0.26.0 — Phase 18)
**Touches.** `crates/aaai-cli/src/cmd/merge.rs` (stub, no changes needed),
`crates/aaai-cli/src/tests.rs` (3 tests).

Merges two definition files; overlay wins on conflict.
`--detect-conflicts`, `--dry-run`, `--out`.
