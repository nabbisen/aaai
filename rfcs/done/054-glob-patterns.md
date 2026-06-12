# RFC 054 — Glob pattern entries in Inspector

**Status.** Implemented (v0.25.0 — Phase 17)
**Tracks.** Phase 17 — Glob Rules & Power Workflow
**Touches.** `crates/aaai-gui/src/app.rs` (InspectorState, messages,
handlers), `crates/aaai-gui/src/views/inspector.rs` (pattern toggle +
text input), `crates/aaai-gui/locales/{en,ja}.yaml` (3 new keys × 2).

## Background

`AuditEntry.path` has always supported glob patterns — `is_glob()` and
`glob_matches()` exist in aaai-core and `find_entry()` already does
exact-first, then glob-fallback lookup. The `glob` crate is a current
dependency. However, the GUI has no way to *create* glob entries: the
Inspector hard-codes `far.diff.path` as the entry path, so every
approval always creates an exact-path entry.

This RFC exposes the existing glob engine through the GUI.

## Use case

The user is auditing a repo with 40 files under `node_modules/`. All
are expected changes (dependency update). Currently they must approve
each file individually. With glob support:

1. Select any file (e.g. `node_modules/lodash/package.json`)
2. Toggle **▸ Use pattern**
3. Edit the pattern field to `node_modules/**`
4. Type one reason: "Third-party dependency update"
5. `Ctrl+Enter` — creates one glob entry covering all 40 files
6. Background rerun marks all 40 as OK

## Design

### New state in `InspectorState`

```rust
pub use_pattern: bool,
pub pattern_path: String,  // editable path / glob
```

`pattern_path` is initialized to `far.diff.path` when an entry is
selected (same as the exact path). `use_pattern` defaults to `false`.

### New messages

```rust
ToggleUsePattern,
PatternChanged(String),
```

### Inspector view changes

Between the path header and the Reason field, when `use_pattern` is
false:

```
node_modules/lodash/package.json   Modified   Pending
▸ Use pattern
```

When `use_pattern` is true:

```
node_modules/lodash/package.json   Modified   Pending
▾ Use pattern
Pattern  [node_modules/**____________] 
         ✓ valid glob                    ← or ✗ invalid pattern
```

The text input is pre-filled with the current diff path. The user
edits it to a glob. Live validation checks `glob::Pattern::new(&s)`.

### `ApproveEntry` change

```rust
let entry_path = if self.inspector.use_pattern {
    self.inspector.pattern_path.trim().to_string()
} else {
    far.diff.path.clone()
};
let entry = AuditEntry { path: entry_path, … };
```

### `SelectEntry` reset

When a new file is selected, reset `use_pattern = false` and
`pattern_path = far.diff.path.clone()`. This ensures the toggle
doesn't leak across selections.

### Validation

Add `pattern_error: Option<String>` to `InspectorValidation`. When
`use_pattern` is true and the pattern is not a valid glob:
- Validation fails → Approve button stays disabled
- Error shown inline below the pattern input

When `use_pattern` is true but the pattern is empty:
- Treat as empty path → validation error "Pattern cannot be empty"

`can_approve()` returns false when `pattern_error.is_some()`.

## i18n (3 new keys × 2 locales)

```yaml
# en.yaml
inspector:
  use_pattern:          "Use pattern"
  pattern_label:        "Pattern"
  pattern_placeholder:  "e.g. node_modules/**"

# ja.yaml
inspector:
  use_pattern:          "パターンを使用"
  pattern_label:        "パターン"
  pattern_placeholder:  "例: node_modules/**"
```

## Acceptance criteria

- [ ] `use_pattern: bool` + `pattern_path: String` in `InspectorState`
- [ ] `ToggleUsePattern` / `PatternChanged` messages and handlers
- [ ] "▸ Use pattern" toggle visible in Inspector (primary view)
- [ ] Text input shown when toggled on, pre-filled with diff path
- [ ] Live validation: valid glob ✓ / invalid ✗ / empty error
- [ ] `ApproveEntry` uses `pattern_path` when `use_pattern` is true
- [ ] `SelectEntry` resets `use_pattern = false`
- [ ] 3 new i18n keys in en.yaml + ja.yaml
- [ ] `check-i18n-keys.py --quiet` → 0/0/0 (222/222/222)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All tests pass (101 / 70 / 15)
