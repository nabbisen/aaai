# RFC 103 — Safe Reports, Exports, and Masking

**Status.** Implemented (2026-07-31). Shipped in
`4d4996a7517e5cbd8f09ee645c9086c9899b2e33`; hosted evidence in run
`30551474446`. All eight §6 acceptance items discharged — see §12. Completes
S2's **output half**; S2 itself remains open on RFC 098's symlink half. Ships in
release unit 1. Two residuals, both deliberate and both recorded in §12.3: GUI
report export stays unmasked, and SARIF navigation targets stay unmasked.

**Tracks.** `ROADMAP.md` M2 / WS-05 / gate S2

**Depends.** RFC 098 / WS-04 (reporting contract), RFC 097 / B0 (test matrix)

**Design owner.** high-capability model

**Decision owner.** nabbisen, project owner

**Proposed implementer.** mid-capability model, after design review and owner approval

**Evidence location.** `.git-exclude/evidence/103-safe-output-surfaces/`

**Touches.** `crates/aaai/src/report/{generator,html,sarif}.rs`,
`crates/aaai/src/masking/`, `crates/aaai-cli/src/cmd/{export,report}.rs`, and
their tests. Additionally **two lines** in `crates/aaai-gui/src/app.rs` — the
masker arguments at `:1139` and `:1143` only, which the §5.1 signature change
makes non-compiling. Nothing else in that file may change; RFC 099's V1 greps
must still pass. One public signature changes (§5.1). No persisted format, no
dependency, no workflow change.

**Handoff.** Required:
[`rfcs/handoffs/103-safe-output-surfaces/README.md`](../handoffs/103-safe-output-surfaces/README.md)

## 1. Summary

Five defects across the output surfaces. Two are High, and **four of the five
mean the documented masking guarantee has never held for any report file.**
Findings F1-F4 and their reachability are recorded in
`.git-exclude/reviewed/046-ws05-output-surface-security-findings-2026-07-29.md`;
F5 and the corrected root cause in
`.git-exclude/reviewed/051-rfc103-scope-clarification-and-f5-2026-07-29.md`.
Neither is restated here.

This RFC fixes them and adds the cross-surface guard that prevents the class
from recurring. The point fixes are small; the guard is the durable part.

## 2. Root cause

> **Revised 2026-07-29** after the implementer's escalation
> (`.git-exclude/reviewed/051-rfc103-scope-clarification-and-f5-2026-07-29.md`).
> An earlier version described four defects with three causes. The true position
> is **one cause with five symptoms**, and it is worse than finding 046
> recorded.

**Masking is opt-in, and no caller opts in.** `MaskingEngine` is constructed in
exactly two places in the workspace — `benches/diff_bench.rs:47` and
`cmd/audit.rs:98` — and **no `write_*` call site anywhere passes
`Some(masker)`**. `cmd/report.rs` passes literal `None` at `:67`, `:73`, and
`:78`; `crates/aaai-gui/src/app.rs` passes `None` at `:1139` and `:1143`.

So **no report file produced by `aaai report` or by the GUI has ever been
masked, in any format.** Masking executes only on `audit.rs`'s console path.
NF-14, SEC-3, and `docs/src/overview.md:45` advertise it without
qualification; that claim is false for every file the tool writes.

The five symptoms:

| | Defect | Shape |
|---|---|---|
| F1 | CSV/TSV formula injection | missing transform |
| F2 | HTML `ticket` unescaped | one missed call site |
| F3 | SARIF never masked | `write_sarif` has no masker parameter |
| F4 | CSV/TSV never masked | `export.rs` bypasses the report API |
| **F5** | **JSON never masked** | `build_json` takes `_masker` and ignores it |

F3, F4, and F5 are the same defect three times. F2 is a genuine one-off;
`html_escape` exists, is correct, and is applied everywhere else. F1 is a
missing transform rather than a missing call.

The API shape that permitted all of it:

```rust
pub fn write_markdown(…, masker: Option<&MaskingEngine>) -> …
pub fn write_json    (…, masker: Option<&MaskingEngine>) -> …
pub fn write_html    (…, masker: Option<&MaskingEngine>) -> …
pub fn write_sarif   (…)                                 -> …   // no masker at all
```

`write_sarif` never had the parameter — F3 is a missing argument, not an
oversight inside `sarif.rs`. And `Option<&MaskingEngine>` makes masking opt-in:
`None` is a legal, silent, unmarked choice to emit unmasked output. `export.rs`
bypasses the API entirely and builds its own rows, which is F4.

**A fix that only patches four call sites leaves the shape that produced them.**

## 3. Goals and non-goals

### 3.1 Goals

- No output surface can emit an untrusted field without masking, enforced by
  the type system rather than by discipline.
- Every surface applies its own correct encoding: HTML escaping, CSV/TSV
  formula neutralisation, JSON/SARIF serializer escaping.
- A cross-surface test enumerating untrusted fields × surfaces, so a newly
  added surface or field fails until it is handled.
- Adversarial fixtures run through the B0 three-OS matrix, per the S2 contract.

### 3.2 Non-goals

- Changing masking patterns or adding new ones. The patterns are unit- and
  property-tested; the defect is surface coverage.
- Changing report content, layout, or format selection.
- GUI display paths beyond what shares the report code. Guided-flow output is
  RFC 101.
- The full S2 gate. S2 also needs the threat model and cross-root-link cases;
  this RFC is WS-05's output half.

## 4. The untrusted field set

Normative. These carry attacker- or third-party-influenced content.

**Encoding and masking are different obligations and must not be conflated.**
*Encoding* makes a value inert in its destination syntax; every untrusted field
needs it on every surface. *Masking* redacts secrets; it applies only to
free-text fields. **Masking a path would corrupt it** — the path is the audit's
primary identifier, and a redacted path makes the finding unusable and breaks
re-audit matching.

| Field | Source | Why untrusted | Encode | Mask |
|---|---|---|:--:|:--:|
| `path` | audited tree | filenames are attacker-controlled — the premise of RFC 098 | ✅ | ❌ identifier; redaction destroys it |
| `reason` | definition | human-written; may paste secrets | ✅ | ✅ |
| `ticket` | definition | free-form `Option<String>`, unvalidated | ✅ | ✅ |
| `error_detail` | engine | may embed a path | ✅ | ✅ |
| strategy rule content | definition | user-supplied patterns | ✅ | ✅ |
| root paths | invocation | may embed usernames or hostnames | ✅ | ✅ |

An earlier draft of this section required every field to be "masked and
encoded", which would have directed the implementer to redact paths. That is
wrong and is corrected here.

## 5. Selected design

### 5.1 Make masking non-optional at the boundary

Replace `masker: Option<&MaskingEngine>` with an explicit two-state type:

```rust
pub enum Masking<'a> {
    Enabled(&'a MaskingEngine),
    /// Caller asserts the sink is trusted. Must be justified at the call site.
    Disabled,
}
```

`write_sarif` gains the same parameter. The behaviour of `Disabled` is
identical to today's `None`; what changes is that choosing it becomes explicit,
named, and greppable, and that **no surface can omit the parameter**.

This is a public API change to a re-exported type. Pre-v1 with no stability
promise, so permitted. The compatibility impact is recorded in §9.1 below, per
RFC 095 §8.3's requirement that *"every workstream changing public API must
record compatibility impact"* — the record belongs to the changing workstream;
RFC 095 itself is settled and is not edited. WS-12/D1 is the acceptance owner
that consumes it.

**No source-compatibility shim may be added.** Not a `Default` impl, not
`From<Option<&MaskingEngine>>`, not an `Into`-based signature. Any of these
would let existing `None` call sites keep compiling, preserving exactly the
silent default that produced F3, F4, and F5. Breaking those call sites is the
mechanism, not a side effect.

### 5.1a Callers must enable masking, not merely declare a choice

Owner decision, 2026-07-29: **`aaai report` begins masking its file output.**

Changing the signature forces a decision at each call site; it does not by
itself fix anything. The decisions are:

| Call site | Value | Reason |
|---|---|---|
| `cmd/report.rs` — all four formats | **`Enabled`** | A report file is written to be reviewed and circulated. It is the paradigm untrusted sink, and no honest justification for `Disabled` exists. Build the engine as `cmd/audit.rs:98` does, so custom patterns are honoured |
| `aaai-gui/src/app.rs` — two sites | `Disabled`, with a limitation note | `App` constructs no `MaskingEngine`, so enabling it is more than a mechanical change. Out of scope here; recorded as follow-up, not as a resolved case |

This is a **behaviour change**: reports will now redact where a secret pattern
matches. It is the behaviour NF-14, SEC-3, and the published documentation
already describe, and it is the change that makes those claims true for the
first time. §3.2's non-goal against changing report content targets layout and
format selection, not applying a documented security control.

### 5.2 Per-surface encoding obligations

Normative table. Each surface must satisfy its row for every §4 field:

| Surface | Encoding obligation |
|---|---|
| Markdown | escape `|` and backticks in table cells |
| HTML | `html_escape` on **every** interpolation, including `ticket` (F2) |
| JSON | `serde_json` escaping — already correct |
| SARIF | `serde_json` escaping, **plus** masking (F3) |
| CSV / TSV | RFC 4180 quoting **plus** formula neutralisation (F1), **plus** masking (F4) |

### 5.3 Formula neutralisation

A cell whose first character is `=`, `+`, `-`, `@`, tab, or CR is prefixed with
a single apostrophe. RFC 4180 quoting is retained and is **not** a substitute:
the CSV parser consumes the quotes before the spreadsheet evaluates the cell.

`\r` is added to the quoting trigger alongside `\n`, so a bare CR cannot break
row framing.

### 5.4 The cross-surface guard

One test that renders a single `AuditResult` — populated with an adversarial
value in **every** §4 field — through **every** surface, then asserts per
surface:

- no canary secret appears anywhere in the output — asserted for the maskable
  fields in §4, **not** for `path`, which must appear verbatim once encoded;
- HTML contains no unescaped `<`, `>`, or `"` from field content;
- no CSV/TSV cell begins with a formula character;
- Markdown table cells contain no unescaped `|`.

The encode/mask split in §4 means the guard has two assertion families, not
one. Applying the masking assertion to `path` would make the test unimplementable
against a correct implementation.

Structured as field × surface so that adding a surface, or adding a field to
§4, fails the test until handled. This is the same shape as RFC 099's contrast
gate, which converted a recurring manual check into one that cannot silently
regress.

## 6. Acceptance contract

1. `Masking` is non-optional on all four `write_*` functions.
2. `grep -rn "Option<&.*MaskingEngine>" crates/` returns nothing, and no
   source-compatibility shim exists (§5.1).
2a. Every `write_*` call site passes an explicit `Masking` value; each
   `Disabled` carries a written justification. `cmd/report.rs` passes
   `Enabled` at all four sites (§5.1a).
2b. F5 closed: `build_json` uses its masker rather than ignoring it.
3. The §5.4 guard passes; it must **fail** if any single fix is reverted —
   demonstrated by reverting each in turn during development.
4. F1–F4 each have a dedicated regression test naming the finding.
5. `cargo +1.91 test --workspace --locked` green; counts grow only by the named
   new tests.
6. Adversarial fixtures execute on all three platforms through B0.
7. No persisted format, CLI flag, dependency, or workflow change.
8. The `Masking` signature change is recorded as a compatibility impact in
   §9.1, for WS-12/D1 to consume.

## 7. Risks and mitigations

| Risk | Mitigation |
|---|---|
| The apostrophe prefix corrupts legitimate data | It is a spreadsheet display convention, not stored data; only applied to leading formula characters. Documented in the CSV export docs |
| `Masking::Disabled` becomes the reflexive default | It is greppable and reviewable; §6.2 asserts no `Option` remains. Each use must carry a justifying comment |
| A public signature change breaks a consumer | Pre-v1, no stability promise; `aaai` is not yet published under this name. Recorded for WS-12 |
| The guard tests the fixes rather than the property | §6.3 requires demonstrating it fails when each fix is reverted |
| Scope creep into the full S2 threat model | Explicit non-goal; this RFC is WS-05's output half only |

## 8. Alternatives considered

| Option | Decision |
|---|---|
| Non-optional `Masking` + per-surface encoding + cross-surface guard | **Selected.** Fixes the four defects and the shape that produced them |
| Patch the four call sites only | Rejected: leaves masking opt-in and `write_sarif` asymmetric; the next surface repeats it |
| Keep `Option`, add a lint or convention | Rejected: discipline already failed twice here |
| Mask centrally in the engine before any surface sees data | Rejected: masking is presentation, not model. It would corrupt the audit result and break diffing |
| Drop CSV/TSV export | Rejected: it is a D0-approved format |
| Defer to the Aug 31 window | Owner decision recorded in finding 046 §4 |

## 9. Compatibility impact record

Recorded here per RFC 095 §8.3, "Public `aaai` engine API" row: the changing
workstream records, WS-12/D1 accepts before R1.

### 9.1 Public API change

| | |
|---|---|
| **Surface** | `aaai::report::generator::ReportGenerator` — `write_markdown`, `write_html`, `write_json`, `write_sarif`; and the new `aaai::Masking` re-export |
| **Change** | Final parameter `masker: Option<&MaskingEngine>` becomes `masking: Masking<'_>`. `write_sarif` gains the parameter, which it never had |
| **Kind** | Breaking, source-level, for any external caller of these four functions |
| **Migration** | `None` → `Masking::Disabled`; `Some(&engine)` → `Masking::Enabled(&engine)`. Mechanical and compiler-guided |
| **Deliberately not softened** | No `From<Option<…>>`, `Default`, or `Into` shim. A shim would let `None` keep compiling, preserving the silent default that caused F3, F4, and F5. The break *is* the mechanism (§5.1) |
| **Permitted because** | v0.x is pre-release; RFC 095 §8 states the contract starts at v1.0.0 and v0.x gains no retroactive promise |
| **Behaviour change** | `aaai report` now masks its file output on all formats. Owner-approved 2026-07-29 (§5.1a). Not an API break, but a user-visible one |
| **Known limitation carried** | GUI report export remains unmasked; `App` constructs no `MaskingEngine`. Follow-up, not resolved here |
| **Downstream consumers** | None known. `aaai` is not published under this name, so no external caller can exist yet |

### 9.2 Not changed

Persisted formats, CLI command names, options, exit codes, the `audit.yaml`
schema, config-file locations, and report *structure* are all untouched. The
machine-format contracts (JSON, SARIF, CSV, TSV) gain masked values in existing
fields; no field is added, removed, or renamed.

## 10. Review questions

1. Is the §4 untrusted field set complete for the current surfaces?
2. Is `Masking::Disabled` the right escape hatch, or should masking be
   unconditional with no opt-out at all?
3. Is the apostrophe prefix the right neutralisation, versus rejecting or
   stripping such values?
4. Should the guard live in the engine crate, the CLI crate, or both — given
   `export.rs` is CLI-side and the report surfaces are engine-side?
5. Does the public signature change need an owner decision under RFC 095's
   compatibility ownership, or is pre-v1 latitude sufficient?

## 11. Sources

- `.git-exclude/reviewed/046-ws05-output-surface-security-findings-2026-07-29.md`
- `crates/aaai/src/report/{generator,html,sarif}.rs`,
  `crates/aaai-cli/src/cmd/export.rs`
- `ROADMAP.md` WS-05 and the S2 gate contract
- NF-14 / SEC-3 and `docs/src/overview.md:45`

## 12. Implementation record

Added at the 2026-07-31 disposition checkpoint
(`.git-exclude/reviewed/055-rfc102-rfc103-disposition-checkpoint-2026-07-31.md`).

Implemented as **one** commit rather than the six slices the handoff
anticipated: S3 and S5 are interleaved in `export.rs`, and S1, S3, and S5 all
touch `crates/aaai-cli/tests/cli.rs`, so a split would have meant reconstructing
working states that never existed. The owner chose one commit. The per-slice
progression the split was meant to demonstrate is preserved in
`guard-progression.md` and `revert-demo.md`, which is where it carries more
weight anyway.

Review of record:
`.git-exclude/reviewed/053-rfc103-safe-output-surfaces-implementation-review-2026-07-29.md`
(Approved). Escalation and scope resolution:
`.git-exclude/reviewed/052-rfc103-clarification-review-result-2026-07-29.md`.

### 12.1 Acceptance contract disposition

| § 6 item | State | Evidence |
|---|---|---|
| 1. `Masking` non-optional on all four `write_*` | **Met** | signatures in `crates/aaai/src/report/generator.rs` |
| 2. No `Option<&MaskingEngine>`, no shim | **Met** | grep returns only two doc-comment mentions in `crates/aaai/src/masking.rs`, no signature and no `From`/`Default`/`Into` impl |
| 2a. Explicit value at every call site; `report.rs` all `Enabled` | **Met** | `cmd/report.rs` binds `Masking::Enabled(&masker)` once; the four `Disabled` sites each carry a written justification |
| 2b. F5 closed — `build_json` uses its masker | **Met** | `_masker` removed |
| 3. Guard passes, and fails if any single fix is reverted | **Met** | `revert-demo.md`, five reversions, each turning at least one test red |
| 4. F1–F4 each have a dedicated regression test naming the finding | **Met** | `findings-map.md` |
| 5. Workspace green; counts grow only by named new tests | **Met** | 145 / 13 / 97 / 27 / 3 |
| 6. Adversarial fixtures execute on all three platforms through B0 | **Met** | run `30551474446`; eight named guards confirmed `ok` on Linux, macOS, **and** Windows individually, not inferred from totals |
| 7. No persisted format, CLI flag, dependency, or workflow change | **Met** | `scope.diffstat` |
| 8. Signature change recorded as compatibility impact | **Met** | §9.1 |

Item 6 and item 8 were the two outstanding at review 053; both closed after
integration.

### 12.2 The behaviour change, restated where it will be found

`aaai report` now redacts secret-pattern matches in every format. Before this
RFC it never masked any file output, on any format — masking ran only on
`audit.rs`'s console path. Owner-approved 2026-07-29; see §5.1a. This is the
behaviour NF-14, SEC-3, and `docs/src/overview.md:45` already described, so the
change closes a gap between documentation and code rather than opening one.

### 12.3 Residuals — deliberate, not deferred defects

1. **GUI report export remains unmasked.** `crates/aaai-gui/src/app.rs` passes
   `Masking::Disabled` at both sites. `App` constructs no `MaskingEngine`, so
   enabling it is more than a mechanical change. Declared a non-goal in §3.2 and
   confirmed as a known limitation in review 052 §5. It is the one place where a
   user-visible surface still contradicts NF-14. **Owned by
   [RFC 104](../proposed/104-gui-report-export-masking.md)**, opened
   2026-07-31 and scheduled after RFC 100.
2. **SARIF `originalUriBaseIds` and `artifactLocation.uri` remain unmasked.**
   Accepted in review 053 §4 Q2: these are functional navigation targets a SARIF
   consumer resolves back to a real file, the same reason §4 exempts `path`
   itself. Masking them would break navigation without improving security,
   since a filesystem path matches no built-in pattern unless it literally
   embeds a secret-shaped string.

### 12.4 Open questions from §10, resolved

| §10 question | Resolution |
|---|---|
| 1 — field set complete? | Yes as specified; `error_detail` and strategy rule content are in the guard fixture but rendered by no current surface, so no surface was added (that would be new report content, prohibited by §3.2) |
| 2 — should `Masking::Disabled` exist? | Yes, retained; every use carries a written justification, and the four current uses do |
| 3 — apostrophe prefix vs. rejecting or stripping? | Apostrophe prefix, applied before RFC 4180 quoting; quoting retained, since a CSV parser consumes quotes before the spreadsheet evaluates the cell |
| 4 — where does the guard live? | **Both crates.** Layer 1 in the engine proves each surface *can* behave; layer 2 in the CLI spawns the real binary and proves the wiring *does*. Layer 1 alone would have passed even if `report.rs` reverted to `Disabled` — which is exactly how F3, F4, and F5 survived |
| 5 — does the signature change need an owner decision? | Recorded in §9.1 under RFC 095's compatibility ownership; pre-v1 latitude applies, so no separate decision was required |
