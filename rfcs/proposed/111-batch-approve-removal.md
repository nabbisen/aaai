# RFC 111 — Remove the Batch Approve Feature

**Status.** Proposed

**Tracks.** Precursor to MG2 / V2 (GUI files split below the size limit). Not a
milestone item of its own — see §7.

**Depends.** Nothing. **Blocks nothing, but should land before RFC 100** — see
§7 for why the order matters and is not merely preference.

**Design owner.** high-capability model

**Decision owner.** nabbisen, project owner

**Proposed implementer.** mid-capability model, after design review and owner
approval

**Evidence location.** `.git-exclude/evidence/111-batch-approve-removal/`

**Touches.** `crates/aaai-gui/src/app.rs`, `crates/aaai-gui/src/views/batch.rs`
(deleted), `crates/aaai-gui/src/views/mod.rs`,
`crates/aaai-gui/src/views/main_view.rs`, and both locale files. No engine, CLI,
persisted-format, or dependency change.

**Handoff.** Required, after acceptance.

---

## 1. Summary

**The Batch Approve feature cannot be reached by any input, and has not been
reachable since v0.15.0** (RFC 007), twenty-six minor releases ago. Roughly
300 lines of view, state, messages, and translations remain in the tree for a
feature the project decided against.

This RFC deletes it. There is no behaviour to preserve, because there is no
behaviour: nothing in the running application can enter this code.

Found by the dev team while investigating an unrelated instruction
(`.git-exclude/review-requests/072-snora-0-39-1-review-2026-08-20.md` §0–§1),
confirmed and traced to its decision in
`.git-exclude/reviewed/072-snora-0-39-1-review-2026-08-20.md` §4.

## 2. The finding

### 2.1 Unreachable, verified

`Message::OpenBatchSheet` is the only entry point. It is **declared** at
`app.rs:500` and **handled** at `app.rs:988`, and **constructed nowhere** in the
workspace. `self.batch_sheet_open` is assigned `true` at exactly one site —
`app.rs:989`, inside that handler.

Every other `Batch*` message is sent only from inside `views/batch.rs` itself:

| Message | Only sender |
|---|---|
| `CommitBatchApprove` | `views/batch.rs:153` |
| `CloseBatchSheet` | `views/batch.rs:167` |
| `BatchReasonChanged`, `BatchStrategySelected` | `views/batch.rs` inputs |
| `ToggleBatchSelect` | **no sender at all** |

So the cluster has no entry from anywhere in the application, and its internal
messages are reachable only from a view that cannot be shown.

### 2.2 This was decided, not lost

The temptation is to read an orphaned feature as an accident and restore the
button. **The record says otherwise, in two steps.**

- **RFC 007** removed the Batch Approve toolbar button, conditionally:
  *「削除。バッチ承認機能はキーボードショートカット等で代替」* — delete, replace
  with a keyboard shortcut or similar — deferring the replacement to
  *「RFC 008 と合わせて検討」*.
- **RFC 008** then resolved it differently: *「RFC 007（Batch Approve ボタン削除）
  が先行することで、ボトムバーが「唯一の承認手段」として機能する」* — with RFC 007
  landing first, the bottom action bar functions as the **sole** approval
  mechanism.

RFC 008 supersedes RFC 007's provisional "replace with a shortcut". **No
keyboard replacement is owed**, and RFC 106 does not inherit one. What is left
is residue of a decision that was properly made and never swept up.

### 2.3 Why no gate caught it

Rust warns on unused *items*, not unreachable *features*. Every piece here is
constructed, referenced, and compiled: the messages are matched, the view
function is called from a live `if`, the state is initialised. Only the
condition guarding it is never true. Nothing in the type system, the linter, or
the test suite can see that.

The i18n checker cannot see it either — `scripts/check-i18n-keys.py` verifies
that used keys exist and locales agree, which they do. **Seven translated
strings in two languages have been maintained for a screen no user can open.**

## 3. Goals and non-goals

### 3.1 Goals

1. The Batch Approve feature is removed entirely: code, state, messages,
   translations.
2. No behaviour change, because there is no reachable behaviour to change.
3. `crates/aaai-gui/src/` shrinks ahead of RFC 100's restructure.

### 3.2 Non-goals

- **Restoring or redesigning batch approval.** RFC 008 settled this. Reopening
  it is a product decision and would be its own RFC with its own justification.
- Touching the bottom action bar or any live approval path.
- Any other dead-code sweep. This RFC removes one feature whose deadness is
  proven; it does not go looking.

## 4. Selected design

**Delete it.**

| Option | Assessment |
|---|---|
| Leave it | Rejected. It is 300 lines of maintained-looking code that RFC 100 must extract, move, and byte-compare, and that every future reader must work out is dead |
| Restore an entry point | Rejected. RFC 008 decided the bottom bar is the sole approval mechanism; re-adding a second one is a product change, not cleanup |
| Mark it `#[allow(dead_code)]` and keep for later | Rejected. It is not dead code the compiler flags — it compiles cleanly. An annotation would document the deadness without removing the cost, and "for later" has already run from v0.15.0 to v0.41.0 |
| **Delete the whole cluster** | **Selected.** Recoverable from git if RFC 008 is ever revisited, which is what version control is for |

### 4.1 Exact removal list

| Item | Location |
|---|---|
| `BatchApproveState` struct | `app.rs:112-117` |
| `batch` field on `App` | `app.rs:235` |
| Field initialiser | `app.rs:349` |
| 6 `Message` variants + `// Batch` comment | `app.rs:495-502` |
| Handler arms | `app.rs:974-1035` |
| Sheet render branch | `app.rs:2065-2072` |
| `Sheet`, `SheetEdge`, `SheetSize` from the import | `app.rs:15` — **used nowhere else**; the import becomes `use snora::{AppLayout, Toast, ToastIntent, ToastPosition, render};` |
| Whole view module | `views/batch.rs` (202 lines), deleted |
| Module declaration | `views/mod.rs:1` `pub mod batch;` |
| Dead marker | `main_view.rs:554` `let _is_batch = …` |
| 7 keys under top-level `batch:` | `locales/en.yaml:3-10`, `locales/ja.yaml` |
| `error.batch.reason_required.message` | both locale files |

**Keep `AuditStrategy` and `StrategyKind`** — used throughout the live
inspector. **Keep `error.inspector.reason_required.message`** — a different,
live key.

### 4.2 The one item that is not a deletion

`Message::CloseModals` (`app.rs:1947`) does exactly one thing today: close the
batch sheet. After this RFC its body is empty.

**Keep the variant and keep `on_close_modals(Message::CloseModals)` wired**
(`app.rs:2061`), with the body reduced to a no-op carrying a comment pointing at
RFC 110. RFC 110 §4.1 makes `CloseModals` the single dismissal path for all
three real overlays, so deleting it here only to have RFC 110 re-add it would
churn the same lines twice.

This is the one place where the two RFCs touch, and it is deliberate.

## 5. Acceptance contract

1. `grep -rn "batch" crates/aaai-gui/src/` returns nothing but unrelated
   matches, none referring to this feature.
2. `grep -rn "Batch" crates/aaai-gui/src/` returns nothing.
3. `crates/aaai-gui/src/views/batch.rs` does not exist.
4. `python3 scripts/check-i18n-keys.py` is clean, and `en.yaml` and `ja.yaml`
   lost the **same** keys — 7 under `batch:` plus `error.batch.reason_required`.
5. `cargo +1.91 clippy --workspace --all-targets -- -D warnings` clean; in
   particular **no new `dead_code` warning appears**, which would mean something
   was reachable only through the deleted code and is now orphaned.
6. `cargo +1.91 test --workspace --locked` — **all counts unchanged**: 146 / 13
   / 97 / 27 / 3. No test touches this feature; if any count moves, stop and
   report, because it means a test was exercising unreachable code.
7. `cargo +1.91 fmt --check` clean.
8. The application builds and runs; the bottom action bar approves as before.
9. No diff outside `crates/aaai-gui/`.

**Report the measured ELOC delta for `app.rs` and for
`crates/aaai-gui/src/` as a whole.** RFC 100 sizes its work from those numbers
and will need re-measuring after this lands — see §7.

## 6. Risks

| Risk | Mitigation |
|---|---|
| Something is reachable only via the deleted code and becomes orphaned | Acceptance item 5 — a new `dead_code` warning is the detector, and it is why clippy runs with `-D warnings` rather than by eye |
| The feature was wanted after all | §2.2 traces the decision to RFC 008 in the record, not to inference. If the owner disagrees with RFC 008, that is a product decision and this RFC should be rejected rather than amended |
| A test covers it | Acceptance item 6 makes any count change a stop-and-report, not a thing to fix in passing |
| i18n drift between locales | Acceptance item 4 checks both files lost the same keys, not merely that the checker passes |
| Collides with RFC 100 | Real — both touch `app.rs` and `views/mod.rs`, which RFC 100 renames to `views.rs`. §7 sequences it first for exactly this reason |

## 7. Sequencing — why this lands before RFC 100

RFC 100 extracts 107 `update()` arm bodies into methods and proves the
restructure by requiring each extracted body be **byte-identical** to the arm it
replaced (RFC 100 §6a).

Landing this RFC first is better on three counts, and landing it *after* is
worse on all three:

1. **Less work.** RFC 100 extracts, moves, and byte-compares 6 fewer arms and
   one fewer view file.
2. **The review method stays intact.** Folding a deletion into RFC 100 would
   break its central check — a reviewer cannot distinguish a deleted arm from a
   moved one, which is the exact ambiguity RFC 100 §6a exists to remove.
3. **No rename collision.** RFC 100 converts `views/mod.rs` → `views.rs`. This
   RFC edits `views/mod.rs`. Doing it after means editing a file that has moved.

**RFC 100's figures will need re-measuring after this lands.** RFC 100 §3's line
ranges and §6's ELOC targets are stated against `f67ad56`; this RFC changes
them. That is a known, one-line consequence — acceptance item 9's measured delta
is what feeds it — and it is cheaper than the alternative, which is RFC 100
carefully relocating code we are about to delete.

**RFC 100 is already authorized for implementation.** If the owner prefers not
to hold it, this RFC can follow instead — the deletion still works, it simply
costs the wasted extraction and a second pass over `views.rs`.

## 8. Sources

- `crates/aaai-gui/src/app.rs` — `:15`, `:112-117`, `:235`, `:349`, `:495-502`,
  `:974-1035`, `:1947`, `:2061`, `:2065-2072`
- `crates/aaai-gui/src/views/batch.rs`, `views/mod.rs:1`, `main_view.rs:554`
- `crates/aaai-gui/locales/{en,ja}.yaml`
- RFC 007 and RFC 008, both shipped **v0.15.0** — the decision, in two steps
- `.git-exclude/review-requests/072-snora-0-39-1-review-2026-08-20.md` §0–§1
- `.git-exclude/reviewed/072-snora-0-39-1-review-2026-08-20.md` §4
- RFC 100 §6a (the byte-identical-body check), RFC 110 §4.1 (`CloseModals`)
