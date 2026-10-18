# RFC 114 — Lint policy, and restoring CI

**Status.** **Accepted** — approved for implementation by the owner
2026-10-19. Lives in `rfcs/accepted/`; the folder is the source of truth for
this state (RFC 000).

**Tracks.** `ROADMAP.md` M4B / gate **C2** (formatting and lint policy is
settled and passing). Unblocks gate **C1** (the whole workflow green), and
with it the advisory scan that gate **C0** depends on.

**Depends.** RFC 107 settled formatting and is Implemented; RFC 112 removed the
stale crate names from `ci.yaml`. Both are prerequisites and both are done.

**Design owner.** high-capability model

**Decision owner.** nabbisen, project owner

**Proposed implementer.** mid-capability model, after approval

**Evidence location.** `.git-exclude/evidence/114-lint-policy-and-ci-restoration/`

**Touches.** Roughly 30 sites across all three crates; `.github/workflows/ci.yaml`;
`CONTRIBUTING.md`. No public API change except as §4.2 records. No persisted
format, no dependency.

**Handoff.** Required, after acceptance.

---

## 1. Summary

**`ci.yaml` has been parked since RFC 102 §5.4 for three stated reasons. Two
are already resolved and this RFC resolves the third.** What is left is
`cargo clippy --all-targets -- -D warnings`, failing on **33 findings**
measured today.

This is not only a cleanup. C2 is *"formatting and lint policy is **settled**
and passing"*, and RFC 107 settled formatting without settling lint. **A
backlog cleared without a policy returns.** §5 is therefore as much the
deliverable as §3.

**What restoring CI buys.** The repository's own advisory scan
(`rustsec/audit-check`) lives in the parked workflow. Between 2026-09-12 and
2026-09-17, **seven advisories reached this project from another team's
letters** rather than from a scan we already own and had switched off. That is
the concrete cost of leaving C2 open.

## 2. The debt, measured

Measured on `3ce0453` with `cargo +1.91 clippy -p <crate> --all-targets --no-deps`:

| Crate | Findings |
|---|---:|
| `aaai` (engine) | 10 |
| `aaai-cli` | 10 |
| `aaai-gui` | 13 |
| **Total** | **33** |

By category, across all three:

| Lint | Count | Nature |
|---|---:|---|
| `collapsible_if` | 10 | mechanical |
| `map_or` simplification | 5 | mechanical |
| `unnecessary to_string` | 3 | mechanical |
| `single_match` → `if let` | 3 | mechanical |
| `field_reassign_with_default` | 3 | mechanical |
| `useless_conversion`, `match_single_binding`, `cmp_owned`, `derivable_impls`, `redundant_closure`, `ptr_arg`, `needless_borrow` | 1 each | mechanical |
| `should_implement_trait` — `IgnoreRules::from_str` | 1 | **public API — §4.2** |
| `too_many_arguments` — `print_human_audit` 9/7 | 1 | **judgement — §4.3** |

**31 of 33 are mechanical.** The two that are not are the whole of the design
content here.

## 3. Goals and non-goals

### 3.1 Goals

1. `cargo +1.91 clippy --workspace --all-targets -- -D warnings` exits 0.
2. A written lint policy, so the next finding is caught at the commit rather
   than accumulating.
3. `ci.yaml` runs again on push and pull request.

### 3.2 Non-goals

- **Changing the public API.** §4.2's finding is deferred, not fixed.
- Behaviour changes of any kind. Every fix in §3's mechanical set is
  semantics-preserving; if one is not, it stops and is reported.
- Re-opening RFC 107's formatting decision.
- The four accepted advisory warnings (C0). Separate gate, separate decision.
- MSIX, parked 2026-10-18.

## 4. Selected design

### 4.1 Fix the 31 mechanical findings

No judgement required; each has a single obvious rewrite and clippy names it.
Grouped per crate so the diff stays reviewable.

### 4.2 `IgnoreRules::from_str` — allow, with the reason recorded

```
warning: method `from_str` can be confused for the standard trait method
  --> crates/aaai/src/diff/ignore.rs:40:5
```

`IgnoreRules` is re-exported from `aaai::lib`, so this is **public API**.
clippy's remedy is to implement `std::str::FromStr` instead, which changes the
call shape for every consumer.

`docs/src/compatibility.md` treats engine API changes as needing a major
version, and the decision log already defers one such change (the
`is_approvable()` error type) on the same grounds. **A lint is not a reason to
break compatibility.**

**Decision: `#[allow(clippy::should_implement_trait)]` with a comment naming
the compatibility constraint and the v2 follow-up.** An allow carrying its
reason is settled policy; a bare allow is debt in disguise — §5.2.

### 4.2a The CI toolchain — pinned to 1.91 (added 2026-10-19)

`ci.yaml`'s `test` job installed `dtolnay/rust-toolchain@stable`, so the gate
CI ran was not the gate `CONTRIBUTING.md` and §7 item 1 name. On stable 1.99.1
the workspace fails with three lints that do not exist in 1.91.

**It is pinned to `@1.91`, which is conformance rather than a new decision:**
`CONTRIBUTING.md:21` already requires `+1.91` for both fmt and clippy, `b0.yaml`
pins 1.91, and `ci.yaml`'s own `check-msrv` pins 1.91. The `test` job was the
only floating toolchain in the estate, and it was parked when the rest
converged.

Nothing is lost by pinning: no workflow currently builds on stable, so there is
no forward-compatibility coverage to give up. Acquiring some is a separate
deliberate choice and is **not** taken here — an advisory job that is
permanently yellow is ignored exactly as reliably as one that is permanently
red.

### 4.3 `print_human_audit` — fix by grouping, not by allowing

Nine arguments on a private CLI print helper. It is not API-bound and nothing
forces the shape.

**Decision: group the audit-presentation arguments into a small struct.** This
is the one finding where the lint is pointing at a real readability problem
rather than a style preference, and the owner's stated preference is for
*finally clean … and sophisticated design*. Allowing it would be the cheaper
and worse answer.

If grouping turns out to touch call sites beyond the one, stop and report —
that would make it a refactor rather than a lint fix.

### 4.4 The 16 clippy `#[allow]`s RFC 100 left — keep, and justify in place

Measured on `3ce0453`, workspace-wide: **16 `#[allow(clippy::…)]`** — 9
`needless_return`, 6 `collapsible_if`, 1 carrying both. Plus 4 non-clippy
allows (3 `dead_code`, 1 `unreachable_code`) which pre-date RFC 100.

> Earlier notes in this project said 17. That figure was correct when taken and
> is not now — RFC 111's deletion removed one along with the code it guarded.
> Re-measured rather than quoted.

RFC 100's phase A added them for lints that only became visible once arm bodies
left the 2,000-line `match`. They were correct then: RFC 100 was a pure move
under a byte-identical-body check, and fixing them would have broken that
proof.

**That constraint is gone.** The bodies are now ordinary methods.

**Decision: keep them, but each must carry a one-line reason.** Removing them
means editing 16 bodies, which is a behaviour-risk for zero functional gain,
and they are a true record of how that code arrived.

**None of the 20 allows in the workspace carries a `//` reason comment** —
verified, not assumed. That is the actual gap, and §5.2 is what closes it. The
implementer adds the reason to all 20, clippy and non-clippy alike; the
suppressions stay.

> **Corrected 2026-10-19 at review.** "None carries a reason" was overstated:
> two of the three `dead_code` allows already explained themselves in an
> adjacent **doc** comment, which is a reason even though my grep for `//`
> did not see one. The implementer added explicit `//` lines so all are
> uniformly greppable, and the final count is **21** — §4.2 adds the
> twenty-first.

## 5. The policy — the half that makes this stick

### 5.1 `-D warnings` in CI, workspace-wide

`ci.yaml`'s clippy step becomes:

```yaml
run: cargo clippy --workspace --all-targets -- -D warnings
```

Not `--no-deps`. That flag was the correct **diagnostic** while the backlog
existed — it let reviewers compare a crate's own findings across a change —
but as a gate it would let a dependency-surfaced finding through silently.
With the backlog at zero, the strict form is the honest one.

### 5.2 Every `#[allow]` carries a reason

A suppression without a stated reason is indistinguishable from an oversight.
The policy, to go in `CONTRIBUTING.md` beside RFC 107's formatting rule:

> Any `#[allow(clippy::…)]` must carry a comment saying why the lint does not
> apply here, or which decision defers it. An allow with no reason is a defect
> in review.

This is deliberately not machine-enforced. A grep for allows without an
adjacent comment is easy to write and easy to defeat, and the real enforcement
is review — the same conclusion RFC 110 §6a.1 reached about grepping for
colours.

### 5.3 No `clippy.toml`

RFC 107 chose rustfmt defaults over a config file, reasoning that a project's
style should be the ecosystem's unless it has earned a difference. The same
applies: **clippy's default lint set, unconfigured.** If a specific lint proves
wrong for this project repeatedly, that is an RFC amendment with evidence, not
a config line added in passing.

## 6. Restoring CI

Once §3's goal 1 holds, remove the parking note from `ci.yaml`'s header and
restore `on: push` and `on: pull_request`.

**State of the three stated parking reasons:**

| Reason (ci.yaml header) | State |
|---|---|
| Repository-wide `cargo fmt --check` debt | **Resolved** — RFC 107 |
| Jobs naming the nonexistent `aaai-core` | **Resolved** — RFC 112 |
| Open advisories in the security-audit job | **Resolved** 2026-09-17 — `cargo audit` exits 0 |
| *(unlisted fourth)* clippy `-D warnings` | **This RFC** |

The header's narrative of why it was parked should be kept as history, marked
resolved, rather than deleted — the same treatment RFC 112 gave it.

**Expect `audit-security` to report four warnings and still pass.** `cargo
audit` exits 0 on unmaintained and unsound advisories; only vulnerabilities
fail it. Those four need C0 exception records, which is the owner's separate
decision and not a blocker here.

## 7. Acceptance contract

1. `cargo +1.91 clippy --workspace --all-targets -- -D warnings` exits 0.
2. **All 20 `#[allow]`s in the workspace carry a reason comment** — the 16
   clippy ones from RFC 100, the 4 non-clippy ones that pre-date it, and the
   one §4.2 adds. Today none of them does.
3. `IgnoreRules::from_str` is unchanged in signature and behaviour.
4. `print_human_audit` takes a grouped argument; no call site outside
   `cmd/audit.rs` changes.
5. `cargo +1.91 test --workspace --locked` — **every count unchanged**:
   re-measure the baseline on current `main` first and report the figures.
   Any movement stops the work.
6. `cargo +1.91 fmt --check --all` exits 0.
7. `ci.yaml` runs on push and pull request; its header records the four
   resolved reasons rather than deleting them.
8. `CONTRIBUTING.md` carries §5.2's rule.
9. No behaviour change. Each mechanical fix is semantics-preserving; any that
   is not is reported rather than applied.

## 8. Risks

| Risk | Mitigation |
|---|---|
| A "mechanical" fix changes behaviour | Item 5 — counts unchanged, any movement stops the work. `map_or` and `collapsible_if` rewrites are the two worth a second read, since both can alter short-circuit order if done carelessly |
| Unparking CI floods the project with failures from jobs nobody has run in months | Item 1 and item 6 are run locally first. The remaining jobs — build, test, i18n — are run continuously by B0 already |
| §4.3's grouping grows into a refactor | §4.3 says stop and report if it reaches beyond `cmd/audit.rs` |
| The policy is written and ignored | §5.2 is in `CONTRIBUTING.md` next to the formatting rule that has held since RFC 107, and acceptance item 2 makes it checkable once |
| 33 findings is a large diff to review | §4.1 groups per crate. The diff is mechanical and clippy names each rewrite, so it is reviewable by reproduction rather than by reading — RFC 107's method |

## 9. Sources

- `.github/workflows/ci.yaml` — the parking note and the four reasons
- RFC 102 §5.4 — the parking decision; RFC 107 — formatting policy and the
  defaults-over-config reasoning
- `docs/src/compatibility.md` — why §4.2 defers rather than fixes
- `.git-exclude/reviewed/080-rfc-disposition-checkpoint-2026-10-18.md` §3 — C0's
  dependence on this gate
