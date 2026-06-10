# RFC 012 — LineMatch Rule Color Blocks

**Status.** Proposed  
**Priority.** 重要改善（v1.0 推奨）  
**Tracks.** 設計書 p.5 インスペクター③ / p.7 監査戦略ごとの編集フォーム  
**Touches.** `aaai-gui/src/views/inspector.rs`

## Summary

設計書のインスペクターは LineMatch ルールを色付きコードブロックで表示している：  
赤背景に `- action: Removed / line: "port = 80"` 、  
緑背景に `- action: Added / line: "port = 8080"` 。  
現在の実装は選択ドロップダウン + テキスト入力のプレーンフォームであり、  
視覚的にルールの意味が伝わらない。

## Current vs Design

| 設計書 | 現実装 | 差分 |
|---|---|---|
| 色付きコードブロック（赤=Removed, 緑=Added） | プレーンテキスト入力フィールド | 視覚的表現なし |
| ルール一覧がコードとして読める | フォームとして入力するのみ | 承認前の視認性が低い |

## Design

### ルール表示コンポーネント

各 LineRule を「編集モード」と「表示モード」に切り替えて表示する：

**表示モード（デフォルト）:**

```
┌─────────────────────────────────────────┐
│ - action: Removed                        │  ← 薄い赤背景
│   line: "port = 80"                      │
└─────────────────────────────────────────┘
┌─────────────────────────────────────────┐
│ - action: Added                          │  ← 薄い緑背景
│   line: "port = 8080"                    │
└─────────────────────────────────────────┘
[ + ルールを追加 ]
```

**編集モード（行クリックで展開）:**

```
┌ action: [Removed ▼]  [ × 削除 ] ───────┐
│ line: [port = 80                       ] │
└─────────────────────────────────────────┘
```

### 実装

```rust
fn rule_block<'a>(
    rule: &'a LineRule,
    idx: usize,
    editing: bool,
) -> Element<'a, Message> {
    let (bg_color, label) = match rule.action {
        LineAction::Removed =>
            (Color::from_rgba(0.85, 0.20, 0.20, 0.12), "Removed"),
        LineAction::Added =>
            (Color::from_rgba(0.10, 0.65, 0.30, 0.12), "Added"),
    };

    if editing {
        // 編集フォーム
        column![
            row![
                pick_list(/* Removed/Added */),
                button("×").on_press(Message::RemoveRule(idx)),
            ],
            text_input("line content", &rule.line)
                .on_input(move |s| Message::RuleLineChanged(idx, s)),
        ]
        // ...
    } else {
        // 表示ブロック
        button(
            container(
                column![
                    text(format!("- action: {label}")).size(11),
                    text(format!("  line: {:?}", rule.line)).size(11),
                ]
                .spacing(1)
            )
            .padding(Padding::from([6.0, 10.0]))
            .style(move |_| ContainerStyle {
                background: Some(Background::Color(bg_color)),
                border: Border { radius: 4.0.into(), ..Default::default() },
                ..Default::default()
            })
        )
        .on_press(Message::EditRule(idx))
        .style(button::text)
        .into()
    }
}
```

### 新 Message

```rust
EditRule(usize),       // ルールを編集モードに切り替え
```

`InspectorState` に `editing_rule: Option<usize>` を追加。

### i18n

```yaml
inspector:
  add_rule:  "+ ルールを追加"  # / "+ Add rule"
```

## Open Questions

- 編集モードへの切り替えをクリックとするか、常に編集可能とするか。
  設計書の見た目は常時「表示ブロック」に見えるが、編集性を確保するためクリック切り替えが合理的。
- 新規ルール追加時は即座に編集モードで展開する。
