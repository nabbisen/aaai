# aaai RFCs

RFC の配置ルールは [done/000-rfc-lifecycle-policy.md](done/000-rfc-lifecycle-policy.md) を参照。

---

## Proposed — 実装待ち

| ID | タイトル | 優先度 | 概要 |
|---|---|---|---|
| [007](proposed/007-toolbar-navigation.md) | Toolbar & Navigation | v1.0 blocker | 「開く」追加・ツールバー再構成 |
| [008](proposed/008-bottom-action-bar.md) | Bottom Action Bar | v1.0 blocker | 承認ボタンをボトムバーに固定・未解決件数 |
| [009](done/009-reason-textarea.md) | Reason Field Textarea | v1.0 blocker → **v0.15.0** | 単行→複数行テキストエリア |
| [010](done/010-diff-view-legend.md) | Diff View Legend | v1.0 blocker → **v0.15.0** | 差分ハイライト凡例 |
| [011](proposed/011-diff-view-tabs.md) | Diff View Tabs | 重要改善 | 左右差分 \| 統合 \| 変更のみ タブ |
| [012](proposed/012-linematch-rule-display.md) | LineMatch Rule Color Blocks | 重要改善 | ルールを色付きコードブロックで表示 |
| [013](proposed/013-file-tree-icon-unification.md) | File Tree Icon Unification | 重要改善 | 行頭=statusアイコン・右端=diff-type記号 |

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

---

## Archive — 取り下げ・置換済み

_なし_

---

## 中期計画との対応

詳細は [PLAN.md](PLAN.md) を参照。

| Sprint | RFC | 予想バージョン |
|---|---|---|
| A | 001 + 004 | v0.11.0 |
| B | 002 + 003 | v0.12.0 |
| C | 005 + 統合テスト | v1.0.0-rc |
