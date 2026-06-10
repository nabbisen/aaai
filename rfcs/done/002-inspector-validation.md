# RFC 002 — Inspector Validation & Primary Action

**Status.** Implemented (v0.12.0)  
**Tracks.** GUI インスペクターの UX。設計書 p.5, 7「インスペクター設計」  
**Touches.** `crates/aaai-gui/src/views/inspector.rs` · `app.rs` (InspectorState, Message) · `aaai-core/src/config/definition.rs`

## Summary

インスペクターパネルに戦略別のリアルタイムバリデーション（Regex コンパイルエラー・
Checksum 形式不正・LineMatch 空ルールなど）を追加し、エラー時は承認ボタンを
無効化する。あわせて「承認して保存」を主操作ボタンに変更し、
「承認 → 保存」の 2 ステップを 1 アクションに統合する。

## 問題

### 1. 戦略別の入力エラーが表示されない

現状: `strategy.validate()` の結果が `validation_error: Option<String>` に
格納されるが、エラーがあるフィールドが視覚的にハイライトされない。  
Regex パターンが不正でも、エラーテキストがページ下部に小さく表示されるだけで
どのフィールドが問題か分からない。

設計書 p.7: 「リアルタイム検証 — Regex コンパイル、空行、重複ルール、
Checksum 形式などを保存前に表示する」

### 2. 承認と保存が 2 ステップ

現状: `ApproveEntry` (in-memory 更新) → `SaveDefinition` (disk 書き込み) が別操作。  
設計書 p.5: 「下部アクションは『承認して保存』を主操作に固定。補助操作は分離する」  
承認後の保存忘れによる変更消失リスクがある。

### 3. ボタンの無効化条件が不完全

現状: `reason.is_empty()` のみでボタン無効化。戦略エラーがあっても
reason が入力されていれば承認できてしまう。

## 設計

### 2-1. InspectorValidation — 戦略別バリデーション構造体

```rust
/// Per-field validation result for the inspector.
/// Computed on every input change; drives visual state and approve-gate.
#[derive(Debug, Clone, Default)]
pub struct InspectorValidation {
    /// reason フィールドのエラー
    pub reason_error: Option<String>,
    /// strategy フィールドのエラー（フィールド名 → エラーメッセージ）
    pub strategy_errors: Vec<FieldError>,
    /// expires_at フィールドのエラー
    pub expires_at_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FieldError {
    /// エラーがあるフィールドの識別子（"pattern", "rule[0].line", "expected_sha256"）
    pub field: String,
    pub message: String,
}

impl InspectorValidation {
    /// 承認可能かどうか
    pub fn can_approve(&self) -> bool {
        self.reason_error.is_none()
            && self.strategy_errors.is_empty()
            && self.expires_at_error.is_none()
    }
}
```

### 2-2. 戦略別バリデーション定義

| 戦略 | チェック項目 | エラー条件 |
|---|---|---|
| None | なし | — |
| Checksum | `expected_sha256` | 64 文字・16 進数以外 |
| LineMatch | `rules` | ルール数 = 0 / 各ルールの `line` が空 |
| Regex | `pattern` | `regex::Regex::new()` エラー |
| Exact | `expected_content` | 空文字列 |

Reason: `trim().is_empty()` → エラー  
ExpiresAt: 非空かつ `NaiveDate::parse_from_str` 失敗 → エラー

### 2-3. 視覚的フィードバック

```
┌─ Regex pattern ────────────────────────────────┐
│  ^version = "\d+\.\d+\.                        │  ← 未完成パターン
└────────────────────────────────────────────────┘
  ✗ Invalid regex: unclosed group at position 18    ← エラー行（赤・12px）
```

- エラー状態フィールド: 赤いアウトライン（border color → `ERROR_COLOR`）  
- エラーメッセージ: フィールド直下に 12px 赤テキスト  
- 承認ボタン: `can_approve() == false` → `on_press_maybe(None)` + 薄いスタイル  
- 理由フィールドの「必須」表示: ラベル横に `*` を赤で付与

### 2-4. 「承認して保存」主操作ボタン

| 変更前 | 変更後 |
|---|---|
| `ApproveEntry` (in-memory) + `Ctrl+S` (save) | `ApproveAndSave` (in-memory + disk atomic) |
| 「承認して適用」 | **「承認して保存」** |

`ApproveEntry` は **廃止せず残す**（バッチ承認・undo スタックで引き続き使用）。  
GUI の主ボタンのみ `ApproveAndSave` に変更。

```rust
// 新メッセージ
ApproveAndSave,   // ApproveEntry + SaveDefinition のアトミック版

// app.rs での処理
Message::ApproveAndSave => {
    // 1. ApproveEntry と同じ処理
    // 2. SaveDefinition と同じ処理
    // 3. Toast: "Saved."
}
```

`Ctrl+S` は引き続き `SaveDefinition` としてグローバルショートカットに残す
（未保存の一括変更を保存する用途）。

### 2-5. ボタンレイアウト変更

```
[ 承認して保存 ]          ← 主操作（幅広・プライマリカラー）
  ← バリデーションエラーがある場合は薄くなり押せない
```

旧「承認して適用」ボタンは削除。  
バッチ承認はツールバーの「Batch Approve」ボタンから引き続き使用可能。

## データモデル詳細設計

### InspectorState 変更

```rust
pub struct InspectorState {
    pub reason:           String,
    pub ticket:           String,
    pub approved_by:      String,
    pub expires_at_str:   String,
    pub note:             String,
    pub strategy:         AuditStrategy,
    // 追加フィールド
    pub validation:       InspectorValidation,  // ← NEW (旧 validation_error: Option<String> を置換)
}
```

旧 `validation_error: Option<String>` を `validation: InspectorValidation` に
昇格させる（フィールドレベルのエラーが必要なため）。

### validate_inspector() 実装変更

```rust
fn validate_inspector(&mut self) {
    let ins = &self.inspector;
    let mut v = InspectorValidation::default();

    // Reason
    if ins.reason.trim().is_empty() {
        v.reason_error = Some(t!("validation.reason_required").to_string());
    }

    // ExpiresAt
    if !ins.expires_at_str.trim().is_empty() {
        if chrono::NaiveDate::parse_from_str(&ins.expires_at_str, "%Y-%m-%d").is_err() {
            v.expires_at_error = Some(t!("validation.date_format").to_string());
        }
    }

    // Strategy
    match &ins.strategy {
        AuditStrategy::Checksum { expected_sha256 } => {
            let s = expected_sha256.trim();
            if s.len() != 64 || !s.chars().all(|c| c.is_ascii_hexdigit()) {
                v.strategy_errors.push(FieldError {
                    field: "expected_sha256".into(),
                    message: t!("validation.checksum_format").to_string(),
                });
            }
        }
        AuditStrategy::LineMatch { rules } => {
            if rules.is_empty() {
                v.strategy_errors.push(FieldError {
                    field: "rules".into(),
                    message: t!("validation.linematch_empty").to_string(),
                });
            }
            for (i, rule) in rules.iter().enumerate() {
                if rule.line.trim().is_empty() {
                    v.strategy_errors.push(FieldError {
                        field: format!("rule[{}].line", i),
                        message: t!("validation.rule_line_empty").to_string(),
                    });
                }
            }
        }
        AuditStrategy::Regex { pattern, .. } => {
            if let Err(e) = regex::Regex::new(pattern) {
                v.strategy_errors.push(FieldError {
                    field: "pattern".into(),
                    message: format!("{}", e),
                });
            }
        }
        AuditStrategy::Exact { expected_content } => {
            if expected_content.trim().is_empty() {
                v.strategy_errors.push(FieldError {
                    field: "expected_content".into(),
                    message: t!("validation.exact_empty").to_string(),
                });
            }
        }
        AuditStrategy::None => {}
    }

    self.inspector.validation = v;
}
```

### i18n キー追加

```yaml
# en.yaml / ja.yaml
validation:
  reason_required:    "Reason is required before approval."
  date_format:        "Use YYYY-MM-DD format."
  checksum_format:    "Must be exactly 64 hex characters."
  linematch_empty:    "At least one rule is required."
  rule_line_empty:    "Rule line cannot be empty."
  exact_empty:        "Expected content cannot be empty."
```

## 代替案

**A. エラーをダイアログで表示**: モーダルは UX フローを断ち切る。インライン表示が優先。却下。  
**B. 承認と保存を引き続き分離**: 設計書の明示的要求に反する。却下。  
**C. Checksum のみブラウザで計算**: aaai-gui は iced でありファイル選択ダイアログが必要。別 RFC 候補として保留。

## Open Questions

- `ApproveEntry` message を Deprecated にして将来削除するか、維持するかは別 RFC で判断。
- バッチ承認は `ApproveAndSave` か旧 `ApproveEntry + SaveDefinition` どちらを使うか検討必要。
