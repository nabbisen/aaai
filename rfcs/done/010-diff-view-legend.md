# RFC 010 — Diff View Legend & Stats Bar

**Status.** Implemented (v0.15.0)  
**Priority.** v1.0 blocker  
**Tracks.** 設計書 p.5 中央ペイン②「差分ハイライト」凡例  
**Touches.** `aaai-gui/src/views/diff_view.rs`

## Summary

差分ビューアの下部に「削除」「追加」の色凡例がなく、  
初見ユーザーは赤・緑の意味を自力で理解する必要がある。  
設計書には明示的に差分ハイライト凡例が示されている。  
合わせて差分統計バー（+N lines / −N lines）の位置を設計書に合わせる。

## Current vs Design

| 設計書 | 現実装 | 差分 |
|---|---|---|
| 下部に「差分ハイライト: [■削除] [■追加]」 | なし | 凡例がない |
| ヘッダー下に統計バー | 実装済み | ほぼ合致 |

## Design

### 凡例レイアウト（diff ペインの最下部）

```
┌──────────────────────────────┐
│ 差分ハイライト  [■ 削除] [■ 追加]  │
└──────────────────────────────┘
```

```rust
fn diff_legend<'a>() -> Element<'a, Message> {
    container(
        row![
            text(t!("diff.legend_label")).size(11)
                .color(Color::from_rgb(0.55, 0.55, 0.60)),
            space().width(Length::Fixed(12.0)),
            // 削除（赤）
            container(space().width(10).height(10))
                .style(|_| ContainerStyle {
                    background: Some(Background::Color(
                        Color::from_rgba(0.85, 0.20, 0.20, 0.25))),
                    ..Default::default()
                }),
            text(t!("diff.legend_removed")).size(11)
                .color(Color::from_rgb(0.55, 0.55, 0.60)),
            space().width(Length::Fixed(12.0)),
            // 追加（緑）
            container(space().width(10).height(10))
                .style(|_| ContainerStyle {
                    background: Some(Background::Color(
                        Color::from_rgba(0.10, 0.65, 0.30, 0.25))),
                    ..Default::default()
                }),
            text(t!("diff.legend_added")).size(11)
                .color(Color::from_rgb(0.55, 0.55, 0.60)),
        ]
        .spacing(6)
        .align_y(iced::Alignment::Center)
        .padding(Padding::from([4.0, 10.0]))
    )
    .width(Length::Fill)
    .into()
}
```

`side_by_side()` 関数の末尾 `column!` に追加：

```rust
column![
    stats_bar(diff),
    scrollable(/* diff rows */),
    diff_legend(),   // ← 追加
]
```

### i18n キー

```yaml
diff:
  legend_label:   "差分ハイライト"  # / "Diff highlight"
  legend_removed: "削除"           # / "Removed"
  legend_added:   "追加"           # / "Added"
```

## Open Questions

- 凡例はバイナリファイルパネルにも表示するか（バイナリには不要なので非表示が適切）。
