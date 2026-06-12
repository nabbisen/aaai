# RFC 036 — App Settings dialog

**Status.** Implemented (v0.23.0 — Phase 15)
**Tracks.** GUI usability, app-level configuration
**Touches.**
`crates/aaai-core/src/profile/prefs.rs` (extend `UserPrefs`),
`crates/aaai-gui/src/app.rs` (new state, messages, audit-start merge),
`crates/aaai-gui/src/views/settings_dialog.rs` (new file),
`crates/aaai-gui/locales/{en,ja}.yaml` (9 new keys × 2 locales).

## Summary

This RFC adds a **Settings dialog** — app-level configuration that
persists across sessions. First-iteration scope is deliberately
narrow: two settings only.

1. **Language** — the existing locale `pick_list` from the footer
   moved here.
2. **Global ignored directories** — directory names (e.g. `.git`,
   `target`, `node_modules`) that are silently excluded from every
   audit regardless of project, merged with any per-project
   `.aaaiignore` rules.

The dialog replaces the language picker in the bottom-right footer.
A ⚙ gear button in that position launches the dialog. Language can
still be changed at any time; it just lives in a dedicated place now.

## Requirement definition

### FR-1 — Settings launcher

A gear icon button `⚙` replaces the language `pick_list` in
`view_footer()`. Clicking it opens the Settings dialog. The version
string to the right remains.

### FR-2 — Settings dialog

The dialog is a modal overlay (centred, with a semi-transparent
backdrop that intercepts clicks to dismiss). Content:

```
┌─── Settings ────────────────────────────────┐
│                                             │
│  Language                                   │
│  [English ▾]                                │
│                                             │
│  Ignored Directories                        │
│  Applied to every audit. Directory names    │
│  that are always excluded (e.g. .git).      │
│                                             │
│  .git                [×]                    │
│  target              [×]                    │
│  node_modules        [×]                    │
│  __________________  [×]   ← editing        │
│                                             │
│  [+ Add directory]                          │
│                                             │
│  [Cancel]               [Save]              │
└─────────────────────────────────────────────┘
```

### FR-3 — Draft editing

Changes in the dialog modify a **draft copy** of the settings.
Pressing **Save** applies and persists. Pressing **Cancel** or
clicking the backdrop discards the draft. This prevents partial
changes from leaking into running audits.

### FR-4 — Persistence

Settings are saved to `~/.aaai/prefs.yaml` (existing path used by
`UserPrefs`). Unknown YAML keys are ignored by serde, so the file
remains forward-compatible.

### FR-5 — Defaults

Default ignored directories: `.git`, `target`, `node_modules`,
`.DS_Store`. These are added when no `prefs.yaml` exists.

### FR-6 — Audit integration

At audit start, global ignored directories from `AppSettings` are
prepended as `<name>/**` glob patterns to the `IgnoreRules` before
any project-level `.aaaiignore` patterns. The rules merge is done in
the GUI layer without changing aaai-core's API.

## External design

### Footer change

```
Before:   [unsaved notice]  ─────────  [English ▾]  v0.22.0
After:    [unsaved notice]  ─────────  [⚙]  v0.22.0
```

The language picker is removed from the footer. The ⚙ button has
a tooltip `t!("settings.button_tooltip")` = "Settings".

### Dialog layout

Two labelled sections separated by padding:

**Language section**
- Label: `t!("settings.language")`
- Control: `pick_list` over `SUPPORTED_LOCALES`, same as before

**Ignored Directories section**
- Label: `t!("settings.ignored_dirs")`
- Hint: `t!("settings.ignored_dirs_hint")`
- List: each entry is a `text_input` with a `[×]` remove button
- Footer: `[+ Add directory]` button adds a new blank entry

**Dialog buttons** — a `row!` right-aligned:
- `[Cancel]` → `Message::CloseSettings`
- `[Save]` → `Message::SaveSettings`

Backdrop: a full-screen `mouse_area` below the dialog box on the
z-stack that sends `Message::CloseSettings` on click.

## Internal design

### `UserPrefs` extension (aaai-core)

`UserPrefs` already notes: *"Future preferences (font size, language
override, etc.) can be added here."*

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrefs {
    #[serde(default)]
    pub theme: Theme,

    /// Locale code (e.g. "en", "ja"). Empty string = system default.
    #[serde(default)]
    pub language: String,

    /// Directory names excluded from every audit. Converted to
    /// `<name>/**` glob patterns and prepended to IgnoreRules.
    #[serde(default = "default_ignored_dirs")]
    pub global_ignored_dirs: Vec<String>,
}

fn default_ignored_dirs() -> Vec<String> {
    vec![
        ".git".into(),
        "target".into(),
        "node_modules".into(),
        ".DS_Store".into(),
    ]
}
```

`#[serde(default = "...")]` ensures old `prefs.yaml` files without
the new field get the sensible default list on next load.

### New App state

```rust
// In App struct:
pub prefs: UserPrefs,           // loaded on startup, saved on Save
pub settings_open: bool,        // controls modal visibility
pub settings_draft: UserPrefs,  // editable copy; applied only on Save
```

### New Messages

```rust
OpenSettings,
CloseSettings,                      // discard draft
SaveSettings,                       // apply draft + persist
SettingsLanguageChanged(String),    // code, not label
SettingsIgnoreDirAdd,
SettingsIgnoreDirEdit(usize, String),
SettingsIgnoreDirRemove(usize),
```

### Message handlers

```rust
Message::OpenSettings => {
    self.settings_draft = self.prefs.clone();
    self.settings_open = true;
}
Message::CloseSettings => {
    self.settings_open = false;
    // draft is abandoned; prefs unchanged
}
Message::SaveSettings => {
    self.prefs = self.settings_draft.clone();
    // Apply language immediately
    if !self.prefs.language.is_empty() {
        rust_i18n::set_locale(&self.prefs.language);
        self.locale = self.prefs.language.clone();
    }
    self.prefs.save();
    self.settings_open = false;
}
Message::SettingsLanguageChanged(code) => {
    self.settings_draft.language = code;
}
Message::SettingsIgnoreDirAdd => {
    self.settings_draft.global_ignored_dirs.push(String::new());
}
Message::SettingsIgnoreDirEdit(i, s) => {
    if let Some(entry) = self.settings_draft.global_ignored_dirs.get_mut(i) {
        *entry = s;
    }
}
Message::SettingsIgnoreDirRemove(i) => {
    let dirs = &mut self.settings_draft.global_ignored_dirs;
    if i < dirs.len() { dirs.remove(i); }
}
```

### Startup — load prefs

In `App::new()` / `init()`, after the existing profile load:

```rust
let prefs = UserPrefs::load();
if !prefs.language.is_empty() {
    rust_i18n::set_locale(&prefs.language);
}
// App { prefs, ..., locale: prefs.language.clone().or_else(|| "en"), ... }
```

### Audit-start — merge global ignores

In the `Message::RunAudit` handler, replace the current
`IgnoreRules::load(path)` call:

```rust
// Build merged ignore text: global dirs first, project file second
let mut ignore_text = String::new();
for dir in &self.prefs.global_ignored_dirs {
    let dir = dir.trim();
    if !dir.is_empty() {
        ignore_text.push_str(&format!("{}/**\n", dir));
    }
}

// Append project-level rules
let ignore_path = /* existing logic to find .aaaiignore */;
if ignore_path.exists() {
    if let Ok(project_text) = std::fs::read_to_string(&ignore_path) {
        ignore_text.push('\n');
        ignore_text.push_str(&project_text);
    }
}

let ignore = IgnoreRules::from_str(&ignore_text)
    .unwrap_or_default();
```

No changes to `aaai-core`'s `IgnoreRules` or `DiffEngine` API.

### `views/settings_dialog.rs`

New view-layer module. Exports:

```rust
pub fn view<'a>(draft: &'a UserPrefs, locale: &'a str) -> Element<'a, Message>
```

Builds the dialog box. The modal overlay (backdrop + centering)
is assembled in `App::view()` using iced's `stack!` widget:

```rust
// In App::view():
let base = /* normal view */;
if self.settings_open {
    use iced::widget::{mouse_area, stack};
    stack![
        base,
        mouse_area(full_screen_backdrop()).on_press(Message::CloseSettings),
        iced::widget::center(settings_dialog::view(&self.settings_draft, &self.locale)),
    ].into()
} else {
    base
}
```

The `full_screen_backdrop()` helper returns a semi-transparent
container that fills the screen.

### i18n keys (9 × 2 locales = 18 entries)

```yaml
# en.yaml
settings:
  title:              "Settings"
  button_tooltip:     "Settings"
  language:           "Language"
  ignored_dirs:       "Ignored Directories"
  ignored_dirs_hint:  "Directory names skipped in every audit (e.g. .git, target)."
  dir_placeholder:    "directory name"
  add_dir:            "+ Add directory"
  save:               "Save"
  cancel:             "Cancel"

# ja.yaml
settings:
  title:              "設定"
  button_tooltip:     "設定"
  language:           "言語"
  ignored_dirs:       "除外ディレクトリ"
  ignored_dirs_hint:  "すべての監査で除外するディレクトリ名 (例: .git, target)。"
  dir_placeholder:    "ディレクトリ名"
  add_dir:            "+ ディレクトリを追加"
  save:               "保存"
  cancel:             "キャンセル"
```

### Locale counts

| Pre-RFC | Post-RFC |
|---|---|
| 180 / 180 / 180 | **189 / 189 / 189** (+9 keys × 2 locales) |

## Testing

### aaai-core — `prefs.rs` (2 new tests)

```rust
#[test]
fn new_fields_round_trip() {
    let p = UserPrefs {
        language: "ja".into(),
        global_ignored_dirs: vec![".git".into(), "target".into()],
        ..Default::default()
    };
    let yaml = serde_yaml::to_string(&p).unwrap();
    let r: UserPrefs = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(r.language, "ja");
    assert_eq!(r.global_ignored_dirs, vec![".git", "target"]);
}

#[test]
fn missing_fields_get_defaults() {
    // Old prefs.yaml without the new fields
    let yaml = "theme: light\n";
    let p: UserPrefs = serde_yaml::from_str(yaml).unwrap();
    assert!(!p.global_ignored_dirs.is_empty(), "default dirs applied");
    assert_eq!(p.language, "");
}
```

aaai-core test count: 97 → 99.

## Acceptance criteria

- [ ] `UserPrefs` extended with `language` and `global_ignored_dirs`
- [ ] Defaults: `.git`, `target`, `node_modules`, `.DS_Store`
- [ ] Prefs loaded on `App::new()`; language applied immediately if set
- [ ] `App` gains `prefs`, `settings_open`, `settings_draft` fields
- [ ] 7 new Messages defined and handled
- [ ] Footer `pick_list` → ⚙ button (tooltip "Settings")
- [ ] `views/settings_dialog.rs` — language section + ignored dirs section
- [ ] Modal overlay via `stack!` + backdrop + `center()`
- [ ] Draft pattern: `OpenSettings` clones prefs; `CloseSettings` discards;
      `SaveSettings` applies + persists
- [ ] Audit-start handler merges global dirs + project file into one `IgnoreRules`
- [ ] 9 i18n keys in en.yaml + ja.yaml
- [ ] `check-i18n-keys.py --quiet` → 0/0/0 (189/189/189)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All tests pass (99 / 70 / 15)
- [ ] CHANGELOG entry under `[Unreleased]`

## Open questions

None at acceptance. Future Settings sections (font size, theme,
report format defaults, keyboard shortcuts) can be added to the
same dialog without breaking the YAML or the dialog structure.
