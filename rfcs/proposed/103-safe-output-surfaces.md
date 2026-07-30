# RFC 103 — Safe Reports, Exports, and Masking

**Status.** Proposed

**Tracks.** `ROADMAP.md` M2 / WS-05 / gate S2

**Depends.** RFC 098 / WS-04 (reporting contract), RFC 097 / B0 (test matrix)

**Design owner.** high-capability model

**Decision owner.** nabbisen, project owner

**Proposed implementer.** mid-capability model, after design review and owner approval

**Evidence location.** `.git-exclude/evidence/103-safe-output-surfaces/`

**Touches.** `crates/aaai/src/report/{generator,html,sarif}.rs`,
`crates/aaai/src/masking/`, `crates/aaai-cli/src/cmd/export.rs`, and their
tests. One public signature changes (§5.1). No persisted format, no dependency,
no workflow change.

**Handoff.** Required:
[`rfcs/handoffs/103-safe-output-surfaces/README.md`](../handoffs/103-safe-output-surfaces/README.md)

## 1. Summary

Four defects across the output surfaces, two of them High, and two of them
falsifying the documented masking guarantee. Findings and reachability are
recorded in
`.git-exclude/reviewed/046-ws05-output-surface-security-findings-2026-07-29.md`
and are not restated here.

This RFC fixes them and adds the cross-surface guard that prevents the class
from recurring. The point fixes are small; the guard is the durable part.

## 2. Root cause

The four defects are not four unrelated mistakes. Three of them share one
cause: **each surface independently decides whether to mask and how to encode,
and the API lets it decide "not at all."**

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

F2 (the unescaped `ticket`) is a genuine one-off: `html_escape` exists, is
correct, and is applied to every other field. F1 is a missing transform rather
than a missing call.

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
promise, so permitted — but it must be recorded in the RFC 095 compatibility
matrix, and it is the only public change here.

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
2. `grep -rn "Option<&.*MaskingEngine>" crates/` returns nothing.
3. The §5.4 guard passes; it must **fail** if any single fix is reverted —
   demonstrated by reverting each in turn during development.
4. F1–F4 each have a dedicated regression test naming the finding.
5. `cargo +1.91 test --workspace --locked` green; counts grow only by the named
   new tests.
6. Adversarial fixtures execute on all three platforms through B0.
7. No persisted format, CLI flag, dependency, or workflow change.
8. The `Masking` signature change is recorded for the RFC 095 compatibility
   matrix.

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

## 9. Review questions

1. Is the §4 untrusted field set complete for the current surfaces?
2. Is `Masking::Disabled` the right escape hatch, or should masking be
   unconditional with no opt-out at all?
3. Is the apostrophe prefix the right neutralisation, versus rejecting or
   stripping such values?
4. Should the guard live in the engine crate, the CLI crate, or both — given
   `export.rs` is CLI-side and the report surfaces are engine-side?
5. Does the public signature change need an owner decision under RFC 095's
   compatibility ownership, or is pre-v1 latitude sufficient?

## 10. Sources

- `.git-exclude/reviewed/046-ws05-output-surface-security-findings-2026-07-29.md`
- `crates/aaai/src/report/{generator,html,sarif}.rs`,
  `crates/aaai-cli/src/cmd/export.rs`
- `ROADMAP.md` WS-05 and the S2 gate contract
- NF-14 / SEC-3 and `docs/src/overview.md:45`
