# RFC 009 — Reason Field Multi-line Textarea

**Status.** Implemented (v0.15.0)  
**Priority.** v1.0 blocker  
**Tracks.** 設計書 p.5 インスペクター / p.7「Reason は必須。入力例を出し、何を書けばよいか迷わせない」  
**Touches.** `aaai-gui/src/views/inspector.rs`

## Summary

現在の理由フィールドは単行の `text_input` であり、長い理由を書きにくい。  
設計書は複数行テキストエリアを示しており、実際の監査作業では  
「〇〇のため △△ を変更。チケット番号: INF-42 にて承認済み。」といった  
複数文の記述が必要になる。

## Current vs Design

| 設計書 | 現実装 | 差分 |
|---|---|---|
| 複数行テキストエリア（大きめ） | 単行 `text_input` | 長い理由が書けない |
| 入力例の提示 | `placeholder` は存在 | placeholder の内容を充実させる |

## Design

### iced の multi-line text_editor

iced 0.14 には `widget::text_editor` が存在し複数行入力を実現できる。  
`text_editor::Content` を `InspectorState` に保持する。

### データモデル変更

```rust
// InspectorState に追加
pub reason_content: iced::widget::text_editor::Content,
```

`reason: String` は内部的には `reason_content.text()` から取得する。  
（既存の `reason: String` は Approval 時のみ使用するため、sync を明示する。）

### ビュー変更

```rust
use iced::widget::text_editor;

let reason_editor = text_editor(&ins.reason_content)
    .on_action(Message::ReasonAction)
    .placeholder(t!("inspector.reason_placeholder"))
    .height(Length::Fixed(72.0))   // 約4行分
    .padding(Padding::from([8.0, 10.0]));
```

高さは 72px（約 4 行）に固定。将来的にはリサイズ可能にする。

### 新 Message

```rust
ReasonAction(iced::widget::text_editor::Action),
```

### validate_inspector との接続

```rust
// ReasonAction ハンドラ内
Message::ReasonAction(action) => {
    self.inspector.reason_content.perform(action);
    self.inspector.reason = self.inspector.reason_content.text()
        .trim_end_matches('\n').to_string();
    self.validate_inspector();
}
```

### SelectEntry でのリセット

エントリ選択時に `reason_content` を entry の reason で初期化：

```rust
self.inspector.reason_content =
    iced::widget::text_editor::Content::with_text(&entry.reason);
```

### i18n: placeholder の充実

```yaml
inspector:
  reason_placeholder: |
    例: ポート番号を 80 → 8080 に変更。本番環境への適用に伴う変更。
    チケット: INF-42 にて承認済み。
```

## Open Questions

- `text_editor` と `text_input` の theme/style の統一（枠線・背景色）。
- 将来的なリサイズ対応は別 RFC で検討。
