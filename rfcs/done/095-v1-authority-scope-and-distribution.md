# RFC 095 — v1 Authority, Scope, GUI, Compatibility, and Distribution

**Status.** Implemented (D0 decision — 2026-07-17)

**Tracks.** `ROADMAP.md` M0 / WS-01 / gate D0

**Depends.** Accepted v1 trustworthiness-remediation roadmap at `f24d5ee`

**Decision owner.** nabbisen, project owner

**Owner decision.** Approved as proposed on 2026-07-17; maintainer authority
confirmed through the owner-controlled review and approval workflow

**Design agent.** Codex, one serial workstream at a time; WS-01 has no product
implementation

**Touches.** Product authority, v1 feature disposition, GUI baseline,
compatibility boundary, distribution scope, and constraints for WS-02 through
WS-13. This RFC does not change product code.

**Revision.** Addresses both blocking findings from the 2026-07-17 initial
architecture review: determinate supported-feature release disposition and
complete compatibility ownership/evidence allocation.

## 1. Summary

This RFC establishes the single product baseline that governs aaai v1 planning:

1. Project rules govern process and release authority; this owner-approved RFC
   governs the v1 product baseline.
2. aaai remains one product with a shared engine, CLI, and desktop GUI.
3. The v1 GUI defaults to the simplified guided review flow, while the current
   three-pane workspace remains an explicitly selected expert surface.
4. v1 compatibility protects audit semantics, actual CLI contracts, versioned
   persisted formats, core GUI tasks, English/Japanese availability, and the
   public engine API. It does not freeze a three-pane layout, pixel appearance,
   wording, internal translation keys, or performance.
5. The v1 distribution set is crates.io, direct GitHub Release archives for the
   declared targets, source builds, and the required maintainer delivery
   tarball. Microsoft Store/MSIX availability, signing, and submission are
   deferred until after the v1 release decision.
6. Existing capabilities stay in scope only under the remediation gates. They
   must be enabled and acceptance-backed before R1 or be narrowed through an
   owner-approved D0 amendment before D1/WS-13. Their presence in current code
   is not evidence that they are safe or release-ready.

The independent re-review accepted the design with only non-blocking notes, and
the project owner approved every decision in Section 13 on 2026-07-17. D0 is
therefore complete. This approval does not authorize product implementation,
release, publication, Store activity, or a tag.

## 2. Motivation

The repository currently contains incompatible descriptions of the intended
v1 product:

- the project handoff describes the current three-pane GUI and declares the
  v1 feature set implemented;
- the important simplified wireframe makes a single guided review card the
  default and removes the always-visible three-pane inspector from the beginner
  path;
- current compatibility documentation attempts to freeze the three-pane layout
  and internal i18n keys across v1.x;
- public documentation claims Microsoft Store availability, while Store
  artwork, local installation, alias, signing, and submission evidence is not
  complete;
- current code exposes features whose test-state, untrusted-path, output,
  persistence, scale, and release-assurance risks remain open.

Without one owner-approved authority, later RFCs could optimize different
products, and documentation could turn an implemented feature into a false
release-readiness claim. D0 resolves those contradictions before safety work
begins.

## 3. Goals and non-goals

### 3.1 Goals

- Define how conflicting product sources are resolved.
- Freeze the v1 feature boundary during remediation.
- Select one default GUI experience and state the status of the existing
  three-pane workspace.
- Define what v1.x compatibility will and will not protect.
- Select the distribution channels whose evidence is required before v1.
- Give every later workstream a stable scope input.

### 3.2 Non-goals

- Designing or implementing the guided GUI in detail; WS-11 owns
  that work.
- Fixing tests, CI, symlink handling, reports, persistence, dependencies,
  formatting, performance, documentation, packaging, or release automation.
- Claiming that any current feature or distribution channel has passed its
  release gate.
- Authorizing commits, tags, pushes, publication, Store work, or release.
- Reserving RFC numbers beyond RFC 095.

## 4. Product identity and invariants

aaai remains one audit-for-asset-integrity product with three build surfaces:

```text
aaai engine library
  ├── aaai CLI
  └── aaai GUI
```

The following are non-negotiable v1 product invariants:

1. The engine owns diff, audit, definition, masking, report, history, profile,
   and project semantics. Front ends do not invent different verdict logic.
2. An accepted change requires a non-empty human reason. Empty reason remains
   Pending and never becomes OK.
3. Audited folders are inputs and are never mutated by an audit.
4. GUI and CLI verdicts agree for identical inputs and definitions.
5. Security or data-safety remediation may narrow or disable unsafe behavior
   before v1; feature presence never overrides a safety gate.
6. English is the documentation and code language. The GUI supports English
   and Japanese in v1.

Changing an invariant requires an explicit owner-approved amendment or a
superseding RFC; downstream implementation RFCs cannot change one implicitly.

## 5. Normative authority model

The project has domain-specific authorities rather than pretending every file
belongs in one total order.

| Priority and domain | Authority | Rule |
|---|---|---|
| 1 — governance | Direct project-owner instructions and `.git-exclude/rules/` | Govern repository policy, engineering process, security posture, RFC lifecycle, and release authorization. A product RFC cannot silently override them. |
| 2 — v1 product baseline | This owner-approved RFC | Governs v1 scope, default GUI, compatibility boundary, and distribution set. |
| 3 — program execution | The accepted current-program section of `ROADMAP.md` | Governs schedule, workstream order, entry dependencies, gates, and review boundaries. |
| 4 — detailed design | Later owner-approved RFCs | May refine implementation inside this baseline. Any scope expansion or product-contract conflict requires an RFC 095 amendment first. |
| 5 — incorporated external design | Simplified v0.33.2 wireframe | Normative for the approved guided beginner path; RFC 095 clarifications take precedence. |
| 6 — requirements and evidence input | The v0.40 handoff bundle | Supplies requirements, decisions, risks, and observed evidence. Claims of completion or readiness do not override current gates. |
| 7 — historical design | `rfcs/done/` | Retained unless this RFC or a later approved RFC explicitly revises the decision. Implemented status is historical evidence, not automatic v1 scope authority. |
| 8 — observed state | Code, tests, CI, packaging, README, mdBook, and CHANGELOG | Describe current behavior and claims. They must converge to approved authorities; drift does not redefine the product. |

When two authorities in different domains appear to conflict, work stops at
the narrower decision boundary and the project owner resolves the conflict.
Code or documentation never wins merely because it already exists.

## 6. v1 feature disposition

The inventory uses three tiers:

- **Core commitment:** release-blocking v1 behavior.
- **Supported existing:** included, enabled, and acceptance-backed at v1.0,
  although not every UI placement or implementation detail is a compatibility
  promise.
- **Deferred:** outside v1 and must not be advertised as available.

### 6.1 Core commitments

| Area | v1 commitment | Required later gate |
|---|---|---|
| Folder comparison | Compare two selected directory trees and classify Added, Removed, Modified, Unchanged, TypeChanged, Unreadable, and Incomparable paths | S2, E1 |
| Audit evaluation | Apply one shared engine and produce OK, Pending, Failed, Ignored, or Error with GUI/CLI verdict parity | S1, S2, R1 |
| Audit definition | Load, validate, create, merge, and safely save human-readable `version: 1` YAML | P1 |
| Approval | Require a human reason; support the existing rule metadata needed to explain who approved what, why, and until when | P1, D1 |
| Content strategies | None, Checksum, LineMatch, Regex, and Exact | S2 |
| Input selection | Ignore rules and project configuration without escaping or mutating selected roots | S2 |
| Reports | Markdown, JSON, HTML, and SARIF reports | S2 |
| Portable export | CSV and TSV audit-entry export | S2 |
| Secret handling | Consistent configured and built-in masking across approved CLI, GUI, report, and export surfaces | S2 |
| CLI | The 16 current commands: `audit`, `snap`, `report`, `check`, `history`, `config`, `dashboard`, `watch`, `completions`, `diff`, `merge`, `init`, `export`, `version`, `lint`, and `exit-codes` | S1, S2, C1, D1 |
| GUI workflow | Select folders, check, review, explain, approve, save, re-check, and save a report through the approved guided and expert surfaces | P1, U1, D1 |
| Locale and access | English/Japanese GUI, colour-independent status, keyboard access to primary tasks, light/dark and existing high-contrast themes | U1 or R1, D1 |
| Distribution | Only the channels selected in Section 9, with observed package/release evidence | C1, R1 |

### 6.2 Supported existing capabilities

The following implemented capabilities remain supported v1 product behavior,
but their exact UI placement, wording, or internal representation is not frozen:

- profiles and recent projects;
- preferences in the OS-standard config directory;
- append-only audit history;
- project `.aaai.yaml` configuration and `.aaaiignore`;
- rule templates, glob-pattern assistance, ticket, approver, expiry, and notes;
- dashboard, filters, search, batch approval, undo, and revert-to-Pending;
- watch mode, shell completions, progress indication, and structured CLI
  output;
- side-by-side, unified, changes-only, and binary-summary views;
- theme selection and saved locale/theme preference.

These capabilities may be simplified, hidden under an expert surface, or
temporarily disabled **during remediation only** when necessary to meet a
safety gate. At D1 and R1, every supported-existing capability must be enabled
and its grouped acceptance evidence in Section 6.4 must pass. If that cannot be
achieved, the owner must approve an RFC 095/D0 amendment moving the capability
to Deferred or explicitly narrowing its promise before WS-12 assembles public
documentation and before WS-13 assembles release evidence.

Retaining a CLI subcommand name while its intended operation is unavailable
does not satisfy this contract. A command stub, permanent `disabled` response,
or undocumented no-op requires the same owner-approved scope amendment.

### 6.3 Deferred from v1

- New end-user functionality unrelated to remediation.
- Languages beyond English and Japanese.
- Screen-reader interoperability that requires unavailable iced hooks. The CLI
  remains the non-visual alternative; this limitation must be explicit.
- Automatic OS theme selection until the GUI toolkit provides reliable system
  theme detection.
- Windows ARM64 packages.
- Direct signed MSIX distribution, automatic signing, automated Store
  submission, and a public Store availability claim.
- Semantic image/media diffing or other new binary-analysis features beyond
  the existing summary.
- Performance work beyond the WS-10 full-scale or approved replacement budgets.

### 6.4 Grouped feature acceptance and evidence ownership

Existing code and historical tests are inputs, not automatic acceptance. The
following matrix assigns every core and supported-existing capability group to
the workstream and gate that must produce acceptance evidence before R1.

| Capability group | Tier | Accountable workstream and gate | Minimum evidence class |
|---|---|---|---|
| Folder comparison, diff classification, audit statuses, strategies, and GUI/CLI verdict parity | Core | WS-04/S2, WS-10/E1, WS-12/D1 | Shared-engine functional/parity suite; adversarial selected-root cases; approved scale fixtures and budgets; documented behavior matrix |
| Definition creation/load/check/merge/save, mandatory reason, and approval metadata | Core | WS-06/P1, WS-12/D1 | Versioned fixture round trips; invalid-input cases; atomic/concurrent save and recovery cases; executable definition/approval contract |
| Reports, exports, masking, and structured CLI output | Core and supported | WS-05/S2, WS-12/D1 | Format/schema conformance; contextual encoding, formula, and masking cases; documented version/compatibility contract |
| All 16 CLI commands and exit codes | Core | WS-02/S1, WS-03/B0, WS-09/C1, WS-12/D1 | Isolated stateful-command suite; hosted command/help/invocation matrix; canonical long-option and exit-code snapshot; release workflow execution |
| Guided and expert GUI core workflow | Core | WS-06/P1, WS-11/U1, WS-12/D1 | Shared-state transition scenarios; dirty-state/save/recovery cases; keyboard, visual, and task-completion evidence on the required display set |
| English/Japanese, colour-independent status, keyboard access, and light/dark/high-contrast themes | Core | WS-11/U1, WS-12/D1 | Locale completeness; primary-task keyboard matrix; contrast and real-display visual evidence; documented limitations |
| Profiles, recent projects, preferences, and append-only history | Supported | WS-02/S1, WS-12/D1 | Operator-state isolation; current/compatibility fixture read-write tests; history append/retention cases; enabled-surface checks |
| Project config, `.aaaiignore`, and input-selection assistance | Supported | WS-04/S2, WS-12/D1 | Discovery/precedence cases; cross-root and adversarial pattern cases; CLI/GUI behavior and public contract |
| Templates, glob assistance, ticket, approver, expiry, and notes | Supported | WS-04/S2, WS-06/P1, WS-12/D1 | Rule matching and path cases; persistence round trips; expiry/metadata behavior; enabled-surface checks |
| Dashboard, filters, search, batch approval, undo, and revert-to-Pending | Supported | WS-06/P1, WS-11/U1, WS-12/D1 | Result/filter/search scenarios; multi-entry and rollback persistence; guided/expert task and keyboard evidence |
| Watch, completions, and progress indication | Supported | WS-02/S1, WS-03/B0, WS-10/E1, WS-12/D1 | Isolated watch state; hosted command matrix; shell output checks; progress behavior under approved scale fixtures |
| Side-by-side, unified, changes-only, and binary-summary views; saved theme/locale choice | Supported | WS-05/S2, WS-11/U1, WS-12/D1 | Safe rendering cases; view-selection/state persistence; real-display visual and locale/theme persistence evidence |
| Selected v1 distribution channels | Core | WS-09/C1, WS-13/R1 | Package/publish dry runs, declared-target archives, source/MSRV build, maintainer tarball checks, and final zero-blocker review |

An earlier workstream may split a group into more precise acceptance cases, but
it may not delete a listed capability or transfer it to WS-13 without an
owner-approved RFC 095/D0 amendment. WS-13 consumes this evidence; it does not
define missing behavior at the release boundary.

## 7. GUI baseline

### 7.1 Approved decision: guided default plus retained expert workspace

The approved v1 GUI baseline is:

- **Default beginner path:** the simplified guided flow from the important
  v0.33.2 wireframe—Start, Checking, one-change review, Done, and Save report.
- **Expert surface:** retain the existing three-pane file tree / diff /
  inspector workspace behind an explicit `For technical users` or equivalent
  action.
- **One product state:** both surfaces operate on the same engine result,
  definition, selection, unsaved state, and persistence code. There is no
  second audit implementation.

The guided path must:

1. show one dominant next action;
2. use Original folder, Changed folder, Review record, Why is this change OK?,
   Save this answer, Check again, and Save report language by default;
3. keep strategy variant names, raw YAML terms, full status taxonomy, technical
   errors, and report-format choice behind details;
4. keep the reason field as the primary review input;
5. expose text changes on demand instead of forcing a diff as the first focus;
6. preserve unsaved-work guards and keyboard access to primary actions;
7. remain usable at 800 × 550 pixels;
8. allow entry to and return from the expert workspace without losing work.

WS-11 owns the detailed external/internal design, migration, and visual
acceptance. Approval of this RFC selects the guided roadmap branch and moves
the serial program target to 2027-04-23; it does not begin WS-11 early.

### 7.2 Alternatives considered

| Option | Result | Assessment |
|---|---|---|
| Retain three-pane as the default | Shorter program ending 2027-03-19 | Lower cost, but conflicts with the important simplified wireframe and the project rule that first-time users should see less. |
| Guided default plus retained expert workspace | Program ending 2027-04-23 | **Selected.** Resolves the beginner/expert conflict while preserving existing expert capability and migration safety. |
| Guided-only; remove the expert workspace | Program must absorb removal/migration risk | Rejected for v1 because it discards working capability and makes rollback harder without improving the core safety gates. |

## 8. v1 compatibility boundary

The contract starts at v1.0.0. Versions v0.x remain pre-release and do not gain
retroactive compatibility promises.

### 8.1 Stable through v1.x

Subject to observed D1/R1 evidence, v1.x protects:

- the product invariants in Section 4;
- the 16 CLI subcommand names;
- canonical long options that actually exist in the approved v1.0 CLI help
  snapshot;
- audit-result exit codes 0 through 4;
- the versioned `audit.yaml` schema and its documented migration rules;
- the OS-standard config location and readable formats for `prefs.yaml`,
  `profiles.yaml`, and `history.jsonl`, after compatibility tests verify the
  promised reader/writer behavior;
- Markdown, JSON, HTML, SARIF, CSV, and TSV availability, with machine formats
  governed by an explicit schema/version contract before v1 release;
- SemVer-compatible public `aaai` engine APIs, with deprecation and migration
  notes for an unavoidable breaking safety fix;
- the core GUI tasks: select, check, review, reason, approve, save, re-check,
  and report;
- English and Japanese GUI availability and keyboard access to primary tasks;
- the existence of light, dark, and current high-contrast choices.

### 8.2 Not frozen by v1.x

- Guided/expert layout, pane structure, exact placement, spacing, colours,
  animation, or pixel output.
- Human-readable wording, provided the meaning and actionable error structure
  remain compatible.
- Internal i18n key names. Locale completeness is a gate; internal keys are not
  a public API.
- Short CLI flags when they conflict; canonical long forms govern.
- Help prose and progress rendering.
- Performance characteristics outside the approved E1 budgets.
- Cache, temporary, and other derived internal artifacts.
- Private Rust modules and front-end implementation details.

This explicitly replaces the prospective three-pane and internal-i18n-key
promises in the current pre-v1 compatibility document. WS-12 must rewrite that
document from the final implementation and must not claim a contract that lacks
tests or executable evidence.

### 8.3 Compatibility ownership and acceptance matrix

Every stable promise has a prerequisite contract owner and a final test-backed
acceptance owner before R1.

| Stable v1.x surface | Prerequisite design/change owner | Final acceptance owner and gate | Required evidence class |
|---|---|---|---|
| Product invariants and feature availability | Each workstream that changes the relevant surface; RFC 095 amendment for narrowing | WS-12/D1; WS-13 only assembles for R1 | Section 6.4 feature matrix with every included capability enabled and accepted |
| CLI command names, canonical long options, and exit codes | Any RFC changing CLI behavior; WS-09 hosts the approved checks in C1 | WS-12/D1 | Generated v1.0 help snapshot, command invocation matrix, and exit-code tests through hosted CI |
| `audit.yaml` schema and migration rules | WS-06/P1 owns versioned persistence and any migration affected by its changes | WS-12/D1 | Prior/current-version fixtures, unknown/invalid-field cases, semantic round trips, atomic write/recovery evidence, and published migration rule |
| `prefs.yaml`, `profiles.yaml`, and `history.jsonl` location and read/write behavior | Any workstream that changes a format must define compatibility and migration before implementation | WS-12/D1 | Prior/current fixture corpus, unknown-field behavior, read-write/append tests, config-location matrix, and migration/limitation statement |
| JSON, SARIF, CSV, and TSV machine contracts | WS-05/S2 owns schema/version rules together with safe encoding and masking | WS-12/D1 | Versioned schema or explicit stable field contract, golden/conformance fixtures, consumer-safe encoding cases, and public format documentation |
| Markdown and HTML availability | WS-05/S2 owns safe rendering behavior | WS-12/D1 | Golden structural cases, encoding/masking cases, and documented human-format stability boundary |
| Public `aaai` engine API | Every workstream changing public API must record compatibility impact; WS-09/C1 hosts the approved comparison check | WS-12/D1 | Approved public-API baseline, SemVer comparison, API docs, and migration/deprecation record for any exception |
| Core GUI tasks and guided/expert state behavior | WS-11/U1, with WS-06/P1 for persisted/dirty state | WS-12/D1 | Task/state-transition scenarios, keyboard paths, save/recovery evidence, and public workflow contract |
| English/Japanese, primary keyboard access, and theme choices | WS-11/U1 | WS-12/D1 | Locale completeness, primary-task keyboard matrix, contrast/visual evidence, and supported-theme inventory |

Changing to the retained three-pane branch requires an owner-approved RFC 095
amendment and roadmap revision assigning the GUI rows to WS-12/D1 and WS-13/R1
with an explicit real-display operator; U1 cannot be silently omitted while
keeping the same promises.

If observed evidence cannot support any approved compatibility promise, work
stops and the owner approves an RFC 095/D0 amendment before D1 documentation
and WS-13 planning. WS-13 may not weaken, invent, or substitute compatibility
evidence.

## 9. v1 distribution set

### 9.1 Included, contingent on C1 and R1

| Channel | v1 scope |
|---|---|
| crates.io | Publish `aaai`, `aaai-cli`, and `aaai-gui` in dependency order after package dry runs and crate-identity repair. `aaai-gui` is an installable binary package, not a supported library API. |
| GitHub Releases — Linux | x86_64 GNU/Linux CLI and GUI archives. |
| GitHub Releases — macOS | Apple Silicon CLI and GUI archives. |
| GitHub Releases — Windows | x86_64 MSVC CLI-only, GUI-only, and combined ZIP archives. |
| Source build | Build the workspace from source with the declared MSRV on supported platforms. |
| Maintainer delivery | Provide the project release tarball required by project policy, version-suffixed with files at archive root and no intermediate parent directory. |

The target/format list is a v1 scope decision, not a statement that the current
workflow passes. WS-09 must reconcile the release workflow, crate rename,
release instructions, archive policy, and package/publish commands.

### 9.2 Deferred distribution

Microsoft Store and MSIX work is not in the v1 distribution set. RFC 091's
one-product/two-executable packaging model remains the preferred future design,
but its implemented label does not prove Store availability. The following are
post-v1-decision work unless the owner later amends D0 and assigns the required
owners and evidence:

- MSIX candidate as a required release artifact;
- local MSIX install and terminal-alias validation;
- real Store artwork and screenshots;
- signing identity and direct signed MSIX distribution;
- Store account metadata, certification, and submission;
- automated or semi-automated Store submission;
- Windows ARM64 packaging.

Until those items pass their own review, public documentation must not say that
aaai is available from the Microsoft Store.

After D0 approval, the owner should authorize a minimal public-truth correction
promptly rather than wait for the full WS-12 convergence milestone. That
correction removes present-tense Store availability and installation claims
only; it does not implement Store work or broaden M0.

## 10. Security and release posture

This RFC narrows authority; it does not lower any remediation gate.

- S1 still proves that tests cannot read, print, prune, or mutate operator
  state.
- S2 still governs symlinks, path/error reporting, contextual output encoding,
  spreadsheet neutralization, and masking.
- P1 still governs atomic replacement, locking, collision, interruption, and
  recovery.
- C0/C2/C1 still govern dependency disposition, format/lint policy, CI,
  packaging, and release operations.
- E1 still governs full-scale or owner-approved replacement budgets.
- D1 still reconciles every public claim to the approved baseline.
- R1 still requires an independent review with zero blocking findings.

No feature in Sections 6–9 may be advertised as release-ready merely because
this RFC includes it in v1 scope.

## 11. Downstream workstream constraints

| Workstream | Constraint supplied by RFC 095 |
|---|---|
| WS-02 | Isolation covers all user-state surfaces used by the included CLI and existing supported capabilities. It may not change a promised persisted format without defining compatibility/migration acceptance first. |
| WS-03 | Hosted bootstrap uses the three current crate names and declared MSRV; it need not build Store/MSIX artifacts. |
| WS-04 | Selected-root policy applies to CLI and both GUI surfaces. |
| WS-05 | Secure Markdown/JSON/HTML/SARIF and CSV/TSV; define machine schema/version contracts and human-format stability; keep masking consistent across CLI, guided GUI, and expert GUI. |
| WS-06 | Definition persistence must serve guided and expert surfaces through one versioned contract and own `audit.yaml` read/write, migration, atomicity, and recovery acceptance. |
| WS-07–WS-09 | Release assurance targets the Section 9.1 distribution set only. WS-09/C1 hosts the approved CLI and public-API compatibility checks without defining their contracts. |
| WS-10 | Scale design serves the included CLI and both GUI surfaces. |
| WS-11 | Required because this RFC selects the guided branch; the existing three-pane workspace is retained as expert mode. U1 owns shared-state tasks, keyboard, locale, theme, visual, and accessibility acceptance. |
| WS-12 | Produce and pass the Section 8.3 compatibility acceptance matrix, including CLI snapshot, persisted user-state fixtures, machine-output contracts, public API baseline, core GUI tasks, locales/keyboard, and themes; then rewrite public scope, GUI, compatibility, crate identity, and distribution claims. D1 is test-backed, not documentation-only. |
| WS-13 | Assemble prior feature, compatibility, and Section 9.1 distribution evidence only. It cannot define missing acceptance, accept a disabled included capability, or invent a compatibility exception; deferred Store/MSIX evidence is not a release blocker. |

Every downstream RFC that changes a Section 6.4 capability or Section 8.3
stable surface must identify the affected matrix row, preserve its evidence
owner, and define any required migration before implementation approval.

## 12. Risks and mitigations

| Risk | Mitigation |
|---|---|
| Guided work extends the critical path | The roadmap records the selected guided schedule; WS-11 begins only after M5 and receives its own review and handoff. |
| Two GUI surfaces duplicate logic or drift | Require one engine, result model, persistence contract, and unsaved-state model; guided/expert are views over shared state. |
| Retaining many existing features weakens focus | Section 6.4 makes each group release-determinate; guided mode hides secondary tools and any narrowing requires an owner-approved D0 amendment. |
| Compatibility promises exceed evidence | Section 8.3 assigns prerequisite contracts and test-backed D1 ownership; unsupported current prose is explicitly non-authoritative. |
| Deferring Store disappoints users who read current docs | Make a minimal owner-authorized truth correction promptly after D0, then complete full convergence in WS-12. |
| crates.io or target packaging proves infeasible | C1 blocks release and requires an owner-approved D0 amendment rather than silently dropping a channel. |

## 13. D0 decision and acceptance record

The project owner approved every row after independent design review.

| Decision | Approved selection | Owner status |
|---|---|---|
| Product authority | Domain-specific precedence in Section 5 | Approved 2026-07-17 |
| Product identity and invariants | One engine with CLI and GUI; mandatory reason; read-only audited roots | Approved 2026-07-17 |
| Feature scope and release disposition | Section 6 inventory and acceptance matrix; every included capability enabled and accepted before R1 or moved/narrowed by owner-approved D0 amendment before D1/WS-13 | Approved 2026-07-17 |
| GUI baseline | Guided default plus retained expert three-pane workspace | Approved 2026-07-17 |
| Program branch | Guided branch ending 2027-04-23 | Approved 2026-07-17 |
| Compatibility boundary and ownership | Section 8 boundary and acceptance matrix; prerequisite contracts assigned before implementation and final test-backed acceptance owned by WS-12/D1 | Approved 2026-07-17 |
| v1 distribution | crates.io, declared GitHub archives, source build, maintainer tarball | Approved 2026-07-17 |
| Store/MSIX | Deferred from v1 distribution and release evidence | Approved 2026-07-17 |
| Feature freeze | No unrelated end-user features until the v1 release decision | Approved 2026-07-17 |

D0 completion evidence:

1. Independent architecture re-review returned `Accept with notes` and stated
   that every note is non-blocking for the owner decision.
2. The owner approved every row above without revision.
3. The roadmap records the guided branch and completed D0 gate.
4. The Section 6.4 feature and Section 8.3 compatibility matrices cover every
   included capability and stable promise with a workstream, gate, and evidence
   class.
5. No v1 feature, compatibility, or distribution decision remains ambiguous.

This decision RFC is now operational and is stored under `rfcs/done/`. Under
the accepted serial program, RFC 096 design may begin. No product
implementation is authorized by this lifecycle transition.

## 14. Review, handoff, and amendment

- **Review completed:** independent architecture re-review accepted the design
  with explicitly non-blocking notes on 2026-07-17.
- **Owner approval completed:** nabbisen approved the Section 13 matrix as
  proposed on 2026-07-17.
- **Developer handoff:** none. This RFC is a product decision and acceptance
  matrix; downstream RFCs own implementation handoffs.
- **Handoff approver:** not applicable.
- **Amendment before dependent implementation:** revise this RFC and affected
  roadmap/workstream contracts, then repeat independent review and owner
  approval before relying on the changed decision.
- **Change after D0:** create an explicit amendment or superseding RFC, update
  the roadmap branch and affected downstream contracts, and re-review the
  changed scope. Do not silently edit product authority after dependent work
  begins.

## 15. Evidence and limitations

Design inputs inspected for this RFC:

- accepted `ROADMAP.md` current program;
- `.git-exclude/rules/project-instructions-rust-gui.md` and RFC lifecycle
  policy;
- v0.40 project handoff bundle and decision log;
- important v0.33.2 simplified wireframe;
- current RFC index and relevant completed RFCs;
- current CLI command surface, Cargo manifests, release workflows, packaging,
  README, mdBook GUI/compatibility/Store documentation, and release guidance.

Limitations:

- No product tests, hosted jobs, GUI runs, platform builds, package dry runs, or
  Store checks establish this design decision.
- The inventory describes intended v1 scope at repository baseline `f24d5ee`;
  it does not certify current safety or correctness.
- Exact downstream commands, fixtures, thresholds, and evidence paths remain
  owned by their workstream RFCs and required handoffs.
