# RFC 058 — Pending count in window title

**Status.** Implemented (v0.25.0 — Phase 17)
**Tracks.** Phase 17 polish
**Touches.** `crates/aaai-gui/src/main.rs` (title closure).

## Summary

Window title now shows `(N pending)` when Pending > 0 so users can
see audit progress from the OS taskbar without switching focus:

| State | Title |
|---|---|
| Opening screen | `aaai` |
| Main, 12 pending | `aaai — audit.yaml ● (12 pending)` |
| Main, 0 pending, clean | `aaai — audit.yaml` |
| Main, 0 pending, dirty | `aaai — audit.yaml ●` |

The count goes to zero when all items are approved; the title updates
as the background rerun completes.
