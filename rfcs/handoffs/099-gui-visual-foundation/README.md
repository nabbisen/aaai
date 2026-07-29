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

## 7. Required assertions

1. Zero `.size(N)` literals; zero `Color::from_rgb` outside `design_tokens.rs`.
2. Every text role application carries its `line_height`.
3. Contrast test green for all four presets at the correct thresholds.
4. Test counts: 144 / 8 / 91 / **27** / 3.
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
