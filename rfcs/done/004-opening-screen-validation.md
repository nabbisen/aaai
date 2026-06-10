# RFC 004 — Opening Screen Input Validation

**Status.** Implemented (v0.11.0)  
**Tracks.** GUI 初期画面の入力 UX。設計書 p.10 バックログ A「初期画面」  
**Touches.** `crates/aaai-gui/src/views/opening.rs` · `app.rs` (Message, App)

## Summary

Opening 画面の入力フィールドに、Submit 時ではなくフィールド変更時に
インライン検証フィードバックを追加する。パス不在・必須フィールド未入力の場合は
「監査を開始」ボタンを無効化し、何が問題かを明示する。

## 問題

### 1. エラーが Submit 時にしか出ない

現状: `Message::StartAudit` のハンドラ内でパス存在確認を行い、
`self.open_error = Some(...)` に格納する。  
エラーは画面全体のバナーとして表示されるが、どのフィールドに問題があるかが
分かりにくい。

設計書 p.10 A: 「入力不足時の理由表示」

### 2. 「監査を開始」が常に有効

Before / After どちらも空欄のまま「監査を開始」を押せる。
これは「主操作のボタン状態が入力の完了状態を反映する」という設計原則に反する。

### 3. パスの存在確認が遅延フィードバック

存在しないパスを入力しても、「監査を開始」を押すまで分からない。

## 設計

### 4-1. OpeningValidation 構造体

```rust
#[derive(Debug, Clone, Default)]
pub struct OpeningValidation {
    pub before_error: Option<String>,
    pub after_error:  Option<String>,
    // definition_path と ignore_path は任意フィールドなのでエラーなし
}

impl OpeningValidation {
    pub fn can_start(&self) -> bool {
        self.before_error.is_none() && self.after_error.is_none()
    }
}
```

### 4-2. バリデーション実行タイミング

| イベント | アクション |
|---|---|
| `BeforePathChanged(s)` | `validate_opening()` を呼び出し |
| `AfterPathChanged(s)` | `validate_opening()` を呼び出し |
| `StartAudit` | 再度 validate して can_start() をチェック |

```rust
fn validate_opening(&mut self) {
    let mut v = OpeningValidation::default();
    let before = std::path::Path::new(self.before_path.trim());
    let after  = std::path::Path::new(self.after_path.trim());

    if self.before_path.trim().is_empty() {
        v.before_error = Some(t!("opening.before_required").to_string());
    } else if !before.exists() {
        v.before_error = Some(t!("opening.path_not_found").to_string());
    } else if !before.is_dir() {
        v.before_error = Some(t!("opening.not_a_directory").to_string());
    }

    if self.after_path.trim().is_empty() {
        v.after_error = Some(t!("opening.after_required").to_string());
    } else if !after.exists() {
        v.after_error = Some(t!("opening.path_not_found").to_string());
    } else if !after.is_dir() {
        v.after_error = Some(t!("opening.not_a_directory").to_string());
    }

    self.opening_validation = v;
}
```

### 4-3. フィールドレイアウト変更

```
Before folder (必須) *
┌────────────────────────────────────────────────┐
│ /path/to/before                                │  ← 赤アウトライン（エラー時）
└────────────────────────────────────────────────┘
  ✗ Folder not found.                               ← エラーメッセージ（12px 赤）

After folder (必須) *
┌────────────────────────────────────────────────┐
│                                                │  ← グレー（未入力）
└────────────────────────────────────────────────┘
  ✗ This field is required.
```

- `*` = 必須フィールドマーク（赤）
- 有効なパスが入力されたら ✓ マーク（緑）を行末に表示
- 任意フィールド（definition, ignore）にはエラーを出さない

### 4-4. 「監査を開始」ボタンの状態管理

```rust
// opening.rs の view 関数内
let can_start = !app.before_path.trim().is_empty()
    && !app.after_path.trim().is_empty()
    && app.opening_validation.can_start();

let start_btn = button(text(t!("opening.start_button").to_string()))
    .on_press_maybe(if can_start && !app.is_loading {
        Some(Message::StartAudit)
    } else {
        None
    })
    // ボタンスタイルで disabled 状態を視覚化
    .style(if can_start { primary_button_style } else { disabled_button_style });
```

### 4-5. App 構造体の変更

```rust
// App struct に追加
pub opening_validation: OpeningValidation,  // ← NEW

// Default に追加  
opening_validation: OpeningValidation::default(),
```

### 4-6. i18n キー追加

```yaml
opening:
  before_required:  "Before folder is required."
  after_required:   "After folder is required."
  path_not_found:   "Folder not found."
  not_a_directory:  "Path is not a directory."
  start_button:     "Start Audit"   # (既存キーの確認)
```

## データモデル

`OpeningValidation` 構造体を `aaai-gui/src/app.rs` に追加。
`aaai-core` のデータモデル変更なし。

## 代替案

**A. ファイルシステムアクセスをデバウンス（500ms）で遅延**: ネットワークドライブ等で
パス存在チェックがブロックする場合の考慮。初期実装では同期で行い、問題が出れば
`tokio::spawn_blocking` に切り替える。  
**B. ファイルダイアログを提供**: iced では native file picker が利用可能だが、
snora の依存ツリーとの競合確認が必要。別 RFC で検討。

## Open Questions

- ファイル変更監視（notify）と組み合わせて、パスが削除された場合に
  Opening 画面に自動遷移するかどうかは別 RFC 候補。
