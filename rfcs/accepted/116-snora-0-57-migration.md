# RFC 116 — snora 0.57 Migration

**Status.** **Accepted** — approved for implementation by the owner on
2026-11-09, with §5 settled as **X: ship `Theme::System`**, keeping iced's
`linux-theme-detection`, and keeping `Light` as the default. Lives in
`rfcs/accepted/`; the folder is the source of truth for this state (RFC 000).

**Tracks.** Dependency currency; prerequisite for RFC 106 (keyboard
operability), RFC 110 (modal overlays) and RFC 115 §S2b (target-size
remediation). Precedent: RFC 108 (the 0.37 migration).

**Depends.** RFC 115 unit 1 committed; the advisory lockfile update
(review `.git-exclude/reviewed/087-rfc115-unit1-review-2026-11-09.md` §6)
committed.

**Design owner.** high-capability model

**Decision owner.** nabbisen, project owner

**Proposed implementer.** mid-capability model

**Evidence location.** `.git-exclude/evidence/116-snora-0-57-migration/`

**Touches.** `Cargo.toml` (workspace dependencies), `Cargo.lock`,
`crates/aaai-gui/src/design_tokens.rs`, `crates/aaai-gui/src/app.rs` (toast
state), `crates/aaai-gui/src/app/update/dialogs.rs` (toast sweep),
`crates/aaai-gui/src/target_size_check/tests.rs` (one exemption retired),
`docs/src/threat-model.md` §6.2 and its Japanese copy, `CHANGELOG.md`.
§S2 only, if §5 chooses X: `crates/aaai/src/profile/prefs.rs`,
`crates/aaai-gui/src/app/subscription.rs`, `crates/aaai-gui/src/main.rs`.

**Handoff.** `rfcs/handoffs/116-snora-0-57-migration/README.md`.

---

## 1. Summary

We pin `snora = "0.38"`. The bump to a newer snora has been specified inside
RFC 106 §5a since 2026-08-20 and has moved six times. RFC 106 is now held on
its §6b, so the bump is held with it — while RFC 110 and RFC 115 also need it.

**This RFC detaches the bump** so it ships on its own, ahead of everything that
depends on it. RFC 106 §5a becomes a pointer here.

**Three things make 0.57 more than a version string:**

1. **It removes an API we call.** `snora::toast::sweep_expired` is gone in
   0.57.0; we call it at `crates/aaai-gui/src/app/update/dialogs.rs:180`.
   The bump cannot compile without the replacement (§S1.2).
2. **It hands us the target-size fix.** Every snora control now takes its
   minimum pointer target from a token, `Tokens::targets.min`, documented for
   exactly our case — *"`44.0` is the figure the platform guidelines use"*
   (`snora-design-0.57.0/src/targets.rs:1-27`). The toast close mark honours it
   on the design path we use (`snora-0.57.0/src/toast.rs:63-64,389`,
   `design/render.rs:207`). One line closes the dependency exemption RFC 115
   §S2 had to record (§S1.3).
3. **It makes buttons focusable** — snora's own controls are focus targets
   that Enter and Space press, with a focus ring (`snora-0.57.0/src/lib.rs`
   doc on `design`). That is what RFC 106 §6b found missing in iced 0.14, and
   it is RFC 106's business, not this RFC's; this RFC only keeps the feature
   that makes it reachable (§S1.1).

## 2. Why 0.57 and not an earlier stop

snora's letter of 2026-11-08
(`.git-exclude/upstream/snora/receive/0.57.0/snora-0.57.0-to-aaai-iced-features-answer-2026-11-08.md`):
0.55.0 shipped a regression — a click gave a snora control keyboard focus, so a
ring appeared after every press — fixed in 0.57.0. Stopping at 0.54–0.56 would
take the regression. **The version written here is a tested floor; re-check at
implementation time and read every migration guide in the span**, as RFC 106
§5a's standing instruction already required.

## 3. Goals and non-goals

### 3.1 Goals

- Bump to 0.57 with no behaviour we did not choose.
- Collect the target-size token fix.
- Keep the focusable-control path open for RFC 106.
- Settle the `linux-theme-detection` question on its merits (§5).

### 3.2 Non-goals

- **Migrating our buttons to snora controls.** That is RFC 106's rescope
  (§6b option d), and it touches every view.
- **RFC 110's overlay consolidation.** It follows this RFC.
- **Baselines.** RFC 115 regenerates them once, after its §S4. Captures here
  answer specific questions only.

## 4. Slice S1 — the migration

### S1.1 Manifest

```toml
snora = { version = "0.57", default-features = false, features = ["design", "widgets"] }
```

**`widgets` stays on**, reversing RFC 106 §5a.1's plan to drop it. That plan
was right when we imported nothing from it; under RFC 106 §6b option (d) we
will — `snora::design::button::{primary, secondary, ghost, danger, link}`, the
focusable text buttons, are `widgets`-gated (`snora-0.57.0/src/design.rs:103-111`).
If the owner rejects option (d), drop `widgets` here and nothing else changes.

`default-features = false` still drops `iced-default`, so iced's features come
from **our** `iced` line only — which §5 decides.

### S1.2 Toasts — forced by the removed API

`sweep_expired` → a `snora::toast::Holds` on `App`, swept with
`self.holds.sweep(&mut self.toasts, Instant::now())` at
`app/update/dialogs.rs:180`. Update the module doc at `crates/aaai-gui/src/app.rs:7`
that names `sweep_expired`. `Holds::hover` / `on_toast_hover` are deprecated
(removed in 0.59); **we call neither** — confirm by grep and say so.

**A behaviour change ships with this, and it is a good one.** Since 0.54.0 an
error toast stays until dismissed, and other toasts stay as long as their text
takes to read. For elderly readers that removes a timed message (WCAG 2.2.1),
and our file-path toasts get time in proportion to their length. It also means
**the dismiss `×` must be hit to clear an error** — which is why §S1.3 is in
this slice, not deferred. `CHANGELOG.md` entry required.

### S1.3 `targets.min = 44.0`

In `crates/aaai-gui/src/design_tokens.rs`, set `tokens.targets.min = 44.0` on
all four presets returned by `tokens_for()`. Every preset ships `24.0`, so
today this changes **no** size we draw ourselves — it raises every snora
control, and the toast `×`, to at least 44 × 44.

Then in `crates/aaai-gui/src/target_size_check/tests.rs`: **retire the toast
`×` external exemption** and replace it with an assertion that `tokens_for()`
yields `targets.min >= 44.0` for all four presets — the guarantee is snora's,
the token is ours, so the token is what we assert (RFC 110 §6b.2's rule:
assert our decisions, not our dependency's constants). Update
`docs/src/abdd-audit.md` §6 and the Japanese copy: the exemption row goes.

### S1.4 Carried over from RFC 106's handoff §3

After the bump, diff `Cargo.lock` and confirm `gpu-allocator` and `wgpu-hal`
still agree on `windows 0.58.0`; if a `windows 0.56` appears, the fix is
`cargo update -p gpu-allocator`. Another consumer hit ten Windows-only compile
errors from exactly that skew.

### S1.5 Re-runs

`contrast_check`, `theme::tests` (including the v0.35 pixel-identity lock),
`target_size_check`, `i18n-key-audit`, `cargo audit` — the advisory set must
match `docs/src/threat-model.md` §6 exactly, after §5's effect on the three
`zbus` entries.

## 5. The open question — `Theme::System`, and therefore `linux-theme-detection`

### 5.1 A premise in our decision log is wrong

DEC-010 reads *"iced 0.14 has no OS dark-mode query"*. **iced 0.14 has one,
public:** `iced::system::theme() -> Task<theme::Mode>` and
`iced::system::theme_changes() -> Subscription<theme::Mode>`
(`iced-0.14.0/src/lib.rs:602-604`). On Windows and macOS it comes from winit
natively. On Linux it exists **only with iced's `linux-theme-detection`
feature** — `mundy` asking the XDG desktop portal over D-Bus through `zbus` 5.

RFC 093 §5.1, which DEC-010 points to, was more careful — *"no **reliable**
OS-dark-mode query"* — and on Linux that judgement has substance: without a
portal, `mundy` reports `NoPreference`. But "unreliable on one platform" was
hardened into "absent" in the register, and Q-1's trigger ("ship `System` only
after OS detection exists") **has in fact already fired** on two of our three
platforms.

### 5.2 What the feature does at runtime, measured

From `iced_winit-0.14.0/src/lib.rs:524-551`, on Linux with the feature on:

- a **colour-scheme stream** over D-Bus, held for the life of the application;
- a **blocking query at startup, timeout 200 ms** (`once_blocking`), before the
  event loop begins.

Today we discard both: `crates/aaai-gui/src/main.rs:61` maps
`AppTheme::System` to a fixed `Light`. And the feature is what brings
`RUSTSEC-2026-0370/-0371/-0372` (unmaintained, not vulnerable) into the graph.

**Since snora 0.54 we can turn it off** — the coordination our threat model
§6.2 said was impossible was done by snora on our report.

### 5.3 Options

| | X — ship `Theme::System` | Y — drop the feature |
|---|---|---|
| What the user gets | A "Match system" theme that follows the OS light/dark setting, live | Nothing new; `System` stays hidden |
| Linux runtime | Portal stream kept; ≤ 200 ms startup wait when the portal is slow or absent | No D-Bus connection, no startup wait |
| Advisories | Three `zbus` notices stay, accepted by ID; they clear when iced moves to `zbus` 6 | Three notices gone |
| Manifest | Our `iced` line unchanged (defaults on) | Explicit iced feature list, plus a guard test that `iced_wgpu` and `iced_tiny_skia` stay and `zbus` is absent |
| Work | §S2 below — small | One manifest line plus the guard |

### 5.4 Recommendation — X (**decided 2026-11-09: X**)

**Ship `Theme::System`.** RFC 115 §4's lesson applies directly: *the
accommodation a user does not have to find is the best one.* An elderly user
who has set their OS to dark mode — common with glare sensitivity — gets it in
aaai without opening Settings at all. iced already does the hard part on every
platform we ship.

The cost is three **unmaintained, not vulnerable** notices that clear upstream,
and the same portal read every Linux desktop application performs. The 200 ms
wait occurs only when the portal does not answer, and then the result is
`NoPreference`, which maps to Light — exactly today's behaviour.

**Keep `Light` as the default.** Switching the default would change the look of
every existing install that never chose a theme; `System` is offered, not
imposed. Revisit after it has shipped.

Y is a sound choice if security surface outweighs the feature for you. It is
not wrong, only smaller.

### 5.5 Slice S2 — `Theme::System` (in scope: §5 decided X)

- `prefs.rs`: add `Theme::System` back to `choices()` (DEC-010's reversal;
  the variant and its i18n key `settings.theme_system` already exist).
- `subscription.rs`: subscribe `iced::system::theme_changes()`; seed it with
  `iced::system::theme()` at startup; store the `Mode` on `App`.
- **One resolver, used everywhere a concrete preset is needed.** *(Corrected
  2026-11-09, before handoff: an earlier draft said only "map `System` at
  `main.rs:61`", which would have rendered iced's chrome dark around aaai's own
  widgets still drawn in Light.)* Two consumers pick a preset today:
  `main.rs:61`'s `.theme(...)` **and** `design_tokens::tokens_for()`
  (`crates/aaai-gui/src/design_tokens.rs:15-23`, where `System` returns
  `Tokens::light()`), whose result is cached as `App::design_tokens` at
  `app.rs:200` and `app/update/dialogs.rs:6,115,133,145`. Add one function that
  takes the user's choice and the OS mode and returns a concrete preset —
  `System` with `Dark` → `Dark`, with `Light` or `None` → `Light`, every other
  choice unchanged — and route **both** consumers through it. **No other code
  may match on `System`.** When the OS mode changes while `System` is chosen,
  recompute `App::design_tokens`. **High-contrast is not inferred**: the portal
  reports light/dark only.
- `docs/src/threat-model.md` §6.2 and the Japanese copy: rewrite the reasoning.
  "We do not use the feature" and "we cannot switch it off" both stop being
  true; the decision to accept the three by ID stays, for the new reason.
- Decision log: DEC-010 superseded by this RFC; Q-1 closed.

## 6. Acceptance contract

| # | Condition |
|---|---|
| 1 | Builds and passes every gate on MSRV 1.91, `--locked`, three-OS B0 green |
| 2 | `sweep_expired` replaced by `Holds::sweep`; no deprecated snora item in use; `-D warnings` clean |
| 3 | `tokens_for()` sets `targets.min = 44.0` on all four presets, asserted |
| 4 | The toast `×` exemption is gone from `target_size_check` and from both `abdd-audit.md` copies |
| 5 | `gpu-allocator` / `wgpu-hal` agree on `windows 0.58.0` |
| 6 | `cargo audit` shows exactly the set in `docs/src/threat-model.md` §6. A trial bump (2026-11-09) dropped `rustybuzz` from the graph, so expect `RUSTSEC-2026-0206` to leave and remove its row from both copies |
| 7 | `theme::tests`' v0.35 pixel-identity lock still passes, or its failure is reported with snora's guide section that explains it — not re-baselined silently |
| 8 | Every migration guide 0.38 → 0.57 read; anything touching us reported, either way |
| 9 | A unit test covers the resolver for every choice, and `System` under each of the three OS modes; aaai's own widgets and iced's chrome resolve to the **same** preset; `NoPreference` renders Light. Live OS-switch evidence **only with the owner's agreement** — switching the desktop's colour scheme changes the owner's real setting, which the GUI rules forbid touching; otherwise the test plus review of the subscription wiring, and the report says which |
| 10 | `CHANGELOG.md` entries for the toast behaviour change and, if X, the new theme |

## 7. Risks

| Risk | Mitigation |
|---|---|
| Twenty minor versions of rendered change at once | Acceptance 7 and 8; snora's guides are per-step; captures answer named questions only |
| The v0.35 pixel lock fails for a legitimate upstream change | Report it with the guide section; the owner decides re-baselining, not the implementer |
| `widgets` kept for a path the owner then rejects | Dropping it is one word in §S1.1 |
| Windows-only compile skew | §S1.4 |

## 8. Sequencing

After RFC 115 unit 1 and the lockfile update; **before** RFC 106 (rescoped),
RFC 115 §S2b, and RFC 110 — each of which consumes this.

## 9. Sources

- `.git-exclude/upstream/snora/receive/0.57.0/snora-0.57.0-to-aaai-iced-features-answer-2026-11-08.md`
- `snora-design-0.57.0/src/targets.rs`; `snora-0.57.0/src/toast.rs:63-64,389`;
  `snora-0.57.0/src/design/render.rs:207`; `snora-0.57.0/src/design.rs:103-111`
- `snora-interaction-0.57.0/src/lib.rs` — the iced 0.14 `button` focus gap,
  stated by snora independently of RFC 106 §6b
- `iced-0.14.0/src/lib.rs:602-604`; `iced_winit-0.14.0/src/lib.rs:524-551`
- RFC 093 §5.1; DEC-010 / Q-1; RFC 106 §5a, §6b; RFC 108; RFC 115 §4, §S2
