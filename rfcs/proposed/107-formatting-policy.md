# RFC 107 — Formatting Policy: adopt rustfmt

**Status.** Proposed

**Tracks.** `ROADMAP.md` M4B / gate C2 (formatting and lint policy settled and
passing) — the formatting half. Clippy is the other half and is **not** in this
RFC; see §8.

**Depends.** Nothing. M4B's entry dependency was narrowed on 2026-08-10 to the
owner's formatting decision, which is now made.

**Design owner.** high-capability model

**Decision owner.** nabbisen, project owner — **decided 2026-08-10: adopt
rustfmt.**

**Proposed implementer.** mid-capability model, after design review

**Evidence location.** `.git-exclude/evidence/107-formatting-policy/`

**Touches.** Every `.rs` file in the workspace, mechanically. `docs/src/` and
its Japanese counterpart for the policy statement. No behaviour change, no
dependency, no i18n key, no workflow change.

**Handoff.** Required, after acceptance.

---

## 1. Summary

The workspace is hand-aligned — `=` signs padded to a column, trailing comments
padded to a column — and `cargo fmt --check` has therefore failed
repository-wide for the project's whole life. **Adopt rustfmt with default
settings**, reformat once, and the check becomes meaningful.

The decision is the owner's and is made. This RFC records it, specifies the
change, and settles the two questions the decision implies: whether to
configure rustfmt, and where the check is enforced.

## 2. What this overturns

The hand-alignment convention is referenced throughout the project's review
history as **DEC-006**. Example, from `crates/aaai/benches/diff_bench.rs`:

```rust
-    let after  = tempfile::tempdir().unwrap();          // aligned '='
+    let after = tempfile::tempdir().unwrap();
-            format!("after content {i}\n")   // modified   // aligned comment
+            format!("after content {i}\n") // modified
```

**rustfmt cannot be configured to preserve this.** Its entire configuration
surface contains no alignment option — the only `align` setting concerns tabs
versus spaces for indentation. So "keep the convention and make the check pass"
is not available; the real choice was *adopt the standard formatter* or *have
no formatter*, and the owner chose the former.

### 2.1 A record problem worth fixing on the way past

**DEC-006 has no findable definition.** It is cited in a dozen artifacts under
`.git-exclude/reviewed/`, and it gates a milestone, but no tracked file in this
repository states what it says. DEC-003 and DEC-005 are the same.

That means this RFC cannot formally supersede a text it cannot locate. It
instead **states the new policy positively** (§4) in a tracked, findable place,
and the decision register — wherever it lives — should be updated by the owner
to point here. Recorded because a decision that gates work and has no canonical
text is a governance defect independent of formatting.

## 3. Why now

The reformat touches every source file and conflicts with anything in flight.
**Right now nothing is in flight**: v0.41.0 shipped, RFC 098 closed, the dev
team has no assigned implementation, and unit 2 has not started.

This is the cheapest moment the project will have. After RFC 100 begins, the
same change would collide with a large module restructure.

## 4. Selected design

### 4.1 rustfmt defaults, no `rustfmt.toml`

Do not add a configuration file.

| Option | Decision |
|---|---|
| **Pure defaults, no config** | **Selected.** What every Rust contributor and tool expects; nothing to maintain or defend |
| Tune `max_width` etc. to reduce churn | Rejected — the churn is one-time, the config is forever |
| Nightly-only options (`imports_granularity`, `group_imports`) | Rejected — MSRV is 1.91 **stable**; a policy that needs nightly is not a policy |

A configuration file is a set of small decisions each of which must be
justified later. Defaults need no justification.

### 4.2 One mechanical commit, alone

`cargo fmt --all`, committed by itself with no other change. Measured on a probe
run (reverted): **66 files, 3717 insertions, 1945 deletions.**

A diff that large is only reviewable if it is **regenerable**: a reviewer runs
`cargo fmt --all` on the parent commit and confirms the result is identical.
That property is lost the moment anything else rides along.

### 4.3 Enforcement — the local check, not a new CI job

`cargo fmt --check` stays where it already is: the per-RFC verification command
list every handoff carries. **Nothing changes mechanically. What changes is that
it becomes meaningful** — it has always failed, so it has always been reported
with a disclaimer and skipped over.

| Option | Decision |
|---|---|
| **Keep the local check; it now signals honestly** | **Selected.** C2's contract is *"format and Clippy commands pass"* — it does not require CI enforcement |
| Add `cargo fmt --check` to B0's Linux leg | Rejected **for now**. B0 is RFC 097's declared-MSRV bootstrap authority and its gate is a stop-work signal; making a formatting violation stop work conflates concerns and broadens a contract another RFC owns |
| Wait for `ci.yaml` repair (C1 / M4C) | Rejected as the *only* answer — it leaves the policy unenforced for an unknown period — but it is the right long-term home |

**Stated honestly: this is weaker than CI enforcement.** Between this RFC and
M4C's `ci.yaml` repair, nothing mechanically prevents drift. The mitigation is
that drift is now *visible* — a red `cargo fmt --check` in an implementation
report means something for the first time. If drift happens anyway, that is
evidence for promoting the check, and M4C is where it belongs.

## 5. Acceptance contract

1. `cargo fmt --all` applied; **no manual edits** in the same commit.
2. The commit contains **only** formatting changes — no `.rs` file gains or
   loses a statement, and no non-`.rs` file changes except as item 6 requires.
3. **Regenerable:** `git checkout <parent> && cargo fmt --all && git diff` is
   empty against the reformat commit. This is the reviewability property and
   the item most worth checking.
4. `cargo +1.91 fmt --check --all` exits 0.
5. `cargo +1.91 test --workspace --locked` — counts **exactly**
   146 / 13 / 97 / 27 / 3 on Linux, unchanged. A formatting change that moves a
   test count has done something other than formatting.
6. `docs/src/` and its Japanese counterpart state the policy: rustfmt defaults,
   no configuration file, run before committing.
7. No `rustfmt.toml` is added.
8. RFC 099's V1 greps still return nothing (`.size(N)` literals,
   `Color::from_rgb` outside `design_tokens.rs`) — rustfmt does not change
   these, and confirming it costs one command.

## 6. Verification sequence

```sh
cargo fmt --all
git diff --stat                                   # expect ~66 files
cargo +1.91 fmt --check --all                     # must exit 0
cargo +1.91 test --workspace --locked             # 146/13/97/27/3
cargo +1.91 check --target x86_64-pc-windows-gnu -p aaai --tests --locked
git diff --check
grep -rE "\.size\([0-9]" crates/aaai-gui/src/     # nothing
grep -rn "Color::from_rgb" crates/aaai-gui/src/   # nothing outside design_tokens.rs
```

Hosted B0 confirms the other two platforms. Expect Windows `aaai` 134,
Linux/macOS 146 — unchanged.

## 7. Risks

| Risk | Mitigation |
|---|---|
| A 3700-line diff hides an unintended change | Acceptance item 3: the diff is regenerable, so a reviewer verifies it by reproducing it rather than reading it |
| Reformat breaks a macro or string that depends on layout | Acceptance item 5 — test counts and results unchanged. Rust formatting is token-level and cannot alter semantics, but the check is cheap |
| Blame history becomes noisy | Real and permanent. Mitigated by the commit being formatting-only, so `git blame --ignore-rev` can skip it; record its SHA in `docs/` |
| Policy decays before CI enforces it | Acknowledged in §4.3, not solved. C1 / M4C owns the durable fix |
| Conflicts with in-flight work | §3 — there is none right now, which is why this is scheduled now |

## 8. Not in this RFC

**Clippy.** M4B's gate C2 covers *"format and Clippy commands pass"*, and
Clippy currently fails with roughly 10 findings in `aaai` and 18–20 in
`aaai-gui`. That is a separate decision — which lints, which are allowed, and
whether any are genuine defects — and folding it in would delay a change whose
whole value is that it is mechanical and can land today.

**C2 does not close with this RFC.** Its formatting half will be satisfied; its
Clippy half needs its own decision, and its third clause about oversized files
overlaps RFC 100.

## 9. Sources

- Owner decision, 2026-08-10, in session
- `.git-exclude/reviewed/064-roadmap-consistency-audit-2026-08-10.md` §7 (F10),
  which narrowed M4B's entry dependency and unblocked this
- `ROADMAP.md` M4B row and the C2 gate contract
- Probe run measuring the reformat: 66 files, 3717 insertions, 1945 deletions
  (applied and reverted, tree left clean)
