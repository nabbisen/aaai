# aaai RFCs

For where RFCs live and how they move between states, see
[done/000-rfc-lifecycle-policy.md](done/000-rfc-lifecycle-policy.md).

For the medium-term plan, see [PLAN.md](PLAN.md) (Rev. 4 — Phases 12–16, the
route to v1.0.0).

---

## Proposed — awaiting review

| ID | Title | Tracks |
|---|---|---|
| [101](proposed/101-guided-review-flow.md) | Guided Review Flow | MG3 / WS-11 / U1 |
| [105](proposed/105-visual-verification-scope-and-assertions.md) | Visual Verification: Scope, Assertions, Evidence Location | Successor to RFC 017 |

RFCs 099–101 are the GUI remediation sequence and must be implemented in that
order; **RFC 099 shipped in v0.41.0 and RFC 100 in v0.42.0**, so the remaining
order is **104 → 106 → 110 → 101**. RFC 111 led and is done — it deleted the
unreachable code the later RFCs would otherwise have carried.
Each one's handoff states its entry conditions. See
`.git-exclude/reviewed/038-gui-remediation-roadmap-and-milestones-2026-07-28.md`.

RFC 104 and RFC 106 rewire `crates/aaai-gui/`, so both had to follow RFC 100's
module restructure rather than be undone by it — that restructure shipped in
v0.42.0. RFC 110 comes after them, because it touches both `view()` and
`update()` which RFC 106 also rewires. See RFC 104 §7, RFC 106 §8, and
RFC 110 §8.

---

## Accepted — owner-approved, awaiting implementation

| ID | Title | Tracks |
|---|---|---|
| [106](accepted/106-keyboard-operability.md) | Keyboard Operability | NF-4 / ABDD |
| [110](accepted/110-modal-overlay-consolidation.md) | Modal Overlay Consolidation — hardcoded backdrops to snora's dim | V1 (after RFC 106) |

RFCs the owner has approved for implementation and that have not shipped. The
folder is the source of truth for this state (RFC 000); each moves to `done/`
when it ships.

---

## Done — implemented

| ID | Title | Shipped in |
|---|---|---|
| [000](done/000-rfc-lifecycle-policy.md) | RFC Lifecycle Policy | — (policy) |
| [001](done/001-cli-output-ux.md) | CLI Output UX Consistency | v0.11.0 |
| [002](done/002-inspector-validation.md) | Inspector Validation & Primary Action | v0.12.0 |
| [003](done/003-abdd-status-display.md) | ABDD Status Display | v0.12.0 |
| [004](done/004-opening-screen-validation.md) | Opening Screen Input Validation | v0.11.0 |
| [005](done/005-keyboard-navigation.md) | Keyboard Navigation & Focus | v0.12.0 |
| [006](done/006-report-output-ux.md) | Report Output UX | v0.14.0 |
| [007](done/007-toolbar-navigation.md) | Toolbar & Navigation | v0.15.0 |
| [008](done/008-bottom-action-bar.md) | Bottom Action Bar | v0.15.0 |
| [009](done/009-reason-textarea.md) | Reason Field Textarea | v0.15.0 |
| [010](done/010-diff-view-legend.md) | Diff View Legend | v0.15.0 |
| [011](done/011-diff-view-tabs.md) | Diff View Tabs | v0.16.0 |
| [012](done/012-linematch-rule-display.md) | LineMatch Rule Color Blocks | v0.16.0 |
| [013](done/013-file-tree-icon-unification.md) | File Tree Icon Unification | v0.17.0 |
| [014](done/014-view-fixes.md) | View Fixes (silent failure remediation) | v0.17.0 |
| [015](done/015-opening-screen-redesign.md) | Opening Screen Redesign | v0.18.0 |
| [016](done/016-i18n-repair.md) | i18n Repair (rust-i18n v4 split-file) | v0.19.0 |
| [017](done/017-visual-verification-harness.md) | Visual Verification Harness & Protocol | v0.20.0 |
| [018](done/018-i18n-fallback-strategies.md) | i18n Locale Fallback Strategies (B/C) — *partial*, §3.4 only | v0.20.0 |
| [019](done/019-documentation-refresh.md) | Documentation Refresh for v0.15–v0.19 Realities | v0.20.0 |
| [020](done/020-abdd-audit-and-error-messages.md) | ABDD Audit & Action-oriented Errors | v0.20.0 |
| [021](done/021-screen-navigation-continuity.md) | Screen Navigation Continuity — *partial*, banner deferred | v0.20.0 |
| [022](done/022-empty-states-and-first-run.md) | Empty States & First-run Guidance | v0.20.0 |
| [023](done/023-opening-dnd-and-recent.md) | Opening Drag-and-Drop & Recent Polish | v0.20.0 |
| [024](done/024-cli-dashboard-and-help.md) | CLI Dashboard & Help Discoverability Polish | v0.20.0 |
| [025](done/025-v1-0-release-prep.md) | v1.0.0 Release Preparation — *partial*, docs groundwork only | v0.20.0 |
| [026](done/026-toast-error-hints.md) | Toast Error Hints & i18n Key Re-introduction | v0.21.0 |
| [027](done/027-ci-mdbook-build.md) | CI mdbook build job | v0.21.0 |
| [028](done/028-field-error-hint.md) | FieldError native `hint` field | v0.21.0 |
| [029](done/029-field-error-i18n.md) | FieldError i18n migration | v0.21.0 |
| [030](done/030-field-error-hint-authoring.md) | FieldError hint authoring (selective) | v0.21.0 |
| [031](done/031-app-rs-i18n-sweep.md) | User-facing string i18n migration sweep in app.rs | v0.21.0 |
| [032](done/032-views-i18n-sweep.md) | views/*.rs user-facing string i18n migration | v0.21.0 |
| [033](done/033-picklist-display-value-separation.md) | pick_list display/value separation | v0.22.0 |
| [034](done/034-toast-dialog-format-i18n-sweep.md) | Toast / dialog / format-string i18n sweep | v0.22.0 |
| [035](done/035-strategy-label-display-value-separation.md) | Strategy label display/value separation | v0.22.0 |
| [036](done/036-app-settings-dialog.md) | App Settings dialog | v0.23.0 |
| [037](done/037-async-rerun.md) | Async rerun with audit-dirty indicator | v0.23.0 |
| [038](done/038-keyboard-help-overlay.md) | Keyboard shortcuts help overlay | v0.23.0 |
| [039](done/039-revert-to-pending.md) | Revert-to-Pending + Opening profile delete | v0.23.0 |
| [040](done/040-report-file-picker.md) | Report export with native file picker | v0.23.0 |
| [041](done/041-nav-guard.md) | Unsaved-changes navigation guard dialog | v0.23.0 |
| [042](done/042-title-and-auto-profile.md) | Dynamic window title + auto-profile on audit run | v0.23.0 |
| [043](done/043-filter-bar-counts.md) | Status counts in filter bar + bottom-bar count i18n | v0.23.0 |
| [044](done/044-expires-at-enforcement.md) | expires_at enforcement in audit engine | v0.23.0 |
| [045](done/045-opening-optional-settings-cleanup.md) | Opening screen Optional settings cleanup | v0.24.0 |
| [046](done/046-save-as-dialog.md) | Save-as dialog for new approvals files | v0.24.0 |
| [047](done/047-profile-approvals-visibility.md) | Opening screen: profile approvals visibility | v0.24.0 |
| [048](done/048-less-is-more.md) | Inspector progressive disclosure + profile row simplification | v0.24.0 |
| [049](done/049-validation-visibility.md) | Inspector validation visibility + Approvals file placeholder | v0.24.0 |
| [050](done/050-auto-advance.md) | Auto-advance to next Pending entry after approval | v0.24.0 |
| [051](done/051-ctrl-enter-approve.md) | Ctrl+Enter keyboard shortcut for approval | v0.24.0 |
| [052](done/052-auto-select-first-pending.md) | Auto-select first Pending entry on audit start | v0.24.0 |
| [053](done/053-dashboard-all-clear-cta.md) | Dashboard all-clear CTA buttons | v0.24.0 |
| [054](done/054-glob-patterns.md) | Glob pattern entries in Inspector | v0.25.0 |
| [055](done/055-pattern-suggestions.md) | Auto-suggest glob patterns from current path | v0.25.0 |
| [056](done/056-aaai-watch.md) | aaai watch completion | v0.25.0 |
| [057](done/057-aaai-export.md) | aaai export completion | v0.25.0 |
| [059](done/059-aaai-lint.md) | aaai lint activation | v0.26.0 |
| [060](done/060-aaai-merge.md) | aaai merge activation | v0.26.0 |
| [061](done/061-aaai-check.md) | aaai check activation | v0.26.0 |
| [062](done/062-aaai-history.md) | aaai history activation | v0.26.0 |
| [063](done/063-aaai-dashboard.md) | aaai dashboard activation | v0.26.0 |
| [065](done/065-aaai-init.md) | aaai init activation | v0.27.0 |
| [066](done/066-definition-unit-tests.md) | AuditDefinition direct unit tests | v0.27.0 |
| [069](done/069-diff-scroll-sync.md) | Diff pane scroll synchronisation | v0.29.0 |
| [070](done/070-toolbar-layout.md) | Toolbar layout stability and Undo relocation | v0.29.0 |
| [071](done/071-search-in-filetree.md) | Search bar moved inside file tree pane | v0.29.0 |
| [072](done/072-status-badge.md) | Compact status badge and i18n cleanup | v0.29.0 |
| [075](done/075-strategy-recommendation.md) | Strategy pre-selection and plain-language descriptions | v0.30.0 |
| [076](done/076-status-legend.md) | Status legend popover | v0.30.0 |
| [078](done/078-fix-stale-icon-ref.md) | Fix stale □ Open icon reference | v0.31.0 |
| [079](done/079-onboarding-context.md) | Opening onboarding WHY context | v0.31.0 |
| [081](done/081-docs-phase-20-22-update.md) | docs/gui.md update for Phase 20–22 | v0.31.1 |
| [083](done/083-plain-language-actions.md) | Plain-language action labels | v0.32.0 |
| [084](done/084-plain-language-status.md) | Plain-language status labels and hints | v0.32.0 |
| [085](done/085-plain-language-strategy.md) | Plain-language strategy labels | v0.32.0 |
| [087](done/087-disabled-save-tooltip.md) | Save and continue disabled tooltip | v0.33.0 |
| [088](done/088-disabled-check-hint.md) | Check changes disabled inline hint | v0.33.0 |
| [089](done/089-help-overlay-labels.md) | Help overlay plain-language update | v0.33.0 |
| [091](done/091-windows-store-packaging.md) | Windows Store packaging model | v0.33.0 |
| [093](done/093-theme-picker-ui.md) | Theme Picker UI | v0.37.0 |
| [092](done/092-design-system-adoption.md) | Design System Adoption | v0.36.0 |
| [094](done/094-high-contrast-themes.md) | High-Contrast Themes | v0.38.0 |
| [095](done/095-v1-authority-scope-and-distribution.md) | v1 Authority, Scope, GUI, Compatibility, and Distribution | D0 decision (2026-07-17) |
| [096](done/096-test-environment-and-user-state-isolation.md) | Test Environment and User-State Isolation | S1 local Linux (2026-07-17) |
| [097](done/097-safe-hosted-ci-bootstrap.md) | Safe Hosted-CI Bootstrap | B0 hosted bootstrap (2026-07-21) |
| [098](done/098-selected-folder-and-symlink-policy.md) | Selected-folder and Symlink Policy | v0.41.0 (release unit 1) |
| [099](done/099-gui-visual-foundation.md) | GUI Visual Foundation — *partial*, acceptance item 7 deferred to RFC 101 | v0.41.0 (release unit 1) |
| [102](done/102-b0-trigger-scope-and-ci-disposition.md) | B0 Trigger Scope and Legacy CI Disposition | v0.41.0 (release unit 1) |
| [107](done/107-formatting-policy.md) | Formatting Policy: adopt rustfmt | v0.42.0 |
| [100](done/100-gui-module-boundaries.md) | GUI Module Boundaries | v0.42.0 |
| [108](done/108-snora-0-37-migration.md) | snora 0.25.1 → 0.38.0 migration — accessibility repairs and typography | v0.42.0 |
| [111](done/111-batch-approve-removal.md) | Remove the Batch Approve feature — unreachable since v0.15.0 | Unreleased (after v0.42.0) |
| [109](done/109-rfc-lifecycle-sync-and-five-folder.md) | Sync the lifecycle policy, adopt the 5-folder variant, bound the decision register | Unreleased (after v0.42.0) |
| [112](done/112-rename-and-layout-conformance.md) | Finish the `aaai-core` rename and the DEC-003 layout | Unreleased (after v0.42.0) |
| [104](done/104-gui-report-export-masking.md) | GUI Report Export Masking | Unreleased (after v0.42.0) |
| [103](done/103-safe-output-surfaces.md) | Safe Reports, Exports, and Masking | v0.41.0 (release unit 1) |
| [091b](done/091b-ci-handoff-windows-msix.md) | CI handoff — Windows MSIX build | v0.33.0 |
| [090](done/090-count-summary-wording.md) | Count summary wording | v0.33.0 |
| [086](done/086-nav-guard-hide-discard.md) | Navigation guard: hide Discard | v0.32.0 |
| [082](done/082-readme-path-releasing.md) | Fix aaai-core README path + RELEASING.md | v0.31.1 |
| [080](done/080-checksum-howto.md) | Checksum how-to hint | v0.31.0 |
| [077](done/077-coach-line.md) | First-audit coach line | v0.30.0 |
| [074](done/074-reason-guidance.md) | Reason field guidance for newcomers | v0.30.0 |
| [073](done/073-bottom-bar-visibility.md) | Bottom bar hidden when no file selected | v0.29.0 |
| [068](done/068-snora-0-18.md) | snora 0.18.0 dependency update | v0.28.0 |
| [067](done/067-readme-fix.md) | README accuracy fix | v0.27.0 |
| [064](done/064-suggest-patterns-tests.md) | GUI suggest_patterns unit tests | v0.26.0 |
| [058](done/058-pending-count-title.md) | Pending count in window title | v0.25.0 |

> **Note**: RFCs 007–016 are "code implemented, visual verification not yet
> passed". Evidence is gathered with RFC 017's harness, and any discrepancy it
> reveals becomes its own fix RFC.

> **What v0.20.0 was about**: Phase 12 implemented 9 RFCs and cleared 19
> pre-existing bugs. From Phase 13 onward, see ROADMAP.md instead.
>
> **Phase 13 progress** (targeting v0.21.0):
> - RFC 026 (Toast Error Hints) landed — extended RFC 020's message+hint pattern to toasts and restored the 4 i18n keys Phase 12 had deleted
> - RFC 027 (CI mdbook build) landed — added `mdbook build` to CI, bluntly preventing the orphan-chapter problem Phase 12 exposed
> - RFC 028 (FieldError native hint) landed — separated the inline `💡` composition RFC 026 had used as a workaround into structure. `FieldError` now carries `hint: Option<String>` and the inspector renders it as a second line in muted style
> - RFC 029 (FieldError i18n) landed — put the 4 hardcoded English validation messages left in `app.rs` through `t!()`, so Japanese users read every validation error in Japanese
> - RFC 030 (FieldError hint authoring, selective) landed — extending RFCs 028/029, added actionable hints at the 2 sites where a hint genuinely helps (Checksum hex format, LineMatch empty rule). The other 2 sites got none, because the message already states the action and a hint would be noise. Wording was matched against UI labels such as `+ Add rule`
> - RFC 031 (app.rs i18n sweep) landed — put all **8** user-facing hardcoded English strings remaining in `app.rs` (progress / batch validation / inspector validation / opening inline validation) through i18n, leaving **zero** user-facing hardcoded strings in that file. Error wording originating from the engine's `is_approvable()` is explicitly deferred, because changing it needs a major version bump
> - RFC 032 (views/*.rs i18n sweep) landed — put **20** user-facing hardcoded English strings in `views/{batch,dashboard,diff_view,inspector,main_view,opening}.rs` through i18n (4 diff_view binary labels, 3 SHA-256 labels, 2 hash-match states, 11 others). The 5 pick_list strings that double as display text and Message protocol values were marked out of scope and deferred to a separate Message-protocol refactor RFC

> **Phase 14 progress** (targeting v0.22.0):
> - RFC 033 (pick_list display/value separation) landed — put the 5 pick_list strings RFC 032 deferred (`Added`/`Removed` for the LineMatch action picker; `Added lines`/`Removed lines`/`All changed lines` for the RegexTarget picker) through i18n. `Message::LineRuleActionChanged(String)` and `Message::RegexTargetChanged(String)` now carry `LineAction` / `RegexTarget` enum payloads, with a `LocalizedOption<T>` adapter added to `util.rs`. Every user-facing string in `aaai-gui` now goes through `t!()`; what remains in English comes from the engine (Display impls, `Result<(), String>` errors)
> - RFC 034 (toast/dialog/format-string i18n sweep) landed — swept the patterns RFCs 031–033 could not detect (direct `&str` arguments to `push_toast()`, English inside `format!()`, `rfd::AsyncFileDialog::set_title()` arguments) using three greps for `format!`, `push_toast` and `.set_title`. 16 call sites and 13 new keys (toast bodies, native dialog titles, the diff size_inline format string). Applying RFC 031's lesson — run the exhaustive grep before drafting — it finished in one implementation pass with zero scope creep
> - RFC 035 (strategy label display/value separation) landed — closed the last i18n gap RFC 034 had ruled out of scope (`STRATEGIES: &["None", "Checksum", "LineMatch", "Regex", "Exact"]`, 5 strings) with a `StrategyKind` discriminator enum plus the `LocalizedOption<StrategyKind>` pattern. `AuditStrategy` is a struct-shaped enum carrying associated data, so unlike `LineAction`/`RegexTarget` in RFC 033 it cannot be used directly; the discriminator enum was added to `aaai-gui/src/util.rs` without touching the engine API. `Message::StrategySelected(String)` became `StrategySelected(StrategyKind)`, `InspectorState.strategy_label: String` became `strategy_kind: StrategyKind`, and `strategy_from_label()` plus two `STRATEGIES` constants were deleted. **This closes the GUI i18n loop** — the only English left comes from the engine or is documented as out of scope

---

## Archive — withdrawn or superseded

_None._

---

## Mapping to the medium-term plan (Phases 12–16)

See [PLAN.md](PLAN.md) Rev. 4 for detail.

| Sprint | Target version | RFCs | Focus |
|---|---|---|---|
| E-1 | v0.20.0 | 017, 018, 019 | Visual verification / i18n finishing / docs refresh |
| F-1 | v0.21.0 | 020 | ABDD audit + actionable error messages |
| G-1 | v0.22.0 | 021, 022 | Screen-relation continuation + empty states |
| H-1 | v0.23.0 | 023, 024 | Opening drag-and-drop + CLI finishing |
| I-1 | v1.0.0-rc → v1.0.0 | 025 | Pass the release-decision gate + compatibility declaration |

### Dependencies

```
017 ──┬─> 018          (if verification shows B/C are needed)
      └─> 019          (settle docs from the verification result)
019 ──> 020 ──> 021 ──┬─> 023
                       └─> 024
021, 022, 023, 024 ──> 025 (release decision)
```

---

## Past sprints (Phases 1–11)

| Sprint | RFC | Shipped in |
|---|---|---|
| A | 001 + 004 | v0.11.0 |
| B | 002 + 003 | v0.12.0 |
| C | 005 + integration tests | v0.12.0 |
| (D onward) | 006–016 | v0.13.0–v0.19.0 |
