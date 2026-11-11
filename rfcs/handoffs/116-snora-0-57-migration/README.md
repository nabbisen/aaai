# RFC 116 handoff — snora 0.57 migration

**RFC.** [`rfcs/accepted/116-snora-0-57-migration.md`](../../accepted/116-snora-0-57-migration.md)
**Status.** Accepted 2026-11-09; §5 decided **X — ship `Theme::System`**,
keep iced's `linux-theme-detection`, default stays `Light`.
**Evidence location.** `.git-exclude/evidence/116-snora-0-57-migration/`

## 0. When to start

**After** these three are committed and B0-green, in this order (review
`.git-exclude/reviewed/087-rfc115-unit1-review-2026-11-09.md` §9):

1. RFC 115 unit 1, commit 1 (S0 + S1);
2. RFC 115 unit 1, commit 2 (S2, after its corrections and my re-review);
3. the four-advisory lockfile update (review 087 §6).

Then this unit. **Do not start RFC 106 or RFC 115 §S2b** — both consume this
unit and get their own handoffs after it lands.

## 1. What I already checked, so you know what to expect

I ran a **trial bump in a throwaway worktree of `8d677d1`** — not in this tree —
to scope this handoff from measurement rather than from snora's notes. The
trial does not contain unit 1 or the lockfile update, so re-check everything;
treat this as a forecast.

- `cargo +1.91 check -p aaai-gui --all-targets` failed with **exactly one
  error**: `E0425: cannot find function sweep_expired in module snora::toast`
  at `crates/aaai-gui/src/app/update/dialogs.rs:180`.
- With that replaced (§2.2, three lines), **`clippy -D warnings` was clean and
  all 30 then-existing GUI tests passed**, including `contrast_check` and
  `theme::tests::light_theme_pixels_identical_to_v035_constants`.
- `gpu-allocator` and `wgpu-hal` both resolved to **`windows 0.58.0`** — the
  Windows-only skew another consumer hit did not appear.
- `cargo audit` lost `rustybuzz` (`RUSTSEC-2026-0206`): the graph fell from 570
  to 533 crates.

If your run differs from any of this, that is a finding — report it.

## 2. Slice S1 — the migration

### 2.1 Manifest and lockfile

Workspace `Cargo.toml`:

```toml
snora = { version = "0.57", default-features = false, features = ["design", "widgets"] }
```

**`widgets` stays on** — RFC 106's chosen option (d) uses
`snora::design::button::*`, which it gates. **Do not touch the `iced` line**:
with snora's defaults off, iced's features come from ours, and ours keeps
iced's defaults, which is what §5's decision X needs.

`cargo update -p snora`, then `cargo +1.91 build --locked` from then on.
**Re-check for a release newer than 0.57 first**, read every migration guide
from 0.38 to the version you take, and report what you find either way.

### 2.2 Toasts — the one forced change

In the trial this was the whole fix:

- `App` (`crates/aaai-gui/src/app.rs`) gains a `snora::toast::Holds` field,
  initialised with `Holds::default()`.
- `app/update/dialogs.rs:180`:
  `snora::toast::sweep_expired(&mut self.toasts, Instant::now())` →
  `self.<holds>.sweep(&mut self.toasts, Instant::now())`.
- `app.rs:7`'s module doc names `sweep_expired`; update it.

**Confirm by grep** that we call neither `Holds::hover` nor `on_toast_hover`
(deprecated, removed in 0.59), and say so in your report.

**Behaviour change to record in `CHANGELOG.md`:** since snora 0.54 an error
toast stays until it is dismissed, and other toasts stay as long as their text
takes to read. Write it as a user-facing change.

### 2.3 `targets.min = 44.0`

In `crates/aaai-gui/src/design_tokens.rs`, every preset `tokens_for()` returns
gets `targets.min = 44.0` (the field is public; snora documents exactly this,
`snora-design-0.57.0/src/targets.rs:1-27`).

Then, in `crates/aaai-gui/src/target_size_check/tests.rs`:

- **retire the toast `×` external exemption** — on the design path snora sizes
  the toast close mark from `targets.min` (`snora-0.57.0/src/toast.rs:63-64,389`);
- **add a test** that `tokens_for()` yields `targets.min >= 44.0` for all four
  presets. The guarantee is snora's; the token is ours, so the token is what we
  assert.

`docs/src/abdd-audit.md` §6 and `docs/ja/src/abdd-audit.md` §6: remove the
toast row from the exemptions; state that snora controls take their minimum
target from our token. **Also correct the *Current state* paragraph:** it says
*"snora 0.57 sets a 44 px minimum on its own controls"*. Snora provides the
`targets.min` token and ships it at 24; **we** set it to 44. The same fix
applies to the toast row's closing parenthesis, which goes with the row
(review `.git-exclude/reviewed/088-rfc115-s2-ratchet-re-review-2026-11-09.md`).

**Expect the known-debt list to be unchanged.** `targets.min` affects snora's
controls only, and today none of our own buttons is one — they are iced
buttons until RFC 106. If any debt entry's height changes, report it.

### 2.4 Re-runs and the advisory register

All gates on `+1.91 --locked`: fmt, clippy `-D warnings`, the full workspace
test suite, `i18n-key-audit`, and three-OS B0 after push.

`cargo audit` must show **exactly** the set in `docs/src/threat-model.md` §6.
Expect `RUSTSEC-2026-0206` (`rustybuzz`) to leave: **remove its row** from the
§6 table in both language copies and correct the table's count sentence.

**The v0.35 pixel-identity lock:** if it fails, **do not re-baseline it.**
Report it with the snora guide section that explains the change; the owner
decides.

## 3. Slice S2 — `Theme::System`

### 3.1 The design point that matters

**Two places pick a preset today**, and both must agree:

1. `crates/aaai-gui/src/main.rs:61` — iced's `.theme(...)`;
2. `design_tokens::tokens_for()` (`design_tokens.rs:15-23`), cached as
   `App::design_tokens` at `app.rs:200` and `app/update/dialogs.rs:6,115,133,145`.

Today both map `System` to Light. Fix only the first and iced's chrome turns
dark around aaai's widgets still drawn Light — the RFC's own first draft made
exactly that mistake.

**Write one resolver** — your choice and the OS mode in, a concrete preset out:
`System` + `Dark` → `Dark`; `System` + `Light` or `None` → `Light`; every other
choice unchanged. Put it beside `tokens_for()` in `design_tokens.rs`, and route
**both** consumers through it. **After this change no code but the resolver
matches on `Theme::System`.** Say in your report how you checked that.

### 3.2 Wiring

- **Subscribe** to `iced::system::theme_changes()`
  (`iced-0.14.0/src/lib.rs:602-604`) and store the latest `theme::Mode` on
  `App`. One new `Message` variant for it.
- **Seed it at startup** with `iced::system::theme()`. Our builder is
  `iced::application(App::default, ...)`; if a startup `Task` needs a boot
  function instead of `App::default`, that change is in scope. If iced offers a
  cleaner way you can show works, use it and explain.
- **When the mode changes and the choice is `System`**, recompute
  `App::design_tokens` through the resolver.
- **`crates/aaai/src/profile/prefs.rs`:** add `Theme::System` back to
  `choices()`, third — after `Dark`, before the two high-contrast presets — and
  rewrite the doc comment that explains why it was excluded. The variant and
  its i18n key `settings.theme_system` (`Match system` / `システムに合わせる`)
  already exist. **The default stays `Light`.**
- **High-contrast is never inferred.** The portal reports light or dark only.

### 3.3 Tests and evidence

- A **unit test of the resolver**: every choice, and `System` under each of the
  three OS modes.
- A test that the iced theme and the token preset resolve to the **same** preset
  for the same inputs.
- **Live evidence of an OS switch only with the owner's agreement.** Switching
  the desktop's colour scheme changes the owner's real setting, which
  `.git-exclude/rules/project-instructions-gui-common.md` forbids touching. If
  you have not been given that agreement, the tests above plus my review of the
  subscription wiring are the evidence — and your report says which you used.
- `CHANGELOG.md`: the new "Match system" theme, user-facing.

### 3.4 Documents

`docs/src/threat-model.md` §6.2 and `docs/ja/src/threat-model.md` §6.2: rewrite
the reasoning. Two of its sentences stop being true — *"we do not use the
feature"* and *"we cannot switch it off"* (snora 0.54 made it switchable). The
decision stays — **accept the three `zbus` notices by ID** — for the new reason:
the feature now powers `Theme::System` on Linux. Keep it short; RFC 116 §5 is
the record and the threat model points to it.

**The decision log is mine**, not yours: I update DEC-010 and Q-1 when this
ships.

## 3a. Evidence plan

Required by `.git-exclude/rules/project-instructions-gui-common.md`
(*Test Optimization Rules*): the questions, the level each is checked at, and
the captures that remain. **The headless level-2 harness does not exist yet** —
RFC 106 brings it — so this unit uses level 1 wherever it can prove the
property and level 3 only where nothing else can.

| # | Question | Level | How |
|---|---|---|---|
| 1 | Does it build and behave on 0.57? | 1 | The full suite, fmt, clippy `-D warnings`, three-OS B0 |
| 2 | Do toasts still expire, and do errors now stay? | 1 | A unit test on the toast state through `Holds::sweep`: an info toast is gone after its hold; an error toast is still present |
| 3 | Is every snora control at least 44 px? | 1 | The `targets.min >= 44.0` assertion over all four presets (§2.3) |
| 4 | Did snora's rendering move under our colour and pixel guarantees? | 1 | `contrast_check` and the v0.35 pixel-identity lock. A failure is reported, not re-baselined |
| 5 | Does `System` pick the right preset, and do iced's frame and our widgets agree? | 1 | The resolver tests (§3.3) |
| 6 | Does `System` follow a live OS switch? | 3 | **Only with the owner's agreement** (§3.3); otherwise questions 5 and the wiring review stand in for it |
| 7 | Is a long file path in a toast fully readable now that snora wraps it (0.52)? | 3 | **One capture, the worst case:** Japanese, the longest path your fixture can produce, 800 × 550 if the compositor gives it — record the actual size otherwise. No baseline: nothing is being compared |

**Question 7 is the only capture in this unit.** Write down what you will look
for before you take it, and abort at the first anomaly. When RFC 106 builds the
level-2 harness, question 7 becomes an automated check — note it in your report
so it is not forgotten.

## 4. Acceptance

RFC 116 §6, all ten conditions. Unconditionally: `cargo +1.91 fmt --check
--all`, `clippy --workspace --all-targets -- -D warnings`, no `unsafe`, full
workflow green, three-OS B0 green after push.

## 5. Suggested commits

1. **S1** — manifest, lockfile, `Holds`, `targets.min`, the retired exemption
   and its test, both `abdd-audit.md` copies, the threat-model §6 table, the
   toast changelog entry.
2. **S2** — the resolver, wiring, `choices()`, tests, threat-model §6.2 in both
   languages, the theme changelog entry.

Explicit paths only, never `git add -A`. Push each; B0 between them.

## 6. Report back

1. The release you took and every guide you read, with anything that touched us.
2. Whether the trial's forecast in §1 held, item by item.
3. The `cargo audit` output after S1.
4. How you established that only the resolver matches `Theme::System`.
5. Which §3.3 evidence route you used.
