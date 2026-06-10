# RFC 005 — Keyboard Navigation & Focus Management

**Status.** Implemented (v0.12.0)  
**Tracks.** GUI キーボード操作の完全化。設計書 p.8「キーボードファースト」  
**Touches.** `crates/aaai-gui/src/app.rs` (subscription, Message) · `views/main_view.rs` · `views/inspector.rs`

## Summary

GUI の主要操作をキーボードで完結できるよう、新規ショートカットと
フォーカス管理を整備する。設計書が定義するキーボード完結フロー
「ツリー移動 → 差分確認 → 理由入力 → 承認保存 → 再実行 → レポート出力」を
キーボードのみで実行できる状態を目標とする。

## 問題

### 1. ペイン間の Tab 移動がない

現状: ファイルツリー・差分ビュー・インスペクターの間を
キーボードで遷移する方法がない。マウス必須。

設計書 p.8: 「キーボード — ツリー移動、差分移動、編集、保存、再監査、
レポート出力までキーボードで完結できる」

### 2. インスペクターへの Enter キー遷移がない

ファイルツリーでエントリを選択（↑↓）した後、
Enter キーでインスペクターの理由フィールドにフォーカスを移す操作がない。

### 3. 検索フォーカスのショートカットがない

設計書の凡例: `/` キーで検索バーにフォーカス（未実装）。

### 4. レポート出力ショートカットがない

設計書の凡例: `Ctrl+E` でレポート出力（未実装）。

## 設計

### 5-1. 新規ショートカット一覧

| キー | アクション | 条件 |
|---|---|---|
| `Enter` | インスペクターの reason フィールドにフォーカス | Main 画面・エントリ選択中 |
| `/` | 検索バーにフォーカス | Main 画面 |
| `Escape` | 検索バーのフォーカスを解除 / エントリ選択解除 | Main 画面 |
| `Ctrl+E` | レポート出力（Markdown） | Main 画面 |
| `Tab` | ファイルツリー → インスペクター → ファイルツリーの循環 | Main 画面 |
| `Shift+Tab` | 逆順循環 | Main 画面 |
| `Enter`（インスペクター内） | 「承認して保存」実行 | reason 入力済み + validation OK |

既存ショートカット（変更なし）:

| キー | アクション |
|---|---|
| `Ctrl+S` | 保存 |
| `Ctrl+R` | 監査再実行 |
| `Ctrl+Z` | Undo |
| `↑` / `↓` | ファイルツリー移動 |

### 5-2. フォーカス状態の管理

iced は DOM ベースのフォーカス管理を持たないため、アプリ側で
論理フォーカス（`FocusTarget`）を管理する。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusTarget {
    FileTree,
    Search,
    Inspector,
}

impl Default for FocusTarget {
    fn default() -> Self { FocusTarget::FileTree }
}

// App struct に追加
pub focus_target: FocusTarget,
```

`FocusTarget` に基づいてキーボードイベントのルーティングを変更:

```rust
// subscription のキーボードイベント処理
keyboard::Key::Named(key::Named::Tab) => {
    if modifiers.shift() {
        Message::FocusPrev
    } else {
        Message::FocusNext
    }
}
keyboard::Key::Named(key::Named::Enter) => {
    match app.focus_target {
        FocusTarget::FileTree => Message::FocusInspectorReason,
        FocusTarget::Inspector => {
            if app.inspector.validation.can_approve() {
                Message::ApproveAndSave
            } else {
                Message::Noop
            }
        }
        _ => Message::Noop,
    }
}
keyboard::Key::Character("/") => {
    if !modifiers.control() && !modifiers.alt() {
        Message::FocusSearch
    } else {
        Message::Noop
    }
}
```

### 5-3. フォーカスの視覚化

iced の `text_input` / `button` はフォーカス時に自動でアウトラインを描画するが、
`container` ベースのカスタムウィジェット（ファイルツリー行）は手動で対応が必要。

```rust
// ファイルツリー行のスタイル
let bg = move |_: &iced::Theme| iced::widget::container::Style {
    background: if is_selected {
        Some(iced::Background::Color(SELECTION_COLOR))
    } else {
        None
    },
    // focus_target == FileTree かつ選択中の行にのみフォーカスリング
    border: if is_selected && focus_target == FocusTarget::FileTree {
        iced::Border { color: FOCUS_RING_COLOR, width: 2.0, radius: 0.0.into() }
    } else {
        iced::Border::default()
    },
    ..Default::default()
};
```

フォーカスリング色: `#3B82F6`（青・十分なコントラスト）

### 5-4. / キー + Escape のフォーカス制御

```rust
Message::FocusSearch => {
    self.focus_target = FocusTarget::Search;
    // iced の text_input::focus() を使用（iced 0.14: id ベース）
}
Message::FocusInspectorReason => {
    self.focus_target = FocusTarget::Inspector;
    // inspector の reason text_input に focus
}
Message::FocusNext => {
    self.focus_target = match self.focus_target {
        FocusTarget::FileTree  => FocusTarget::Inspector,
        FocusTarget::Inspector => FocusTarget::FileTree,
        FocusTarget::Search    => FocusTarget::FileTree,
    };
}
Message::FocusPrev => {
    // FocusNext の逆順
}
```

### 5-5. Ctrl+E（レポート出力）

```rust
keyboard::Modifiers::CTRL if key == keyboard::Key::Character("e") => {
    Message::ExportReport("markdown".into())
}
```

既存の `ExportReport` メッセージを再利用。

### 5-6. iced の text_input フォーカス API

iced 0.14 では `text_input::Id` を使ってプログラム的にフォーカスを移動できる。

```rust
// ID 定義
static REASON_INPUT_ID: std::sync::LazyLock<text_input::Id> =
    std::sync::LazyLock::new(|| text_input::Id::unique());

// inspector.rs の reason input
text_input(..., &ins.reason)
    .id(REASON_INPUT_ID.clone())
    ...

// app.rs の FocusInspectorReason ハンドラ
Message::FocusInspectorReason => {
    self.focus_target = FocusTarget::Inspector;
    return text_input::focus(REASON_INPUT_ID.clone());
}
```

## データモデル

```rust
// App struct への追加
pub focus_target: FocusTarget,

// 新規 Message variants
FocusNext,
FocusPrev,
FocusSearch,
FocusInspectorReason,
```

## 実装ノート

iced 0.14 の `text_input::focus()` は `Task<Message>` を返す。
`update()` から `Task` を返す既存の仕組みを使う。

ファイルツリー行の `button` ウィジェットは iced 標準のフォーカスを持つが、
PaneGrid 内の論理フォーカスとの干渉を避けるため `FocusTarget` で管理する。

## 代替案

**A. iced の自動 Tab 順序に任せる**: PaneGrid 内の Tab 順序が
ファイルツリー → 全ボタン → インスペクター入力 と非常に長くなる。
論理フォーカスの方が UX が良い。  
**B. フルキーボードマップを将来 RFC で拡張**: 本 RFC の範囲を
最小限に抑え、主要フローのみカバー。より高度な VI 風ナビゲーション等は別 RFC。

## Open Questions

- `text_input::Id` をモジュールグローバルで定義するか、
  `App` のフィールドとして持つかは実装時に決定。
- インスペクター内での Tab 移動順序（reason → ticket → approved_by → ...）は
  iced の自動 Tab 順序に委ねる（別途 ID を設定すれば制御可能）。
- `Escape` でエントリ選択解除すると差分ビューがダッシュボードに戻る。
  これが望ましい動作か確認が必要。
