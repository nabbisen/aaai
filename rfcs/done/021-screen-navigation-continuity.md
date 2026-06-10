# RFC 021 — Screen Navigation Continuity & Re-run Visibility

**Status.** Implemented partial (v0.20.0) — Save/Report freshness marks; audit-dirty banner deferred pending architecture decoupling
**Priority.** v1.0 改善
**Tracks.** 設計書 p.6 (画面リレーション: Opening → Audit → Review → Save/Report → Re-run の循環)
**Touches.** `crates/aaai-gui/src/views/main_view.rs` (toolbar / status banner) · `app.rs` (audit dirtiness flag) · `views/opening.rs` (戻り動線)

---

## 1. 要件定義

### 1.1 目的

設計書 p.6 の画面リレーションは「分岐を少なく、戻れる構造にする」とある。
現状の v0.19.0 は Opening と Main の 2 画面構成、Main 内では Ctrl+R で再監査できるが、
**「いま再監査が必要な状態かどうか」「次にどの操作に進めるか」が画面上で明示されていない**。
本 RFC ではこれを視覚的に伝えることで、ユーザーが循環フローのどこにいるかを常に把握できる
状態にする。

### 1.2 解決すべき問題

| 問題 | 現状 |
|---|---|
| 監査定義を編集したあと、再監査が必要なのか分からない | 「未保存」表示はあるが「再監査要」表示はない |
| 「Re-run」が単なるショートカット (Ctrl+R) で、循環フローの一段階として認識されない | toolbar の「監査実行」ボタンは押せるが、緊急度が伝わらない |
| Opening 画面に戻ったとき、未保存変更が消える可能性があることが伝わりにくい | RFC 007 で `BackToOpening` 経由で警告は出るが、画面遷移前の状態が見えない |
| Save / Report は終端のように感じられ、再監査して再確認するという循環が UX 上見えない | レポート出力後に「Re-run しますか?」のような誘導なし |

### 1.3 機能要件

| ID | 要件 | 必須/任意 |
|---|---|---|
| FR-1 | 監査定義を編集したあと、Main 画面上部に「監査結果は古い可能性があります — 再監査を実行してください」のバナーを表示する | 必須 |
| FR-2 | バナーには「監査実行」ボタンへの誘導 (またはバナー全体がボタン化) を含める | 必須 |
| FR-3 | 監査再実行が成功したらバナーが消える | 必須 |
| FR-4 | Save / Report が成功したあと、トーストではなく「Save 済み」「Report 済み」を画面上の小さな確定マークとして示し、次に再監査するかの判断材料を残す | 任意 |
| FR-5 | Opening に戻る前 (`BackToOpening`) に、未保存変更がある場合のみ確認ダイアログを表示する | 必須 (既存 RFC 007 で確認警告あり — 本 RFC で改善) |
| FR-6 | 設計書 p.6 の「Re-run loop」を docs/src/gui.md に図示する | 任意 |

### 1.4 非機能要件

| ID | 要件 |
|---|---|
| NFR-1 | バナーは画面上部 (toolbar 直下 or filter bar 上) に固定。スクロールで隠れない |
| NFR-2 | ABDD 準拠: バナーは色だけでなくアイコン (例: 🔁) を併用 |
| NFR-3 | 多言語対応 (en / ja) |

---

## 2. 外部設計（基本設計）

### 2.1 「監査結果は古い可能性」バナーの見た目

```
┌──────────────────────────────────────────────────────────────────────┐
│ 🔁  監査定義を更新しました。最新の状態を反映するには再監査が必要です。  │
│                                            [ 監査を再実行 ]            │
└──────────────────────────────────────────────────────────────────────┘
```

- 背景色: 薄い橙 (light orange) — Warning ではないが Attention を示す
- ABDD: 🔁 アイコン + テキストで色非依存
- ボタンは右端に。クリックで `Message::RerunAudit` を発火

### 2.2 トリガー条件

```
audit_dirty = true となる条件:
  - インスペクター入力により AuditEntry が編集された
  - 監査定義が save された (=) 監査結果と乖離する可能性
  - .aaaiignore が変更された (現状は未検知 — 将来検討)

audit_dirty = false に戻る条件:
  - 監査再実行が成功した (Message::AuditFinished)
```

### 2.3 Save / Report 完了マークの表示

設計書 p.6 で Save / Report は「監査定義保存・レポート出力」とあるが、両者は連続的な
フローの中間ステップ。完了をトーストで一瞬出すよりも、画面上にチェックマークを残す
ほうがフローの認識を助ける:

```
toolbar: [ □ 開く ] [ □ 保存 ✓最終] [ ▶ 監査実行 ] [ ↑ レポート出力 ✓2分前 ]
```

ボタンの右下に小さなチェックマーク + タイムスタンプ。次のアクション (再監査) で消える。

---

## 3. 内部設計（詳細設計）

### 3.1 状態モデル

```rust
// app.rs に追加
pub struct App {
    // ... 既存フィールド
    pub audit_dirty: bool,           // 再監査必要フラグ
    pub last_saved_at: Option<Instant>,
    pub last_reported_at: Option<Instant>,
}
```

### 3.2 状態遷移

| イベント | 遷移 |
|---|---|
| `Message::ApproveAndSave` 完了 | `audit_dirty = true; last_saved_at = Some(now)` |
| `Message::SaveDefinition` 完了 | 同上 |
| `Message::ExportReport(_)` 完了 | `last_reported_at = Some(now)` |
| `Message::AuditFinished(_)` (再監査完了) | `audit_dirty = false` |
| `Message::BackToOpening` | フラグはそのまま (Opening に戻った時点で view が切り替わるため) |

### 3.3 view 上の挿入位置

```rust
// main_view.rs::view()
column![
    toolbar,
    audit_dirty_banner(app),   // ← 新規 (audit_dirty=true のみ表示)
    filter_bar,
    search_bar,
    pg,
    bottom_bar,
]
```

### 3.4 i18n キー追加

```yaml
# en.yaml
banner:
  audit_dirty_message: "Audit definition has been updated. Run audit again to refresh results."
  audit_dirty_action: "Re-run audit"
  saved_just_now: "Saved just now"
  saved_minutes_ago: "Saved {n} min ago"
  reported_just_now: "Report exported just now"

# ja.yaml
banner:
  audit_dirty_message: "監査定義を更新しました。最新の状態を反映するには再監査が必要です。"
  audit_dirty_action: "監査を再実行"
  saved_just_now: "保存しました"
  saved_minutes_ago: "{n} 分前に保存"
  reported_just_now: "レポートを出力しました"
```

### 3.5 「N 分前」表示の更新頻度

`subscription` で 30 秒おきに `Message::Tick` を発火し、相対時刻表示を更新する。
既存の toast TTL subscription と同じパターンを流用。

---

## 4. プログラム設計

### 4.1 実装手順

| Step | 作業 | 検証 |
|---|---|---|
| 1 | `app.rs` に `audit_dirty` / `last_saved_at` / `last_reported_at` を追加 | コンパイル通過 |
| 2 | 各 update ハンドラで上記フラグを更新 | デバッグログで遷移確認 |
| 3 | `main_view.rs` に `audit_dirty_banner()` を実装 | 視覚確認 |
| 4 | toolbar の Save / Report ボタンに完了マーク描画を追加 | 視覚確認 |
| 5 | i18n キーを `locales/en.yaml` / `ja.yaml` に追加 | check-i18n-keys.sh 通過 |
| 6 | `subscription` に 30 秒 tick を追加 (相対時刻更新) | 動作確認 |
| 7 | docs/src/gui.md の「典型的なワークフロー」章に Re-run loop 図を追加 | レビュー |

### 4.2 影響範囲

| ファイル | 変更行 (推定) |
|---|---|
| `crates/aaai-gui/src/app.rs` | 約 30 行 (フィールド + Message + update) |
| `crates/aaai-gui/src/views/main_view.rs` | 約 50 行 (banner + toolbar 完了マーク) |
| `crates/aaai-gui/locales/{en,ja}.yaml` | 約 20 行 |
| `docs/src/gui.md` + `docs/ja/src/gui.md` | 約 20 行 |

### 4.3 リスク

| リスク | 対策 |
|---|---|
| バナーが画面占有率を高め 3 ペインを圧迫する | バナー高さは 32〜40px 程度に制限。表示は audit_dirty=true のみ |
| 「N 分前」表示で 30 秒おきの再描画がパフォーマンスを下げる | iced は差分描画なので影響は限定的。実機で確認 |
| `audit_dirty` がフラグだけだと「実は影響のない変更だった」場合も表示される | 過剰検出を許容 (false positive は再監査を促すだけ。害がない) |

---

## 5. 完了条件

- [ ] 監査定義を編集すると Main 画面上部にバナーが表示される
- [ ] 「監査を再実行」ボタンが機能する
- [ ] 再監査成功でバナーが消える
- [ ] Save / Report 完了マークが toolbar に表示される
- [ ] 「N 分前」表示が 30 秒おきに更新される
- [ ] en / ja 両ロケールで自然な文言で表示される
- [ ] docs/src/gui.md / docs/ja/src/gui.md に Re-run loop 図が追加されている
- [ ] RFC 017 の Visual Verification で「設計書 p.6 循環フロー一致」と確認できる

## 6. 依存

- **RFC 017** (verification): 完了視覚化を裏付けるため
- **RFC 019** (docs refresh): docs にループ図を追加する前提

## 7. 後続 RFC との関係

- 本 RFC は **Re-run の必要性** を可視化する。
- RFC 022 (Empty States) は **audit_result が無い状態** を可視化する。両者で循環フローの
  すべての状態を網羅する。
