# RFC 007 — Toolbar & Navigation Restructure

**Status.** Implemented (v0.16.0)  
**Priority.** v1.0 blocker  
**Tracks.** 設計書 p.5「GUI メイン画面」ツールバー / p.6 画面リレーション  
**Touches.** `aaai-gui/src/views/main_view.rs` · `app.rs` (Message, Screen)

## Summary

現在のツールバーは設計書のボタン構成と大きく乖離している。  
主な問題は「Opening 画面への戻り手段がない」こと。  
合わせてボタン名称・配置を設計書に合わせる。

## Current vs Design

| 設計書 | 現実装 | 差分 |
|---|---|---|
| □ 開く | **なし** | Opening 画面に戻れない |
| □ 保存 | 保存 | ほぼ同等 |
| ▶ 監査実行 | 再実行 | 意味が「再」に限定されている |
| ↑ レポート出力 | Export MD + Export JSON (2 ボタン) | 設計書は単一ボタン |
| 右端: 監査ステータス: FAILED | 記号バッジ + 件数 | 位置・表現が異なる |
| (なし) | Batch Approve | 設計書に記載なし |

## Design

### ツールバー構成

```
[ □ 開く ] [ □ 保存 ] [ ▶ 監査実行 ] [ ↑ レポート出力 ]
                                          監査ステータス: FAILED
```

- **開く** → `Message::BackToOpening`（Opening 画面に戻る）
  - 未保存の変更がある場合は確認ダイアログ的なトーストを出す
- **保存** → `Message::SaveDefinition`（既存）
- **監査実行** → `Message::RerunAudit`（既存の再実行と同じ処理。ラベル変更のみ）
- **レポート出力** → 押下で Markdown レポートを出力（`Message::ExportReport("markdown")`）
  - JSON/HTML/SARIF は `aaai report` CLI コマンドを案内する
- **Batch Approve** → 削除。バッチ承認機能はキーボードショートカット等で代替

### 監査ステータス表示

設計書: 右端に `監査ステータス: FAILED` のテキスト表示  
→ 現在の記号バッジ（`✗ FAILED  ✓N ⚠N …`）は情報量が多く設計書から外れている  
→ シンプルに `監査ステータス: FAILED` / `監査ステータス: PASSED` のテキストに変更

### `Message::BackToOpening`

```rust
Message::BackToOpening => {
    if self.dirty {
        self.push_toast(ToastIntent::Warning,
            "Unsaved changes", "Save before leaving or changes will be lost.");
        return Task::none();
    }
    self.screen = Screen::Opening;
    self.audit_result = None;
    self.diffs.clear();
    self.definition = None;
    self.selected_index = None;
}
```

## Open Questions

- Batch Approve を削除した場合の代替 UX を RFC 008 と合わせて検討。
- レポート出力ボタンは「Markdown のみ」か「形式選択ダイアログ」か。
  設計書は単一ボタンなので Markdown をデフォルトとし、詳細は CLI を案内する方針。
