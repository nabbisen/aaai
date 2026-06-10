# aaai RFCs

RFC の配置ルールは [done/000-rfc-lifecycle-policy.md](done/000-rfc-lifecycle-policy.md) を参照。

中期計画は [PLAN.md](PLAN.md)（Rev. 4 = Phase 12〜16 / v1.0.0 までの道筋）を参照。

---

## Proposed — 実装待ち

(現在 proposed は空。Phase 12 の RFC 017〜025 はすべて Done に移行済み。)

次の Phase（Phase 13）の新規 RFC はここに追加していく。

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

> **注意**: RFC 007〜016 は「コード実装は完了したが視覚検証は未通過」の状態。
> RFC 017 のハーネスでエビデンスを取得し、判明した差分は個別 fix RFC を別途切る。

> **v0.20.0 の特徴**: Phase 12 で 9 件の RFC が実装され、19 件の pre-existing バグも整理された。
> Phase 13 以降は別途 ROADMAP.md を参照。

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
