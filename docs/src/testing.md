# UI/UX Test Specification

This document defines the manual test cases for `aaai-gui` and the
`aaai` CLI. It is intended for testers performing pre-release verification.

Run the CLI tests automatically with:

```sh
cargo test -p aaai-cli -- --test-threads=1
```

The GUI cases below require manual execution.

---

## Environment Setup

```sh
cargo build --release -p aaai-cli -p aaai-gui

# Prepare two small test directories
mkdir -p /tmp/aaai-test/{before,after}
echo 'port = 80'  > /tmp/aaai-test/before/config.toml
echo 'port = 8080' > /tmp/aaai-test/after/config.toml
echo 'v1' > /tmp/aaai-test/before/version.txt
echo 'v2' > /tmp/aaai-test/after/version.txt

aaai snap --left /tmp/aaai-test/before --right /tmp/aaai-test/after \
          --out /tmp/aaai-test/audit.yaml
```

---

## 1. Opening Screen

| # | Step | Expected |
|---|---|---|
| 1-1 | Launch `aaai-gui` | Opening screen displayed; no errors |
| 1-2 | Leave all fields blank, press "Start Audit" | Button is disabled or shows validation error |
| 1-3 | Enter Before / After paths that do not exist, press "Start Audit" | Error message shown |
| 1-4 | Enter valid Before / After paths, no definition file, press "Start Audit" | Audit runs with empty definition; all entries Pending |
| 1-5 | Enter all valid paths, press "Start Audit" | Loading spinner shown; main screen opens |
| 1-6 | Enter a `.aaaiignore` path, press "Start Audit" | Ignored files do not appear in the file tree |
| 1-7 | Save a profile and reload it | Fields restored correctly |

---

## 2. Main Screen — File Tree

| # | Step | Expected |
|---|---|---|
| 2-1 | Open main screen | File tree shows changed files; dashboard visible |
| 2-2 | Click "Changed only" filter | Unchanged entries hidden |
| 2-3 | Click "Pending only" filter | Only Pending entries shown |
| 2-4 | Type a path fragment in the search bar | File tree filters live |
| 2-5 | Click a directory header (▼/▶) | Children collapse / expand |
| 2-6 | Select a Pending entry | Diff viewer and inspector update |
| 2-7 | Press ↓ key | Next entry selected |
| 2-8 | Press ↑ key | Previous entry selected |
| 2-9 | Entry with `AuditWarning` | `⚠N` badge visible on the row |

---

## 3. Diff Viewer

| # | Step | Expected |
|---|---|---|
| 3-1 | Select a Modified text file | Side-by-side diff shown with +/− colouring |
| 3-2 | Select an Added file | Right pane shows content; left pane empty |
| 3-3 | Select a Removed file | Left pane shows content; right pane empty |
| 3-4 | Select a binary file | Binary panel shown with SHA-256 hashes |
| 3-5 | Modified file | Stats bar shows `+N lines` / `−N lines` |
| 3-6 | Select no file (dashboard) | Summary cards and attention list shown |

---

## 4. Inspector

| # | Step | Expected |
|---|---|---|
| 4-1 | Select an entry | Inspector shows path, diff type, status badge |
| 4-2 | Entry has `AuditWarning` | Yellow warning block shown below divider |
| 4-3 | Leave reason blank, press "Approve" | Button disabled / validation error |
| 4-4 | Enter reason, select strategy, press "Approve" | Entry moves to OK; file tree badge updates |
| 4-5 | Enter ticket and approved_by | Values saved in definition YAML |
| 4-6 | Enter expires_at in wrong format | Validation error shown |
| 4-7 | Apply a template | Strategy fields populated from template |
| 4-8 | Press Ctrl+Z after approval | Last approval undone; entry returns to Pending |

---

## 5. Save / Re-run

| # | Step | Expected |
|---|---|---|
| 5-1 | Approve an entry | Footer shows "Unsaved changes" |
| 5-2 | Press Ctrl+S | Definition file saved; "Unsaved" indicator clears |
| 5-3 | Press Ctrl+R | Audit re-runs; results refresh |
| 5-4 | Modify before/after files externally, Ctrl+R | Updated diff shown |

---

## 6. Theme & Footer

| # | Step | Expected |
|---|---|---|
| 6-1 | Switch to Dark theme in footer | UI switches to dark palette immediately |
| 6-2 | Restart `aaai-gui` | Previous theme restored |
| 6-3 | Switch language to English | Labels change to English |
| 6-4 | Main screen footer | Shortcut legend visible (`Ctrl+S`, `Ctrl+R`, …) |

---

## 7. Export & Reports

| # | Step | Expected |
|---|---|---|
| 7-1 | Press "Export MD" | `aaai-report.md` created in current directory |
| 7-2 | Press "Export JSON" | `aaai-report.json` created, valid JSON |
| 7-3 | `aaai report --format html` | Valid HTML file with summary cards |
| 7-4 | `aaai report --format sarif` | Valid SARIF 2.1.0 JSON |

---

## 8. CLI Smoke Tests

```sh
# Audit — PASSED
aaai audit --left /tmp/aaai-test/before --right /tmp/aaai-test/after \
           --config /tmp/aaai-test/audit.yaml --no-history
# Expected exit 2 (Pending entries)

# Fill in reasons
sed -i 's/reason: .*/reason: "test change"/' /tmp/aaai-test/audit.yaml
aaai audit --left /tmp/aaai-test/before --right /tmp/aaai-test/after \
           --config /tmp/aaai-test/audit.yaml --no-history
# Expected exit 0 (PASSED)

# Dashboard
aaai dashboard --left /tmp/aaai-test/before --right /tmp/aaai-test/after \
               --config /tmp/aaai-test/audit.yaml
# Expected: coloured stat cards, no error

# Lint
aaai lint /tmp/aaai-test/audit.yaml
# Expected: exit 0, "No issues found"

# History stats (requires at least one prior run)
aaai history --stats
```

---

## 9. Acceptance Criteria

The release is ready when:

- [ ] All Opening screen cases (1-1 to 1-7) pass
- [ ] All File tree cases (2-1 to 2-9) pass
- [ ] All Diff viewer cases (3-1 to 3-6) pass
- [ ] All Inspector cases (4-1 to 4-8) pass
- [ ] All Save/Re-run cases (5-1 to 5-4) pass
- [ ] All Theme cases (6-1 to 6-4) pass
- [ ] All Export cases (7-1 to 7-4) pass
- [ ] All CLI smoke tests exit with expected codes
- [ ] No `cargo check --all-targets` warnings
- [ ] `cargo test -p aaai-core --lib` — 92 passing
- [ ] `cargo test -p aaai-cli -- --test-threads=1` — 30 passing
