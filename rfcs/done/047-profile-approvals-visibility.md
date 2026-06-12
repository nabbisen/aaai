# RFC 047 — Opening screen: profile approvals visibility

**Status.** Implemented (v0.24.0)
**Tracks.** Opening screen UX, profile discoverability
**Touches.** `crates/aaai-gui/src/app.rs` (2 one-line additions),
`crates/aaai-gui/src/views/opening.rs` (profile row + Optional section),
`crates/aaai-gui/locales/{en,ja}.yaml` (1 new key × 2).

## Problems

**A — Profile rows don't show which approvals file is loaded.**
The detail line in each Recent Projects row shows
`before: … → after: …` but gives no indication of the associated
`audit.yaml`. A user with two profiles pointing to the same Before/After
folders but different approvals files cannot tell them apart.

**B — Loaded approvals file is invisible after LoadProfile.**
When the user clicks "Open →" on a profile that has a `definition` path,
`App.definition_path` is populated, but `optional_settings_expanded`
stays `false`. The Approvals file field is hidden inside the collapsed
"Optional settings" section — the user never sees which file was loaded.

The same gap exists after a save-as (`DefinitionSavePathPicked`): the
path is set but the section stays collapsed.

## Fixes

### Fix A — approvals filename in profile row

When `prof.definition` is `Some(def_path)`, add a third sub-line to the
profile row showing the filename stem (not the full path — just the
filename, e.g. `📋 audit.yaml`):

```
▸ my-project                    3 min ago
  before: /a/before  →  after: /a/after
  📋 audit.yaml
```

When `prof.definition` is `None`, the third line is absent.

### Fix B — auto-expand Optional settings

In **two handlers** that set `definition_path` to a non-empty value:

```rust
// In Message::LoadProfile:
self.definition_path = p.definition.unwrap_or_default();
if !self.definition_path.is_empty() {
    self.optional_settings_expanded = true;  // ← RFC 047
}

// In Message::DefinitionSavePathPicked(Some(path)):
self.definition_path = chosen.display().to_string();
self.optional_settings_expanded = true;      // ← RFC 047
```

The user can still collapse the section manually via the `▸` button.

## i18n (1 new key × 2 locales)

```yaml
# en.yaml
opening:
  recent_project_definition: "📋 %{file}"

# ja.yaml
opening:
  recent_project_definition: "📋 %{file}"
```

(The icon and filename format is language-neutral, but the key is
added for consistency with other `opening.*` format strings.)

## Acceptance criteria

- [ ] Profile rows with `definition` set show a `📋 {filename}` sub-line
- [ ] Profile rows without `definition` show no third sub-line
- [ ] `LoadProfile` with non-empty definition_path auto-expands Optional settings
- [ ] `DefinitionSavePathPicked(Some(...))` auto-expands Optional settings
- [ ] 1 new i18n key in en.yaml + ja.yaml
- [ ] `check-i18n-keys.py --quiet` → 0/0/0 (215/215/215)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All tests pass (101 / 70 / 15)
