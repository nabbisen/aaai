# RFC 099 — GUI visual foundation: developer handoff

Companion to [`RFC 099`](../../proposed/099-gui-visual-foundation.md). The RFC
records what was decided and why; this records how to implement and verify it
safely. It must not override the RFC — if execution uncovers a design conflict,
stop and amend the RFC first.

## 1. Authority and entry conditions

Implementation may begin only after **all** of:

- the high-capability model's design review accepts RFC 099 and this handoff;
- the owner explicitly approves implementation;
- a GUI/UX verification operator with a **real display** is available for T7 —
  Xvfb renders iced/wgpu panes black and is not acceptable evidence. This is a
  capability requirement; the owner may fill the role, and it gates T7 rather
  than blocking T1–T6;
- `main` is green on hosted B0 and the working tree is clean.

> **Owner decision of record, 2026-08-10 — T8 is approved and blocks the
> release.** nabbisen chose to fix D1 (RFC 099 §6a) before tagging v0.41.0,
> rather than shipping it as a known limitation. **T8 is therefore authorised
> to begin now**, and release unit 1 does not cut until T8 lands and T6
> re-runs green for the workspace screen in all four themes.
>
> This is the only outstanding item between here and V1. T1–T7 are otherwise
> complete, subject to the `contrast-results.md` gap in §6, which blocks
> RFC 099's move to `done/` but not the release.

## 2. Role split

| Role | Party | Scope |
|---|---|---|
| Implementer | GUI developer | T1–T6 |
| Verification operator | GUI/UX operator, real display | T7 screenshots |
| Architect | RFC 099 author | Consulted only for §4 mapping ambiguity; does not implement |
| Integrator | nabbisen | Reviews, commits, pushes, observes B0 |

## 3. Mandatory implementation boundary

**Only** `crates/aaai-gui/src/` may change: `design_tokens.rs`, `style.rs`,
`views/*.rs`, and the new contrast test.

**Must not:**

- add any dependency, or edit `Cargo.toml` / `Cargo.lock`;
- define a new type or spacing scale — use `snora`'s;
- change engine, CLI, persisted formats, public API, or message protocol;
- add, remove, or rename an i18n key;
- add a screen, widget, or feature — RFC 095 §11 defers UI expansion;
- change status-colour semantics (DEC-005, DEC-011);
- split `app.rs` or touch `views/mod.rs` — that is MG2;
- reintroduce any `.size(N)` literal or `Color::from_rgb` anywhere.

## 4. Role mapping — authoritative

Map by **purpose**, never by nearest number. A 9–11 px muted label becomes
`body_small` (14 px); the increase is the intent.

| Current site | Role | Token |
|---|---|---|
| Reason field, diff line text, primary explanatory copy, notice bodies | `body` | `tokens.typography.body` |
| Hints, counts, SHA-256 labels, size labels, timestamps, empty-state help, coach lines | `body_small` | `tokens.typography.body_small` |
| Buttons, pick-list values, chips, filter and tab labels, field labels | `label` | `tokens.typography.label` |
| Card, dialog, notice, and pane titles; inspector section titles | `title` | `tokens.typography.title` |
| Screen and section headings, dashboard headings | `heading` | `tokens.typography.heading` |
| Opening hero and empty-state hero (current 32 px and 48 px sites) | `display` | `tokens.typography.display` |

Every application passes both `size` **and** `line_height` from the role.
This includes `text_input` field content, not only `text` widgets — both
builders expose `.size()` and `.line_height()`; map a text input's role by
the content it holds (e.g. `body_small` for a compact form field), same as
any other text.

**Spacing:** map to the nearest **larger** step — 1–4 → `xs` (4); 5–8 → `sm`
(8); 9–12 → `md` (12); 13–16 → `lg` (16); 17–24 → `xl` (24); 25 and above →
`xxl` (32). `spacing(0)` stays 0 only where adjacency is deliberate, e.g.
elements sharing a border.

**If a site does not fit any row, stop and ask the architect.** Do not choose
by eye — drift across 11 files is exactly what produced the current state.

## 5. Developer sequence — one concern per commit

Three commits keep the ~300-site diff reviewable. Do not combine them.

### T1 — Colour (commit 1)

Replace all **128** `Color::from_rgb` sites with palette lookups.
**Start with `style.rs:32`** — `panel_style` hardcodes
`bg = Color::from_rgb(0.95, 0.96, 0.97)`, which is why the toolbar, filter bar,
and bottom action bar stay near-white in both dark themes.

Per-file counts to work through: `views/diff_view.rs` 30, `views/main_view.rs`
28, `views/inspector.rs` 26, `views/opening.rs` 25, `views/dashboard.rs` 5,
`views/settings_dialog.rs` 4, `views/help_overlay.rs` 4, `views/nav_guard.rs` 3,
`style.rs` 1, `views/batch.rs` 1, `app.rs` 1.

**Verify:** `grep -rn "Color::from_rgb" crates/aaai-gui/src/` returns nothing
outside `design_tokens.rs`.

### T2 — Typography and line height (commit 2)

Replace all **173** `.size(N)` literals per §4, passing `line_height` at every
site. The GUI currently references `line_height` zero times, so every text
block is at the renderer default — this is a large part of the readability
problem and is not optional.

**Verify:** `grep -rE "\.size\([0-9]" crates/aaai-gui/src/` returns nothing.

### T3 — Spacing (commit 3)

Replace `.spacing`/`.padding` literals per §4.

### T4 — Contrast test (same commit as T3)

Add a GUI unit test asserting, for each of the four presets, that every
text-on-surface pair reaches ≥ 4.5:1, and ≥ 7.0:1 for both high-contrast
presets. Use `snora::design::contrast::contrast_ratio` — **no new dependency,
no hand-rolled luminance maths.**

Seed it with the four pairs known to fail today (3.17, 3.71, 3.81, 4.13 : 1)
so the test demonstrably fails before T1 and passes after.

Place it per DEC-003 in a `tests.rs` sibling, not inline.

### T5 — Local verification

```sh
cargo +1.91 fmt --check -p aaai-gui
cargo +1.91 clippy -p aaai-gui --all-targets -- -D warnings
cargo +1.91 test --workspace --locked
python3 scripts/check-i18n-keys.py
git diff --check
git diff --stat
```

Expected: aaai 144, CLI unit 8, CLI integration 91, **GUI 26 + 1 new contrast
test = 27**, doctests 3. i18n clean, no key added or removed. `git diff --stat`
shows files only under `crates/aaai-gui/src/`.

### T6 — Layout check at minimum size

Run the GUI at **800 × 550** in all four themes. RFC 095 §7.1 item 7 requires
usability at that size, and body text has moved 11–12 px → 16 px.

If content clips, overlaps, or becomes scroll-trapped, remedies in order are:
reduce nesting; move secondary content behind progressive disclosure; allow
region scrolling. **Reintroducing smaller sizes is not a remedy** — if none of
the three suffices, stop per §8.

### T7 — Real-display evidence (verification operator)

Screenshots for **all four themes** at 800 × 550 and at a typical working size,
covering the opening screen, the three-pane workspace, the inspector, and the
diff view. Xvfb output is not acceptable.

> **Clarified 2026-08-10.** Those four are **surfaces, not screens.** The
> application has exactly two screens — `Screen::Opening` and `Screen::Main`
> (`crates/aaai-gui/src/app.rs`) — and the workspace, inspector, and diff view
> are three panes of one `PaneGrid` rendered simultaneously within `Main`
> (`views/main_view.rs`). **One `Main` capture with a file selected covers all
> three.** The matrix is therefore 4 themes × **2 screens** × 2 sizes = **16**,
> not 32. A file must be selected, or the inspector and diff panes render their
> empty states and the capture proves nothing about either. Established in
> `.git-exclude/reviewed/057-rfc099-t6-t7-addendum-completion-review-2026-08-10.md` §3.

### T6 / T7 addendum — tooling (added 2026-08-04)

Added by the architect after the T6 pass stalled. See
`.git-exclude/rules/gui-automation-niri-xdotool.md` for the general reference
and `.git-exclude/reviewed/056-rfc099-t6-t7-tooling-addendum-2026-08-04.md`
for the full procedure and reasoning.

**Two facts recorded in `layout-800x550.md` are wrong and must not be carried
forward.**

1. **"Synthetic mouse input failed to reach the app at all."** It does reach it.
   Verified 2026-08-04: a synthetic click on *Pick folder* opened the
   `org.gnome.Nautilus` chooser. The earlier attempt used `xdotool` against a
   **Wayland-native** window, which cannot work; launched under XWayland
   (`env -u WAYLAND_DISPLAY DISPLAY=:1`), mouse input works normally.
2. **"Tab moved focus but Return/space did not activate."** Not reproducible —
   five `Tab` presses produced five byte-identical screenshots. This was almost
   certainly `xdotool key --window`, which uses `XSendEvent` and is discarded by
   most toolkits. **Treat it as withdrawn, not as a suspected NF-4 defect**, and
   correct `layout-800x550.md` accordingly.

**The stated blocker is also gone.** The remaining three screens do not require
the folder dialog: the Opening screen already lists a saved profile under *Or
open a recent project* whose **Open** button (`Message::LoadProfile`) loads both
paths in one click. Note that the folder cards themselves have **no** text
input — the `text_input` in `views/opening.rs` belongs to the definition-file
row, not to folder selection.

**Working tooling split — use each tool only for what it is good at:**

| Job | Tool |
|---|---|
| Run the app so input can reach it | `env -u WAYLAND_DISPLAY DISPLAY=:1` |
| Window geometry | `niri msg action set-window-width/height --id` — **logical** units |
| Make geometry stick | `niri msg action move-window-to-floating --id` — required; tiling overrides size otherwise |
| Mouse input | `xdotool mousemove … click 1` |
| Capture | `niri msg action screenshot-window --id --path <absolute>` |

**Never size with `xdotool windowsize`** — it takes physical pixels, so on a
scaled output the app lays out at the wrong size while the PNG still measures
what you asked for. At scale 1.2, `xdotool windowsize 800 550` gives the
application a **667 × 459** window. A correct 800 × 550 logical window yields a
960 × 660 PNG; judge size from `niri msg windows`, never from the image.

**Prove the input channel before recording any finding.** These tools fail
silently, and a no-op is indistinguishable from a passing check. Record in the
evidence which channel was proven and how.

**T7's keyboard question is not settled by this tooling and remains open.**
`xdotool` keyboard (XTEST) does not reach the focused surface on this
compositor — confirmed by `Escape` failing to close a Wayland-native dialog. It
needs either real key presses or `wtype`, which is not installed. Do not report
a keyboard result obtained with `xdotool`.

**Role note.** §2 assigns T7 to the verification operator with a real display.
A `niri` screenshot of a physical output is a real display and satisfies the
"not Xvfb" intent, so the implementer may capture T7 screenshots. **The keyboard
portion still requires the operator.** This reading is the architect's; the
owner may overrule it.

## 6. Evidence package

Create `.git-exclude/evidence/099-gui-visual-foundation/`:

```
environment.md        toolchain, OS, display, snora version
scans.log             the T1/T2 greps, before and after
contrast-results.md   ratio per text-on-surface pair, per preset, before/after
local-results.md      fmt, clippy, test, i18n, diff --check outputs
layout-800x550.md     T6 findings and any remedy applied
screenshots/          T7, named <theme>-<screen>-<size>.png
scope.diffstat        final boundary
hosted-runs.md        the B0 run for the integrated SHA
```

`contrast-results.md` must show the four known failures failing beforehand and
passing afterwards — that is what proves the test is load-bearing.

> **Gap recorded 2026-08-10.** Four of these are **absent** from the evidence
> directory: `scans.log`, `contrast-results.md`, `local-results.md`, and
> `scope.diffstat`. They belong to the T1–T5 implementation, which predates the
> T6/T7 addendum, and the T6/T7 implementer correctly flagged rather than
> backfilled them. **`contrast-results.md` is the one that matters**: without
> the before/after ratios, nothing demonstrates the contrast test is
> load-bearing rather than merely green, and acceptance item 4 rests on it.
> This blocks RFC 099's move to `done/`; it does not block T8 or T6 re-run.

### T8 — fix the diff pane's horizontal overflow (added 2026-08-10)

Required by RFC 099 §6a and acceptance item 9. **The remedy is decided — apply
it, do not re-derive it.** §6a records the rejected alternatives and why.

> **Corrected 2026-08-10 after a first attempt broke the diff view.** This
> section originally named **three** sites. The third was wrong and the
> instruction to change it destroyed the diff body's rendering. See
> `.git-exclude/reviewed/059-rfc099-t8-implementation-review-2026-08-10.md`.
> If you have already implemented against the earlier wording, keep the tab bar
> and legend changes and revert `side_by_side` only.

Give a horizontal scroll direction to **two** sites in
`crates/aaai-gui/src/views/diff_view.rs`:

| Site | Currently |
|---|---|
| `build_tab_bar` (`:52`) | `row(tab_items)` in a `Length::Fill` container, no scroll |
| `diff_legend` (`:356`) | `legend_row` in a `Length::Fill` container, no scroll |

**Do not touch `side_by_side` (`:303`, `:310`).** Its body is not clipped — it
**wraps**, and a long line renders across many rows fully readable. Making its
`scrollable` `Direction::Both` requires the inner `column` to become
`Length::Shrink`, and each `diff_line` returns a `Length::Fill` container;
`Fill` inside `Shrink` collapses and **the diff body renders nothing at all**.
Verified by A/B against the pre-fix build.

`scrollable` is already imported and used in this file. **Do not** change tab
labels, add a widget type or dependency, introduce responsive mode-switching, or
alter the pane count — the first is an i18n workaround for a layout defect and
the rest are behaviour changes this RFC's §3 forbids.

RFC 069's scroll synchronisation (`DIFF_BEFORE_ID` / `DIFF_AFTER_ID`,
`Message::DiffBeforeScrolled` / `DiffAfterScrolled`) is untouched by the two
remaining sites, so the earlier escalation clause no longer applies.

Then re-run T6 for the workspace screen in all four themes and confirm **three**
things: `Changes only` reachable, `Added` reachable, and **the diff body still
renders** — that last one is the regression guard this round produced, and it is
the check that would have caught the first attempt. Acceptance items 1–4
unregressed. Update `layout-800x550.md` with the after state alongside the
recorded before state.

## 7. Required assertions

1. Zero `.size(N)` literals; zero `Color::from_rgb` outside `design_tokens.rs`.
2. Every text role application carries its `line_height`.
3. Contrast test green for all four presets at the correct thresholds.
4. Test counts: **145 / 13 / 97 / 27 / 3** — corrected 2026-08-10; the previous
   144 / 8 / 91 / 27 / 3 predates RFC 103, which moved three of them. **27** is
   the GUI figure and the only one this RFC can affect.
5. i18n key audit clean, zero key delta.
6. No diff outside `crates/aaai-gui/src/`.
7. Real-display screenshots exist for four themes × two sizes.

## 8. Stop and escalation conditions

Stop and request an RFC amendment when:

- 800 × 550 cannot be satisfied by the three §5 T6 remedies — **do not** adopt a
  compact scale locally; `Density::Compact` is documented "reserved; not
  resolved in v0.20" in snora 0.25.1 and no compact spacing values exist;
- a site fits no §4 mapping row;
- a token value would have to be overridden to reach AA;
- behaviour, i18n keys, or message protocol would have to change;
- a new dependency appears necessary;
- test counts move other than the single added contrast test;
- work expands toward `app.rs` splitting (MG2) or the guided flow (MG3).

## 9. Rollback

Before integration: discard the working tree. After integration: revert the
offending commit through the normal reviewed path and correct at a new SHA. The
three commits are independently revertible by design — colour, typography, and
spacing can each be rolled back without the others.
