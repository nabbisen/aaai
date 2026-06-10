# GUI UI/UX Production-Ready Plan

## 背景

`aaai-gui` は機能的には v0.10.x で一通り揃ったが、設計書
（aaai_uiux_design.pdf）が定義する「説明可能な差分監査支援ツール」としての
UX 品質には到達していない。  
本計画は production ready の水準を定義し、そこへ至るための RFC 群を
定める中期計画である。

## 到達目標

> 同一入力に対して CLI と GUI の状態判定が一致し、利用者が YAML を直接書かず
> に「理由付き承認 → 保存 → 再監査 → レポート出力」まで完了できる。
>
> — aaai_uiux_design.pdf, p.10 完了条件

加えて：
- 状態を色だけで表さない（ABDD: Accessible by Default Design）
- すべての主操作がキーボードで完結できる
- CI/CD 用の CLIと GUI が同じ状態語彙を使う

## 現状と設計書のギャップ

| 領域 | 現状 | 設計書の要求 | 乖離度 |
|---|---|---|---|
| CLI 出力順序 | Result が末尾 | **結論を先に出す** | ★★☆ |
| CLI ステータス表示 | 色付きテキストのみ | 文字＋記号を併用 | ★★☆ |
| インスペクター検証 | strategy.validate() のみ | リアルタイム・フィールド別エラー表示 | ★★★ |
| 承認アクション | Approve + 別途 Save | **承認して保存** を主操作に固定 | ★★☆ |
| ファイルツリー状態 | 色バッジ（diff アイコン） | 色＋アイコン＋テキスト（ABDD） | ★★★ |
| Opening 入力検証 | Submit 時のみエラー | インライン・リアルタイム検証 | ★★☆ |
| キーボードナビ | Ctrl+S/R/Z・↑↓ のみ | Tab 遷移・Enter 承認・/ 検索 | ★★★ |

★★★ = v1.0 ブロッカー / ★★☆ = 重要改善 / ★☆☆ = 軽微

## RFC 一覧

| ID | タイトル | 難度 | v1.0 必須 |
|---|---|---|---|
| RFC 001 | CLI Output UX Consistency | 低 | ✅ |
| RFC 002 | Inspector Validation & Primary Action | 高 | ✅ |
| RFC 003 | ABDD Status Display | 中 | ✅ |
| RFC 004 | Opening Screen Input Validation | 低中 | ✅ |
| RFC 005 | Keyboard Navigation & Focus | 中高 | ✅ |

## 優先順位と依存関係

```
RFC 001 (CLI)           ─────────────────────────► v1.0
RFC 002 (Inspector)     ──────────────────────────► v1.0
  └─ RFC 003 (Status)   ─────────────────────────► v1.0
RFC 004 (Opening)       ────────────────────────► v1.0
RFC 005 (Keyboard)      ────────────────────────► v1.0
```

依存関係なし。並列実装可。ただし RFC 002 が最大工数のため先行着手を推奨。

## バージョン計画

| マイルストーン | 対象 RFC | 予想バージョン |
|---|---|---|
| Sprint A | RFC 001 + RFC 004 | v0.11.0 |
| Sprint B | RFC 002 + RFC 003 | v0.12.0 |
| Sprint C | RFC 005 + 統合テスト | v1.0.0-rc |
| Release | UI/UX テスト通過後 | v1.0.0 |
