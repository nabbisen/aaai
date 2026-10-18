# RFC 113 — Threat model

**Status.** **Accepted** — approved for implementation by the owner
2026-10-18. Lives in `rfcs/accepted/`; the folder is the source of truth for
this state (RFC 000).

**Tracks.** `ROADMAP.md` M2 / gate **S2** (untrusted input and output are
safe). S2's last open half — RFC 098 closed symlinks, RFC 103 the engine and
CLI output, RFC 104 the GUI export, and each of those RFCs names the threat
model as the remaining piece without being scoped to write it.

**Depends.** RFC 098, RFC 103, RFC 104 — all Implemented. Nothing blocks this.

**Design owner.** high-capability model

**Decision owner.** nabbisen, project owner

**Proposed implementer.** high-capability model — the deliverable is a
document, not code. One finding (§5) needs the dev team and is routed there.

**Evidence location.** `.git-exclude/evidence/113-threat-model/`

**Touches.** A new `docs/src/threat-model.md` and its `SUMMARY.md` entry. No
crate, no behaviour, no dependency. §5's finding, if accepted, becomes its own
RFC against `crates/aaai/`.

**Handoff.** Not required for the document. Required for §5 if it is accepted.

---

## 1. Summary

**aaai reads folders it is explicitly told not to trust, and writes a file
meant to be shared with third parties.** That is the whole of its security
posture, and it has never been written down in one place.

Three RFCs closed three concrete surfaces. This RFC writes the model they were
each a part of, which S2 requires and which
`project-instructions-general-common.md` requires be kept current at every
release where data flows change.

**Writing it found one unmitigated threat** — §5, unbounded file reads — which
is the justification for doing this as analysis rather than as paperwork. A
threat model that confirms everything is fine has usually not been written
carefully enough.

## 2. What is being protected

| Asset | Property | Why it matters |
|---|---|---|
| Contents of the audited folders | **Confidentiality** | They may be a customer's release artifact or a production config. aaai reads them and writes summaries elsewhere |
| The audit record — reasons, strategies, approvals | **Integrity** | The product's entire value is that six months later the reason is trustworthy. A silently altered reason is worse than no reason |
| Everything outside the two selected roots | **Confidentiality and integrity** | The user pointed at two folders. Anything else on the disk is out of bounds in both directions |
| The operator's configuration | **Integrity, modest confidentiality** | `profiles.yaml` holds filesystem paths that usually contain the account name |
| The exported report | **Confidentiality at the sink** | It exists to be sent to someone who was not there |

## 3. Trust boundaries

**B1 — folder contents are untrusted.** The central boundary. In aaai's own
stated use cases the "after" folder is an artifact whose provenance is the
thing under audit. Filenames, directory structure, file contents, link targets
and file sizes are all attacker-influenced.

**B2 — the audit definition file is semi-trusted.** It is usually written by
the operator through the GUI, but it is a YAML file designed to be committed
and shared, so it can arrive from a repository the operator does not control.
It carries regular expressions and expected content.

**B3 — the report sink is outside the trust boundary.** The export goes to a
path the user chooses, to be read by someone else, possibly machine-parsed.

**B4 — the dependency graph.** 570 crates, none of them ours.

**Explicitly not boundaries, because the attack surface does not exist:**

- **No network.** No HTTP client, no socket code, no telemetry, no update
  check — verified by dependency-tree and source inspection for `PRIVACY.md`.
  There is no remote attacker.
- **No authentication, no authorization, no multi-tenancy.** A single-user
  desktop and CLI tool running with the invoking user's own privileges. There
  is no privilege to escalate within aaai.
- **No persistent service.** Nothing listens; nothing runs between invocations.

## 4. Threats and their current state

| # | Threat | Boundary | State |
|---|---|---|---|
| **T1** | A link inside a selected root resolves outside it, so content from elsewhere on the disk reaches a diff or report | B1 | **Mitigated.** RFC 098 — every link-like entry is metadata-only and never traversed, on all three platforms; adversarial matrix discharged, cross-root canary tests on Unix and Windows |
| **T2** | A secret in a `reason`, `ticket` or `note` reaches the shared report | B3 | **Mitigated.** RFC 103 for the engine and CLI, RFC 104 for the GUI. Unconditional on both — `mask_secrets: false` does not un-redact |
| **T3** | Hostile content in a filename or diff line is interpreted by the report's consumer — spreadsheet formula injection, HTML or SARIF escaping | B1 → B3 | **Mitigated.** RFC 103 — contextual encoding and spreadsheet neutralisation per format |
| **T4** | A pathological regular expression in a definition file hangs the audit | B2 | **Structurally impossible.** `regex` 1.12.4 is a finite-automata engine with linear-time matching and no backtracking. ReDoS is not expressible. Recorded because the absence is a property of the dependency choice and would be lost if that choice ever changed |
| **T5** | A very large file in an audited folder exhausts memory | B1 | **Not mitigated; scheduled.** Slice A recommended for unit 4, slice B owned by M5 / gate E1 — see §5 |
| **T6** | aaai modifies the folders it is auditing | B1 | **Mitigated.** SEC-2; the engine opens inputs read-only, and canary tests in `diff/tests.rs` assert byte-and-metadata stability across a run |
| **T7** | A compromised or vulnerable dependency | B4 | **Partly mitigated.** `cargo audit` reports no vulnerabilities; four accepted warnings remain. The scan is configured but its workflow is parked, so it runs only when someone runs it — gate C0 |
| **T8** | A memory-safety defect in aaai's own code | all | **Mitigated.** SEC-1 / DEC-012 — no `unsafe` anywhere in project code, verifiable by grep |
| **T9** | A user cannot tell whether a shared report was redacted | B3 | **Accepted limitation.** RFC 104 §5.1 — the `Masking` API does not report whether a pattern matched. Recorded here because "I thought it was safe" is a security outcome, not only a UX one |

## 5. The finding — file reads are unbounded

**`path_boundary::read_file` returns `Vec<u8>` and has no size limit.** Every
file in both trees is read whole into memory before anything else happens:

```rust
let bytes = match path_boundary::read_file(file) { … };
let (sha, size) = digest(&bytes);
let (text, is_binary) = classify_bytes(&bytes);
```

`LARGE_FILE_THRESHOLD` (1 MB, `diff/entry.rs:133`) looks like a guard and is
not one. Its only use is `audit/warning.rs:70`, which warns when an **Exact**
strategy is applied to a large file. It is a UX hint, applied after the read.

**Consequence.** A single large file in an audited folder — a disk image, a
core dump, a database, a log nobody rotated — is read entirely into memory. On
a 64-bit machine with a file larger than available RAM this is an
out-of-memory abort, not a graceful error. With `rayon` comparing files in
parallel, several large files can be resident at once.

**Severity: moderate, and availability-only.** It does not leak or corrupt
anything. aaai is a desktop and CI tool, not a service, so the blast radius is
one aborted audit. But NF-3 promises "tens of thousands of files" and an abort
is not a result the user can act on — §15.1 of the development instructions
requires errors to tell the user what to do next, and an OOM kill tells them
nothing.

### 5.1 Can it be fixed, and should it be — yes to both, and it is already planned

**Can it?** Yes, and the shape is not in doubt:

| Slice | Change | Effect |
|---|---|---|
| **A — bound the read** | `stat` before reading; above a threshold return a `PathIssue` ("too large to compare") instead of a `Vec<u8>` | Converts an OOM abort into an `Error`-status entry the user can act on, which is what §15.1 of the development instructions requires |
| **B — stream the hash** | Feed `sha2` incrementally rather than hashing a resident buffer; skip text classification above the threshold and mark the entry binary/large | Restores *comparability* for large files — checksum strategies keep working at bounded memory |

Slice A is a subset of the end state, not a throwaway: **even with streaming,
there will always be a size above which line-diffing is refused**, so the
threshold and its user-facing issue are permanent furniture.

**Should it?** Yes. NF-3 promises tens-of-thousands-of-files scale and the
current failure mode is an out-of-memory kill, which tells the user nothing and
cannot be distinguished from a crash.

**It is already in the plan.** `ROADMAP.md`'s M5 scope reads *"staged bounded
scan and lazy retention"* and E1's criteria already name large-file budgets.
This RFC does not add work; it **connects the threat to the plan item that
owns it**, which is what was missing.

### 5.2 Scheduling recommendation

**Slice B belongs to M5 / gate E1, unchanged.** It is measurement-bound work
needing fixtures and benchmark hardware, and designing it piecemeal would waste
the measurement.

**Slice A should be pulled forward, and I recommend unit 4.** M5's entry is
*"D0, S2, P1, and C1 complete"* — behind persistence and a green workflow,
which is a long wait for a crash-to-message change that is perhaps thirty lines.
Unit 4 is reachable soon and "code quality" covers robustness fairly.

**This is an authorization, not a design decision** — the work is already
scoped. **Owner: shall slice A ride with unit 4?** My recommendation is yes.
If you would rather keep unit 4 to supply chain and lint alone, slice A waits
for M5 and T5 stands as written, which is the safe fallback rather than the
better one.

T5's row is marked **scheduled**, not merely accepted, on that basis.

## 6. The deliverable

`docs/src/threat-model.md`, carrying §2–§5, plus:

1. **A maintenance rule.** `project-instructions-general-common.md` requires
   the threat model be updated when a release introduces new data flows,
   external integrations, or auth logic. The document states that rule in its
   own first section, so the obligation is where the reader is.
2. **What aaai does not defend**, stated plainly — §3's non-boundaries.
   snora's threat model does this and it is the half that stops a reader
   assuming more than is claimed.
3. **A pointer per mitigated threat** to the RFC and the test that holds it, so
   a reviewer can check rather than believe.

Placed in `docs/src/` beside `abdd-audit.md` and `compatibility.md`, which are
the same kind of artifact: maintainer- and auditor-facing, not tutorial.

## 7. Acceptance contract

1. `docs/src/threat-model.md` exists and is listed in `docs/src/SUMMARY.md`.
2. Every threat in §4 marked **Mitigated** names the RFC that closed it **and**
   at least one test or evidence artifact. A mitigation with no pointer is a
   claim, not a mitigation.
3. Every **non-boundary** in §3 states why the surface does not exist, not
   merely that it is out of scope.
4. T9 appears as an accepted limitation with its reasoning. **T5 appears as
   a scheduled defect**, naming its slices and the units that own them — not as
   an accepted limitation, because it is neither accepted nor unowned.
5. `mdbook build` succeeds — RFC 027's CI check covers orphan chapters.
6. No change under `crates/`.
7. **S2 can then be assessed.** This RFC does not declare S2 met; it supplies
   the half that was missing. The gate is the owner's to call.

## 8. Risks

| Risk | Mitigation |
|---|---|
| The document becomes a one-time artifact that rots | §6 item 1 puts the maintenance rule inside the document, and the release checklist in `project-instructions-general-common.md` already requires the update |
| A mitigation is claimed that no test holds | Acceptance item 2 requires a pointer per claim; I verified each §4 row against the source before writing it |
| Writing the model is mistaken for closing the gate | Acceptance item 7 is explicit: S2 remains the owner's call |
| §5 is read as a vulnerability disclosure | It is availability-only, local, and requires the user to point aaai at the file themselves. §5 says so |

## 9. Sources

- RFC 098 §9.1 and `.git-exclude/evidence/098-selected-folder-and-symlink-policy/threat-model-matrix.md`
- RFC 103 §114 and §12.3; RFC 104 §5.1
- `crates/aaai/src/diff/path_boundary.rs` — `read_file`; `crates/aaai/src/diff/engine.rs:146`; `crates/aaai/src/diff/entry.rs:133`; `crates/aaai/src/audit/warning.rs:70`
- `Cargo.lock` — `regex 1.12.4`
- SEC-1…SEC-6 — `.git-exclude/specs/aaai-handoffs-files-260630/cross-cutting-ops-security.md`
- `.git-exclude/specs/aaai-development-instructions-v1.md` §15.1, §17
