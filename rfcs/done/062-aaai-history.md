# RFC 062 — `aaai history` activation

**Status.** Implemented (v0.26.0 — Phase 18)
**Touches.** `crates/aaai-cli/src/cmd/history.rs` (stub, no changes needed),
`crates/aaai-cli/src/tests.rs` (2 tests).

Shows recent audit runs from `~/.aaai/history.jsonl`.
`--stats` trend analysis; `--prune N` rotation; `--json-output`.
