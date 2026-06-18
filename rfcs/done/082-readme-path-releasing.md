# RFC 082 — Fix aaai-core README path + add RELEASING.md

**Status.** Implemented (v0.31.1 — Phase 23)
**Tracks.** Pre-1.0 Housekeeping
**Touches.** `crates/aaai-core/Cargo.toml`, `RELEASING.md` (new).

`readme = "../../README.md"` caused a cargo packaging warning ("path
outside of the package"). Changed to `readme = "README.md"` to point
to the crate-local README, silencing the warning.

Added RELEASING.md documenting the correct publish order
(aaai-core → aaai-cli → aaai-gui) and the `--no-verify` flag needed
for local packaging before aaai-core is live on crates.io.
