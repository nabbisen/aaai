# RFC 048 — Inspector progressive disclosure + profile row simplification

**Status.** Implemented (v0.24.0)
**Tracks.** Design principle: Less is more
**Touches.** `crates/aaai-gui/src/app.rs` (1 new state field, init),
`crates/aaai-gui/src/views/inspector.rs` (advanced section behind toggle),
`crates/aaai-gui/src/views/opening.rs` (remove RFC 047 Fix A third line),
`crates/aaai-gui/locales/{en,ja}.yaml` (2 new keys, 1 removed → net +1).

## Context

"Less is more. Sophisticated UI/UX comes from limited information and
considered user workflows. Excessive data at a glance brings serious
noise to immature users. We can provide an advanced view to matured
users in addition."

## Item A — Inspector progressive disclosure

### Problem

The Inspector currently shows 8–10 elements for every selected file:

```
[header: path / change type / status badge]
[Reason textarea]          ← essential
[Ticket input]             ← expert
[Approved by input]        ← expert
[Expires at input]         ← expert
[Strategy picker + rules]  ← needed for non-None only
[Template picker]          ← expert
[Note textarea]            ← expert
[Revert to Pending btn]    ← contextual (OK entries only)
[Validation errors]        ← contextual
```

A first-time user who wants to type a reason and approve faces seven
fields they don't need to touch. The form intimidates without adding
value at first contact.

### Fix

Split into **primary** (always shown) and **secondary** (behind a
toggle):

**Primary — always visible:**
- Header (path, change type, status, EXPIRED badge)
- Reason textarea
- Strategy picker + strategy-specific rules

**Secondary — behind "▸ More options" / "▾ More options" toggle:**
- Ticket
- Approved by
- Expires at
- Template picker
- Note textarea

**Why this split?**
- Reason is the _raison d'être_ of aaai (approvals without reason
  are forbidden). It must be front-and-centre.
- Strategy is visible because selecting "None" vs "LineMatch" vs
  "Regex" is a meaningful choice even for first-time users; hiding
  it would cause confusion when a diff needs content verification.
- The other five fields (Ticket, Approved by, Expires at, Template,
  Note) are metadata and workflow annotations. They add value for
  teams but are noise for individual or first-time users.

**State:** `App.advanced_inspector_expanded: bool` — global across
entries, so a user who expands once stays expanded for the session.
Initialized `false`.

**New messages:** `ToggleAdvancedInspector`

**New i18n keys:**
```yaml
inspector.advanced_toggle_show: "More options"
inspector.advanced_toggle_hide: "Fewer options"
```

### Result

New user sees:
```
config/server.toml — Modified — Pending
Reason  [type a reason here            ]
Strategy  [None ▼]
▸ More options
[Approve & Save]
```

Power user expands once, sees the full form permanently during that session.

## Item B — Profile row third line removed

RFC 047 Fix A added a `📋 audit.yaml` sub-line to each Recent Projects
row. By "less is more": the profile name is already auto-derived from
the definition stem (RFC 042 `auto_save_profile()`), so the row
`▸ audit` already implies `audit.yaml`. Adding the third line
increases visual density without adding differentiation for the common
case.

**Revert:** remove `def_line` from the profile row layout and remove
the `opening.recent_project_definition` i18n key.

RFC 047 Fix B (auto-expand Optional settings on `LoadProfile`) is kept
— it solves a genuine discoverability gap.

## i18n delta

| Key | Change |
|---|---|
| `inspector.advanced_toggle_show` | **+new** `"More options"` / `"詳細オプション"` |
| `inspector.advanced_toggle_hide` | **+new** `"Fewer options"` / `"詳細を隠す"` |
| `opening.recent_project_definition` | **removed** (RFC 047 Fix A reverted) |

Net: **+1 key** × 2 locales → 215 → **216 / 216 / 216**

## Acceptance criteria

- [ ] `advanced_inspector_expanded: bool` in `App`, init `false`
- [ ] Inspector shows Reason + Strategy only by default
- [ ] `▸ More options` / `▾ Fewer options` toggle works
- [ ] All five advanced fields hidden when collapsed
- [ ] Profile row `📋 …` third line absent
- [ ] `opening.recent_project_definition` removed from both locales
- [ ] 2 new inspector toggle keys in both locales
- [ ] `check-i18n-keys.py --quiet` → 0/0/0 (216/216/216)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All tests pass (101 / 70 / 15)
