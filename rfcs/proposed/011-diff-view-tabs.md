# RFC 011 — Diff View Tabs

**Status.** Proposed  
**Priority.** 重要改善（v1.0 推奨）  
**Tracks.** 設計書 p.5 中央ペイン②「左右差分 ｜ 統合 ｜ 変更のみ」タブ  
**Touches.** `aaai-gui/src/views/diff_view.rs` · `app.rs` (DiffViewMode, Message)

## Summary

現在の差分ビューアは「左右差分（side-by-side）」のみを表示する。  
設計書は 3 種類の表示モードを定義している。  
変更のみを見たい場合や unified diff を好むユーザーへの対応として実装する。

## Current vs Design

| 設計書 | 現実装 | 差分 |
|---|---|---|
| 左右差分 ｜ 統合 ｜ 変更のみ（3 タブ） | 常に左右差分のみ | タブなし |

## Design

### 表示モード定義

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiffViewMode {
    #[default]
    SideBySide,   // 左右差分（現在の実装）
    Unified,      // 統合（unified diff 形式）
    ChangedOnly,  // 変更のみ（変更行だけを表示）
}
```

`App` に `diff_view_mode: DiffViewMode` フィールドを追加。  
`Message::SetDiffViewMode(DiffViewMode)` で切り替え。

### タブ UI

```
┌─────────────────────────────────────────────┐
│ config/server.toml                           │
│ [左右差分] [統合] [変更のみ]                   │  ← タブ行
├─────────────────────────────────────────────┤
│ Left (before)         │ Right (after)        │
│ ...                   │ ...                  │
└─────────────────────────────────────────────┘
```

タブボタン: snora の `tab` widget が使用可能か検討する。  
なければ `button` で実装し、アクティブタブに強調スタイルを適用。

### 統合表示（Unified）

`similar::TextDiff` の `unified_diff()` を使用して unified format を生成：

```rust
fn unified_view<'a>(diff: &'a DiffEntry) -> Element<'a, Message> {
    let before_text = diff.before_text.as_deref().unwrap_or("");
    let after_text  = diff.after_text.as_deref().unwrap_or("");
    let d = TextDiff::from_lines(before_text, after_text);
    // unified diff 形式で表示
    ...
}
```

### 変更のみ表示（ChangedOnly）

side-by-side と同じ実装だが、`ChangeTag::Equal` の行をフィルターして非表示にする。

### i18n キー

```yaml
diff:
  tab_side_by_side:  "左右差分"  # / "Side by side"
  tab_unified:       "統合"      # / "Unified"
  tab_changed_only:  "変更のみ"  # / "Changes only"
```

## Data Model

```rust
// App struct への追加
pub diff_view_mode: DiffViewMode,
```

## Open Questions

- `before_text` / `after_text` は現在 `DiffEntry` に含まれているか確認必要。
  含まれていない場合は `diff_view.rs` 内で再読み込みが必要（パフォーマンス考慮）。
