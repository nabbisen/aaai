# aaai RFCs

RFC の配置ルールは [done/000-rfc-lifecycle-policy.md](done/000-rfc-lifecycle-policy.md) を参照。

中期計画は [PLAN.md](PLAN.md)（Rev. 4 = Phase 12〜16 / v1.0.0 までの道筋）を参照。

---

## Proposed — 実装待ち

design-system 採用に向けた pre-v1.0 の RFC 群（設計のみ、未実装）:

| ID | タイトル | 依存 |
|---|---|---|
| [093](proposed/093-theme-picker-ui.md) | Theme Picker UI | RFC 092 |
| [094](proposed/094-high-contrast-themes.md) | High-Contrast Themes | RFC 092, 093 |

実装順序: 092 → 093 → 094。各 RFC は独立してレビュー・実装・リバート可能。

---

## Done — 実装済み

| ID | タイトル | 実装バージョン |
|---|---|---|
| [000](done/000-rfc-lifecycle-policy.md) | RFC Lifecycle Policy | — (ポリシー) |
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
| [092](done/092-design-system-adoption.md) | Design System Adoption | v0.36.0 |
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

> **注意**: RFC 007〜016 は「コード実装は完了したが視覚検証は未通過」の状態。
> RFC 017 のハーネスでエビデンスを取得し、判明した差分は個別 fix RFC を別途切る。

> **v0.20.0 の特徴**: Phase 12 で 9 件の RFC が実装され、19 件の pre-existing バグも整理された。
> Phase 13 以降は別途 ROADMAP.md を参照。
>
> **Phase 13 進捗** (v0.21.0 想定):
> - RFC 026 (Toast Error Hints) landed — RFC 020 の message+hint パターンを toast 側にも拡張し、Phase 12 で削除された 4 件の i18n キーを復活
> - RFC 027 (CI mdbook build) landed — `mdbook build` を CI に組み込み、Phase 12 で発覚した orphan chapter 問題などをハンマー的に防ぐ
> - RFC 028 (FieldError native hint) landed — RFC 026 で workaround として使った `💡` インライン合成を構造分離。FieldError が `hint: Option<String>` を持ち、inspector がそれを muted style で第 2 行表示
> - RFC 029 (FieldError i18n) landed — `app.rs` に残っていた 4 件の英語ハードコードされた validation メッセージを `t!()` 経由で i18n 化。日本語ユーザーでも全 validation エラーが日本語で読めるようになる
> - RFC 030 (FieldError hint authoring, 選択的) landed — RFC 028/029 の延長として、ヒントが本当に役立つ 2 サイト（Checksum hex format、LineMatch 空ルール）に actionable hint を追加。残りの 2 サイトは「メッセージ自体が action を示している」ためヒント未追加（noise になる）。`+ Add rule` / `+ ルール追加` のような UI ラベルとの引用整合に注意して文言を作成
> - RFC 031 (app.rs i18n sweep) landed — `app.rs` に残っていた **8 件**の user-facing 英語ハードコード文字列（progress / batch validation / inspector validation / opening inline validation）を全て i18n 化。これにより `app.rs` 内に user-facing ハードコード文字列は **0 件**になった。aaai-core の `is_approvable()` 由来のエラー文言は major version bump が必要なため明示的に deferred
> - RFC 032 (views/*.rs i18n sweep) landed — `views/{batch,dashboard,diff_view,inspector,main_view,opening}.rs` の **20 件**の user-facing 英語ハードコード文字列を i18n 化（diff_view バイナリラベル 4種・SHA-256 ラベル 3種・ハッシュ一致状態 2種、その他 11 件）。display と Message protocol value を兼ねる pick_list 文字列 5 件は out-of-scope として明示し、別 RFC で Message protocol refactor として扱う

> **Phase 14 進捗** (v0.22.0 想定):
> - RFC 033 (pick_list display/value separation) landed — RFC 032 で deferred とした 5 件の pick_list 文字列 (`Added`/`Removed` × LineMatch action picker, `Added lines`/`Removed lines`/`All changed lines` × RegexTarget picker) を i18n 化。`Message::LineRuleActionChanged(String)` / `Message::RegexTargetChanged(String)` を `LineAction` / `RegexTarget` enum payload に変更し、`LocalizedOption<T>` adapter を `util.rs` に追加。これにより `aaai-gui` の user-facing 文字列はすべて `t!()` 経由となり、英語のまま残るのは aaai-core 由来 (Display impls / `Result<(), String>` errors) のみ
> - RFC 034 (toast/dialog/format-string i18n sweep) landed — RFCs 031-033 では検出できなかったパターン（`push_toast()` への直接 `&str` 引数、`format!()` 内部の英語、`rfd::AsyncFileDialog::set_title()` 引数）に対して `format!` / `push_toast` / `.set_title` の 3 種類の grep で網羅 sweep。16 call site / 13 new keys を i18n 化（toast bodies, native dialog titles, diff size_inline format string）。RFC 031 の教訓「draft 前に網羅 grep を流す」を適用し、scope creep ゼロで 1 回の implementation pass で完了
> - RFC 035 (strategy label display/value separation) landed — RFC 034 で out-of-scope とした最後の i18n gap (`STRATEGIES: &["None", "Checksum", "LineMatch", "Regex", "Exact"]` 5 件) を `StrategyKind` discriminator enum + `LocalizedOption<StrategyKind>` パターンで i18n 化。`AuditStrategy` が struct-shaped enum (associated data 持ち) のため、`LineAction`/`RegexTarget` (RFC 033) と違い直接使えないので、discriminator enum を `aaai-gui/src/util.rs` に追加（aaai-core API は touch しない）。`Message::StrategySelected(String)` → `StrategySelected(StrategyKind)`、`InspectorState.strategy_label: String` → `strategy_kind: StrategyKind`、`strategy_from_label()` 関数および 2 個の `STRATEGIES` 定数を削除。**これで GUI i18n loop は完全閉じる** — 残る英語文字列は aaai-core 由来および documented out-of-scope のみ

---

## Archive — 取り下げ・置換済み

_なし_

---

## 中期計画 (Phase 12〜16) との対応

詳細は [PLAN.md](PLAN.md) Rev. 4 を参照。

| Sprint | バージョン目標 | 含まれる RFC | 主眼 |
|---|---|---|---|
| E-1 | v0.20.0 | 017, 018, 019 | 視覚検証 / i18n 仕上げ / docs 刷新 |
| F-1 | v0.21.0 | 020 | ABDD 監査 + 行動可能エラー文 |
| G-1 | v0.22.0 | 021, 022 | 画面リレーション継続 + 空状態 |
| H-1 | v0.23.0 | 023, 024 | Opening DnD + CLI 仕上げ |
| I-1 | v1.0.0-rc → v1.0.0 | 025 | リリース判定ゲート通過 + 互換性宣言 |

### 依存関係

```
017 ──┬─> 018          (検証で B/C が必要と判明したら)
      └─> 019          (検証結果に基づき docs を確定)
019 ──> 020 ──> 021 ──┬─> 023
                       └─> 024
021, 022, 023, 024 ──> 025 (リリース判定)
```

---

## 過去の Sprint （Phase 1〜11 までの履歴）

| Sprint | RFC | 実装バージョン |
|---|---|---|
| A | 001 + 004 | v0.11.0 |
| B | 002 + 003 | v0.12.0 |
| C | 005 + 統合テスト | v0.12.0 |
| (D 系列以降) | 006〜016 | v0.13.0〜v0.19.0 |
