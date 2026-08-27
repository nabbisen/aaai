# RFC 104 — GUI report export masking: implementer handoff

**RFC.** [`rfcs/proposed/104-gui-report-export-masking.md`](../../proposed/104-gui-report-export-masking.md)

**Status.** Awaiting owner approval to implement. Design is settled.

**Baseline.** `main` at `cee1d74` (RFC 111 landed, B0 green). **Re-measure
before relying on any line number here** — see §6.

**Evidence location.** `.git-exclude/evidence/104-gui-report-export-masking/`

| Role | Who | Scope |
|---|---|---|
| Design owner | high-capability model | The RFC and this handoff |
| Implementer | GUI developer | T1–T4 |
| Reviewer | high-capability model | On request |
| Integrator | nabbisen | Commit, push, observe CI |

---

## 1. What this is

**A report exported from the GUI is not redacted; the same report exported from
the CLI is.** RFC 103 made masking non-optional at the report boundary and wired
it at every CLI call site, deliberately leaving the GUI's two sites passing
`Masking::Disabled`. This closes that.

Gate **S2** — untrusted input and output surfaces are safe — has this as its
last open output-side item (RFC 103 §12.3 residual 1).

**The fix is small.** Most of this document is about the *test*, because RFC
103's own review found that its first three attempts asserted capability rather
than wiring, and this RFC exists to close a gap of exactly that shape.

## 2. The two sites

`crates/aaai-gui/src/app/update/save.rs:152` and `:161`, both inside
`report_path_picked`, both `aaai::Masking::Disabled`, with a comment naming
this RFC as the follow-up.

**Mirror `cmd/report.rs:57-62` exactly:**

```rust
let custom_patterns = ProjectConfig::discover(&before)
    .unwrap_or(None)
    .map(|(config, _)| config.custom_mask_patterns)
    .unwrap_or_default();
let masker = MaskingEngine::with_custom(&custom_patterns);
let masking = Masking::Enabled(&masker);
```

`ProjectConfig::discover` takes the **before** path — in the GUI that is
`PathBuf::from(&self.before_path)`, already bound in the handler. Build the
binding **once, above the format branch**, and pass it to both arms. Do not
inline it twice.

**Masking is unconditional** (RFC §4.2). It is not gated on
`mask_secrets` in project config, because `aaai report` is not either. Do not
add a condition, a flag, or a toggle.

## 3. Tasks

### T1 — Extract the write (commit 1)

Per RFC §4.3. Add an associated function on `App`:

```rust
fn write_report(
    result: &AuditResult,
    before: &Path,
    after: &Path,
    def_path: Option<&Path>,
    out: &Path,
    masking: Masking<'_>,
) -> std::io::Result<()>
```

It contains the format derivation and the `write_json` / `write_markdown`
branch, nothing else. `report_path_picked` keeps every `self` read, builds the
masking, calls it, and keeps the toast handling exactly as it is.

**This is a pure extraction. No behaviour changes in T1** — it still passes
`Masking::Disabled` at the end of T1 if you want to commit the two steps
separately. Doing so makes T2's diff one line.

### T2 — Enable masking (same or next commit)

Replace both `Masking::Disabled` with the single `masking` binding from §2.

### T3 — The test that proves the file is masked (commit 2)

**This is the item that matters.** RFC §6 item 3:

> A test writes a report through the GUI export path with a secret-pattern
> canary in a `reason`, reads the **written file**, and asserts the canary is
> absent. Capability-level assertions do not satisfy this item.

Call `write_report` **directly**. Do not construct an `App` — see §4.

Shape:

1. Build an `AuditResult` with a canary in a `reason` field. Use a pattern the
   default `MaskingEngine` already recognises; take it from
   `crates/aaai/src/**` masking tests rather than inventing one, and say in the
   evidence which pattern you used and where it came from.
2. Write to a `tempfile` path, with
   `Masking::Enabled(&MaskingEngine::with_custom(&[]))`.
3. Read the file back. Assert the canary is **absent** and the surrounding
   report content is **present** — a test that passes because nothing was
   written is worse than no test.
4. **Both formats.** Markdown and JSON are separate write functions and the RFC
   requires both.

**`tempfile` is not yet a dev-dependency of `aaai-gui`.** It is already a
workspace dependency (`Cargo.toml:86`) and a dev-dependency of `crates/aaai`,
so add it as `tempfile = { workspace = true }` — this adds nothing new to the
project's dependency set. Report the build-time delta, and if `aaai-gui` has no
`[dev-dependencies]` section yet, say so; that is a manifest change worth a
reviewer seeing.

**Write it before T2 and watch it fail.** With `Masking::Disabled` still in
place the canary appears in the file. Red before, green after — that is the
proof this RFC is not another capability-present, wiring-absent result.

### T4 — CLI/GUI consistency (same commit)

RFC §6 item 4: the same input exported through the CLI path produces the same
masking outcome. This fails if project-config discovery diverges between the two
surfaces, which is §8's first risk.

The cheapest honest form: drive both `write_markdown` calls — the one
`write_report` makes and the one `cmd/report.rs` makes — over the same
`AuditResult` and the same root, and compare the written bytes. If that is not
practical, say why in the evidence and assert the next-best thing; do not
silently drop the item.

## 4. Do not build an `App` in the test

`App::default()` calls `UserPrefs::load()` three times (`app.rs:192`, `:199`,
`:200`), which reads `$XDG_CONFIG_HOME/aaai/prefs.yaml` — **the operator's real
config**. Two existing tests already do this; that is a known gap recorded in
RFC 104 §7a, and it is not yours to fix here.

§4.3's extraction exists precisely so this test does not inherit it. If you find
yourself needing an `App`, stop and report — the extraction is wrong, not the
constraint.

**Related correction, so it is not rediscovered:** RFC 104 §7 originally said
RFC 100 would fix the GUI's test hermeticity. It did not and could not — RFC 100
was a pure move under a byte-identical-body check. That claim is struck in the
RFC.

## 5. Acceptance

RFC §6 is the contract. Four notes:

**Item 1** — `grep -rn "Masking::Disabled" crates/aaai-gui/` returns nothing.
After this RFC, `Masking::Disabled` anywhere in the GUI crate is a defect.

**Item 5** — counts grow **only** by the named new tests. Report the measured
figure with the reason, not a prediction.

**Item 8** — mark RFC 103 §12.3 residual 1 closed, naming this RFC. That text
is at `rfcs/done/103-safe-output-surfaces.md:364-367` and its §-header summary
at line 7-8 also says "GUI report export stays unmasked". **Both need
updating** — the header claim is the one a reader meets first.

**Clippy** — `cargo +1.91 clippy -p aaai-gui --all-targets --no-deps`, not
`-- -D warnings`: the crate carries 13 pre-existing findings and that flag fails
on all of them. The check is that no new finding appears.

## 6. A standing check this project has learned three times

**Any line number, ELOC figure, or test count in this document is stale until
you re-measure it on current `main`.**

RFC 104's own §2 quoted a code block from `app.rs` that RFC 100 moved to
`app/update/save.rs`, and RFC 111's removal table had to be regenerated for the
same reason. Re-measure first; if a figure is wrong, report it rather than
working around it.

## 7. Out of scope

- **The S1 hermeticity gap** (RFC §7a). Real, recorded, not this RFC's.
- **HTML or SARIF export in the GUI**, a masking toggle, or telling the user
  whether anything was redacted — RFC §3.2 and §5.1/§5.2 settle all three.
- **Changing masking patterns or the `Masking` type.**
- **The C2 lint debt** from RFC 100.
- Anything outside `crates/aaai-gui/` except RFC 103's residual text in
  `rfcs/done/`.

## 8. When you are done

Package a review request as usual, entry point stated in chat. Then Integrator
pushes and B0 runs.

Sequence after this: **106 → 110 → 101**.
