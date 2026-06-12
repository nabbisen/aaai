# RFC 057 — `aaai export` completion

**Status.** Implemented (v0.25.0 — Phase 17)
**Tracks.** Phase 17 CLI
**Touches.** `crates/aaai-cli/src/cmd/export.rs` (pre-existing stub, no changes needed),
`crates/aaai-cli/src/tests.rs` (3 tests).

## Summary

The `export.rs` stub was already feature-complete: CSV/TSV output with proper
quoting, all AuditEntry fields, stdout or `--out file`, `--all` flag for
Unchanged entries. Three integration tests added and passing.

```sh
aaai export -l ./before -r ./after -c audit.yaml          # CSV to stdout
aaai export -l ./before -r ./after -c audit.yaml -f tsv   # TSV to stdout
aaai export -l ./before -r ./after -c audit.yaml -o out.csv
```
