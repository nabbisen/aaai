# RFC 114 — Lint policy and CI restoration: implementer handoff

**RFC.** [`rfcs/accepted/114-lint-policy-and-ci-restoration.md`](../../accepted/114-lint-policy-and-ci-restoration.md)

**Status.** **Accepted by the owner 2026-10-19.** Ready to implement.

**Baseline.** `main` at `a24928b`. **Re-measure every figure below before
relying on it** — see §7.

**Evidence location.** `.git-exclude/evidence/114-lint-policy-and-ci-restoration/`

| Role | Who | Scope |
|---|---|---|
| Design owner | high-capability model | The RFC and this handoff |
| Implementer | developer | T1–T5 |
| Reviewer | high-capability model | On request |
| Integrator | nabbisen | Commit, push, observe CI |

---

## 1. What this is

**Clear 33 clippy findings, write the lint policy that stops them coming
back, and turn `ci.yaml` on again.**

Gate **C2** — formatting and lint policy settled and passing — is the last
thing between this repository and a running CI workflow. The other three
reasons it was parked are already resolved.

**Why it matters beyond tidiness:** the repository's own advisory scanner
(`rustsec/audit-check`) lives inside the parked workflow. Seven advisories
reached this project through another team's letters in September because a
scan we already own was switched off. That is what C2 is costing.

## 2. Measure first

Every number in the RFC was measured on `3ce0453`. Re-measure on current
`main` before you start:

```sh
for c in aaai aaai-cli aaai-gui; do
  echo "== $c"
  cargo +1.91 clippy -p $c --all-targets --no-deps 2>&1 \
    | grep -E '^warning: ' | grep -v generated | sort | uniq -c | sort -rn
done
cargo +1.91 test --workspace --locked   # baseline counts
```

Expected on `3ce0453`: engine 10, CLI 10, GUI 13. **If the totals differ,
report the new ones and carry on** — the figures are a starting point, not an
acceptance criterion. Acceptance is "zero under `-D warnings`".

## 3. Tasks

### T1 — The 31 mechanical fixes (commit per crate)

RFC §2 lists them by category. Each has one obvious rewrite and clippy names
it. **Three commits, one per crate**, so the diff stays reviewable.

**Two categories deserve a second read before you apply them:**

- **`map_or`** (5 sites) — the simplification can change which side is
  evaluated. Check short-circuit order is preserved.
- **`collapsible_if`** (10 sites) — merging conditions with `&&` is only
  equivalent if neither arm has an `else` and nothing sits between them.

Everything else — `to_string`, `single_match`, `useless_conversion`,
`cmp_owned`, `derivable_impls`, `redundant_closure`, `ptr_arg`,
`needless_borrow`, `field_reassign_with_default`, `match_single_binding` — is
a local rewrite with no control-flow content.

### T2 — `IgnoreRules::from_str`: allow, do not fix (same commits)

`crates/aaai/src/diff/ignore.rs:40`. clippy wants `std::str::FromStr`
implemented instead. **Do not.** `IgnoreRules` is re-exported from
`aaai::lib`, so that is a public API change, and `docs/src/compatibility.md`
puts engine API changes behind a major version.

Add `#[allow(clippy::should_implement_trait)]` with a comment naming the
compatibility constraint and that it is a v2 candidate. RFC §4.2.

### T3 — `print_human_audit`: fix by grouping (same commits)

`crates/aaai-cli/src/cmd/audit.rs:264`, nine arguments against a limit of
seven. Private, not API-bound, and the lint is pointing at a real readability
problem rather than a style preference — so **group the presentation
arguments into a small struct** rather than allowing it.

**If grouping reaches beyond `cmd/audit.rs`, stop and report.** That would
make it a refactor, which this RFC is not.

### T4 — Every `#[allow]` gets a reason (same commits)

**There are 20 in the workspace and not one carries a reason comment today.**
Verified, not assumed:

| Attribute | Count | Origin |
|---|---:|---|
| `#[allow(clippy::needless_return)]` | 9 | RFC 100 phase A |
| `#[allow(clippy::collapsible_if)]` | 6 | RFC 100 phase A |
| `#[allow(clippy::needless_return, clippy::collapsible_if)]` | 1 | RFC 100 phase A |
| `#[allow(dead_code)]` | 3 | pre-dates RFC 100 |
| `#[allow(unreachable_code)]` | 1 | pre-dates RFC 100 |

**Keep every one of them; add a one-line reason to each.** The 16 clippy ones
exist because RFC 100 was a pure move under a byte-identical-body check and
fixing them would have broken that proof — say so. For the 4 non-clippy ones,
work out why they are there and write it; **if you cannot work out why one
exists, say so in the review request rather than inventing a reason.** An
allow nobody can justify is a finding, not a formality.

Do not remove the suppressions. Editing 16 bodies is behaviour risk for no
functional gain.

### T5 — Policy and CI (final commit)

**`CONTRIBUTING.md`** gains RFC §5.2's rule, beside RFC 107's formatting rule:

> Any `#[allow(clippy::…)]` must carry a comment saying why the lint does not
> apply here, or which decision defers it. An allow with no reason is a defect
> in review.

**`.github/workflows/ci.yaml`:**

- clippy step becomes `cargo clippy --workspace --all-targets -- -D warnings`
  — **not** `--no-deps`. That flag was the right diagnostic while a backlog
  existed; as a gate it would let a dependency-surfaced finding through.
- restore `on: push` and `on: pull_request`.
- **Keep the parking note as history, marked resolved.** Do not delete it —
  the same treatment RFC 112 gave it. All four reasons are now closed and the
  header should say so.

**No `clippy.toml`.** RFC §5.3 — defaults, for the same reason RFC 107 took
rustfmt's.

## 4. Acceptance

RFC §7 is the contract. Five notes:

**Item 1** — `cargo +1.91 clippy --workspace --all-targets -- -D warnings`
exits 0. This is the gate; the per-crate counts in §2 are not.

**Item 5** — test counts unchanged against the baseline you measured in §2.
**Any movement stops the work and is reported.** A mechanical lint fix that
changes a test result is not mechanical.

**Item 7** — `ci.yaml` runs on push and pull request, header records the four
resolved reasons.

**Expect `audit-security` to pass with four warnings.** `cargo audit` exits 0
on unmaintained and unsound advisories; only vulnerabilities fail it. Those
four need C0 exception records from the owner and are **not** your problem.

**Expect the first CI run to surface jobs nobody has run in months.** Build,
test and i18n are exercised continuously by B0, so they should be fine, but
if `docs-build` or another job fails for an unrelated reason, report it rather
than fixing it inside this RFC.

## 5. Out of scope

- The four accepted advisory warnings (gate C0) — owner's decision.
- Changing `IgnoreRules`' public API — T2 defers it deliberately.
- MSIX, parked 2026-10-18; the owner packages manually.
- Any behaviour change at all.
- RFC 113's T5 file-read bound. Authorized for unit 4 and tracked separately;
  it is robustness work, not lint.

## 6. When you are done

Package a review request as usual, entry point stated in chat. Then Integrator
commits and pushes, and **this is the first push where `ci.yaml` itself will
run** — treat its first run as part of the deliverable, not as someone else's
problem.

## 7. A standing check this project has learned five times

**Any line number, count or figure in this document is stale until you
re-measure it on current `main`.**

RFC 100 shipped acceptance items written wider than its scope; RFC 111's
removal table needed regenerating; RFC 104's §2 quoted a moved file; RFC 109's
tables predicted an overtaken migration; and this RFC itself corrected a
long-quoted "17 allows" to the measured 16 while being written. Re-measure
first.
