# Threat Model

**Scope.** aaai reads folders it is explicitly told not to trust, and writes a
file meant to be shared with third parties. That is the whole of its security
posture. This document states what is protected, where the trust boundaries
are, what is defended, and — equally — what is not.

**Audience.** Maintainers, reviewers, and anyone assessing aaai before using it
on content they care about.

**Last reviewed:** 2026-10-20, against `0.42.0` plus unreleased work.

---

## Keeping this current

**This document must be updated when a release introduces a new data flow, an
external integration, or authentication or authorization logic.** That is the
project's release rule, not a suggestion — see `CONTRIBUTING.md` and the
release checklist.

If a release does none of those, re-read it anyway and update the
*Last reviewed* date. A threat model that is never revisited becomes a claim
about a program that no longer exists.

---

## 1. What is protected

| Asset | Property | Why it matters |
|---|---|---|
| Contents of the audited folders | Confidentiality | They may be a customer's release artifact or a production configuration. aaai reads them and writes summaries elsewhere |
| The audit record — reasons, strategies, approvals | Integrity | The product's value is that six months later the recorded reason is trustworthy. A silently altered reason is worse than none |
| Everything outside the two selected folders | Confidentiality and integrity | You pointed aaai at two folders. Anything else on the disk is out of bounds, in both directions |
| Your configuration | Integrity; modest confidentiality | `profiles.yaml` stores filesystem paths, which usually contain your account name |
| An exported report | Confidentiality at its destination | It exists to be sent to someone who was not there |

## 2. Trust boundaries

**B1 — folder contents are untrusted.** The central boundary. In aaai's own
use cases the "after" folder is an artifact whose provenance is the thing under
audit. Filenames, directory structure, file contents, link targets and file
sizes are all influenced by whoever produced it.

**B2 — the audit definition file is semi-trusted.** Usually written by you
through the GUI, but it is a YAML file designed to be committed and shared, so
it can arrive from a repository you do not control. It carries regular
expressions and expected content.

**B3 — the report destination is outside the boundary.** The export goes to a
path you choose, to be read by someone else, possibly by a machine.

**B4 — the dependency graph.** 570 crates, none of them ours.

## 3. What aaai does not defend — and why the surface does not exist

Stated plainly, so nothing here is assumed to be more than it is.

**No network attacker, because there is no network code.** aaai contains no
HTTP client, no socket code, no telemetry, no update check and no crash
reporting. This is verifiable: no HTTP client crate appears in `Cargo.lock`,
and no socket type appears in the sources. The two URLs in the codebase are
written as text into SARIF reports — a schema identifier and a repository
link — and are never retrieved.

**No privilege escalation, because there are no privileges.** aaai is a
single-user desktop and command-line tool running as the user who invokes it.
There is no authentication, no authorization, no multi-tenancy and no daemon.

**No persistent service.** Nothing listens, and nothing runs between
invocations.

**No defence against a hostile operator.** If you point aaai at a folder and
approve a change with a false reason, aaai records the reason you gave. It
verifies that a reason exists, never that it is true.

**No accessibility-tree integration.** Unrelated to security, stated because
people ask: the GUI provides no AccessKit/AT-SPI/UIA surface. The CLI is the
recommended path for screen-reader users.

## 4. Threats

Every mitigation names the RFC that closed it **and** the test or evidence that
holds it. A mitigation with no pointer is a claim, not a mitigation.

| # | Threat | Boundary | State | Held by |
|---|---|---|---|---|
| **T1** | A link inside a selected folder resolves outside it, so content from elsewhere on disk reaches a diff or report | B1 | **Mitigated** | RFC 098. `selected_root_symlink_is_rejected`, `symlinked_file_is_incomparable_and_outside_content_is_absent`, `symlinked_directory_is_not_traversed`, `broken_and_cyclic_links_are_reported_without_following`, `relative_inside_links_and_link_type_conflicts_are_incomparable`, plus Windows reparse twins — `crates/aaai/src/diff/tests.rs`. Adversarial matrix discharged on all three platforms |
| **T2** | A secret in a `reason`, `ticket` or `note` reaches a shared report | B3 | **Mitigated** | RFC 103 (engine, CLI) and RFC 104 (GUI). `connection_string_password_masked`, `bearer_token_masked`, `multiple_secrets_all_masked` (`masking/tests.rs`, plus property tests); `markdown_export_does_not_contain_the_secret`, `json_export_does_not_contain_the_secret`, `gui_export_matches_cli_report_for_the_same_input` (`aaai-gui/src/app/update/save/tests.rs`) |
| **T3** | Hostile content in a filename or diff line is interpreted by whoever opens the report — spreadsheet formula injection, HTML or SARIF escaping | B1 → B3 | **Mitigated** | RFC 103. `every_surface_encodes_and_masks_every_untrusted_field` — `crates/aaai/src/report/generator/tests.rs` |
| **T4** | A pathological regular expression in a definition file hangs the audit | B2 | **Not expressible** | The `regex` crate is a finite-automata engine with linear-time matching and no backtracking. Recorded because this is a property of a dependency choice, and would be lost if that choice changed |
| **T5** | A very large file in an audited folder exhausts memory | B1 | **Not mitigated — scheduled** | §5 |
| **T6** | aaai modifies the folders it is auditing | B1 | **Mitigated** | Inputs are opened read-only. `link_audit_preserves_namespace_content_and_stable_metadata` and its Windows twin capture size, mode, device, inode and mtime before and after a run and assert equality. **Scope note:** these cover the link-audit path, where the risk is highest; they are not a blanket proof that no code path writes |
| **T7** | A vulnerable or compromised dependency | B4 | **Partly mitigated** | `cargo audit` runs in CI and reports **no vulnerabilities**. Eight informational warnings are accepted — §6 |
| **T7a** | `unsafe-libyaml` parses audit definition files, is unsafe Rust, and is unmaintained | B2 | **Accepted, recorded separately** | §6.1 |
| **T8** | A memory-safety defect in aaai's own code | all | **Mitigated** | No `unsafe` anywhere in project code — verifiable by grep across `crates/` |
| **T9** | You cannot tell whether a report you shared was redacted | B3 | **Accepted limitation** | §5 |

## 5. Known gaps

### T5 — file reads are unbounded (scheduled)

`read_file` returns the whole file as a `Vec<u8>` with no size limit, so every
file in both trees is read fully into memory before hashing or classification.
`LARGE_FILE_THRESHOLD` (1 MB) is not a guard: its only use is to warn when an
**Exact** strategy meets a large file, after the read.

**Consequence.** A single very large file — a disk image, a core dump, an
unrotated log — can exhaust memory. With parallel comparison, several can be
resident at once. The result is an out-of-memory abort rather than an error
you can act on.

**Severity: moderate, availability only.** Nothing leaks and nothing is
corrupted. It requires you to point aaai at the file yourself.

**Scheduled, in two parts.** Bounding the read and returning a "too large to
compare" result is authorized into release unit 4. Streaming the hash, so
large files stay comparable at bounded memory, belongs to the scalable-diff
milestone. See `ROADMAP.md`.

### T9 — redaction is silent (accepted)

Masking is unconditional, but aaai cannot tell you whether anything was
actually redacted — the masking interface does not report whether a pattern
matched. So "no secrets were present" and "secrets were removed" look
identical.

**Accepted deliberately.** Reporting it would require a new signal through the
masking API. Recorded here because *"I assumed it was safe"* is a security
outcome, not only a usability one.

## 6. Accepted dependency advisories

Gate C0 requires the dependency audit to be clean **or** its exceptions to be
formally accepted. `cargo audit` reports **no vulnerabilities**. These eight
informational warnings are accepted, with reachability recorded:

| Advisory | Crate | Kind | Reachability and reason |
|---|---|---|---|
| `RUSTSEC-2024-0449` | `unsafe-libyaml` | unmaintained | **§6.1 — recorded separately** |
| `RUSTSEC-2026-0253` | `lru` | unsound | Runtime, via `cryoglyph` → `iced_wgpu` → `iced`. Use-after-free in `LruCache::pop()` reachable only if a stored key's `Drop` panics with unwinding enabled. **Not fixable by us** — `cryoglyph` holds `lru` below the patched 0.18.2 and has no release that lifts it |
| `RUSTSEC-2024-0436` | `paste` | unmaintained | Build-time proc-macro only; no runtime presence |
| `RUSTSEC-2026-0206` | `rustybuzz` | unmaintained | Runtime, text shaping via `iced`. No replacement available without an upstream change |
| `RUSTSEC-2026-0192` | `ttf-parser` | unmaintained | Runtime, font parsing via `iced`. Same |
| `RUSTSEC-2026-0371` | `zbus_names` | unmaintained | Merged into `zbus` 6.0.0 upstream on 2026-10-16; reached via `iced` → `winit`. Nothing is wrong with the code — the crate moved |
| `RUSTSEC-2026-0370` | `zvariant` | unmaintained | Same event |
| `RUSTSEC-2026-0372` | `zvariant_derive` | unmaintained | Same event |

**All but the first two clear when `iced` updates its own dependencies.** They
are not aaai's to fix and are recorded rather than chased.

### 6.1 `unsafe-libyaml` — the one that is different

```
unsafe-libyaml 0.2.11
└── serde_yaml 0.9.34+deprecated
    ├── aaai  (the engine, directly)
    └── rust-i18n-macro  (build-time)
```

Three facts together make this unlike the others:

- it is **unsafe Rust** — the advisory describes it as "a machine translation
  of libyaml's C code into unsafe Rust";
- it arrives through **`serde_yaml`, itself deprecated**, a direct dependency
  of the engine;
- it parses **audit definition files**, which §2 classifies as **B2,
  semi-trusted**.

So this is the one unmaintained dependency sitting on a named trust boundary,
in code that is not memory-safe, where the advisory's own wording is that bugs
"will not be fixed upstream".

**It is not a vulnerability.** No CVE, no known exploit, and it has been
present since the project began. **Accepted on that basis**, and recorded
separately so it is not read as equivalent to three font crates and a renamed
D-Bus helper.

**Migration off `serde_yaml` is the real answer** and is not scheduled. It
touches the persisted audit-definition, preferences and profiles formats, so
it is a compatibility change rather than a dependency swap, and it needs its
own design.

## 7. Reporting a problem

Open an issue at <https://github.com/nabbisen/aaai/issues>. If the problem is
a vulnerability rather than one of the accepted gaps above, say so in the
title and omit a working exploit from the public report.
