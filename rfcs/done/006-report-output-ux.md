# RFC 006 — Report Output UX

**Status.** Implemented (v0.14.0)  
**Tracks.** レポート出力の UX。設計書 p.10 バックログ E「レポート」  
**Touches.** `crates/aaai-core/src/report/generator.rs` · `html.rs` · `crates/aaai-cli/src/cmd/report.rs`

## Summary

Markdown / HTML レポートの構造を「結論・要確認差分を冒頭に」原則に従い再構成する。
Failed / Pending / Error エントリを最上部に配置し、
理由と内容監査結果を必須フィールドとして常に含める。

## Problem

### 1. Failed / Pending が末尾にある

現状の Markdown レポート順序：
`Result` → `Summary` → `Execution Details` → **OK Entries** → Failed Entries → Pending Entries

設計書 p.10: 「結論・サマリー・要確認差分を冒頭に配置」

### 2. Attention section がない

要確認（Failed + Pending + Error）をひとまとめにした
「Action Required」セクションが存在しない。

### 3. reason が常に表示されていない

現状: reason フィールドは常に表示。ただし `(no reason)` の場合の強調がない。

## Design

### Section 順序の変更

```
# aaai Audit Report

Result: ✗ FAILED   ← 記号付き verdict
Summary cards
Execution Details

## ⚠ Action Required (Failed: N, Pending: N, Error: N)   ← 新: 要確認を先頭に
  各エントリ（理由・チケット・戦略 必ず記載）

## ✓ Passed Entries (OK: N)   ← 後ろに移動
  各エントリ

## — Ignored Entries (N)      ← 最後
```

`is_passing()` が true の場合は「Action Required」セクションをスキップ。

### Markdown エントリの改善

```markdown
### `config/server.toml` — ✗ Failed

| Field | Value |
|---|---|
| Diff type | Modified |
| **Reason** | *(no reason provided)* |
| Strategy | LineMatch |
| Ticket | INF-42 |
| Approved by | alice |

> ✗ Rule check failed: expected line "port = 8080" not found in after-file
```

- `**Reason**` を太字で強調
- reason が空の場合: `*(no reason provided)*` でイタリック表示
- strategy の判定結果（`detail` フィールド）を blockquote で記載

### HTML レポートの改善

- `⚠ Action Required` カードを赤/黄のバナーで冒頭に配置
- OK エントリは折りたたみ（`<details>` タグ）でデフォルト非表示
- reason 未記入は赤字で表示

## Implementation notes

`ReportSection` 列挙を使い、セクションの順序を定数で管理する。
`--no-attention-section` フラグで旧順序に戻せるようにする（後方互換）。
