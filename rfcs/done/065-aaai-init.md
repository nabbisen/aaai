# RFC 065 — `aaai init` activation

**Status.** Implemented (v0.27.0 — Phase 19)
**Touches.** `crates/aaai-cli/src/cmd/init.rs` (stub, no changes needed),
`crates/aaai-cli/src/tests.rs` (3 tests).

Last unactivated CLI stub. `--non-interactive` creates a default `.aaai.yaml`;
interactive mode prompts for paths, approver name, masking, and optionally
runs snap. Tests verify config creation, existing-config guard, and --help.
