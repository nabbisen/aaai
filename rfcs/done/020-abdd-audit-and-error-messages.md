# RFC 020 — ABDD Accessibility Audit & Action-oriented Errors

**Status.** Implemented (v0.20.0)
**Priority.** v1.0 blocker
**Tracks.** 設計書 p.8 (Accessible by Default Design チェック観点) / p.4 CLI 「迷わないヘルプ」
**Touches.** `crates/aaai-gui/src/views/*.rs` (エラー文言) · `crates/aaai-core/src/audit/*.rs` (エラー型) · `crates/aaai-cli/src/cmd/*.rs` (CLI エラー出力) · `docs/src/gui.md` (a11y 章追加) · `docs/src/testing.md` (a11y 検証)

---

## 1. 要件定義

### 1.1 目的

設計書 p.8 が定める ABDD (Accessible by Default Design) チェック観点を、
**項目ごとに合否を判定して、合否表として残せる状態にする**。同時に設計書 p.8 末尾の
「エラー文は利用者の行動につながる文にする」要件を満たすため、現存する一語型・状態語型
エラー文を全数書き換える。

### 1.2 解決すべき問題

| 問題 | 設計書原則 |
|---|---|
| 状態の色依存 (色なしでも判別できるか未確認) | p.8 視認性 / p.8 末尾 |
| Tab / Shift+Tab 順序が画面の読解順と一致しているか未確認 | p.8 キーボード |
| 主操作と破壊的操作が近接していないか未確認 | p.8 キーボード |
| `failed` / `invalid` / `error` といった一語型エラー文がコード中に残存 | p.8 エラー文 / p.2 状態は常に見える |
| エラー文に「何が」「どこで」「次にどう直すか」が含まれない例の残存可能性 | p.8 エラー文 |
| 理由欄の入力例が「専門用語なしで理解できる」かが未検証 | p.8 非専門者への配慮 |

### 1.3 機能要件

| ID | 要件 | 必須/任意 |
|---|---|---|
| FR-1 | 設計書 p.8 のチェック観点 (6 項目) を機械的に列挙したチェックシートを作成する | 必須 |
| FR-2 | 各観点に対し「合格 / 不合格 / 例外 (記録)」を記入する | 必須 |
| FR-3 | コードに残存する一語型エラー文を 0 件にする | 必須 |
| FR-4 | 残存エラー文は「何が・どこで・次にどうする」の 3 要素を含む書式に統一 | 必須 |
| FR-5 | 状態 (OK / Pending / Failed / Error / Ignored) はテキスト＋記号で表現する (色のみは不可) — 既に RFC 003 で実装。a11y 監査ではこれを再点検する | 必須 |
| FR-6 | フォーカスリングが常に見えることを確認する (iced 0.14 のデフォルト挙動を許容) | 必須 |
| FR-7 | 主操作 (承認して保存) と破壊的操作 (ルール削除等) が誤クリックされない距離・色で配置されていること | 必須 |

### 1.4 非機能要件

| ID | 要件 |
|---|---|
| NFR-1 | チェックシートはレビュー時に一覧確認できる |
| NFR-2 | 書き換え後のエラー文は en / ja 両言語で同等の情報を含む |
| NFR-3 | 仕様変更によって既存テストが壊れないこと（壊れた場合はテストも更新） |

---

## 2. 外部設計（基本設計）

### 2.1 ABDD チェックシートの形式

`docs/src/abdd-audit.md` (新規):

```markdown
# ABDD Audit Sheet — vX.Y.Z

設計書 p.8 のチェックリストを項目別に判定する。

## 1. Tab / Shift+Tab の順序

| 画面 | 期待 | 実観測 | 判定 |
|---|---|---|---|
| Opening | Before card → After card → optional → start button | (実観測) | ✅/❌/例外 |
| Main toolbar | open → save → run → report | (実観測) | ✅/❌/例外 |
| Inspector | reason → ticket → approved_by → expires → strategy → ... | (実観測) | ✅/❌/例外 |

## 2. 色なしでの判別可能性

(以下同型)

## 3. 主操作と破壊的操作の距離

## 4. 未保存状態・監査失敗・入力エラーの常時可視性

## 5. 理由欄入力例の専門用語

## 6. クリック領域 (≥44px)
```

### 2.2 エラー文の新書式

```
✅ 良い例:
  "Before フォルダ /home/user/foo が見つかりません。
   パスを変更するか、フォルダを作成してください。"

❌ 悪い例:
  "Path not found."
  "failed"
  "error"
```

### 2.3 i18n キーの命名規則

エラー文 i18n キー命名規則:

```
error.<context>.<short_id>:
  message: <主要メッセージ>
  hint: <ヒント（次の操作）>
```

例:
```yaml
error.opening.before_not_found:
  message: "Before folder not found: {path}"
  hint: "Pick a folder that exists, or create the folder first."
```

view 側で `t!("error.opening.before_not_found.message", path: ...)` + `hint` の 2 行表示にする。

---

## 3. 内部設計（詳細設計）

### 3.1 エラー文書き換え対象の特定

以下の grep で対象を洗い出す:

```bash
rg -n "\"(failed|invalid|error|not found|unknown)\"" \
   crates/aaai-gui/src crates/aaai-cli/src crates/aaai-core/src/audit
```

その結果を `docs/abdd-error-message-audit-checklist.md` (一時資料) として保存し、
1 件ずつ判定 (書き換え必要 / そのままで良い / 機械向け文字列 (JSON キー等) で対象外)。

### 3.2 一語型を残してよい例外

| 例外 | 理由 |
|---|---|
| JSON 出力の `"status": "failed"` | 機械処理用。文字列定数として安定が必要 |
| 終了コード `1=FAILED` 定義 | 仕様 |
| ログ出力の `level=error` | 機械処理用 |
| `AuditStatus::Failed` の Display impl | 集約表示用 (CLI/GUI で再ラベリングされる) |

人間向け表示 (CLI の Zone 4 hint、GUI のメッセージ、エラーバナーなど) のみ書き換える。

### 3.3 GUI 内エラー文の表示パターン

```rust
// 不合格な書き方
text(t!("error.failed").to_string())

// 合格な書き方 (Phase 13)
column![
    text(format!("⚠ {}", t!("error.opening.before_not_found.message", path: &p)))
        .size(13),
    text(t!("error.opening.before_not_found.hint").to_string())
        .size(11)
        .color(theme::HINT_COLOR),
]
```

### 3.4 a11y チェック対応の追加項目

iced 0.14 はネイティブのスクリーンリーダー対応 (NVDA / VoiceOver) を持たないため、
これは v1.0.0 で「制限事項」として明示し、`docs/src/abdd-audit.md` の項目「5 スクリーンリーダー」は
*Not Supported (iced 0.14 limitation)* として記録する。iced の将来バージョンで導入された
時点で再評価とする。

### 3.5 docs/src/testing.md への追加

`## 10. ABDD verification (manual)` を新章として追加:

```markdown
| # | Step | Expected |
|---|---|---|
| 10-1 | Run with display calibrated to monochrome / greyscale mode | Status icons (✓⚠✗!—) remain distinguishable |
| 10-2 | Tab from Opening's Before-card | Focus moves to After-card, then optional, then Start |
| 10-3 | Trigger any error (invalid path) | Message contains "what / where / how to fix" |
```

---

## 4. プログラム設計

### 4.1 実装手順

| Step | 作業 | 検証 |
|---|---|---|
| 1 | 一語型エラー文の grep → 一覧化 | 件数記録 |
| 2 | 各 1 件に対し書き換え or 例外判定 | 結果記録 |
| 3 | 新書式のエラー文 i18n キーを `locales/{en,ja}.yaml` に追加 | YAML 構文 OK |
| 4 | GUI のエラー表示コードを `message + hint` 2 行に変更 | 視覚確認 |
| 5 | CLI のエラー出力にも同じ 2 行書式を適用 | 視覚確認 |
| 6 | `docs/src/abdd-audit.md` を実施・記入 (両言語) | レビュー |
| 7 | `docs/src/testing.md` に ABDD 章追加 | レビュー |
| 8 | `scripts/check-i18n-keys.sh` 実行 | exit 0 |

### 4.2 影響範囲

- `crates/aaai-gui/src/views/opening.rs` (validation error rendering)
- `crates/aaai-gui/src/views/inspector.rs` (validation error rendering)
- `crates/aaai-gui/src/app.rs` (toast messages)
- `crates/aaai-cli/src/cmd/audit.rs` (Zone 4 hint refinement)
- `crates/aaai-cli/src/cmd/check.rs` (validation error format)
- `crates/aaai-cli/src/cmd/lint.rs` (lint warning format)
- `crates/aaai-core/src/audit/result.rs` (`detail` フィールドの埋め込み形式)

### 4.3 リスク

| リスク | 対策 |
|---|---|
| エラー文書き換えで CLI integration test が壊れる | テスト側の文字列マッチを構造化された field match に変更 |
| 翻訳 (ja) で hint が冗長すぎて UI に収まらない | 13〜15 文字以内のサマリ + tooltip パターンで対応 (将来案) |
| 過剰な hint が「色だけに依存しない」原則を読みづらくする | hint は薄い色 (`#666`) で従属表示 |

---

## 5. 完了条件

- [ ] `docs/src/abdd-audit.md` が全 6 項目について判定済み (合格 / 不合格 / 例外)
- [ ] 一語型エラー文が人間向け文脈で 0 件 (機械向け除く)
- [ ] エラー文は「何が・どこで・次にどうする」が含まれる
- [ ] `docs/src/testing.md` に ABDD 章 (`## 10. ABDD verification`) が追加されている
- [ ] `scripts/check-i18n-keys.sh` が新規 error.* キーを含めて exit 0
- [ ] RFC 017 の Visual Verification セクションで該当画面が「ABDD 監査済」と裏付けられる

## 6. 依存

- **RFC 017** (verification harness): 監査結果の判定基準として
- **RFC 019** (docs refresh): docs/src/gui.md の章構成と整合させる必要

## 7. v1.0.0 制限事項として明示する範囲

スクリーンリーダー (NVDA / JAWS / VoiceOver / Orca) との完全な相互運用は、
iced 0.14 のサポート範囲外。v1.0.0 では「キーボード操作は完結する」「色依存しない」
までを ABDD 達成範囲として宣言し、スクリーンリーダー支援は iced 将来バージョンの
動向を見て v1.1 以降で再検討する。
