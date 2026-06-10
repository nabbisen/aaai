# RFC 023 — Opening Drag-and-Drop & Recent Projects Polish

**Status.** Implemented (v0.20.0)
**Priority.** v1.0 nice-to-have
**Tracks.** RFC 015 §1.3 FR-8 (DnD) の積み残し / 設計書 p.10 A. 「最近の組み合わせ再利用」の利便性
**Touches.** `crates/aaai-gui/src/views/opening.rs` · `app.rs` (DnD event handling, profile last_used_at) · `crates/aaai-core/src/profile/store.rs` (last_used_at field)

---

## 1. 要件定義

### 1.1 目的

RFC 015 で「任意（将来）」として留保された **フォルダのドラッグ&ドロップ受け付け** を実装する。
あわせて Recent プロファイルの並び順を「最終使用日時順 (desc)」に固定し、設計書 p.10 A.
バックログの「最近の組み合わせ再利用」を完成させる。

### 1.2 解決すべき問題

| 問題 | 現状 |
|---|---|
| Before/After フォルダの指定はファイルピッカー経由のみ | DnD 不可 |
| プロファイル一覧は保存順 (insert 順) | 直近に使ったものが上部に来ない |
| プロファイルが大量に貯まると見つけにくくなる | フィルタなし |

### 1.3 機能要件

| ID | 要件 | 必須/任意 |
|---|---|---|
| FR-1 | Opening 画面のフォルダカードに対し、フォルダをドラッグ&ドロップで指定できる | 必須 |
| FR-2 | DnD 中はカードがハイライト表示される | 必須 |
| FR-3 | フォルダ以外がドロップされたとき、エラーをインライン表示する | 必須 |
| FR-4 | プロファイルに `last_used_at: Option<DateTime<Utc>>` を追加 | 必須 |
| FR-5 | プロファイル一覧は `last_used_at` desc で並ぶ | 必須 |
| FR-6 | プロファイル「読み込む」操作で `last_used_at` が更新される | 必須 |
| FR-7 | (任意) プロファイル一覧にインクリメンタル検索を追加 | 任意 |

### 1.4 非機能要件

| ID | 要件 |
|---|---|
| NFR-1 | DnD は iced 0.14 がサポートする範囲で実装 (`iced::event::Event::Window`) |
| NFR-2 | プロファイル並び順変更で既存ストレージ (`~/.aaai/profiles.yaml`) が壊れない |
| NFR-3 | `last_used_at` が欠落しているレガシープロファイルは「最古」として扱う |

---

## 2. 外部設計（基本設計）

### 2.1 DnD のフィードバック

```
通常:
┌─────────────────────────────────────────────────┐
│ 📁 比較元 (Before)                                │
│ ✗ 未選択                          [ フォルダを選ぶ ] │
└─────────────────────────────────────────────────┘

ドラッグ中 (フォルダ hover):
┌═════════════════════════════════════════════════┐  ← 太い破線ボーダー
║ 📁 比較元 (Before)                                ║
║ ↓ ここにドロップしてください                       ║
└═════════════════════════════════════════════════┘

ドロップ後:
┌─────────────────────────────────────────────────┐
│ 📁 比較元 (Before)                                │
│ ✓ /home/me/proj/v1/             [ フォルダを変更 ] │
└─────────────────────────────────────────────────┘
```

### 2.2 ドロップ受付ルール

| 種類 | 動作 |
|---|---|
| ディレクトリ 1 個 | 該当カードに反映 |
| ファイル 1 個 | エラー: "フォルダをドロップしてください" |
| 複数項目 | 1 個目のみを採用 (要検討: エラー表示と二択) |
| ウィンドウ外 → ドラッグ離脱 | 通常表示に戻る |

### 2.3 Recent プロファイルの並び替え

```
─── 最近使ったプロジェクト ───

▸ release-2026-Q2       1 分前に使用     [ 開く ]
  before: /a   →  after: /b
▸ staging-check         3 日前に使用     [ 開く ]
  before: /c   →  after: /d
▸ archive               2026-04-10       [ 開く ]
  before: /e   →  after: /f
```

「1 分前」「3 日前」など相対時刻 + 1 週間以上経過したら絶対日付に切り替え。

---

## 3. 内部設計（詳細設計）

### 3.1 iced 0.14 の DnD イベント

```rust
// app.rs::subscription()
iced::event::listen_with(|event, _status, _id| {
    use iced::event::Event;
    use iced::window::Event as WinEvent;
    match event {
        Event::Window(WinEvent::FileDropped(path)) =>
            Some(Message::FileDropped(path)),
        Event::Window(WinEvent::FileHovered(path)) =>
            Some(Message::FileHovered(Some(path))),
        Event::Window(WinEvent::FilesHoveredLeft) =>
            Some(Message::FileHovered(None)),
        _ => None,
    }
})
```

ホバー対象のカードは座標から判定。iced のレイアウト座標を直接取得するのは難しいため、
**画面全体に対するドロップとして受け、currently focused / hovered カードがあれば
そこに割り当てる**。簡易には「先に hover が確定したカードに割り当てる」ロジックで実装。

具体的には:
- マウス位置を `Message::CursorMoved(Point)` で記録 (これは既存）
- Card 描画時にその bounding box を `app.dnd_target_zones: Vec<(Rect, DropTarget)>` に登録
- `FileDropped` 受信時にカーソル位置から target を決定

### 3.2 プロファイルストレージ拡張

```rust
// crates/aaai-core/src/profile/store.rs
pub struct AuditProfile {
    pub name: String,
    pub before: String,
    pub after: String,
    pub definition: String,
    pub ignore: String,
    pub last_used_at: Option<DateTime<Utc>>,  // 新規
}

impl ProfileStore {
    pub fn touch(&mut self, idx: usize) -> Result<()> {
        if let Some(p) = self.profiles.get_mut(idx) {
            p.last_used_at = Some(Utc::now());
            self.save()?;
        }
        Ok(())
    }

    pub fn sorted_by_recent(&self) -> Vec<&AuditProfile> {
        let mut v: Vec<_> = self.profiles.iter().collect();
        v.sort_by(|a, b| b.last_used_at.cmp(&a.last_used_at));
        v
    }
}
```

### 3.3 既存 YAML の後方互換

`AuditProfile` の `last_used_at: Option<DateTime<Utc>>` は `#[serde(default)]` を付与。
レガシープロファイルでは `None` となり、`sorted_by_recent` で「最古」として扱われる。

### 3.4 相対時刻フォーマッタ

```rust
fn humanize_since(t: DateTime<Utc>) -> String {
    let now = Utc::now();
    let delta = now - t;
    if delta.num_seconds() < 60 { return t!("relative.just_now").to_string(); }
    if delta.num_minutes() < 60 { return t!("relative.minutes_ago", n: delta.num_minutes()).to_string(); }
    if delta.num_hours() < 24 { return t!("relative.hours_ago", n: delta.num_hours()).to_string(); }
    if delta.num_days() < 7 { return t!("relative.days_ago", n: delta.num_days()).to_string(); }
    t.format("%Y-%m-%d").to_string()
}
```

### 3.5 i18n キー追加

```yaml
# en.yaml
relative:
  just_now: "Just now"
  minutes_ago: "{n} min ago"
  hours_ago: "{n} h ago"
  days_ago: "{n} d ago"
opening:
  drop_here: "Drop a folder here"
  drop_invalid_kind: "Please drop a folder (file dropped)"

# ja.yaml も同様
```

---

## 4. プログラム設計

### 4.1 実装手順

| Step | 作業 | 検証 |
|---|---|---|
| 1 | `AuditProfile` に `last_used_at` を追加 (`#[serde(default)]`) | 既存 YAML が読める |
| 2 | `ProfileStore::touch` と `sorted_by_recent` を実装 | 単体テスト |
| 3 | `LoadProfile(idx)` ハンドラで `touch` を呼ぶ | 動作確認 |
| 4 | `opening.rs::recent_projects_section` を sorted 順で描画 | 視覚確認 |
| 5 | iced subscription に DnD ハンドラを追加 | 動作確認 |
| 6 | `Message::FileDropped` / `FileHovered` を実装 | 動作確認 |
| 7 | カードへの hover 強調 styling を追加 | 視覚確認 |
| 8 | i18n キー追加 (relative.*, opening.drop_here, opening.drop_invalid_kind) | check-i18n-keys.sh 通過 |

### 4.2 影響範囲

| ファイル | 変更行 |
|---|---|
| `crates/aaai-core/src/profile/store.rs` | 約 25 行 |
| `crates/aaai-gui/src/app.rs` | 約 60 行 (DnD message + state) |
| `crates/aaai-gui/src/views/opening.rs` | 約 40 行 (hover style + relative time) |
| `crates/aaai-gui/locales/{en,ja}.yaml` | 約 16 行 |
| `crates/aaai-core/tests` | 約 20 行 (touch / sort test) |

### 4.3 リスク

| リスク | 対策 |
|---|---|
| iced 0.14 の DnD イベントが Linux Wayland でうまく動かない | 「DnD が動作しない場合はピッカーを使ってください」をエラー文に含める |
| マウス座標からカード判定が複雑になりすぎる | 「カーソルがウィンドウ内 = 一番上のカードに割り当て」のような簡略実装で初期出荷 |
| `last_used_at` の YAML migration で既存ユーザーのファイルが壊れる | `#[serde(default)]` で migration 不要にする |
| 相対時刻表示で chrono の依存が大きくなる | chrono は既に依存に含まれているため追加なし |

---

## 5. 完了条件

- [ ] Opening 画面でフォルダを DnD すると Before/After カードに即時反映される
- [ ] 非フォルダをドロップするとエラーが表示される
- [ ] 「最近使った」一覧が最終使用日時順 (desc) に並ぶ
- [ ] プロファイル「開く」操作で `last_used_at` が更新される
- [ ] 「N 分前」「N 日前」表示が正しい (1 週間以上は絶対日付)
- [ ] 既存の `~/.aaai/profiles.yaml` を読み込んでも壊れない (`last_used_at` 欠落 = None)
- [ ] en / ja で表示される
- [ ] RFC 017 の Visual Verification で確認済

## 6. 依存

- **RFC 017** (verification): 検証用
- **RFC 022** (empty states): プロファイル空のときの表示と整合

## 7. v1.0 範囲外 (任意・将来)

- プロファイル一覧のインクリメンタル検索 (FR-7 任意)
- DnD の複数フォルダ受付 (現状は 1 個のみ)
