# ABDD Audit Sheet — vX.Y.Z

> **Purpose.** This file is the per-release record of how the GUI meets the
> Accessible by Default Design (ABDD) check points from the design document
> `aaai_uiux_design.pdf` p.8. The operator running the GUI fills in the
> "Observed" and "Verdict" columns for each row; this is a complement to the
> RFC 017 visual-verification cards, focused specifically on accessibility.
>
> **How to use.**
>   1. Build the GUI: `cargo build -p aaai-gui --release`.
>   2. For each section below, drive the GUI to the screen named in the
>      "Where" column and check the expected behaviour.
>   3. Fill in **Observed** (one sentence) and **Verdict** with one of:
>      - `✅ pass`
>      - `❌ fail` — follow up with an issue or RFC; note ID in **Notes**
>      - `例外 / exception` — record reason in **Notes**; v1.0 still ships
>   4. Replace `vX.Y.Z` in the title with the verified version once done.
>
> **Scope.** v1.0.0 commits to keyboard completeness, colour-independent
> status display, and visible focus. Screen-reader interoperability is
> deferred to a v1.x release (see §7 below) because iced 0.14 lacks
> native AT-SPI / UIA / NSAccessibility hooks.

## Build verified

- **Version.** vX.Y.Z (git: `<short-sha>`)
- **Platform.** `<Ubuntu 24.04 / macOS 14 / Windows 11>`
- **Verifier.** `<name or handle>`
- **Date.** YYYY-MM-DD

---

## 1. Tab / Shift+Tab order matches reading order (design doc p.8)

| Screen | Expected order | Observed | Verdict | Notes |
|---|---|---|---|---|
| Opening | Before card → After card → optional toggle → optional 2 fields → Start | | | |
| Main toolbar | Open → Save → Run → Report | | | |
| File tree | First entry → next entry … (j/k or arrow keys also work) | | | |
| Inspector | reason → ticket → approved_by → expires → strategy fields … | | | |
| Bottom bar | Approve & Save → (last) | | | |

## 2. Status is distinguishable without colour (design doc p.8)

Run with monochrome / greyscale display (Linux: `xrandr --output <name> --gamma 1:1:1` or macOS: Settings → Accessibility → Display → Color filters → Greyscale).

| What | Expected | Observed | Verdict | Notes |
|---|---|---|---|---|
| File-tree status icons | `✓ ⚠ ✗ ! —` remain distinguishable from each other | | | |
| Toolbar audit-status badge | "Passed" / "Failed" text remains the deciding signal, not its colour | | | |
| Diff-view added/removed lines | `+` / `−` line-start characters remain present and used by readers | | | |
| Inspector LineMatch rule blocks | leading `+` / `−` and the `action:` label persist | | | |

## 3. Primary action vs destructive action distance (design doc p.8)

| Pair | Expected | Observed | Verdict | Notes |
|---|---|---|---|---|
| Approve & Save vs Discard | Separated; different visual weight; no mis-click cluster | | | |
| Approve & Save vs delete-rule | Different ribbons / not adjacent | | | |
| Save vs Re-run | Adjacent on toolbar but distinct labels and icons | | | |

## 4. Unsaved / failed / input-error state always visible (design doc p.8)

| State | Where shown | Observed | Verdict | Notes |
|---|---|---|---|---|
| Unsaved changes | Toolbar / window title / Save button state | | | |
| Audit verdict (Passed/Failed) | Toolbar right side, persistent | | | |
| Inspector input error (e.g. invalid regex) | Below the field, message + hint | | | |
| Opening path-not-found error | Banner above the Start button, message + hint | | | |

## 5. Reason-field example text avoids jargon (design doc p.8)

| Place | Expected (lay-person reads it) | Observed | Verdict | Notes |
|---|---|---|---|---|
| Inspector reason placeholder | "Why is this change allowed? E.g. 'Bumped port to match new firewall rule.'" or equivalent plain wording | | | |

## 6. Click targets are ≥ 44 px (design doc p.8 ABDD common rule)

Measure with the OS accessibility inspector or by hovering and noting feedback area.

| Element | Observed (px h × w) | Verdict | Notes |
|---|---|---|---|
| Toolbar buttons (Open / Save / Run / Report) | | | |
| Opening "Pick folder" buttons | | | |
| Approve & Save (bottom bar) | | | |
| Diff-tab buttons (Side by side / Unified / Changes only) | | | |
| File-tree entry rows | | | |

---

## 7. Screen-reader interoperability — v1.0 limitation

iced 0.14 does not expose its widget tree to platform accessibility APIs
(AT-SPI on Linux, UIA on Windows, NSAccessibility on macOS). Until iced
exposes such hooks, screen-reader users will not get a usable experience.

**v1.0.0 declares this a known limitation.** The `aaai` CLI remains the
recommended path for users who rely on screen readers.

When iced ships native a11y, re-evaluate this section and either fill it in
or open a follow-up RFC that drives the migration.

---

## Summary

| Section | Verdicts (✅ / ❌ / 例外) |
|---|---|
| 1. Tab order | 0 / 0 / 0 |
| 2. Colour independence | 0 / 0 / 0 |
| 3. Distance for destructive actions | 0 / 0 / 0 |
| 4. State visibility | 0 / 0 / 0 |
| 5. Reason-field plain language | 0 / 0 / 0 |
| 6. Click-target size | 0 / 0 / 0 |
| 7. Screen reader | n/a (declared limitation) |

**Overall verdict for this release.** `<pass / pass with exceptions / fail>`
