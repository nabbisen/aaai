# GUI Guide

Launch the desktop app:

```sh
aaai-gui
```

The app opens to the Opening screen. From there you pick the two
folders to compare, then drive an audit through the main 3-pane
workspace. The window title shows the current definition filename
and a `●` marker when there are unsaved changes.

---

## 1. Opening screen

The Opening screen is where every audit starts. It has three logical
parts: required folder selection at the top, optional settings in a
collapsible section, and (depending on what has already been saved)
either a Recent-projects list or first-run onboarding at the bottom.

### Picking the two folders

Two folder cards sit at the top — **Before** (the baseline) and
**After** (what you are auditing). Each card has three states:

- **Empty.** Card shows `✗ Not selected` and a "Pick a folder" button.
- **Valid.** Card shows `✓ /path/to/folder` once a folder is chosen.
- **Invalid.** Card shows `⚠ <reason>` if the path no longer exists or
  is not a directory.

Two ways to fill the cards:

1. **Click the button** — opens a native folder picker (OS-provided).
2. **Drag-and-drop** — drag a folder from your file manager anywhere
   onto the Opening screen. While a drag is active, a hint banner
   appears at the top. Drops fill the first empty card (Before first,
   then After). Dropping a *file* (not a folder) shows an inline error.

### Optional settings

Below the cards is a collapsible section labelled **Optional settings**.
Expand it to load an existing approvals file:

| Field | Purpose |
|---|---|
| **Approvals file** | An existing `audit.yaml` containing saved approvals for this folder pair. Leave empty to start fresh — the first `Ctrl+S` will ask you where to save it. |

Per-project path exclusions go in a `.aaaiignore` file in the Before
folder (auto-detected). Global directory exclusions (`.git`, `target`,
`node_modules`, …) are configured in **App Settings** (⚙).

Once both folder cards are valid, the **Start audit** button enables.
Clicking it saves the current paths as a profile (so they appear in
Recent Projects automatically) and runs the folder comparison on a
background thread — the UI stays responsive even on large trees.

### Recent projects

If you have run at least one audit before, the bottom of the Opening
screen shows a "Recent projects" list. Entries are sorted by
**last used, most recent first**, with a short relative timestamp on
each row (e.g. "3 min ago", "2 d ago"). Each row has two controls:

- **Open →** — fills the folder cards and any saved optional settings.
- **×** — removes the entry from the list.

Each time you click **Start audit**, the current paths are automatically
added or updated in the Recent Projects list — no manual "Save Profile"
step is needed.

### First-run onboarding

If you have never run an audit, the Recent list is replaced by a quick
**Getting started** panel listing the three steps. The panel disappears
the first time you run an audit.

### Error banner

If something goes wrong when starting an audit (a folder vanished,
the audit.yaml is malformed, or the diff failed), an error banner
appears above the Start button with two lines:

- A **message** line in red explaining what went wrong.
- A **hint** line in grey explaining what to do next.

---

## 2. Main screen — 3 panes

After Start, the workspace transitions to a 3-pane layout: the file
tree on the left, the diff view in the middle, and the inspector on
the right. The dividers between panes are draggable.

A toolbar runs across the top, a filter bar and a search bar sit
below it, and an action bar runs across the bottom.

### Toolbar

The toolbar contains four buttons and a status badge:

| Button | Action |
|---|---|
| `□ Open` | Returns to the Opening screen. If you have unsaved changes, a confirmation dialog appears — see [Navigation guard](#navigation-guard). |
| `□ Save` | Saves the audit definition to disk. |
| `▶ Run audit` | Re-runs the folder comparison and audit on a background thread. |
| `↑ Export Report` | Opens a native save-file dialog. Choose a filename ending in `.md` for Markdown or `.json` for JSON — the format is detected from the extension. |

After a successful save or export, a small `✓ Saved N min ago` mark
appears next to that button. The label refreshes every 30 seconds.

On the right side of the toolbar:

- While a rerun is in progress, the status badge shows **Re-running…** in amber.
- Otherwise it shows **Passed** (green) or **Failed** (red). The verdict is always text, not colour-only.

### Filter bar

Below the toolbar, four buttons filter the file tree. When an audit
result is loaded, each button shows the live entry count in parentheses:

- **All (N)** — every entry
- **Changed only (N)** — non-Unchanged entries
- **Pending (N)** — entries needing approval (includes expired approvals)
- **Failed & Error (N)** — Failed and Error entries

The active filter has a subtle background tint.

### Search bar

Incremental case-insensitive substring match on entry paths. Empty
search shows everything the current filter would show.

### File tree (left pane)

Each entry is shown with a status icon, a name, and annotations:

| Icon | Status | Notes |
|---|---|---|
| ✓ | OK | Approved and audit passed |
| ⚠ | Pending | No reason yet — or approval has expired |
| ✗ | Failed | Audit rules did not match |
| ! | Error | Could not read or evaluate |
| — | Unchanged / Ignored | No action needed |

Directories show `▼` / `▶` triangles and can be collapsed. Status
icons use symbols, not only colour.

### Diff view (centre pane)

When a file is selected, the diff view shows its content side-by-side.
Three tabs switch the display mode:

| Tab | What it shows |
|---|---|
| **Side by side** | Left = Before, Right = After. |
| **Unified** | Single column with `+` / `-` line markers. |
| **Changes only** | Hides unchanged context lines. |

Added lines have a green-tinted background with a `+` prefix;
removed lines a red-tinted background with a `-` prefix.

For binary files, the pane shows SHA-256 hashes, file sizes, and
whether the contents match.

When no file is selected, the centre pane shows the **dashboard**:
a summary card per status and the top-priority items needing attention.
When all entries are approved (Pending = 0), the dashboard shows
**Export Report** and **New Audit** action buttons instead of the
attention list.

### Inspector (right pane)

The inspector is where you record *why* a change is allowed. Selecting
a file in the tree loads its current audit entry for editing.

The header shows the entry path, change type, and status badge. If the
entry's **Expires at** date has passed, the header shows a red
**EXPIRED** badge — the entry's file-tree status will be Pending until
the approval is renewed.

The inspector uses **progressive disclosure**: essential fields are
shown by default; expert fields are hidden behind a **▸ More options**
toggle that stays expanded for the rest of the session once opened.

**Always visible:**

| Field | Required | Notes |
|---|---|---|
| Reason | ✅ | Multi-line textarea. Empty = cannot approve. |
| Strategy | — | None / Checksum / LineMatch / Regex / Exact. Fields adapt to the strategy. |

**Under ▸ More options:**

| Field | Notes |
|---|---|
| Ticket | Link to an external tracker (e.g. `JIRA-123`). |
| Approved by | Name or ID of the reviewer. |
| Expires at | `YYYY-MM-DD`. Once this date passes, the entry reverts to Pending automatically. |
| Note | Free-form notes; does not affect the verdict. |

For approved entries (OK status), a **Revert to Pending** button
appears below the strategy section. This removes the approval,
resets the entry to Pending, and triggers a background rerun. The
keyboard shortcut `Ctrl+Shift+Z` performs the same action.

### Bottom action bar

The **Approve & Save** button is enabled only when all required fields
are filled and the strategy is valid. Clicking it approves the entry,
saves the definition, and triggers a background rerun.

The right side of the bar shows `N of M unresolved`.

### Navigation guard

When you click **Open** with unsaved changes, a modal dialog appears:

- **Save and Leave** — saves then navigates to the Opening screen.
  Shows an error and stays if the save fails (e.g. no definition path is set).
- **Discard and Leave** — discards unsaved changes and navigates.
- **Cancel** — stays on the Main screen. Also: `Escape` or backdrop click.

---

## 3. Settings

The **⚙** button in the footer opens the Settings dialog. Settings
are persisted across sessions in `~/.aaai/prefs.yaml`.

| Setting | Description |
|---|---|
| **Language** | Switch the UI between English and Japanese. Takes effect immediately on Save. |
| **Ignored Directories** | Directory names excluded from every audit before any per-project `.aaaiignore` rules. Defaults: `.git`, `target`, `node_modules`, `.DS_Store`. |

Click **+ Add directory** to add a name (not a full path — just the
directory name, e.g. `dist`). Save applies the changes; Cancel discards.

---

## 4. Keyboard shortcuts

Press **?** or click the **?** footer button to open the shortcuts
overlay at any time on the Main screen.

| Shortcut | Action |
|---|---|
| `Ctrl+S` | Save the definition file |
| `Ctrl+R` | Re-run the audit |
| `Ctrl+Z` | Undo the last approval |
| `Ctrl+Shift+Z` | Revert the selected OK entry to Pending |
| `Ctrl+Enter` | Approve and save the current entry |
| `Ctrl+E` | Export report (opens save-file dialog) |
| `↑` / `↓` | Move selection in the file tree |
| `Tab` / `Shift+Tab` | Cycle pane focus |
| `Enter` | Move focus to the Reason field |
| `/` | Focus the search bar |
| `?` | Show / hide the keyboard shortcuts overlay |
| `Escape` | Close any open overlay, or deselect the current entry |

---

## 5. Footer

The footer runs across the bottom of the window:

| Element | Meaning |
|---|---|
| `● Unsaved changes` | Shown when approvals have not been saved to disk |
| `?` | Opens the keyboard shortcuts overlay |
| `⚙` | Opens the Settings dialog |
| Version | The aaai-gui version |

---

## 6. Reports

The toolbar's **↑ Export Report** button opens a native save-file
dialog. The format is determined by the extension you choose:

- `.md` — Markdown (human-readable, the default suggestion)
- `.json` — JSON (machine-readable, for CI pipelines and tooling)

The same action is bound to `Ctrl+E`.

For non-interactive environments such as CI/CD pipelines:

```sh
aaai report --left ./before --right ./after \
            --config audit.yaml --format json --out report.json
```

See the [CLI Reference](cli.md) for the full set of report options.

---

## 7. A typical workflow

```
1. Launch aaai-gui.
2. Drag the Before folder onto the screen, drag After on top, click Start.
3. The first Pending entry is auto-selected — the inspector is ready.
4. Review the diff in the centre pane.
5. Type a reason in the inspector (expand ▸ More options if needed).
6. Press Ctrl+Enter to approve and save.
7. The next Pending entry loads automatically — repeat from step 4.
8. When no Pending entries remain, the dashboard shows "Passed"
   with Export Report and New Audit action buttons.
```

The entire approval loop is keyboard-driven: no mouse needed after
step 2. Use `Enter` to focus the Reason field, `Ctrl+Enter` to
submit, and let the auto-advance carry you through the list.

---

## 8. Accessibility (ABDD)

aaai-gui follows the *Accessible by Default Design* principles
documented in the [ABDD Audit Sheet](abdd-audit.md):

- Status conveyed by symbols and text, not only colour
- Keyboard reaches every action; focus is visibly tracked
- Click targets sized for touch (≥ 44 px)
- Error messages use the `message + hint` two-line pattern
- Primary and destructive actions are visually separated

Screen-reader interoperability is **not yet supported** in v1.0
because iced 0.14 does not expose widget trees to platform
accessibility APIs. The CLI is the recommended path for
screen-reader users.
