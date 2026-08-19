# RFC 108 — snora 0.38.0 migration: developer handoff

Companion to [`RFC 108`](../../proposed/108-snora-0-37-migration.md). The RFC
records what was decided and why; this records how to do it. It must not
override the RFC.

## 1. Authority and entry conditions

**Owner approved the bump and all four §11 decisions, 2026-08-19.**

Begin when `main` is green and the working tree is clean. **Do this before
RFC 100 starts** — RFC 107 §3's argument applies again: the screenshot
re-capture is cheapest while the GUI code is stable, and RFC 100 restructures
2,168 ELOC of `app.rs`.

## 2. Role split

| Role | Party | Scope |
|---|---|---|
| Implementer | mid-capability model | S1–S4 |
| Verification operator | whoever has the display | S3's captures — may be the implementer |
| Integrator | nabbisen | Reviews, commits, pushes, observes B0 |
| Architect | RFC 108 author | Owns any RFC change; consulted if §7 fires |

**Commits are the Integrator's** unless S1's verification requires one, which it
does not. Prepare the working tree and request review.

## 3. The work — four slices

### S1 — the bump

```toml
snora = { version = "0.38", features = ["design"] }
```

One line in the workspace `Cargo.toml`, plus the `Cargo.lock` update. Nothing
else in this slice.

**No API broke across the whole span** — 153 public items at 0.25.0, 157 at
0.37.1, compared as sets. If anything fails to compile, **stop and escalate**;
that contradicts snora's own compatibility claim and they would want to know.

### S2 — delete the `text_muted` exemption

`crates/aaai-gui/src/contrast_check/tests.rs` excludes `text_muted` and explains
why in a doc comment quoting snora. **snora withdrew that exemption as
invented** — WCAG grants no such thing. RFC 108 §3 has the full finding.

1. **Delete the doc-comment paragraph** at lines 18–20, the one beginning
   *"`text_muted` is intentionally excluded"*. Delete it; do not soften it.
2. **Add `text_muted` to `foreground_roles()`**, so it is asserted against all
   three surfaces in all four presets exactly like every other text role.

**Expected: it passes.** snora repaired `light` in 0.34.0 (4.46:1 → ≥4.5:1) and
reports `dark` passing at 4.53:1 — by 0.026, which they record as thin and
deliberately left alone.

**If it fails on any pair: stop and escalate.** Do not re-exempt the role, do
not lower a threshold, do not special-case a preset. A failure means we have a
live WCAG defect on content users read — snora asked to be told either way, and
the answer is worth more than a green test.

### S3 — re-capture RFC 099's screenshots

**This is the largest task here.** The bump changes three rendered things
(RFC 108 §2), two of which appear in every workspace capture, so the existing
16 images are stale.

Re-capture the full matrix: **4 themes × 2 screens × 2 sizes = 16**, per the
corrected matrix in
`rfcs/handoffs/099-gui-visual-foundation/README.md` §5 T7.

**Read `.git-exclude/rules/gui-automation-niri-xdotool.md` first.** Three rules
from it are not optional, because each one has already cost a cycle:

- **Keep the window tiled** — never `move-window-to-floating`. A floated window
  can land on a workspace that is not displayed, and input then goes elsewhere
  with no error;
- **Launch instances sequentially** — two XWayland clients of the same app
  connected at once makes one render at the wrong scale, silently;
- **Validate by cross-capture measurement**, not by eye and not by
  `niri msg windows`. Any capture whose toolbar-button runs differ from its
  siblings is not comparable to them.

**Supersede by adding.** Name the new set `-snora038` and leave every prior
generation in place; reviews 057, 058 and 063 cite them.

### S4 — the visual judgement snora asked for

While the captures are in front of you, record two answers in the evidence.
This is a **human judgement**, not a measurement, and it is the thing snora
explicitly has from nobody:

1. **Do the stronger borders read correctly at our density**, or do they look
   heavy? Their figures are `light` 1.28:1 → 3.12:1 and `dark` 1.19:1 → 3.17:1.
2. **Does the modal dim look right** at `DIM_ALPHA` 0.44, or heavy? It was 0.40.

A short paragraph each. "Looks fine" is an acceptable answer if it is what you
think; the value is that a person looked.

## 4. Verification

```sh
cargo +1.91 fmt --check --all                     # exits 0 — RFC 107's policy
cargo +1.91 clippy -p aaai-gui --all-targets -- -D warnings
cargo +1.91 test --workspace --locked
cargo +1.91 check --target x86_64-pc-windows-gnu -p aaai --tests --locked
python3 scripts/check-i18n-keys.py                # if PyYAML is available
git diff --check

# RFC 099's V1 must not regress
grep -rE "\.size\([0-9]" crates/aaai-gui/src/          # nothing
grep -rn "Color::from_rgb" crates/aaai-gui/src/        # nothing outside design_tokens.rs
```

**Test counts.** Current baseline is **146 / 13 / 97 / 27 / 3** on Linux,
Windows `aaai` 134. The **GUI count grows** by whatever S2's added assertions
produce; nothing else moves. State the new GUI figure explicitly rather than
saying "counts unchanged" — this is the one slice that legitimately changes one.

Re-measure the baseline yourself before relying on those numbers. Three
handoffs in this project have carried stale counts.

## 5. Evidence

`.git-exclude/evidence/108-snora-0-38-migration/`:

```
bump.diffstat          S1's diff
contrast-result.md     S2 — the assertion's outcome, pass or fail, per preset and pair
screenshots/           S3 — 16 new captures, named <theme>-<screen>-<size>-snora038.png
visual-judgement.md    S4 — the borders and the dim, in prose
local-results.md       every command above with its output
hosted-runs.md         the B0 run for the integrated SHA
```

`contrast-result.md` is the one that matters beyond this project — snora asked
for the outcome either way, and three teams excluded that role citing one
sentence of theirs.

## 6. Must not

- Add `rustfmt.toml`, change formatting policy, or hand-edit formatting.
- Adopt the 0.38.0 line-height helpers — RFC 108 §6.4a declines them; our ~170
  direct token reads stay as they are.
- Adopt `snapshot` / `matches_image` / `matches_hash` — §6.3 declines them.
- Touch keyboard handling, focus rings, or `responsive_render`. Those are
  RFC 106 and RFC 101; §6.1, §6.2 and §6.4 route them deliberately.
- Overwrite any existing screenshot.
- Re-exempt `text_muted` or adjust a contrast threshold to obtain a pass.

## 7. Stop and escalate

- Anything fails to compile after the bump.
- The `text_muted` assertion fails on any pair.
- A capture's cross-capture measurement disagrees with its siblings and the
  launch environment is identical — that would be a new instance of the scale
  artifact, or something worse.
- Any test count other than the GUI one moves.
- A rendered change appears that RFC 108 §2 does not list. snora committed to
  naming every rendered change in their release notes; an unlisted one is
  either our misreading or theirs, and both are worth stopping for.

## 8. Rollback

S1 and S2 revert cleanly and independently. S3 and S4 are evidence only. If the
bump is reverted, the `text_muted` assertion must be reverted with it — `light`
would return to 4.46:1 and the assertion would then correctly fail.
