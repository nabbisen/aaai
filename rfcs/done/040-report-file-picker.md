# RFC 040 — Report export with native file picker

**Status.** Implemented (v0.23.0 — Phase 15)
**Tracks.** GUI report UX
**Touches.** `crates/aaai-gui/src/app.rs` (message change, handler
split into open-dialog + write), `crates/aaai-gui/src/views/main_view.rs`
(toolbar button arg removed), `crates/aaai-gui/locales/{en,ja}.yaml`
(1 new key × 2).

## Problem

`Message::ExportReport(String)` writes `aaai-report.{md|json}` to the
**process working directory** without informing the user of the exact
path (beyond the `toast.saved_to_path` toast). Users don't know where
the file went. The format string ("markdown") is hardcoded in the toolbar
and keyboard handler — there is no way to choose JSON from the GUI.

## Solution

Replace the silent write with an `rfd::AsyncFileDialog::save_file()`
dialog (same `rfd` crate already used for folder/YAML pickers), then
derive the format from the chosen file extension. The dialog:

- Has the title `t!("dialog.save_report")`
- Defaults to filename `aaai-report.md`
- Offers two filters: `Markdown (*.md)` and `JSON (*.json)`
- If the user cancels, the export is silently skipped

## Message changes

```rust
// Before:
ExportReport(String),       // String = "markdown" or "json"

// After:
ExportReport,               // opens dialog
ReportPathPicked(Option<PathBuf>),  // dialog result
```

`ExportReport` is used in three places:
- Toolbar "↑ Export Report" button — remove `"markdown".into()` arg
- Keyboard handler `Ctrl+E` — same
- (Only those two; CLI has its own `aaai report` command)

## Handler split

```rust
Message::ExportReport => {
    if self.audit_result.is_none() {
        // Nothing to export yet — toast and abort.
        self.push_toast(ToastIntent::Info,
            t!("toast.export_failed").as_ref(),
            t!("toast.no_audit_result").as_ref());
        return Task::none();
    }
    let title = t!("dialog.save_report").to_string();
    return Task::perform(
        async move {
            rfd::AsyncFileDialog::new()
                .set_title(title)
                .set_file_name("aaai-report.md")
                .add_filter("Markdown", &["md"])
                .add_filter("JSON",     &["json"])
                .save_file()
                .await
                .map(|h| h.path().to_path_buf())
        },
        Message::ReportPathPicked,
    );
}

Message::ReportPathPicked(Some(out)) => {
    let fmt = if out.extension().and_then(|e| e.to_str()) == Some("json") {
        "json"
    } else {
        "markdown"  // default
    };
    // ... existing write logic ...
}

Message::ReportPathPicked(None) => { /* user cancelled — no-op */ }
```

## i18n (1 new key × 2 locales)

```yaml
# en.yaml
dialog:
  save_report: "Save Report"

# ja.yaml
dialog:
  save_report: "レポートを保存"
```

(Additionally: `toast.no_audit_result` for the early-exit toast when
no audit has run yet. Currently the button is always enabled.)

```yaml
# en.yaml
toast:
  no_audit_result: "Run an audit first before exporting a report."

# ja.yaml
toast:
  no_audit_result: "レポートを出力する前に監査を実行してください。"
```

### Locale counts

| Pre-RFC | Post-RFC |
|---|---|
| 210 / 210 / 210 | **212 / 212 / 212** (+2 keys × 2) |

## Acceptance criteria

- [ ] `Message::ExportReport` takes no argument
- [ ] `Message::ReportPathPicked(Option<PathBuf>)` added
- [ ] Both handlers implemented (open dialog / write + toast / no-op)
- [ ] Format derived from extension (`json` → JSON, else → Markdown)
- [ ] Toolbar button and Ctrl+E updated (remove format arg)
- [ ] No-audit early-exit with toast
- [ ] 2 new i18n keys in en.yaml + ja.yaml
- [ ] `check-i18n-keys.py --quiet` → 0/0/0 (212/212/212)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All existing tests pass (99 / 70 / 15)
