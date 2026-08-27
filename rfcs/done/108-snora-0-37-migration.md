# RFC 108 — snora 0.25.1 → 0.38.0: accessibility repairs and typography completion

**Status.** **Implemented in v0.42.0.** S1 and S2 shipped in `b7ceeeb`; S3 and
S4 were evidence-only and are complete.

**Tracks.** `ROADMAP.md` MG-series (GUI) and gate **V1** (GUI draws from design
tokens; contrast verified). Touches **U1** (guided GUI workflow accepted) only
as evidence, not as scope.

**Depends.** Nothing blocking. Sequencing against RFC 100 (GUI module
boundaries) matters — see §7.

**Design owner.** high-capability model

**Decision owner.** nabbisen, project owner

**Proposed implementer.** mid-capability model, after design review and owner
approval

**Evidence location.** `.git-exclude/evidence/108-snora-0-37-migration/`

**Touches.** `Cargo.toml` (one version string), `Cargo.lock`,
`crates/aaai-gui/src/contrast_check/` (remove a stale exemption), and — for
whichever §6 items are adopted — `crates/aaai-gui/src/`. Re-captures RFC 099's
T7 screenshot set. No engine, CLI, persisted-format, or i18n change.

**Handoff.** [`rfcs/handoffs/108-snora-0-38-migration/README.md`](../handoffs/108-snora-0-38-migration/README.md) — written 2026-08-19.

**Source material.** `.git-exclude/tmp/snora-0.25-to-0.37/` — seven release
recaps from the snora team, read in full.

---

## 1. Summary

**The upgrade is one line.** `snora = "0.25.1"` → `"0.38"`. No public item was
removed or renamed across the entire span; the surface went 153 items → 157,
compared as sets at both tags. `rustc ≥ 1.88` is the only prerequisite and our
MSRV is 1.91.

The value is not in the bump. It is in three things the bump delivers or
unlocks:

1. **Two accessibility repairs land automatically** and change what the GUI
   looks like — border contrast and `text_muted` (§2).
2. **A WCAG exemption we rely on has been withdrawn by its author as
   invented**, and we apply it to content users must read (§3). This is the
   most consequential item in this RFC.
3. **Capability we have been working around already exists** — renderable focus
   rings, width-aware layout, in-process keyboard testing (§6).

**Typography needs almost nothing.** RFC 099 already adopted the six-role scale;
snora 0.33.1 was documentation-only for it (§5).

## 2. What the bump changes without being asked

Three rendered changes reach us because we consume `snora::design::render`,
`AppLayout`, `Sheet`, and read `palette.border` 11 times and
`palette.text_muted` 28 times.

| Release | Change | Effect on aaai |
|---|---|---|
| **0.34.0** | `border` contrast repaired — `light` `#D7DBE0`→`#898C8F` (1.28:1 → **3.12:1**), `dark` `#2B313A`→`#69717D` (1.19:1 → **3.17:1**) | Card, dialog-card and chrome borders become visibly more present. High-contrast presets unchanged — already 19.8–21:1 |
| **0.34.0** | `text_muted` on `light` repaired `#6B7280`→`#6A717E` (4.46:1 → ≥4.5:1) | Sub-perceptual; see §3 for why it matters anyway |
| **0.37.0** | Modal dim `DIM_ALPHA` 0.40 → 0.44 | **Our batch sheet only.** The three dialog overlays paint their own hard-coded backdrop outside snora and are unaffected — see RFC 110 |

**0.37.1, 0.37.2 and 0.38.0 contain no rendered change** — each confirmed by
snora directly, under the commitment they made to state it in every note.
Requiring `"0.38"` picks all of them up and the three changes above remain the
complete set for the span.

The direction is one-way: **contrast increases, nothing becomes harder to see.**
snora filed all three as accessibility repairs rather than restyles.

### 2.1 This invalidates RFC 099's V1 screenshot evidence

All sixteen T7 captures were taken against 0.25.1's borders and `text_muted`
and 0.40 dim. Two of those three are visible in every workspace capture.

**The screenshots must be re-taken.** That is not incidental cost — it is the
single largest piece of work in this RFC, and §7 sequences it accordingly. The
capture procedure is now well understood
(`.git-exclude/rules/gui-automation-niri-xdotool.md`), which it was not when
T7 first ran.

## 3. The finding — we rely on a withdrawn exemption, for content that needs reading

`crates/aaai-gui/src/contrast_check/tests.rs:18-20` excludes `text_muted` from
the contrast suite:

> `text_muted` is intentionally excluded: `Palette` documents it as "exempt
> from mandatory contrast checks", used deliberately for non-essential content
> (hints, timestamps, decorative labels) throughout the views.

**snora withdrew that exemption in 0.34.0**, in these words:

> It had gone untested because `Palette`'s doc comment claimed `text_muted` was
> *"exempt from the mandatory body-text contrast checks."* **WCAG grants no such
> exemption** — its exemptions are incidental, decorative or invisible text,
> logotypes, and large text. That exemption was ours, invented, and it is
> withdrawn.

So our test's stated justification quotes a doc comment its author has retracted.

**And the second half of our sentence is false on inspection.** `text_muted`
does not carry decoration in aaai. It carries:

| Site | Content |
|---|---|
| `views/diff_view.rs:508` | **diff line numbers** — needed to reference a line |
| `views/diff_view.rs:296` | file size labels (`diff.size_inline`) |
| `views/opening.rs:253` | **"Not selected"** — a folder's status on the entry screen |
| `views/opening.rs:547` | **onboarding steps ①②③** — first-run guidance |
| `views/opening.rs:417` | the recent-projects section header |

Line numbers, a selection status, and first-run instructions are not
"incidental, decorative or invisible". **28 call sites are excluded from our own
accessibility gate on a rationale that does not hold.**

**Required by this RFC:** delete the exemption and assert `text_muted` against
all three surfaces like every other text role. After 0.34.0's repair both
`light` and `dark` should pass — `dark` at 4.53:1, by 0.026, which snora
records as thin and deliberately left alone. **If the assertion fails, stop and
escalate**; that is a real defect, not a test to loosen.

## 4. What this is worth, plainly

Nothing here is a new feature and none of it is visible as "the app got
better at X". What changes:

- a modal's boundary becomes distinguishable from its backdrop in `light`,
  where it was **2.85:1** against a 3:1 requirement;
- a card's border stops being a 1.28:1 suggestion;
- the smallest text in the diff view — line numbers — comes under the same
  contrast gate as everything else.

That is the whole of it. It is worth doing because these are the defects an
accessibility audit finds first, and because we currently assert a guarantee
(V1: "contrast verified") that has a 28-site hole in it.

## 5. Typography — already adopted; two gaps remain

RFC 099 adopted all six roles (`body` 16/1.4, `body_small` 14/1.35, `label`
14/1.2, `title` 18/1.3, `heading` 24/1.25, `display` 32/1.2) with line heights
at every site. snora 0.33.1 was **documentation-only** — the scale has existed
since v0.20. **The bump adds nothing to typography.**

Two gaps remain, both ours:

**5.1 — the 12-pixel floor is unasserted.** snora's readability guide sets a
hard floor: never a custom size below 12 logical pixels. RFC 099's V1 greps
prove no `.size(N)` literal exists, which enforces the floor *indirectly* —
every size comes from a role, and the smallest role is 14. Worth stating that
the floor is covered by the existing grep rather than adding a check.

**5.2 — snora's own widgets use two of six roles.** `body_small`, `title`,
`heading` and `display` are not applied by anything in `snora-widgets`; the
notice widget renders its title at `label_size`. Where we render snora chrome
rather than our own text, the hierarchy is flatter than our tokens imply. **Not
actionable by us** — recorded so it is not mistaken for our defect.

## 6. Newly available capability — adopt, defer, or decline

Each is opt-in and independent. Recommendations are mine; the decisions are the
owner's.

### 6.1 Focus rings are renderable today — **adopt, inside RFC 106**

snora's documentation said a focus ring "cannot be rendered" on iced 0.14 and
told reviewers not to file it. **0.34.0 corrected that as over-scoped.** The
accurate constraint is narrower:

> iced cannot tell a style closure that a widget **iced** owns is focused.

An application that owns focus as its own state can style it today — a
`container` style closure is an arbitrary `Fn(&Theme) -> Style`, so a focused
boolean can drive border colour and width. `FocusTokens` (`ring_color`,
`ring_width`, `ring_offset`) is usable now.

**aaai owns exactly that state** — `FocusTarget::{FileTree, Inspector, Search}`.
RFC 106 (keyboard operability) found that nothing renders a focus ring anywhere;
this says we can render one without waiting for an iced upgrade. **Fold into
RFC 106** rather than doing it here.

### 6.2 snora agrees with RFC 106 about Tab — **adopt the recommendation**

Independently of us, snora 0.35.0 states:

> **snora does not take Tab or Shift+Tab.** Tab means "next control" to iced and
> to your users; a framework claiming it for region cycling would break in-pane
> navigation for every application with a form or a text input.

That is RFC 106's finding, reached by a different party from first principles.
aaai's `app.rs` claims `Tab` for pane cycling — the exact mistake named.

snora recommends **F6 / Shift+F6** for region cycling. **This supersedes
RFC 106 §5's `Ctrl+Tab` recommendation**, which I proposed without knowing there
was a convention. F6 is the established desktop idiom for pane cycling and
costs the same one i18n key.

`snora_core::focus::next_zone` exists, but its model is `AppLayout`'s four zones
(Header / SideBar / Body / Footer) and ours is three panes. **Take the key
binding and the principle; do not force our panes into their zone enum.**

### 6.3 In-process keyboard testing — **investigate `Simulator`, decline `snapshot`**

> **Revised 2026-08-19** after snora's reply. The original recommendation rested
> on snora having pointed another consumer at `iced_test`'s `tap_key` plus
> `snapshot`. Asked directly whether a worked example existed, they answered:
> **"No. There is not, and we have never used `snapshot` ourselves."** They
> vouch for `iced_test::Simulator` — `render_semantics` drives it with
> `simulate(click())` — but the `snapshot` half was recommended from reading the
> API. Their recommendation was weaker evidence than I treated it as.

The two halves now separate cleanly, and only one is worth having:

**`Simulator` + `tap_key` — investigate.** The simulator is a working dependency
in snora's own suite, so the mechanism is real even if key injection
specifically is unexercised. If `tap_key` drives our update loop in-process,
**RFC 106's acceptance item 5** — asserting keyboard traversal — becomes
achievable rather than "escalate if not testable", and keyboard behaviour moves
from "needs a human" to "assertable" in **RFC 105's** split.

**`snapshot` — decline.** Snapshot tests hold reference images, and snora has
shipped an appearance change **three times in eleven releases**. Every one would
invalidate every baseline. They asked us how we would handle exactly that; the
honest answer is that we would not, and adopting reference images would rebuild
the problem RFC 105 exists to remove — evidence that rots silently and gets
ignored rather than maintained. **Our screenshots stay few, human-judged, and
outside CI.**

The limit snora recorded still stands: `focusable::find_focused()` — *querying*
where focus is — needs iced's `advanced` feature. So moving focus is testable;
asserting where it landed may not be, whichever route we take.

**Timebox it before RFC 106's handoff**, and report the two things snora asked
for if we get that far: whether `snapshot` needs a real GPU in CI, and how
reference images survive an upstream appearance change. Our answer to the second
is already "they do not", which is itself worth sending.

### 6.4 `responsive_render` — **defer to RFC 101, but record it now**

`snora::design::responsive_render` (0.31.0) exposes the layout's available width
so the application can decide what to show. snora deliberately supplies the
number and no thresholds.

This is the missing mechanism for the problem RFC 101 §7a inherited: **three
simultaneous panes is too many at 800 × 550**, and the diff body clips
characters there. A width-aware layout could collapse to fewer panes below a
threshold.

**Not in this RFC** — that is a behaviour and layout decision belonging to
RFC 101. Recorded so RFC 101 does not conclude the capability is missing.

### 6.4a Line-height helpers (0.38.0, RFC-068) — **decline**

Six helpers in `snora-style::text`, one per role, returning
`LineHeight::Relative`. Purely additive. snora notes that if we built a local
wrapper because their old docs said line-height was not wrapped, the helpers
replace it.

**We built no wrapper.** We read `tokens.typography.<role>.line_height`
directly at roughly 170 sites, which is what their old page advised and what
RFC 099 implemented. iced's `.line_height()` takes `impl Into<LineHeight>`, so
we are already passing the same thing the helper returns.

**Declining.** Swapping ~170 call sites for stylistic parity buys nothing
functional, and the insulation argument does not apply either: snora's
additive-only covenant freezes `Typography` and `TextRole` **by name**, so
`tokens.typography.body.line_height` is contractually stable. New GUI code may
use whichever reads better; converting existing sites is not worth a diff.

### 6.5 Pointer target size — **decline for now, record the gap**

snora's checklist mandates 24×24 logical pixels minimum for interactive
controls, 44×44 preferred, and 0.36.0 asserted the **height** axis in
`snora-design` while marking width review-only, because width depends on string,
font and shaping and cannot be computed.

We have never measured ours. The height axis is token-derivable for us too
(`line_box + 2 × vertical_padding`). **Recommend declining here** — it is a new
check, not a migration item, and this RFC is already large. Record it as a
candidate for the same RFC that takes 6.1 and 6.2.

### 6.6 `snora-style` extraction — **no action**

0.32.0 moved the style bridge into a fifth crate, making `design` and `widgets`
independent features. **No public path changed; every import still resolves.**
Our `features = ["design"]` is unaffected.

## 7. Sequencing

**Before RFC 100.** The bump touches `Cargo.toml` and one test file and forces
a screenshot re-capture. RFC 100 restructures 2,168 ELOC of `app.rs`. Doing the
bump first keeps two large, unrelated diffs apart, and the re-capture is
cheapest while the GUI code is stable.

```
RFC 108 (this) → RFC 100 → RFC 104 → RFC 106 (+ 6.1, 6.2) → RFC 101 (+ 6.4)
```

**One risk in that order:** RFC 100's V2 requires every GUI test count
unchanged, and this RFC changes the contrast test by adding `text_muted`
assertions. Doing 108 first means 100 measures against the new baseline, which
is correct. Doing them in the other order would put a count change inside a
"behaviour-neutral" restructure — exactly what V2 exists to prevent.

## 8. Acceptance contract

1. `snora = "0.38"` in the workspace manifest; `cargo update -p snora` reflected
   in `Cargo.lock`.
2. `text_muted`'s exemption is **removed** from
   `crates/aaai-gui/src/contrast_check/tests.rs`, and the role is asserted
   against all three surfaces in all four presets. The stale doc comment
   quoting snora's withdrawn exemption is deleted, not amended.
3. The contrast test passes with `text_muted` included. **If it fails, stop —
   do not exempt, do not adjust a threshold.**
4. `cargo +1.91 test --workspace --locked` — counts grow only by the added
   contrast assertions; every other count unchanged.
5. `cargo +1.91 fmt --check --all` exits 0 (RFC 107's policy now applies).
6. RFC 099's V1 greps still return nothing.
7. **RFC 099's T7 screenshot set re-captured** — four themes × two screens ×
   two sizes = 16, per the corrected matrix in the RFC 099 handoff — against
   0.37.1, using the tiled-window and sequential-launch discipline in
   `.git-exclude/rules/gui-automation-niri-xdotool.md`. The previous set is
   superseded by adding, not overwriting.
8. A short visual judgement recorded on the three rendered changes: do the
   stronger borders read correctly at real density, and does the heavier modal
   dim look right rather than heavy? **snora explicitly asked for this** and has
   it from nobody; §10.

## 9. Risks

| Risk | Mitigation |
|---|---|
| `text_muted` assertion fails on some preset | Then we have a live WCAG defect and want to know. §8 item 3 forbids exempting it away |
| Borders read as too heavy at our density | That is the judgement §8 item 8 exists to capture. snora's figures are measured; whether they *look* right is a separate question they cannot answer |
| The screenshot re-capture is treated as routine | It is the largest task here. Two prior attempts produced invalid captures for reasons now documented; budget accordingly |
| Bump collides with in-flight work | §7 sequences it before RFC 100, when nothing else is in flight |
| Something rendered changes that we have not anticipated | snora now commits to stating explicitly in every release note whether a release contains a rendered change. For this span the three in §2 are the complete set |

## 10. What snora asked us for

Recorded because reciprocity here is cheap and we are a direct beneficiary:

- **borders in situ** — they say the change is measured correct but "not yet
  confirmed to *read* correctly in a real application at real density";
- **the modal dim** — 0.40 → 0.44 is settled on contrast, unsettled on whether
  it looks heavy;
- **which key was bound** for zone navigation, if we adopt 6.2, and whether a
  four-zone model fits.

§8 item 8 produces the first two as a side effect of the re-capture. Sending
them is the owner's call.

## 11. Owner decisions — all four approved 2026-08-19

1. **Adopt the bump** — approved.
2. **6.3 in-process keyboard testing** — approved as a timeboxed investigation.
   Scope narrowed after snora's reply: `Simulator` + `tap_key` only,
   `snapshot` declined.
3. **6.5 pointer target size** — approved: declined here, folded into the
   RFC 106 group.
4. **Reply to snora** — approved and sent 2026-08-19
   (`.git-exclude/outbox/aaai-to-snora-2026-08-19.md`). Their reply is in
   `.git-exclude/tmp/reply-aaai-2026-08-19.tar.gz`; §6.3 and §2 revised from it.

### 11.1 What their reply changed here

- **§6.3 narrowed** — `snapshot` is unproven by its recommender and declined.
- **§2** — 0.37.1 and 0.37.2 confirmed free of rendered change.
- **`text_muted` was the third instance.** orbok's WCAG conformance record and
  knotra's WCAG AA suite both excluded the role citing the same withdrawn doc
  comment, none of the three teams aware of the others. snora shipped **RFC-067**
  in 0.37.2 requiring release notes to name the *re-check*, not only the
  correction. It passes on their surfaces against all three backgrounds; ours
  may differ, which is why §8 item 3 forbids exempting a failure away.
- **One open item back to them**, cheap and unresolved: **arama reports the
  XWayland path delivering pointer motion but not button press/release** — the
  opposite of our result on the same path. See §11.2.

### 11.2 The XWayland divergence

arama reports pointer motion working but **button press/release not** under
forced XWayland. We verified clicks working. snora will not adjudicate and asks
which `xdotool` subcommand each team used and whether the target had focus.

Our answer, from the record:

- **Combined form, single invocation** — `xdotool mousemove <x> <y> click 1`
  (`.git-exclude/rules/gui-automation-niri-xdotool.md:208`), never separate
  `mousedown`/`mouseup`.
- The dev team's captures used the same combined form with `--window $XID`
  added. **Both variants delivered clicks.**
- **Focus was always established first** — `xdotool windowactivate --sync $XID`,
  and in later runs `niri msg action focus-window --id` as well.

If arama used `mousedown`/`mouseup` separately, or clicked without activating
the window, that is the likeliest difference. Worth one line back to snora; not
worth an investigation on our side, since nobody is blocked.

## 11.3 Their doc-test finding, and our self-check

0.38.0's letter reports that **all 111 Rust examples in snora's book were
`rust,ignore` and none compiled**, with one page carrying *"Compile-checked
against the pinned iced 0.14"* directly above an uncompiled block. RFC-069
moved twelve onto a compiled workspace crate; **99 remain unverified**, and they
told us the number rather than the improvement because we copy their snippets.

**Treat any snora book snippet as illustrative unless it is one of the twelve.**
Ours works — the line-height pattern we took from that page compiles in our
tree because our compiler checks it, not because their fence said so.

**We checked ourselves for the same failure.** Result:

- `docs/` — our user-facing book — contains **no Rust code blocks at all**, so
  their exact failure mode cannot occur there;
- `rfcs/` and `rfcs/handoffs/` do carry Rust blocks that implementers copy, but
  **none claims to be compiled or verified**. A specification block that an
  implementer adapts is honest; the defect is claiming verification you do not
  have.

Nothing to fix. Recorded because the check was cheap and the answer could have
gone the other way.

**One principle worth keeping**, theirs: three blocks showing a *type's shape*
rather than usage were deliberately left as prose, because compiling them would
have stripped the field types from a block whose only job is showing field
types. **Compile what is code; leave what is a diagram alone.** That is
RFC 105's assertable-versus-judgemental distinction applied to documentation,
and it argues against ever mechanically compiling every block in `rfcs/`.

## 12. Sources

- `.git-exclude/tmp/snora-0.25-to-0.37/` — all seven recaps
- 0.34.0/0.35.0 notes — border repair, `text_muted` withdrawal, focus-ring
  correction, `snora_core::focus`, the Tab position
- 0.37.0 note — `DIM_ALPHA`; 0.37.1 note — no rendered change, `iced_test` route
- 0.33.1 — typography and readability pages, accessibility checklist
- 0.33.0 — `responsive.md`; 0.36 RFC-061 — pointer target size
- `crates/aaai-gui/src/contrast_check/tests.rs`, `views/*.rs` — our own usage
