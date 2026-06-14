# RFC 080 — Checksum strategy: how-to hint for obtaining the hash

**Status.** Implemented (v0.31.0 — Phase 22)
**Touches.** `crates/aaai-gui/src/views/inspector.rs`,
`locales/{en,ja}.yaml` (+1 key: `inspector.checksum_how_to`).

Adds a greyed size-9 hint below the SHA-256 input field showing the
exact shell command to obtain the hash on Linux/macOS and Windows.
