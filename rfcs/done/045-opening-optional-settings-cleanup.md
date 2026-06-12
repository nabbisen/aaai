# RFC 045 — Opening screen Optional settings cleanup

**Status.** Implemented (v0.24.0)
**Tracks.** Opening screen UX
**Touches.** `crates/aaai-gui/src/views/opening.rs` (remove ignore row,
remove hint text), `crates/aaai-gui/locales/{en,ja}.yaml` (−2 removed,
2 values updated).

## Changes

### 1. Remove the `.aaaiignore` picker row

The `.aaaiignore` file picker (`opening.ignore_label`) is removed from the
Optional settings section. Global ignored directories configured in
**App Settings** (RFC 036) already cover the most common use case
(`.git`, `target`, `node_modules`, …). Per-project ignore files are still
auto-detected from `<Before>/.aaaiignore` silently — no UI needed.

The `App.ignore_path` field and its messages are kept for backward
compatibility with saved profiles, but are no longer surfaced in the UI.

### 2. Rename "Audit definition" → "Approvals file"

`opening.definition_label` changes from `"Audit definition"` to `"Approvals
file"` (EN) / `"承認ファイル"` (JA). This is what the file actually is from
the user's perspective — the record of saved approvals for a folder pair.

### 3. Remove the collapsible hint text

`opening.optional_hint` (`"Collapsed by default. Without these, a new audit
definition is created."`) is removed from the view and from both locale files.

### 4. Update the section header

`opening.optional_section` (`"Optional settings (audit definition /
.aaaiignore)"`) updated to `"Optional settings"` — the parenthetical
content was documentation for developers, not useful to users.

## i18n delta

| Key | Before | After |
|---|---|---|
| `opening.optional_section` | `"Optional settings (audit definition / .aaaiignore)"` | `"Optional settings"` |
| `opening.definition_label` | `"Audit definition"` | `"Approvals file"` |
| `opening.optional_hint` | `"Collapsed by default…"` | **removed** |
| `opening.ignore_label` | `".aaaiignore file"` | **removed** |

Net: **−2 keys** × 2 locales = 216 → **212 / 212 / 212**

## Acceptance criteria

- [ ] `.aaaiignore` picker row absent from the expanded Optional settings section
- [ ] `opening.optional_hint` text absent from the view
- [ ] Section header reads "Optional settings" (no parenthetical)
- [ ] Field label reads "Approvals file" (EN) / "承認ファイル" (JA)
- [ ] `check-i18n-keys.py --quiet` → 0/0/0 (212/212/212)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All existing tests pass
