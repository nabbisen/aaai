# RFC 106 handoff — Keyboard Operability, rescoped

**RFC.** [`rfcs/accepted/106-keyboard-operability.md`](../../accepted/106-keyboard-operability.md)
— read **§6b** (why the original design could not work), **§6b.7** (option
(d), the 2026-11-14 and 2026-11-18 updates), and **§6c** (the rescoped
acceptance contract this handoff implements).
**Status.** Accepted; option (d) chosen 2026-11-09; extended to the six
pickers 2026-11-18. **This document replaces the handoff held on 2026-11-04**,
as that hold promised. Nothing in the old version applies.
**Builds on.** snora **0.65.0**, already on `main` (`aec37ff`). Re-check for a
newer release before you start, read its guide, and report either way.
**Evidence location.** `.git-exclude/evidence/106-keyboard-operability/`

## 0. Two units, reviewed separately

| Unit | What | Changes the UI? |
|---|---|---|
| **A** | The test foundation: the shared session script, tests that never read the operator's settings, and the headless level-2 harness | **No** |
| **B** | The migration: every button to `snora::design::control`, the six pickers to `snora::design::select`, Tab, F6, and the safe-choice focus on the unsaved-changes dialog | **Yes** |

**Do Unit A first and send it for review. Do not start Unit B until A is
approved.** B is proved by A's harness, so building B first would mean proving
it by eye.

**Commit locally and send each unit for review. Do not push either until it is
approved.** Nothing in this handoff is pre-approved.

---

## Unit A — the test foundation

### A0. Start from a working example

I built one level-2 cell end to end before writing this, in a throwaway
worktree:
`.git-exclude/evidence/106-keyboard-operability/architect-level2-probe/level2_probe.rs`,
and its output beside it. It renders the Opening screen headless, Light,
English, 1280 × 720, with a long recent-project path, in **0.56 s**, and lists
every text target's `bounds` against its `visible_bounds`. Of 26 labels it
found **four clipped**:

- **`Open`** (the recent project's button) — zero width, not visible. This is
  RFC 115 finding A, now caught automatically.
- **`×`** (delete the recent project) — zero width, not visible. **New: no
  screenshot or static check had seen it.**
- The long path text — laid out 151 px high, 47 px visible.
- An empty text node — zero width; find out what it is.

Two techniques in the probe are yours to keep:

- **Collecting every target.** `Simulator::find` stops at the first match, and
  the simulator's widget tree is private. A closure selector that records each
  `Candidate::Text` into a shared `Vec` and always returns `None` makes `find`
  walk the whole tree. It then returns `Err(SelectorNotFound)`, which is
  expected and ignored.
- **`ICED_TEST_BACKEND=tiny-skia`.** It needs no GPU, and it is the renderer
  aaai falls back to on real machines anyway.

**One thing in the probe you must not keep:** it isolates state with
`std::env::set_var("AAAI_TEST_STATE_DIR", …)` inside the test. That is unsafe
under parallel tests, and it is exactly what A2 replaces.

### A1. The shared session script

`.git-exclude/rules/project-instructions-gui-common.md`, *Test Optimization
Rules*: start every real-window session with **the project's shared session
script**. Promote your RFC 115 `launch.sh` to
**`.git-exclude/tools/gui-session.sh`** (git-ignored, beside the evidence it
produces). It does **mechanics only**: isolation, launch, window choice,
capture, teardown. **It never decides which evidence a task needs** — that is
the task's evidence plan (RFC 105 §5.4's warning).

What it must do — most of it you already wrote:

- **Isolation:** a fresh scratch root per run, with `AAAI_TEST_STATE_DIR`
  pointing into it (`crates/aaai/src/user_state.rs:8,17` — it is validated and
  works on every platform) **and** the four `XDG_*` directories (the common rule
  asks for them, and they cost nothing). Pin theme and locale through the
  isolated `prefs.yaml`.
- **One instance:** abort if `aaai-gui` is already running; kill by exact
  process name on teardown, on every exit path (`trap`).
- **Backend:** X11 (XWayland) by default, for pointer input; a `--wayland` mode
  for keyboard runs with `wtype`.
- **Window:** chosen by niri window id, by the before/after diff. Record the
  **actual** window size into the run's log, because niri ignores the 1280 × 720
  request for a tiled window.
- **Capture:** `niri msg action screenshot-window --id … --path <absolute>`, into
  `.git-exclude/evidence/<NNN>-<slug>/screenshots/`.
- **Fixtures:** the long path that found finding A, and a Japanese profile.
  Keep the fixture path configurable, not hard-coded to your scratch directory.

**Header comment:** the aaai facts the common rule file no longer carries —
`AAAI_TEST_STATE_DIR`; that the Recent-projects list lives in the **config**
directory; that iced 0.14 publishes no AT-SPI tree; the evidence location. A
dozen lines, and they describe what the script does.

### A2. Tests never read the operator's settings (T0b)

`App::default()` calls `UserPrefs::load()` and `ProfileStore::load()`, which
read the real `prefs.yaml` and `profiles.yaml`. That is gate **S1 (tests never
touch the operator's real config)**: a read-only gap, recorded in RFC 104 §7a,
and it must close **before** the harness, which builds `App` once per matrix
cell.

RFC 104 §7a already names the shape: **an injection point on `App`**. Build an
`App` from a `UserPrefs` and a `ProfileStore` you pass in. `Default` keeps
loading from disk and calls it, so the running application is unchanged.
`UserPrefs::load_from(&paths)` and `ProfileStore::load_from(&paths)` exist. The
name and exact signature are yours.

Then move **every** test that builds an `App` onto it — today the RFC 098 and
RFC 116 tests in `crates/aaai-gui/src/app/tests.rs`, whether they call
`App::default()` or use `..Default::default()`. **Done when** no test in
`crates/aaai-gui` reaches `UserPrefs::load()` or `ProfileStore::load()`, and you
say in the report how you established that.

### A3. The level-2 harness

`iced_test = "0.14"` as a **dev-dependency** of `crates/aaai-gui`; report the
build-time and test-time delta. A module beside `contrast_check` and
`target_size_check`, in their shape: a documented header, named exemptions,
traceability.

**The matrix:**

| Axis | Values |
|---|---|
| Language | `en`, `ja` |
| Theme | the four presets — `Light`, `Dark`, `HighContrastLight`, `HighContrastDark` |
| Window | **800 × 550** (the smallest RFC 099 T8 made usable) and **1280 × 720** (the default) |
| Text size | one value until RFC 115 §S3 adds the others — **build the axis now**, with one entry |

**The screens**, each in a state that exercises it:

- Opening — empty, and with a recent project whose path is long;
- Main — with an audit result (reuse the existing test fixtures);
- the Settings dialog, the Help overlay, the unsaved-changes dialog;
- the Dashboard.

**What each cell asserts:**

1. **Every label is fully visible.** Every `Text` target's `visible_bounds`
   equals its `bounds`, to 0.5 px.
2. **Nothing overflows.** Each `Scrollable`'s content is reachable.

Hit area is **not** asserted in Unit A: our buttons are not focusable yet, so
there is nothing to measure. It arrives in Unit B.

**It will fail on day one.** The probe already shows four failures in one cell.
Land it **as a ratchet**, exactly the pattern you built for `target_size_check`:
a known-debt list keyed by screen, cell and label, which may only shrink — a new
clipped label fails, a listed one that passes must be removed. Finding A and the
`×` go on it, and RFC 115 §S2b removes them.

### A4. Japanese must be measured with Japanese glyphs

aaai bundles no fonts (`crates/aaai-gui/src/main.rs` loads none); it relies on
the system's. The simulator loads Fira Sans by default. **If no CJK font is
present, Japanese text is measured as missing-glyph boxes, and the `ja` cells
pass or fail on numbers that mean nothing.** GitHub's Ubuntu runner does not
ship a CJK font; macOS and Windows do.

**Required:**

- The harness **fails loudly** when it cannot find a font that covers Japanese
  — never measure `ja` silently. How you detect coverage is yours to choose;
  explain it.
- `ci.yaml` installs a CJK font on Ubuntu (`fonts-noto-cjk` is the obvious
  package). This **is a workflow change, so it is part of the review.**
- Report the font each OS actually resolved for `ja`.

### A5. Unit A evidence plan

| # | Question | Level | How |
|---|---|---|---|
| 1 | Does no test read the operator's settings? | 1 | A2's established check |
| 2 | Does the harness see a clipped label? | 2 | The finding A cell — it must report `Open` and `×` |
| 3 | Is `ja` measured with real glyphs, on all three OSes? | 2 | A4's loud failure, and the resolved font in CI's log |
| 4 | Is the matrix fast enough for every CI run? | 2 | Report its wall time locally and in CI |
| 5 | Does the session script work? | 3 | **One** run: launch, capture the Opening screen, tear down. Record the actual window size. No other capture in this unit |

**Done when:** the full suite, fmt, clippy `-D warnings` are clean on
`+1.91 --locked`; the harness passes with its known-debt list; no test reads the
operator's settings. **Commit locally, send for review, do not push.**

---

## Unit B — the migration (after A is approved)

### B1. Every button becomes a focusable control

`snora::design::control(&tokens, content)` — the builder of iced's `button`, so
each `button(content)` becomes `snora::design::control(&tokens, content)`, and
`.on_press`, `.padding` and `.style` stay as they are
(`snora-0.65.0/src/design.rs`; snora's 0.65 letter shows the before and
after). It is a Tab stop; Enter and Space press it; a click presses it without
focusing it; the ring comes from `Tokens::focus`; the minimum target comes from
our `targets.min = 44.0`.

- **Every `button(` site in `crates/aaai-gui/src/views/` and
  `crates/aaai-gui/src/app/view.rs`.** Glyph-and-label rows, file-tree rows and
  suggestion chips included — snora built `control` against exactly those three
  cases.
- **Keep each control's current style closure.** Without `.style(…)` a control
  looks like `ghost`. That is right for a tree row and wrong for *Save and
  continue*.
- **Label size:** a control built to look like a `design::button` variant needs
  its label at `t.typography.label.size`, or it draws larger.
- **The three hand-rolled modal backdrops stay iced `mouse_area`s.** They are
  RFC 110's, and they are not targets.

### B2. The six pickers become `snora::design::select`

`snora::design::select(&tokens, options, selected, on_select,
LayoutDirection::Ltr)` (`snora-widgets-0.65.0/src/design/form.rs:55`) — the shape
of `pick_list`, plus the tokens and a direction:

- `views/settings_dialog.rs:57` (language) and `:202` (**theme** — the route to
  the high-contrast themes and *Match system*);
- `views/inspector.rs:299` (strategy), `:312` (template), `:680` (LineMatch
  action), `:816` (Regex target).

Re-measure those line numbers first. The two `.text_size(11)` / `.text_size(13)`
literals go with them: use a typography role.

### B3. Tab, F6, Enter

- **Tab / Shift+Tab:** `snora::keyboard::traverse(key, modifiers)` in the
  keyboard subscription — iced binds no Tab. **Delete the interception RFC 106
  §2.2 documents**: `Tab` → `Message::FocusNext`, the pane-cycling enum.
- **F6 / Shift+F6** (RFC 106 §5's decision): `snora::keyboard::cycle_zones` +
  `snora::keyboard::focus_zone` (the 0.61 replacement for the removed
  `next_zone`). Keep `FocusTarget`'s three-pane model; only the trigger moves.
  **One new i18n key** for the help overlay's entry, in both locales.
- **Enter** stops being swallowed globally. `Message::FocusInspectorReason`'s
  global `Enter` binding goes; activating the focused control is the control's
  job. `Ctrl+Enter` still reaches the app: since snora 0.62, a chord does not
  press a focused control.

### B4. The unsaved-changes dialog opens on its safe choice

Give **Keep reviewing** an id (`.id("nav-guard-keep")` or similar), and in the
update that opens the dialog, return
`iced::widget::operation::focus(<that id>)`. A reflexive Enter then keeps the
user reviewing, rather than reaching RFC 086's discard. snora's note: the task
runs on the new view; an id must be unique in the window; an unreachable id
focuses nothing.

### B5. What Unit B adds to the harness

1. **Hit area, measured.** Every `Focusable` target is at least 44 × 44, in
   every cell. This is the measurement `target_size_check` could only estimate
   for width. **When a control passes it, delete its entry from
   `target_size_check`'s known-debt list**, as that ratchet requires. Report
   how many left.
2. **Tab reaches every control, in visual order.** On each screen, tap Tab
   repeatedly and read `iced_test::selector::is_focused()` after each press.
   The focused targets' bounds must visit every control once, in reading order
   (top to bottom, then start to end). snora's letter says its controls report
   through `is_focused`. **If they do not, stop and report** rather than falling
   back to inferring order from messages.
3. **The dialog opens focused on Keep reviewing.**
4. **The focus ring, SC 1.4.11 (3:1 against the adjacent colour).** snora
   guarantees the ring at 3:1 or more against the page, `surface` and
   `surface_raised` from 0.63. Assert that pair at level 1 in all four presets,
   in `contrast_check`'s shape — it is cheap, and it fails if we ever override
   a token. **List any control you place on a coloured fill of our own**, which
   snora does not cover, and assert those pairs too.

### B6. Recorded, not solved

Under our three **hand-rolled** dialogs, Tab can still reach the page behind
them. snora's controls are out of reach beneath **snora's** dialogs, and ours
are not snora's until RFC 110 moves them. **Do not build a trap.** The keyboard
help overlay must not claim modal focus containment (RFC 106 §5b). Say in your
report which dialogs are affected.

### B7. Unit B evidence plan

| # | Question | Level | How |
|---|---|---|---|
| 1 | Does every control take focus and respond to Enter / Space? | 2 | B5.2 plus an Enter per screen, checked by message |
| 2 | Is Tab order the reading order? | 2 | B5.2 |
| 3 | Is every focusable target ≥ 44 px? | 2 | B5.1 |
| 4 | Does the dialog open on Keep reviewing? | 2 | B5.3 |
| 5 | Is the focus ring visible enough? | 1 | B5.4 |
| 6 | Does the ring *read* well on a real screen? | 3 | **One** capture through the session script: Settings dialog, theme picker focused, High Contrast Dark, Japanese, 800 × 550 if the compositor gives it. That is the worst case for this question |

## Acceptance

RFC 106 §6c is the contract. Unconditionally: `cargo +1.91 fmt --check --all`,
`clippy --workspace --all-targets -- -D warnings`, the full suite on
`--locked`, no `unsafe`, and three-OS B0 green — after approval and push.

## Out of scope

- **RFC 110's backdrops** (B1, B6).
- **Screen-reader support** — iced 0.14 publishes no accessibility tree.
  `docs/src/abdd-audit.md` §7.
- **RFC 115 §S2b's fixes** to the clipped controls the harness finds. Unit A
  records them as debt; §S2b removes them.
- **New shortcuts** beyond `F6` / `Shift+F6`.

## A standing check

**Every line number and count in this document is stale until you re-measure it
on current `main`.** Report a wrong one; do not work around it.
