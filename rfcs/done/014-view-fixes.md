# RFC 014 — View Layer Fixes (RFC 007 / 009 re-apply + ABDD tap areas)

**Status.** Implemented (v0.18.0)  
**Priority.** v1.0 blocker  
**Tracks.** 設計書 p.5 ツールバー・インスペクター / p.8 ABDD タップ領域  
**Touches.** `aaai-gui/src/views/main_view.rs` · `views/inspector.rs`

## Summary

RFC 007（ツールバー再構成）と RFC 009（理由フィールド textarea）の
データモデル側（app.rs）は正しく適用されたが、**view 側の置換が失敗した**。
加えて ABDD 要件のボタンタップ領域（≥ 44px）が一部未達。
本 RFC はこれら 3 点を修正する。

## Gaps

### 1. Toolbar (RFC 007 view side)

```
設計書:  [ □ 開く ] [ □ 保存 ] [ ▶ 監査実行 ] [ ↑ レポート出力 ]   監査ステータス: FAILED
現実装:  [ 保存 ] [ 再実行 ] [ Batch Approve ]  (verdict badge)  [ Export MD ] [ Export JSON ]
```

Message::BackToOpening は app.rs に存在するが、
ツールバーの build_toolbar() がまだ旧コードのため「開く」ボタンが画面にない。

### 2. Reason Field (RFC 009 view side)

reason_content (text_editor::Content) は InspectorState に追加済みだが、
inspector.rs がまだ `text_input` を使っている。

### 3. ABDD Tap Areas

設計書 p.8: 「十分なタップ領域 (≥ 44px)」

| ウィジェット | 現在の padding | 推定高さ | 要件 |
|---|---|---|---|
| ツールバーボタン | [4.0, 10.0] | ~22px | ≥ 44px |
| Export ボタン | [3.0, 7.0] | ~20px | ≥ 44px |
| フィルターボタン | [3.0, 8.0] | ~20px | ≥ 44px |

→ padding を [10.0, 16.0] 以上に統一して ≥ 44px を確保。

## Design

### 1. build_toolbar() の完全置換

旧 build_toolbar() を以下で置換:

```rust
fn build_toolbar<'a>(app: &'a App) -> Element<'a, Message> {
    // RFC 007 + RFC 014: Design-doc toolbar
    // [ □ 開く ] [ □ 保存 ] [ ▶ 監査実行 ] [ ↑ レポート出力 ]   監査ステータス: XX

    let btn = |icon: &str, label: String, msg: Message| -> Element<'_, Message> {
        button(
            row![text(icon).size(12), text(label).size(12)]
                .spacing(4).align_y(Center)
        )
        .on_press(msg)
        .padding(Padding::from([10.0, 16.0]))  // ≥44px ABDD
        .into()
    };

    let open_btn   = btn("□", t!("toolbar.open").to_string(), Message::BackToOpening);
    let save_btn   = btn("□", t!("toolbar.save").to_string(), Message::SaveDefinition);
    let run_btn    = btn("▶", t!("toolbar.run_audit").to_string(), Message::RerunAudit);
    let report_btn = btn("↑", t!("toolbar.report_output").to_string(),
                         Message::ExportReport("markdown".into()));

    let status_label = if let Some(r) = &app.audit_result {
        let (label, color) = if r.summary.is_passing() {
            (t!("toolbar.passed").to_string(), theme::OK_COLOR)
        } else {
            (t!("toolbar.failed").to_string(), theme::FAILED_COLOR)
        };
        text(format!("{}: {}", t!("toolbar.audit_status"), label))
            .size(13).color(color)
    } else {
        text("").size(13)
    };

    container(
        row![open_btn, save_btn, run_btn, report_btn,
             space().width(Length::Fill), status_label]
        .spacing(6).align_y(Center).padding(Padding::from([4.0, 10.0]))
    ).width(Length::Fill).style(panel_style).into()
}
```

### 2. inspector.rs の reason フィールドを text_editor に

```rust
// 旧: text_input(&t!("inspector.reason_placeholder"), &ins.reason)
//         .on_input(Message::ReasonChanged).padding(8);
// 新:
use iced::widget::text_editor;
let reason_input = text_editor(&ins.reason_content)
    .on_action(Message::ReasonAction)
    .placeholder(t!("inspector.reason_placeholder").as_ref())
    .height(Length::Fixed(72.0))
    .padding(Padding::from([8.0, 10.0]));
```

### 3. ABDD tap area: ボタン padding を統一

全 button の padding を最低 `[10.0, 14.0]` に統一する。
