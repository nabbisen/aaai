# GUI Guide

Launch the desktop app:

```sh
aaai-gui
```

The app opens to the Opening screen. From there you pick the two
folders you want to compare, then drive an audit through the main
3-pane workspace.

---

## 1. Opening screen

The Opening screen is where every audit starts. It has three logical
parts: required folder selection at the top, optional settings in a
collapsible section, and (depending on what's already saved) either
a Recent-projects list or first-run onboarding at the bottom.

### Picking the two folders

Two folder cards sit at the top — **Before** (the baseline) and
**After** (what you're auditing). Each card has three states:

- **Empty.** Card shows `✗ Not selected` and a "Pick a folder" button.
- **Valid.** Card shows `✓ /path/to/folder` once a folder is chosen.
- **Invalid.** Card shows `⚠ <reason>` if the path no longer exists or
  isn't a directory.

Two ways to fill the cards:

1. **Click the button** — opens a native folder picker (OS-provided).
2. **Drag-and-drop** — drag a folder from your file manager anywhere
   onto the Opening screen. While a drag is active, a hint banner
   appears at the top. Drops fill the first empty card (Before first,
   then After). Dropping a *file* (not a folder) shows an inline
   error explaining the rule.

### Optional settings

Below the cards is a collapsible section ("Optional settings"). It
exposes two paths that the default behaviour will infer if you leave
them blank:

| Field | Purpose |
|---|---|
| `audit.yaml` path | Existing audit definition. Leave empty to start with a fresh empty definition. |
| `.aaaiignore` path | gitignore-style file specifying paths to skip. Leave empty to look for `<Before>/.aaaiignore` automatically. |

Once both folder cards are valid, the **Start audit** button enables.
Clicking it runs the folder comparison on a background thread (the
UI stays responsive even on large trees).

### Recent projects

If you've audited at least one project before, the bottom of the
Opening screen shows a "Recent projects" list. Entries are sorted by
**last used, most recent first**, with a short relative timestamp on
each row (e.g. "3 min ago", "2 d ago", or an ISO date after a week).
Clicking *Open* on a row fills the folder cards and any saved
optional settings, and updates that project's timestamp.

### First-run onboarding

If you've never saved a project, the Recent list is replaced by a
quick **Getting started** panel listing the three steps (pick Before,
pick After, click Start). It also notes that an `audit.yaml` file
will be created automatically on first save. The onboarding panel
disappears the first time you save a project.

### Error banner

If something goes wrong when starting an audit (a folder vanished
between picking and clicking Start, the audit.yaml file is malformed,
or the folder comparison failed), an error banner appears above the
Start button with two lines:

- A **message** line explaining what went wrong, in red.
- A **hint** line in grey explaining what to do next.

This is the same `message + hint` pattern used everywhere else in
the app for actionable errors.

---

## 2. Main screen — 3 panes

After Start, the workspace transitions to a 3-pane layout: the file
tree on the left, the diff view in the middle, and the inspector on
the right. The dividers between panes are draggable, so you can
adjust the proportions for your screen.

A toolbar runs across the top of the workspace, a filter bar and a
search bar sit below it, and an action bar runs across the bottom.

### Toolbar

The toolbar contains four buttons and a status badge:

| Button | Action |
|---|---|
| `□ Open` | Returns to the Opening screen. Warns if you have unsaved changes. |
| `□ Save` | Saves the audit definition to disk. |
| `▶ Run audit` | Re-runs the audit against the current diff and definition. |
| `↑ Export Report` | Emits a Markdown report to `aaai-report.md`. |

After a successful save or export, a small green checkmark with a
relative time appears next to that button — for example
`✓ Saved 2 min ago`. The label refreshes every 30 seconds as long
as one of those operations has run at least once. It clears the
next time the audit re-runs.

On the right side of the toolbar, an **Audit status** badge shows
**Passed** (green) or **Failed** (red) based on the most recent
audit run. The label uses text, not just colour, so the verdict
remains visible in greyscale or high-contrast modes.

### Filter bar

Below the toolbar, four buttons filter the file tree:

- **All** — show every entry
- **Changed only** — hide Unchanged entries
- **Pending** — entries that still need a reason and approval
- **Errors** — Failed and Error entries

The currently active filter has a subtle background tint.

### Search bar

To the right of the filter bar, the search field does an incremental
case-insensitive substring match on entry paths. Empty search shows
everything that the filter would show.

### File tree (left pane)

Each entry in the tree is one file or directory, shown with a status
icon, a name, and (where relevant) a small annotation:

| Icon | Status | Notes |
|---|---|---|
| ✓ | OK | Reason provided and audit passed |
| ⚠ | Pending | No reason yet — needs review |
| ✗ | Failed | Audit rules didn't match |
| ! | Error | Could not read or evaluate |
| — | Unchanged / Ignored | No action needed |

Directories show a `▼` / `▶` triangle and can be collapsed. The
status icons use symbols, not only colour — this is part of the
ABDD accessibility commitment.

If you haven't run an audit yet (e.g. you're between Opening and
Start), the file tree shows a small placeholder panel pointing you
at the toolbar's `▶ Run audit` button.

### Diff view (centre pane)

When a file is selected, the diff view shows its content
side-by-side. Three tabs above the diff switch the display mode:

| Tab | What it shows |
|---|---|
| **Side by side** | Left = Before, Right = After. Line numbers on each side. |
| **Unified** | Single column with `+` / `-` line markers. |
| **Changes only** | Hides unchanged context lines. |

Added lines get a green-tinted background; removed lines a red-tinted
one. Each line carries a `+` or `-` character at the start so the
mode is identifiable without colour.

For binary files (where line-by-line comparison doesn't apply), the
pane shows a small panel with SHA-256 hashes, file sizes, and
whether the contents match.

When no audit result is loaded and no file is selected, the centre
pane shows the **dashboard** instead: a summary card per status
(OK / Pending / Failed / Error / Ignored) and the top-priority items
that still need attention.

### Inspector (right pane)

The inspector is where you record *why* a change is allowed.
Selecting a file in the tree loads its current audit entry into the
inspector for editing; saving applies your changes to the in-memory
definition (and persists when you click Save in the toolbar).

The header shows the entry's path, change type (Added / Removed /
Modified / TypeChanged), and current status badge.

Below the header are the editable fields:

| Field | Required | Notes |
|---|---|---|
| Reason | ✅ | The point of the audit. Multi-line textarea. Empty = cannot approve. |
| Ticket | — | Link to an external tracker (e.g. `JIRA-123`). |
| Approved by | — | Name or ID of the reviewer. |
| Expires at | — | `YYYY-MM-DD` if this approval should sunset. |
| Strategy | — | None / Checksum / LineMatch / Regex / Exact. The fields below adapt to the strategy. |
| Note | — | Free-form notes; doesn't affect the verdict. |

Each strategy reveals a different set of strategy-specific inputs.
For example, Regex shows a pattern field with live validation (an
inline error appears below the field with a hint pointing to
[regex101.com](https://regex101.com) if the pattern won't compile).

If nothing is selected, the inspector shows a small placeholder
panel pointing back to the file tree.

### Bottom action bar

The bottom of the workspace has a single primary action button
**Approve & Save**. It's enabled only when the inspector has all
required fields filled and the strategy is valid. Clicking it
approves the current entry, saves the definition, and re-runs the
audit so the status badge updates.

---

## 3. Keyboard shortcuts

| Shortcut | Action |
|---|---|
| `Ctrl+S` | Save the definition file |
| `Ctrl+R` | Re-run the audit |
| `Ctrl+Z` | Undo the last approval |
| `Ctrl+E` | Export a Markdown report |
| `↑` / `↓` | Move selection in the file tree |
| `Enter` | Move focus to the inspector's Reason field |
| `Escape` | Deselect the current entry |

These shortcuts are stable in v1.x — see the
[Compatibility Policy](compatibility.md) for details.

---

## 4. Footer

The footer runs across the bottom of the window:

| Element | Meaning |
|---|---|
| `● Unsaved changes` | Shown when at least one approval hasn't been saved to disk yet |
| Shortcut legend | Compact reminder of the keyboard shortcuts above (Main screen only) |
| Language picker | Switch between Japanese and English |
| Version | The aaai-gui version |

---

## 5. Reports

The toolbar's **↑ Export Report** button emits a Markdown report to
`aaai-report.md` in the current directory. The same action is
bound to `Ctrl+E`.

For JSON output or for embedding into CI pipelines, use the CLI:

```sh
aaai report --format json --out aaai-report.json
```

See the [CLI Reference](cli.md) for the full set of report options.

---

## 6. A typical workflow

```
1. Launch aaai-gui.
2. Drag the Before folder onto the screen, drag After on top, click Start.
3. Look at the dashboard for an overall sense of what changed.
4. Pick a Pending entry from the file tree.
5. Review the diff in the centre pane.
6. Type a reason in the inspector, pick a strategy if needed.
7. Click Approve & Save (or press Enter then Ctrl+S).
8. Press Ctrl+R to re-run and confirm the status badge turns green.
9. Press Ctrl+E to export the Markdown report.
```

A small `✓ Saved Nm ago` mark next to the Save button will tell you
how recently the definition was persisted; the same applies to
Export. The audit-status badge at the right end of the toolbar
stays in sync with the current verdict.

---

## 7. Accessibility (ABDD)

aaai-gui follows the *Accessible by Default Design* principles
documented in the [ABDD Audit Sheet](abdd-audit.md):

- Status conveyed by symbols and text, not only colour
- Keyboard reaches every action; focus is visibly tracked
- Click targets are sized for touch (≥ 44 px)
- Error messages use the `message + hint` two-line pattern
- Primary and destructive actions are visually separated

Screen-reader interoperability is **not yet supported** in v1.0
because the underlying GUI toolkit (iced 0.14) doesn't expose
widget trees to the platform a11y APIs. The CLI is the
recommended path for screen-reader users until iced gains this
capability.
