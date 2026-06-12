# RFC 034 — Toast / dialog / format-string i18n sweep

**Status.** Implemented (v0.22.0 — Phase 14)
**Tracks.** GUI i18n completeness
**Touches.** `crates/aaai-gui/src/app.rs` (10 sites),
`crates/aaai-gui/src/views/diff_view.rs` (1 site),
`crates/aaai-gui/locales/{en,ja}.yaml` (13 new keys × 2 locales),
no test additions.

## Summary

RFCs 031-033 closed user-facing string i18n for `t!()`-style
text widgets and `pick_list` options. A subsequent sweep with a
**`format!()` and unbracketed string-literal grep** turned up
16 hardcoded user-facing string call sites across `app.rs` and
`diff_view.rs` that the previous sweeps' regex patterns hadn't
caught:

- Toast bodies (10 hits across save/export/profile/undo)
- Native file-picker dialog titles (4 hits)
- One inline diff stats format string

This RFC migrates all 13 sites — deduplicated to **10 unique
keys** — and closes the GUI i18n loop conclusively.

## Why now

The earlier sweeps (RFC 031, RFC 032) caught text inside `text()`
widgets and explicit string array literals. They missed:

1. Strings passed as `&str` directly to `push_toast()` second/third
   arguments (no `text(...)` wrapper).
2. `format!()` calls where the body interpolates a runtime value
   into otherwise-static English text.
3. `rfd::AsyncFileDialog::new().set_title("...")` titles, which
   surface as native OS dialogs.

The right fix is to use a richer grep — for `format!`, for
`push_toast`/`push_toast_with_hint` payload literals, and for
`.set_title(`. This RFC closes that final gap and documents the
broader pattern for future scope locks.

The scope was locked **before** drafting by running:

```sh
# Find format! patterns
grep -nE 'format!\("[^"]*[A-Z][a-z]' crates/aaai-gui/src/**/*.rs

# Find push_toast literal payloads
grep -nE 'push_toast\(' crates/aaai-gui/src/**/*.rs

# Find dialog set_title
grep -rn '\.set_title(' crates/aaai-gui/src/
```

Together, these three patterns enumerate everything in this
RFC's scope. No iterative re-scope this time.

## What this RFC does

### 10 new i18n keys (×2 locales = 20 entries)

```yaml
# en.yaml — additions
toast:
  no_definition_path: "No definition file path set."
  saved_to_path:      "Saved to %{path}"          # used 2x (save + export)
  undo:               "Undo"                      # toast title, used 2x
  nothing_to_undo:    "Nothing to undo."
  removed_approval:   "Removed approval for: %{path}"
  profile:            "Profile"                   # toast title, used 2x
  profile_name_empty: "Profile name must not be empty."
  profile_loaded:     "Profile loaded."

dialog:
  pick_before:        "Pick the Before folder"
  pick_after:         "Pick the After folder"
  pick_audit_yaml:    "Pick audit.yaml"
  pick_aaaiignore:    "Pick .aaaiignore"

diff:
  size_inline:        "  Size: %{value}"          # diff stats row variant
```

The `dialog.*` namespace is new (no existing `dialog.*` keys
prior to this RFC). The leading whitespace in `diff.size_inline`
is preserved because it's part of the visual layout (the inline
stats row aligns by it).

### 13 call-site updates

| File:line | Before | After |
|---|---|---|
| `app.rs:825` | `"No definition file path set."` | `t!("toast.no_definition_path")` |
| `app.rs:839` | `format!("Saved to {}", path.display())` | `t!("toast.saved_to_path", path = ...)` |
| `app.rs:886` | `format!("Saved to {}", out.display())` | same key |
| `app.rs:958` | `"Undo"` | `t!("toast.undo")` |
| `app.rs:959` | `format!("Removed approval for: {path}")` | `t!("toast.removed_approval", path = ...)` |
| `app.rs:964` | `"Undo"` / `"Nothing to undo."` | `t!("toast.undo")` / `t!("toast.nothing_to_undo")` |
| `app.rs:1036` | `.set_title("Pick the Before folder")` | `t!("dialog.pick_before")` |
| `app.rs:1048` | `.set_title("Pick the After folder")` | `t!("dialog.pick_after")` |
| `app.rs:1060` | `.set_title("Pick audit.yaml")` | `t!("dialog.pick_audit_yaml")` |
| `app.rs:1073` | `.set_title("Pick .aaaiignore")` | `t!("dialog.pick_aaaiignore")` |
| `app.rs:1214` | `"Profile"` / `"Profile name must not be empty."` | `t!("toast.profile")` / `t!("toast.profile_name_empty")` |
| `app.rs:1256` | `"Profile"` / `"Profile loaded."` | `t!("toast.profile")` / `t!("toast.profile_loaded")` |
| `diff_view.rs:198` | `format!("  Size: {label}")` | `t!("diff.size_inline", value = label)` |

### Japanese translations

Following established conventions: terse for titles, polite
imperative or fact-style for bodies. Path placeholders kept
unchanged.

```yaml
# ja.yaml additions
toast:
  no_definition_path: "監査定義ファイルのパスが設定されていません。"
  saved_to_path:      "%{path} に保存しました"
  undo:               "取り消し"
  nothing_to_undo:    "取り消す操作がありません。"
  removed_approval:   "%{path} の承認を取り消しました"
  profile:            "プロファイル"
  profile_name_empty: "プロファイル名を入力してください。"
  profile_loaded:     "プロファイルを読み込みました。"

dialog:
  pick_before:        "Before フォルダを選択"
  pick_after:         "After フォルダを選択"
  pick_audit_yaml:    "audit.yaml を選択"
  pick_aaaiignore:    ".aaaiignore を選択"

diff:
  size_inline:        "  サイズ: %{value}"
```

The `dialog.pick_*` titles surface in native OS file pickers
(rfd uses the platform's open-folder dialog), so they need to
match the language conventions OS users expect.

## Internal design

### Format-string substitution pattern

All three `format!()` migrations use `rust-i18n`'s `%{name}`
substitution (introduced in RFC 032 for `opening.recent_project_paths`):

```rust
// Before:
&format!("Saved to {}", path.display())

// After:
t!("toast.saved_to_path", path = path.display().to_string()).as_ref()
```

`path.display()` returns a `Display`able value; `.to_string()`
materialises it for the `t!()` call.

### `rfd` dialog title pattern

`rfd::AsyncFileDialog`'s `set_title` accepts `impl Into<String>`.
The pattern:

```rust
// Before:
rfd::AsyncFileDialog::new()
    .set_title("Pick the Before folder")
    .pick_folder()

// After:
rfd::AsyncFileDialog::new()
    .set_title(t!("dialog.pick_before").to_string())
    .pick_folder()
```

The title appears in the **OS-native** dialog window decoration.
Localising it gives Japanese users a fully-Japanese file picker.

### Reused title keys

Two toast titles are reused across multiple call sites:
- `toast.undo` — used for both successful undo and "nothing to undo"
- `toast.profile` — used for both "name empty" error and "loaded" info

The audit script's static-key counting handles this naturally
(referenced once, referenced again — both count as references).

### Locale counts

| Pre-RFC | Post-RFC |
|---|---|
| 162 / 162 / 162 | **175 / 175 / 175** (+13 keys × 2 locales) |

## What this RFC does NOT do

### Strategy label strings (out of scope)

`STRATEGIES: &[&str] = &["None", "Checksum", "LineMatch", "Regex", "Exact"]`
in `batch.rs` and `inspector.rs`, plus the matching arms in
`strategy_from_label()` (app.rs:1639-1642). These display label
strings double as pick_list values **and** discriminators for
constructing the corresponding `AuditStrategy` variants.

Localising them requires the same `LocalizedOption<T>` pattern
as RFC 033, but with a separate discriminator enum
(`StrategyKind`) since `AuditStrategy` itself is a struct-shaped
enum with associated data. The picker would select a
`StrategyKind`, then the construction would build the
corresponding default `AuditStrategy::Variant` shape.

This is a Message protocol change and architecturally parallel
to RFC 033 but small enough to warrant its own RFC. Deferred to
RFC 035.

### Locale display name `"English"` (out of scope)

`crates/aaai-gui/src/i18n.rs:8` has `"English"` as a locale
display label. Locale names are conventionally rendered in
their own language ("English", "日本語"), not localised into
the user's current locale. Out of scope for an i18n sweep
because translating "English" to "英語" in the locale switcher
would be confusing — users about to switch to English want to
see the word "English."

### `mod tests` strings

`#[cfg(test)] mod tests` blocks contain English strings used in
assertions. Test code is never user-facing; ignored.

### aaai-core derived strings

`AuditStatus::Display`, `is_approvable()` errors — same as RFC
031's deferred §5 work. v1→v2 major bump territory.

## Acceptance criteria

- [ ] 16 call sites updated to use `t!()`
- [ ] 10 new keys in en.yaml + 10 in ja.yaml under `toast.*`,
      `dialog.*`, `diff.size_inline`
- [ ] `scripts/check-i18n-keys.py --quiet` returns 0/0/0
      (175/175/175)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All existing tests pass (97 / 70 / 13)
- [ ] CHANGELOG entry under `[Unreleased]` records both the
      migration and the explicit out-of-scope items
- [ ] No changes to aaai-core

## Open questions

None at acceptance. Documented as scope locked via three-grep
discipline (format! / push_toast / .set_title).

Future work this RFC enables:

- **RFC 035 — Strategy label display/value separation.** Parallel
  to RFC 033 but with a discriminator enum since `AuditStrategy`
  is struct-shaped.
- **i18n parity verification end-to-end.** With this RFC,
  `aaai-gui`'s user-facing string surface in app.rs and views
  is fully covered (excluding the documented out-of-scope items).
  A future operator walkthrough in Japanese locale validates the
  end-to-end UX.
