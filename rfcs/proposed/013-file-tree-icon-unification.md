# RFC 013 — File Tree Icon Unification

**Status.** Proposed  
**Priority.** 重要改善（v1.0 推奨）  
**Tracks.** 設計書 p.5 ファイルツリー①  
**Touches.** `aaai-gui/src/views/main_view.rs`

## Summary

設計書のファイルツリーはファイル行ごとに「ステータスアイコン 1 個」だけを使い、  
diff type（追加/削除/変更）は右端の小記号で補完する構成になっている。  
現実装は RFC 003 で導入した「左端: diff-type バッジ（灰色）＋右端: status バッジ」という  
2 重構造のままであり、設計書の意図と逆になっている。  
また設計書に存在しないフィルターバー・検索バーの位置づけも整理する。

## Current vs Design

### ファイルツリー行

| 位置 | 設計書 | 現実装 |
|---|---|---|
| 行頭 | ステータスアイコン 1 個（✓ ✗ ⚠ —） | diff-type バッジ（灰色, RFC 003） |
| 右端 | diff-type の小記号（— × ⚠ ●） | status バッジ（✓ OK / ⚠ Pending / ✗ Failed …） |

設計書では**行頭がステータス**（OK/Pending/Failed）、右端が**差分種別**の補足表示。  
現実装は**行頭が差分種別**、右端がステータスという逆の構造になっている。

### フィルターバー・検索バー

| 要素 | 設計書 | 現実装 |
|---|---|---|
| フィルターバー | **記載なし** | 存在する（All / 変更のみ / 未承認 / 失敗・エラー） |
| 検索バー | **記載なし** | 存在する |

設計書に記載がないが、機能的には有用。  
→ 削除は不要。ただし**視覚的主張を下げる**（薄いスタイルに変更）。

## Design

### ファイルツリー行の再設計

```
[ ✓ ] app.toml                               ─
[ ✗ ] server.toml                            ×
[ ✓ ] README.md                 ⚠1          ~
[ ✓ ] CHANGELOG.md                           ~
```

- **行頭（左）**: ステータスアイコン（色付き記号のみ、テキストラベルなし）
- **中央**: ファイルパス（モノスペース）
- **warn_badge**: 警告件数（存在する場合のみ）
- **右端**: diff-type の小記号（灰色テキスト、12px）

### 行頭ステータスアイコン

```rust
fn status_icon(status: AuditStatus) -> Element<'static, Message> {
    let (sym, color) = match status {
        AuditStatus::Ok      => ("✓", OK_COLOR),
        AuditStatus::Pending => ("⚠", PENDING_COLOR),
        AuditStatus::Failed  => ("✗", FAILED_COLOR),
        AuditStatus::Error   => ("!", ERROR_COLOR),
        AuditStatus::Ignored => ("—", IGNORED_COLOR),
    };
    text(sym).size(13).color(color).into()
}
```

- テキストラベル（"OK"/"Pending" 等）は廃止
- アイコン単体で ABDD 準拠（記号 + 色の組み合わせ）

### 右端 diff-type 記号

```rust
fn diff_type_tag(dtype: DiffType) -> Element<'static, Message> {
    let sym = match dtype {
        DiffType::Added        => "+",
        DiffType::Removed      => "−",
        DiffType::Modified     => "~",
        DiffType::TypeChanged  => "T",
        DiffType::Unchanged    => " ",
        DiffType::Unreadable   => "!",
        DiffType::Incomparable => "?",
    };
    text(sym).size(11).color(Color::from_rgb(0.65, 0.65, 0.68)).into()
}
```

### 廃止するもの

- `colored_badge()` の diff-type 用途（RFC 003 で追加した灰色バッジ）を削除
- `status_badge()` の右端テキストラベル版を削除し `status_icon()` に統一
- バッジの枠線・背景色をなくし、シンプルなテキストアイコンに

### フィルターバー・検索バーの扱い

設計書に記載がないが、機能として残す。  
ただし**ビジュアル上目立たないスタイル**（細いボーダー、薄い背景）に変更し、  
メインの視線がファイルツリーに向くようにする。

### バッチ選択チェックボックス

設計書に記載なし。RFC 008 で Batch Approve ボタンがツールバーから削除される場合、  
チェックボックスの扱いも合わせて検討する。  
→ 本 RFC では現状維持とし、表示可否は RFC 008 に委ねる。

## Dependencies

- RFC 007（ツールバー再構成）完了後に実施することで、Batch Approve との整合を取る。

## Open Questions

- warn_badge は引き続き存在感のあるスタイルが必要か（⚠N の黄色バッジ）。
  設計書でも README.md 行に ⚠ が表示されているため現状維持が適切。
