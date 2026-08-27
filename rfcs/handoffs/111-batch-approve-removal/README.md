# RFC 111 — Batch Approve removal: implementer handoff

**RFC.** [`rfcs/proposed/111-batch-approve-removal.md`](../../proposed/111-batch-approve-removal.md)

**Status.** Accepted by the owner. Ready to implement.

**Baseline.** `main` at `f5385a4` (v0.42.0, B0 green). **Re-measure before
relying on any line number here** — see §6.

**Evidence location.** `.git-exclude/evidence/111-batch-approve-removal/`

| Role | Who | Scope |
|---|---|---|
| Design owner | high-capability model | The RFC and this handoff |
| Implementer | GUI developer | T1–T3 |
| Reviewer | high-capability model | On request |
| Integrator | nabbisen | Commit, push, observe CI |

---

## 1. What this is

**Delete the Batch Approve feature.** It has been unreachable since v0.15.0:
nothing anywhere constructs `Message::OpenBatchSheet`, and every other `Batch*`
message is sent only from inside `views/batch.rs`, a view that cannot be shown.

**This is not a dead-code sweep and not a judgement call.** RFC 007 removed the
toolbar button and RFC 008 — same release — made the bottom action bar the sole
approval mechanism. The decision is in the record; only the leftovers are.

**There is no behaviour to preserve.** That is the whole reason this is safe,
and it is also the thing to keep checking: if you find something that *is*
reachable, stop and report rather than adapting the plan.

## 2. Before you start

Three things that will otherwise cost you time:

**The RFC's line numbers were regenerated on 2026-08-27** against `f5385a4`.
An earlier draft cited a monolithic `app.rs`; RFC 100 shipped in v0.42.0 and
spread this feature across eight files. If anything does not match, re-measure
and say so — do not pattern-match to the old shape.

**Three `batch` matches are iced's API and must survive** (RFC §4.1a):
`Task::batch` at `app/update/approve.rs:11` and `:97`, `Subscription::batch` at
`app/subscription.rs:96`. They combine iced tasks and subscriptions. Deleting
one will not fail a test in an obvious way — it will change control flow.

**The i18n list is 10 keys, not 8.** Two live under `toast:`, not `batch:` —
`toast.batch_approved` and `toast.batch_approved_count`, both used only in the
dead `commit_batch_approve`. The RFC's first draft missed them.

## 3. Tasks

### T1 — Delete the feature (commit 1)

Work through RFC §4.1's table. It is exhaustive as of `f5385a4`.

Order that keeps the tree compiling at each step is your choice; one commit is
right for the whole deletion, because a half-deleted feature is not a
meaningful intermediate state.

**Keep**, and check you have not caught them by accident:

- `AuditStrategy`, `StrategyKind` — used throughout the live inspector;
- `error.inspector.reason_required.message` — a different key from
  `error.batch.reason_required.message`;
- `start_async_rerun` — deleting `commit_batch_approve` removes one of its five
  callers; four remain (`audit.rs:7`, `approve.rs:95`, `:210`, `:298`).

### T2 — `close_modals` becomes a no-op (same commit)

`app/update/dialogs.rs:165-167` is:

```rust
pub(in crate::app) fn close_modals(&mut self) {
    self.batch_sheet_open = false;
}
```

That is its entire body. **Keep the method, keep `Message::CloseModals`, and
keep `.on_close_modals(Message::CloseModals)` wired in `app/view.rs`.** Reduce
the body to a no-op with a comment pointing at RFC 110, which makes
`CloseModals` the single dismissal path for the three real overlays.

Deleting it here only to have RFC 110 re-add it would churn the same lines
twice. This is the one place the two RFCs touch and it is deliberate.

### T3 — Evidence and report (same commit)

Record in `.git-exclude/evidence/111-batch-approve-removal/`:

- the ELOC delta for `crates/aaai-gui/src/` as a whole, per file;
- which i18n method you used for acceptance item 4 (see §4);
- the clippy result, before and after, as sets rather than counts.

## 4. Acceptance

RFC §5 is the contract. Four items need a word here.

**Item 2** — `grep -rn "batch" crates/aaai-gui/src/` must return **exactly the
three iced-API matches** in §2 and nothing else. Not "nothing but unrelated
matches": name them.

**Item 4** — if `PyYAML` is unavailable, **do not install it.** Compare the two
locale files' key sets directly and say which method you used. The RFC 100 work
established this: a green check obtained by exceeding authority is worth less
than a stated gap.

**Item 5** — `cargo +1.91 clippy -p aaai-gui --all-targets --no-deps`. Note
this is **not** `-- -D warnings`: the crate carries 13 pre-existing findings and
that flag would fail on all of them. The check is that **no new finding
appears** and that **no `dead_code` warning appears** — a new one means
something was reachable only through the deleted code and is now orphaned. That
is this RFC's real safety net. Report before and after as sets.

**Item 6** — counts must stay **146 / 13 / 97 / 27 / 3**. If any count moves,
**stop and report.** A moved count means a test was exercising unreachable
code, which is a finding about the test, not something to fix in passing.

## 5. What is out of scope

- **Restoring or redesigning batch approval.** RFC 008 settled it. If you think
  it should come back, that is a product decision and its own RFC.
- **`push_user_error_toast`**, which has no callers either. Recorded in RFC 100's
  T6 visibility audit; RFC 111 §3.2 makes unrelated dead-code sweeps a non-goal
  on purpose. Leave it.
- **The 13 clippy findings and the 17 `#[allow]`s** from RFC 100. Both are gate
  C2 debt awaiting their own RFC.
- **Anything under `crates/aaai/` or `crates/aaai-cli/`.** No diff outside
  `crates/aaai-gui/`.

## 6. A standing check this project has learned twice

**Any line number, ELOC figure, or test count in this document is stale until
you re-measure it on current `main`.**

RFC 100 shipped with three acceptance items written wider than its own scope,
and this RFC's own removal table was regenerated once already after RFC 100
moved everything it referenced. Re-measure first; if a figure is wrong, report
it rather than working around it.

## 7. When you are done

Package a review request as usual, entry point stated in chat. Then Integrator
pushes and B0 runs.

RFC 111 is first in the sequence **111 → 104 → 106 → 110 → 101**; RFC 104 does
not start until this is accepted.
