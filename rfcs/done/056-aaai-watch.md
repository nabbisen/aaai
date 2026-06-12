# RFC 056 — `aaai watch` completion

**Status.** Implemented (v0.25.0 — Phase 17)
**Tracks.** Phase 17 CLI
**Touches.** `crates/aaai-cli/src/cmd/watch.rs` (pre-existing stub, no changes needed),
`crates/aaai-cli/src/tests.rs` (1 smoke test).

## Summary

The `watch.rs` stub was already feature-complete: debounced file-system watcher
using `notify`, watches Before/After/config paths, re-runs the audit on Create/
Modify/Remove events, compact `[HH:MM:SS] PASSED/FAILED — OK:N Pend:N Fail:N`
output per run. No implementation changes were needed; one smoke test added to
confirm the subcommand exists and its help text is correct.

```sh
aaai watch --left ./before --right ./after --config audit.yaml
aaai watch --left ./before --right ./after --config audit.yaml --debounce-ms 1000
```
