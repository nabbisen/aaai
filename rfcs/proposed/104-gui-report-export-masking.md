# RFC 104 — GUI Report Export Masking

**Status.** Proposed

**Tracks.** `ROADMAP.md` M2 / WS-05 / gate S2 — closes the last surface RFC 103
left unmasked

**Depends.** RFC 103 (safe output surfaces), which introduced the `Masking` type
this RFC consumes. Scheduled **after RFC 100** — see §7.

**Design owner.** high-capability model

**Decision owner.** nabbisen, project owner

**Proposed implementer.** mid-capability model, after design review and owner
approval

**Evidence location.** `.git-exclude/evidence/104-gui-report-export-masking/`

**Touches.** `crates/aaai-gui/src/app.rs` (or its post-RFC-100 successor module)
and a GUI test sibling. No engine change, no CLI change, no persisted format, no
dependency, no new i18n key.

**Handoff.** Required, after acceptance.

---

## 1. Summary

RFC 103 made masking non-optional at the report boundary and enabled it at every
CLI call site. It deliberately left the GUI's two export call sites passing
`Masking::Disabled`. The result is that **the same report, exported from the GUI
instead of the CLI, is not redacted.**

This RFC closes that. It is small — the fix is to construct a `MaskingEngine`
in the GUI and pass `Masking::Enabled` — and the reason it was not done inside
RFC 103 is honest and worth preserving: `App` constructs no `MaskingEngine`, so
enabling it is a state-ownership question rather than a two-line edit.

## 2. The defect

`crates/aaai-gui/src/app.rs`, in the report-export handler:

```rust
// RFC 103 §5.1a — GUI report export is out of RFC 103
// scope; carries the same gap `cmd/report.rs` had,
// tracked as follow-up.
let res = if use_json {
    ReportGenerator::write_json(…, aaai::Masking::Disabled)
} else {
    ReportGenerator::write_markdown(…, aaai::Masking::Disabled)
};
```

That comment is accurate and was the right thing to write. This RFC is the
follow-up it names.

**Severity.** The exported file is chosen by the user through a native save
dialog and is meant to be reviewed and shared — the exact sink NF-14 and SEC-3
describe. A GUI user has no indication that the CLI would have redacted the same
content. There is no workaround short of re-exporting from the CLI.

**Why it is not higher.** It requires a secret to be present in a `reason`,
`ticket`, or `note` field in the first place, and the GUI is not the primary
automation surface. It is a real gap, not an incident.

## 3. Goals and non-goals

### 3.1 Goals

1. GUI report export masks exactly what CLI report export masks, for both
   formats the GUI offers (Markdown and JSON).
2. The two `Masking::Disabled` sites in the GUI are removed. After this RFC,
   `Masking::Disabled` in `crates/aaai-gui/` is a defect.
3. A test proves the **written file** is masked — not that the capability
   exists. RFC 103's F3/F4/F5 were all capability-present, wiring-absent.

### 3.2 Non-goals

- Adding HTML or SARIF export to the GUI. The GUI offers two formats; this RFC
  does not change which.
- A GUI toggle for masking. See §5.2.
- Changing masking patterns, or the `Masking` type itself.
- Fixing the GUI's non-hermetic unit tests, which read the operator's real
  `prefs.yaml` via `App::default()`. That is logged against RFC 100 and is a
  dependency of §6's test, not part of this RFC's scope — see §7.

## 4. Selected design

### 4.1 Where the engine lives

Construct the `MaskingEngine` **at export time**, in the export handler, not as
a field on `App`.

| Option | Decision |
|---|---|
| Field on `App`, built at startup | **Rejected.** Adds long-lived state for a rare operation, and the custom-pattern set comes from project config discovered relative to the compared root, which can change during a session |
| Built per export in the handler | **Selected.** Matches `cmd/report.rs` exactly, holds no state, and picks up config changes without invalidation |
| Passed down from a caller | **Rejected.** No caller has it either |

Mirror `cmd/report.rs`:

```rust
let custom_patterns = /* discover from project config, as report.rs does */;
let masker = MaskingEngine::with_custom(&custom_patterns);
let masking = Masking::Enabled(&masker);
```

Build it **once** before the format branch and pass it to both arms, rather than
inlining it twice — the shape review 053 §4 Q1 accepted for `report.rs`.

### 4.3 Extract the write so the test needs no `App` (added 2026-08-27)

`report_path_picked` (`app/update/save.rs:124`) reads five things off `self` —
`audit_result`, `before_path`, `after_path`, `definition_path`, and the chosen
output path — then branches on format and writes. **None of that is
prefs-derived.**

Extract the write into an associated function taking exactly those inputs:

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

`report_path_picked` keeps the `self` reads, the format derivation, and the
toast handling, and calls it. **Acceptance item 3's test then calls
`write_report` directly** with a canary in a `reason`, a `tempfile` output path,
and `Masking::Enabled` — no `App`, no `UserPrefs::load`, nothing that touches
the operator's machine.

This is why §7's second sequencing reason is withdrawn: the test does not need
the GUI to be hermetic, because it does not need the GUI.

### 4.2 Masking is unconditional, matching `report.rs`

`cmd/audit.rs` gates masking on `--mask-secrets` or the project config's
`mask_secrets`. `cmd/report.rs`, after RFC 103, does **not** — it always masks.

**The GUI follows `report.rs`, unconditionally.** The two paths write the same
kind of artifact through the same functions, and a file written to a
user-chosen path for sharing is a different threat model from console output the
operator is already looking at. Diverging here would mean the same report is
redacted or not depending on which button produced it, which is precisely the
inconsistency this RFC exists to remove.

This is worth stating explicitly because it means **`mask_secrets: false` in
project config does not un-redact a report**, from either the CLI or the GUI.
That is already true of `aaai report` as of RFC 103; this RFC extends the same
rule to the GUI rather than introducing it.

## 5. Questions this RFC settles rather than leaves open

### 5.1 Does the user need to be told the file was redacted?

**No new UI in this RFC.** The success toast already names the path. Adding a
"secrets were redacted" state would require knowing whether any pattern actually
matched, which the current `Masking` API does not report, and inventing that
signal is a larger change than the fix.

Recorded as a **deliberate limitation, not an oversight**: a GUI user cannot
currently distinguish "no secrets present" from "secrets redacted". If that
matters, it is a separate RFC against the `Masking` API, not against this
wiring.

### 5.2 Should there be a GUI toggle?

**No.** A control whose only function is to disable a safety default is worth
adding only when a real user need is demonstrated, and none is. The CLI has no
such escape hatch for `report` either, so adding one to the GUI would make the
GUI the *less* safe surface.

## 6. Acceptance contract

1. `grep -rn "Masking::Disabled" crates/aaai-gui/` returns nothing.
2. Both GUI export formats pass `Masking::Enabled`, from a single binding.
3. A test writes a report through the GUI export path with a secret-pattern
   canary in a `reason`, reads the **written file**, and asserts the canary is
   absent. Capability-level assertions do not satisfy this item.
4. The equivalent CLI export of the same input produces the same masking
   outcome — the two surfaces are proven consistent, not each proven correct in
   isolation.
5. `cargo +1.91 test --workspace --locked` green; counts grow only by the named
   new test.
6. RFC 099's V1 greps still pass: no `.size(N)` literal, no `Color::from_rgb`
   outside `design_tokens.rs`.
7. No new i18n key, no dependency, no persisted-format change.
8. RFC 103 §12.3 residual 1 is marked closed, with this RFC named.

## 7. Sequencing — after RFC 100, and why

**This RFC must not be implemented before RFC 100.** Two reasons:

1. RFC 100 restructures `crates/aaai-gui/src/` module boundaries and is
   explicitly about breaking up `app.rs`. Wiring masking into `app.rs` first
   means moving it again during RFC 100, for no benefit.
2. ~~Acceptance item 3 needs a test that can drive the export path and read the
   written file. The GUI's current tests are non-hermetic — they read the
   operator's real `prefs.yaml` through `App::default()`. Writing a file-writing
   test on that foundation would either inherit the non-hermeticity or require
   fixing it here, which is RFC 100's job.~~

   > **Withdrawn 2026-08-27. RFC 100 did not fix this and could not have** — it
   > was a pure move under a byte-identical-body check, so it relocated
   > `App::default()`'s three `UserPrefs::load()` calls (`app.rs:192`, `:199`,
   > `:200`) without changing them. Two tests in `app/tests.rs` still construct
   > `App::default()` and therefore still read the operator's real
   > `prefs.yaml`.
   >
   > **This no longer blocks RFC 104**, because §4.3 removes the need for the
   > test to build an `App` at all. The hermeticity gap is real and is recorded
   > separately — see §7a — rather than being dragged into this RFC.

**Order:** RFC 099 (V1) → RFC 100 (V2) → **RFC 104** → RFC 101.

The gap stays open in the meantime. That is an accepted cost: the alternative is
doing the work twice and building a test on a foundation already scheduled for
replacement. It is recorded in RFC 103 §12.3 and in `ROADMAP.md`'s S2 row so it
does not depend on anyone remembering.

## 7a. The S1 gap this RFC declines to fix (added 2026-08-27)

Gate **S1** reads *"tests never touch the operator's real config"*
(`ROADMAP.md:184`). What M1 actually delivered and verified was narrower:
*"Every **CLI subprocess** test uses isolated home/config directories"*
(`ROADMAP.md:201`). **GUI tests were never in that scope.**

Today, `app/tests.rs:22` and `:40` construct `App::default()`, which calls
`UserPrefs::load()` three times and reads
`$XDG_CONFIG_HOME/aaai/prefs.yaml`. The exposure is **read-only** —
`UserPrefs::load` returns defaults on any failure and never writes — so nothing
of the operator's is corrupted. But a test's behaviour can depend on the
machine it runs on, which is what the gate exists to prevent.

**Not this RFC's to fix**, and §4.3 means it does not have to be. Recorded so
that S1's one-line summary is not read as covering more than it does. It wants
its own small change: `UserPrefs::load_from(&paths)` already exists and is what
`prefs/tests.rs` uses, so the fix is an injection point on `App`, not new
machinery.

## 8. Risks

| Risk | Mitigation |
|---|---|
| Project-config discovery differs in the GUI, so custom patterns are missed | Acceptance item 4 compares GUI and CLI output for the same input, which fails if discovery diverges |
| The test proves capability, not wiring — RFC 103's original defect, repeated | Item 3 requires reading the written file; item 1's grep is the structural backstop |
| RFC 100 lands and this RFC's target file no longer exists | Expected. The Touches line names `app.rs` **or its post-RFC-100 successor**; the handoff is written after RFC 100 lands, against the real structure |
| Unconditional masking surprises a user who set `mask_secrets: false` | Already the behaviour of `aaai report` as of RFC 103; §4.2 makes it explicit rather than incidental. If the owner wants it configurable, that is one decision covering both surfaces, not a GUI-only exception |

## 9. Sources

- RFC 103 §5.1a, §12.3 residual 1, and `.git-exclude/reviewed/052-rfc103-clarification-review-result-2026-07-29.md` §5
- `.git-exclude/reviewed/055-rfc102-rfc103-disposition-checkpoint-2026-07-31.md` §5, which recorded this as the one residual that is a real gap
- `crates/aaai-gui/src/app/update/save.rs:124-180` — the export handler after
  RFC 100; `crates/aaai-cli/src/cmd/report.rs:57-62` — the pattern to mirror
- NF-14 / SEC-3 and `docs/src/overview.md:45`
