# RFC 001 — CLI Output UX Consistency

**Status.** Implemented (v0.11.0)  
**Tracks.** CLI ユーザー体験。設計書 p.4「コンソール画面: CI/CD と人間確認の両立」  
**Touches.** `crates/aaai-cli/src/cmd/audit.rs` · `cmd/snap.rs` · `cmd/report.rs`

## Summary

`aaai audit` の出力を「結論を先に出す」原則に従い再構成する。
現状は `Result: FAILED` が末尾にあり、大量の差分がある場合に利用者がスクロールを
必要とする。加えて、状態表示が色に依存しており CI ログや色覚多様性への配慮が不十分。
本 RFC はヘッダー・サマリー・ステータス記号・フッターの 4 区画レイアウトを定義する。

## 問題

### 1. 結論が末尾にある

```
Before : ./before
After  : ./after

PENDING  f.txt  (Modified)
         Entry exists but has no reason ...

Result: FAILED          ← スクロールしないと見えない
  Total: 1  OK: 0  Pending: 1 ...
```

設計書 p.4: 「1. 結論を先に出す — Result: OK / FAILED を上部に表示」

### 2. ステータスが色のみ

現状: `PENDING` テキストに yellow 色付け。  
設計書 p.4: 「2. 色だけに依存しない — Status 文字列と記号を併用」  
モノクロ端末・CI ログで判別不能。

### 3. snap の出力に結果概要がない

`aaai snap` はファイル生成メッセージのみで、生成エントリ数・Pending 件数の
サマリーがない。

## 設計

### 出力レイアウト（4 区画）

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  aaai audit                     Result: FAILED   ← ヘッダー（区画 1）
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Total: 5  ✓ OK: 3  ⚠ Pending: 1  ✗ Failed: 1  ← サマリー（区画 2）

  ✓  config/app.toml         Modified
  ✗  config/server.toml      Modified   ポート変更の理由が不明
  ⚠  docs/README.md          Modified   理由が未登録
  ✗  scripts/deploy.sh       Modified   許容されていない変更
  ✓  data/schema.sql         Unchanged

  ... （他 N 件 — --verbose で全件表示）                ← エントリ（区画 3）

  Next: fill in 'reason' for ⚠ entries,             ← ネクストアクション（区画 4）
        then re-run `aaai audit`.
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### ステータス記号定義

| AuditStatus | 記号 | テキスト | 意味 |
|---|---|---|---|
| Ok | `✓` | OK | 監査合格 |
| Pending | `⚠` | Pending | reason 未入力 |
| Failed | `✗` | Failed | ルール不一致 |
| Error | `!` | Error | 監査不能 |
| Ignored | `—` | Ignored | 対象外 |

記号は Unicode だが、`--no-unicode` フラグで ASCII 代替（`+`/`?`/`x`/`!`/`-`）に切替可能。

### デフォルト表示件数

- **デフォルト**: Failed + Pending のみ最大 20 件  
- `--verbose`: 全エントリ（Ignored 含む）  
- `--quiet`: サマリー行のみ

### snap の出力改善

```
✓  Snapshot generated: audit.yaml
   14 entries added  (14 Pending — fill in 'reason' before auditing)
   0 entries skipped (already have a reason)

Next: open audit.yaml, fill in 'reason' for each entry,
      then run `aaai audit`.
```

## データモデル

本 RFC はデータモデルの変更を伴わない。出力フォーマット変更のみ。

## 実装方針

1. `audit.rs` の `run()` 冒頭でヘッダーを出力（パス情報を含む）
2. 差分ループの前に `Result:` を表示（`AuditEngine::evaluate` の結果を使う）
3. 既存の `print_entry()` ヘルパーを `format_entry()` に変更しテスト可能にする
4. `--no-unicode` フラグを `AuditArgs` に追加
5. 区切り線は 44 文字固定（端末幅に依存しない）

## 代替案

**A. 色だけ変える（現状維持）**: アクセシビリティ要件を満たさない。却下。  
**B. JSON のみに統一**: CI 用途には良いが人間可読性が低下。却下。

## Open Questions

- `--no-unicode` のデフォルト検出（`NO_COLOR` 環境変数との連携）を検討する。
- 区切り線の文字（`━` vs `-`）はターミナルの対応状況による。 `--no-unicode` で `-` に fallback。
