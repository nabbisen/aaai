# RFC 008 — Bottom Action Bar

**Status.** Implemented (v0.16.0)  
**Priority.** v1.0 blocker  
**Tracks.** 設計書 p.5 下部バー / p.7「承認ボタンは reason + strategy + valid rule が揃ったときのみ有効」  
**Touches.** `aaai-gui/src/views/main_view.rs` · `views/inspector.rs` · `app.rs`

## Summary

設計書は「**承認して保存**を主操作として全ペインの外（下部）に固定」することを明示している。  
現在はインスペクターパネル内に埋め込まれており、設計原則「一つの場所で一つのことをする」に反する。  
合わせて「未解決件数」と「現在の選択ファイル」の常時表示を追加する。

## Current vs Design

| 設計書 | 現実装 | 差分 |
|---|---|---|
| 承認して保存（ボトムバー固定） | インスペクター内ボタン | 場所が違う |
| 選択中: config/server.toml | なし | 現在選択中のファイルが表示されない |
| 14件の差分中 4件が未解決 | なし | 未解決件数の常時表示がない |

## Design

### ボトムバーのレイアウト

```
┌──────────────────────────────────────────────────────────────────────┐
│  [ 承認して保存 ]    選択中: config/server.toml    14件中 4件が未解決  │
└──────────────────────────────────────────────────────────────────────┘
```

- **承認して保存** ボタン: `can_approve()` の場合のみ有効（青）、それ以外は無効（薄い）
- **選択中**: `selected_index` が `Some` のとき選択中のパスを表示
- **N件中 M件が未解決**: `audit_result` から計算（failed + pending）

### 実装方針

`view_bottom_bar()` 関数を `main_view.rs` に追加し、`view()` の `column!` 末尾に追加する：

```rust
fn view_bottom_bar<'a>(app: &'a App) -> Element<'a, Message> {
    let can_approve = app.selected_index.is_some()
        && app.inspector.validation.can_approve();

    let approve_btn = button(
        text(t!("bottombar.approve_and_save")).size(13)
            .font(/* semibold */)
    )
    .on_press_maybe(if can_approve { Some(Message::ApproveAndSave) } else { None })
    .padding(Padding::from([7.0, 20.0]));

    let selected_label: Element<'_, Message> = if let Some(idx) = app.selected_index {
        if let Some(r) = app.audit_result.as_ref().and_then(|r| r.results.get(idx)) {
            text(format!("選択中: {}", r.diff.path)).size(12)
                .color(/* subtle */).into()
        } else { space().width(Length::Fill).into() }
    } else { space().width(Length::Fill).into() };

    let unresolved_label: Element<'_, Message> = if let Some(s) = app.audit_result.as_ref().map(|r| &r.summary) {
        let unresolved = s.failed + s.pending + s.error;
        text(format!("{}件の差分中 {}件が未解決", s.total, unresolved))
            .size(12)
            .color(if unresolved > 0 { /* warning */ } else { /* subtle */ })
            .into()
    } else { space().width(Length::Fill).into() };

    container(
        row![approve_btn,
             space().width(Length::Fixed(16.0)),
             selected_label,
             space().width(Length::Fill),
             unresolved_label]
        .spacing(8).align_y(Center)
        .padding(Padding::from([6.0, 16.0]))
    )
    .width(Length::Fill)
    .style(panel_style)
    .into()
}
```

### インスペクターからの承認ボタン削除

RFC 008 実装後、インスペクターパネル内の `approve_btn` を削除する。  
`val_err`（バリデーションエラー表示）はインスペクター内に残す。

### i18n キー

```yaml
bottombar:
  approve_and_save: "承認して保存"   # / "Approve & Save"
  selected:         "選択中"         # / "Selected"
  unresolved:       "件が未解決"     # / "unresolved"
```

## Dependencies

- RFC 007（Batch Approve ボタン削除）が先行することで、ボトムバーが「唯一の承認手段」として機能する。

## Open Questions

- 承認ボタンが無効状態のとき、ボトムバーに「理由を入力してください」等のヒントを表示するか。
