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
| 1-2 | Leave all fields blank, press "Check changes" | Button is disabled or shows validation error |
| 1-3 | Enter Before / After paths that do not exist, press "Check changes" | Error message shown |
| 1-4 | Enter valid Before / After paths, no definition file, press "Check changes" | Audit runs with empty definition; all entries needing review; first item auto-selected in inspector |
| 1-5 | Enter all valid paths, press "Check changes" | Loading spinner shown; main screen opens |
| 1-6 | Expand "Optional settings" | Section shows only "Approvals file" field with placeholder text |
| 1-7 | Save a profile and reload it | Fields restored correctly; Optional settings expands showing loaded definition path |

---

## 2. Main Screen — File Tree

| # | Step | Expected |
|---|---|---|
| 2-1 | Open main screen | File tree shows changed files; dashboard visible |
| 2-2 | Click "Changed only" filter | Unchanged entries hidden |
| 2-3 | Click "Needs review" filter | Only Pending entries shown |
| 2-4 | Type a path fragment in the search bar | File tree filters live |
| 2-5 | Click a directory header (▼/▶) | Children collapse / expand |
| 2-6 | Select a "Needs review" entry | Diff viewer and inspector update |
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
| 4-1 | Select an entry | Inspector shows path, diff type, status badge; Reason field and Strategy picker visible by default |
| 4-2 | Entry has `AuditWarning` | Yellow warning block shown below divider |
| 4-3 | Leave reason blank, press Ctrl+Enter | "Save and continue" button is disabled; validation error shown |
| 4-4 | Enter reason, press Ctrl+Enter | Entry approved and saved; inspector auto-advances to next item needing review |
| 4-5 | Save the last item needing review | Inspector stays on entry; background re-check marks it "All set"; dashboard shows "All entries are in order" with action buttons |
| 4-6 | Click "▸ More options" toggle | Ticket, Approved by, Expires at, Template, Note fields revealed |
| 4-7 | Enter ticket and approved_by in More options | Values saved in definition YAML |
| 4-8 | Enter expires_at in wrong format | Validation error shown; More options auto-expands |
| 4-9 | Click "▸ Use pattern" toggle | Pattern input appears pre-filled with current path; suggestion chips shown |
| 4-10 | Click a suggestion chip | Pattern input updated; ✓ valid indicator shown |
| 4-11 | Approve with an active glob pattern | Glob entry saved; background rerun marks all matching files OK |
| 4-12 | Apply a template | Strategy fields populated from template |
| 4-13 | Press Ctrl+Z after approval | Last approval undone; entry returns to "Needs review" |

---

## 5. Save / Re-run

| # | Step | Expected |
|---|---|---|
| 5-1 | Save an entry via bottom bar | Definition file saved automatically; no "Unsaved" indicator |
| 5-2 | Press Ctrl+S | Definition file saved (can also save without approving) |
| 5-3 | Press Ctrl+R | Audit re-runs; results refresh |
| 5-4 | Modify before/after files externally, Ctrl+R | Updated diff shown |

---

## 6. Settings & Footer

| # | Step | Expected |
|---|---|---|
| 6-1 | Click `⚙` (footer) | Settings dialog opens as a modal overlay |
| 6-2 | Switch language to Japanese in Settings, click Save | Labels change to Japanese; dialog closes |
| 6-3 | Add `dist` to Ignored Directories, click Save; re-run audit | `dist/` entries excluded from diff |
| 6-4 | Click `?` (footer) | Keyboard shortcuts overlay opens; `?` again or `Escape` closes it |
| 6-5 | Main screen footer | Shows `?` and `⚙` buttons and the version string |

---

## 7. Export & Reports

| # | Step | Expected |
|---|---|---|
| 7-1 | Press "Save report" (toolbar) | Native save-file dialog opens with `aaai-report.md` as default name |
| 7-2 | Save with `.json` extension | JSON file created; valid JSON |
| 7-3 | `aaai report --format html` | Valid HTML file with summary cards |
| 7-4 | `aaai report --format sarif` | Valid SARIF 2.1.0 JSON |

---

## 8. CLI Smoke Tests

```sh
# Audit — PASSED
aaai audit --left /tmp/aaai-test/before --right /tmp/aaai-test/after \
           --config /tmp/aaai-test/audit.yaml --no-history
# Expected exit 2 (entries needing review)

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
- [ ] All Inspector cases (4-1 to 4-13) pass
- [ ] All Save/Re-run cases (5-1 to 5-4) pass
- [ ] All Settings & Footer cases (6-1 to 6-5) pass
- [ ] All Export cases (7-1 to 7-4) pass
- [ ] All CLI smoke tests exit with expected codes
- [ ] No `cargo check --all-targets` warnings
- [ ] `cargo test -p aaai-core --lib` — 104 passing
- [ ] `cargo test -p aaai-cli -- --test-threads=1` — 89 passing
- [ ] `cargo test -p aaai-gui` — 20 passing

---

## 10. Visual Verification (RFC 017)

The acceptance criteria above tell you **what** to test. RFC 017
(`rfcs/proposed/017-visual-verification-harness.md`) tells you **how to
record that you actually did**.

When you finish the tester run for a given release, also produce a
**Visual Verification card** for each RFC whose UI/CLI surface you touched
during that release. The card is appended to the end of the relevant
`rfcs/done/<NNN>-<slug>.md` file. A copy-paste template lives at
`docs/templates/visual-verification-template.md`.

To see which RFCs are still missing a card, run from the repository root:

```sh
scripts/list-unverified-rfcs.sh
```

This script is run informationally on every CI build; failures of the
overall acceptance criteria above are not the same as missing verification
cards, but both should reach zero before tagging a release.

## 11. i18n Key Audit (RFC 018 §3.4)

Static counterpart to the visual verification above. Catches the failure
mode that RFC 016 traced (literal keys rendered in the GUI) before the
GUI is ever launched, by cross-checking every `t!()` call site against
the entries in `locales/en.yaml` and `locales/ja.yaml`.

```sh
scripts/check-i18n-keys.py            # full report
scripts/check-i18n-keys.py --quiet    # summary line only
scripts/check-i18n-keys.py --strict   # also fail on UNUSED entries
```

Exit code is 1 if any key is MISSING (referenced but absent from a
locale) or DIVERGENT (present in one locale but not the other). This
check runs as a blocking step in CI, unlike the visual-verification
reporter which is informational.

UNUSED entries (in a YAML but never called) are listed but do not fail
the build unless `--strict` is given — they are typically left-over from
removed UI features and should be cleaned up by the RFC that removed
the feature, not by an automated sweep.

## 12. ABDD verification (manual)

Accessible-by-Default Design checks per the design document p.8. The
checklist itself lives at `docs/src/abdd-audit.md` — a fresh sheet is
filled in once per release. The cases below describe the steps the
operator performs to fill that sheet.

| # | Step | Expected |
|---|---|---|
| 12-1 | Display in greyscale and run through Opening → audit → Inspector | Status icons `✓ ⚠ ✗ ! —` remain distinguishable from each other |
| 12-2 | From Opening, press Tab six times | Focus visits Before card → After card → optional toggle → optional fields → Start |
| 12-3 | From the main screen, press Tab through the toolbar | Open → Save → Run → Report, no skipped items |
| 12-4 | Click "Start audit" with an invalid Before path | Banner above the Start button shows a message line and a hint line (no silent failure) |
| 12-5 | Enter an invalid regex in the Inspector | Below the field: message line ("Invalid regex") + hint line explaining the next step |
| 12-6 | Hover toolbar / Start / Approve buttons | Click target is at least 44 px on the smaller dimension |
| 12-7 | Look at "Save and continue" versus any destructive (delete-rule) button | They are not adjacent and have distinct visual weight |

The operator transcribes the observations into the corresponding rows of
`docs/src/abdd-audit.md`, then marks the sheet as verified.

### Out of scope for v1.0

Screen-reader interoperability (NVDA / JAWS / VoiceOver / Orca) is not
verified for v1.0. iced 0.14 does not expose accessibility APIs to the
host platform. The ABDD sheet records this as a declared limitation in
its §7. A follow-up RFC will revisit when iced ships native a11y hooks.
