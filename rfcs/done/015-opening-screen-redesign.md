# RFC 015 — Opening Screen Redesign

**Status.** Implemented (v0.19.0) — visual verification pending
**Priority.** v1.0 blocker
**Tracks.** 設計書 p.2 中心体験「選ぶ → 見る → ...」/ p.3 オープニング / プロジェクト選択 / p.6 画面間リレーション
**Touches.** `aaai-gui/src/views/opening.rs` · `app.rs` (Message, App) · `Cargo.toml` (rfd 依存追加)

---

## 1. 要件定義

### 1.1 目的

初回ユーザーが `aaai-gui` を起動したとき、説明なしで監査を開始できるようにする。

### 1.2 解決すべき問題（現状の Opening 画面）

| 問題 | 内容 |
|---|---|
| 何を入力すればよいか不明 | 4 つのテキストボックスが並んでいるだけ |
| パスを手打ちさせる UX | 設計書原則「選ぶ → ...」に反する |
| 必須/任意の区別が見えない | 4 つすべてが同じ重みで表示 |
| ボタン非活性の理由が見えない | 何が足りないかわからない |
| 過去の作業の再利用導線がない | プロファイル機能は下部に小さくしかない |

### 1.3 機能要件

| ID | 要件 | 必須/任意 |
|---|---|---|
| FR-1 | 必須 2 つのフォルダ（Before / After）を選択する操作を画面上で最も目立たせる | 必須 |
| FR-2 | フォルダ選択は OS ネイティブダイアログで行う（手入力強制を廃止） | 必須 |
| FR-3 | 任意設定（audit.yaml / .aaaiignore）は折りたたみで隠す | 必須 |
| FR-4 | 必須 2 つが揃ったとき「監査を開始」ボタンが活性化 | 必須 |
| FR-5 | 過去に使ったプロジェクト（プロファイル）を上位表示し再利用を促す | 必須 |
| FR-6 | ヘッダーに「何をするか」の一文を必ず表示 | 必須 |
| FR-7 | 既存のテキスト入力もサポート（プロファイル読込・タイピング上級者向け） | 任意 |
| FR-8 | パスのドラッグ＆ドロップ受付 | 任意（将来） |

### 1.4 非機能要件

| ID | 要件 |
|---|---|
| NFR-1 | 設計書 p.8 ABDD 準拠: 色だけに依存しない / キーボード操作完結 / タップ領域 ≥ 44px |
| NFR-2 | 日本語 / 英語の両対応（rust-i18n） |
| NFR-3 | フォルダピッカーが非同期で動作し UI をブロックしない |
| NFR-4 | 存在しないパス・ファイル・権限不足を即座にインライン表示 |

---

## 2. 外部設計（基本設計）

### 2.1 画面レイアウト

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                  │
│                        aaai                                      │
│                audit for asset integrity                         │
│                                                                  │
│         監査するための 2 つのフォルダを選んでください              │
│                                                                  │
│   ┌──────────────────────────────────────────────────────────┐  │
│   │ 📁 比較元 (Before)                                        │  │
│   │ ─────────────────────────────────────────────────────────│  │
│   │ ✗ 未選択                              [ フォルダを選ぶ ] │  │
│   └──────────────────────────────────────────────────────────┘  │
│                                                                  │
│   ┌──────────────────────────────────────────────────────────┐  │
│   │ 📁 比較先 (After)                                         │  │
│   │ ─────────────────────────────────────────────────────────│  │
│   │ ✓ /home/me/proj/v2/                   [ フォルダを変更 ] │  │
│   └──────────────────────────────────────────────────────────┘  │
│                                                                  │
│   ▸ オプション設定（監査定義ファイル / .aaaiignore）              │
│     └ 折りたたみ。指定しない場合は新規作成として開始されます       │
│                                                                  │
│                     [  監査を開始  ]                              │
│                                                                  │
│   ─────  または最近使ったプロジェクト  ──────                       │
│   ▸ release-2024-Q4    /path/to/proj           [ 開く ]          │
│   ▸ release-2024-Q3    /path/to/proj           [ 開く ]          │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 状態遷移

#### フォルダカードの状態

```
未選択 ─[フォルダを選ぶ]─→ ダイアログ表示
                            ├─ キャンセル → 未選択へ戻る
                            └─ 選択完了 → 選択済み
選択済み ─[フォルダを変更]─→ ダイアログ表示
                            ├─ キャンセル → 選択済みのまま
                            └─ 選択完了 → 選択済み (新パス)
```

#### 「監査を開始」ボタンの状態

```
非活性 ─(Before+Afterが両方有効)─→ 活性
活性 ─(クリック)─→ ロード中（スピナー表示）─→ Main 画面遷移
活性 ─(Before/Afterが無効化)─→ 非活性
```

### 2.3 ユーザー操作シーケンス

#### 初回利用フロー

```
1. アプリ起動
   → Opening 画面表示
   → ヘッダー文「監査するための 2 つのフォルダを選んでください」が見える
2. 「比較元」カードの「フォルダを選ぶ」をクリック
   → OS ネイティブダイアログが開く
3. フォルダを選んで「Open」
   → カードに ✓ + パスが表示
4. 「比較先」も同様に選択
   → カードに ✓ + パスが表示
5. 「監査を開始」ボタンが活性化
6. クリック → Main 画面遷移
```

#### 上級者フロー（プロファイル再利用）

```
1. 起動
2. 「最近使ったプロジェクト」セクションから過去のエントリを選び「開く」
   → Before / After / 監査定義のパスが復元され、即監査開始
```

#### 任意設定の編集フロー

```
1. 「オプション設定」▸ をクリック
   → 折りたたみが展開
2. 監査定義ファイル / .aaaiignore のパスを必要に応じて選択 or 入力
3. 「監査を開始」
```

### 2.4 画面要素仕様

| 要素 | 表示内容 | アクション |
|---|---|---|
| ヘッダー | アプリ名 + サブタイトル + 1 文ガイド | なし |
| 比較元カード | 📁 アイコン + ラベル + 選択状態 + パス | クリックでダイアログ |
| 比較先カード | 同上 | 同上 |
| オプション設定 (折りたたみ) | ▸/▾ + ラベル + 補足文 | クリックで展開/折りたたみ |
| 監査定義入力 (展開時) | ラベル + パス + 選択ボタン | クリックでファイルダイアログ |
| .aaaiignore 入力 (展開時) | 同上 | 同上 |
| 監査を開始ボタン | テキスト | クリックで監査実行 |
| 最近使ったプロジェクトリスト | プロファイル名 + パス + 開くボタン | クリックで一括復元 |

### 2.5 アクセシビリティ

| 観点 | 対応 |
|---|---|
| キーボード操作 | Tab/Shift+Tab でカード→カード→展開→ボタンの順に移動 |
| Enter | フォーカス中のボタンを押す |
| Space | 折りたたみセクションの展開/閉じる |
| 色だけに依存しない | ✓/✗ アイコン + パステキスト + 色 を併用 |
| タップ領域 | 全ボタン `padding ≥ [12, 16]` で ≥ 44px 確保 |

---

## 3. 内部設計（詳細設計）

### 3.1 依存追加

```toml
# crates/aaai-gui/Cargo.toml
rfd = "0.17"
```

`rfd` (Rusty File Dialogs) は iced 0.14 とも互換性のあるクロスプラットフォーム
ネイティブダイアログライブラリ。`AsyncFileDialog::pick_folder()` は `Future<Output = Option<FileHandle>>` を返すため、`iced::Task::perform` でラップして非同期に統合する。

### 3.2 データモデル変更

#### App struct への追加

```rust
pub struct App {
    // 既存のフィールド…
    pub optional_settings_expanded: bool,   // RFC 015
    // 既存の before_path, after_path, definition_path, ignore_path は維持
}
```

#### Message 列挙への追加

```rust
pub enum Message {
    // 既存…

    // RFC 015: Opening 画面用
    PickBeforeFolder,                       // ピッカー起動
    PickAfterFolder,
    PickDefinitionFile,
    PickIgnoreFile,
    BeforeFolderPicked(Option<PathBuf>),    // ピッカー結果
    AfterFolderPicked(Option<PathBuf>),
    DefinitionFilePicked(Option<PathBuf>),
    IgnoreFilePicked(Option<PathBuf>),
    ToggleOptionalSettings,                 // 折りたたみ切替
    OpenRecentProject(String),              // プロファイル名で一括復元
}
```

### 3.3 メッセージフロー

```
[ユーザー] クリック「フォルダを選ぶ」
    ↓
Message::PickBeforeFolder
    ↓ update()
    Task::perform(
      async {
        rfd::AsyncFileDialog::new()
          .set_title("比較元フォルダを選ぶ")
          .pick_folder().await
          .map(|h| h.path().to_path_buf())
      },
      Message::BeforeFolderPicked
    )
    ↓ async await
Message::BeforeFolderPicked(Some(path))
    ↓ update()
    self.before_path = path.display().to_string();
    self.validate_opening();
    ↓
[再レンダリング] カードに ✓ + パス表示
```

### 3.4 ハンドラ仕様

| Message | ハンドラの動作 |
|---|---|
| `PickBeforeFolder` | rfd ダイアログを非同期起動。結果を `BeforeFolderPicked` で受け取る |
| `BeforeFolderPicked(Some(p))` | `before_path = p.display().to_string()`; `validate_opening()` 呼び出し |
| `BeforeFolderPicked(None)` | キャンセルされたので何もしない |
| `ToggleOptionalSettings` | `optional_settings_expanded` を反転 |
| `OpenRecentProject(name)` | プロファイルから 4 つのパスを復元 → `validate_opening()` |

### 3.5 ファイルピッカーの仕様

| 用途 | rfd 呼び出し | フィルター |
|---|---|---|
| Before / After | `pick_folder()` | なし |
| 監査定義 | `pick_file()` | `*.yaml`, `*.yml` |
| .aaaiignore | `pick_file()` | フィルターなし（拡張子なしの場合があるため） |

### 3.6 検証ロジック（既存の `validate_opening()` を拡張）

```rust
pub fn validate_opening(&mut self) {
    // 既存ロジックを維持
    // 追加: 任意フィールドの妥当性チェック（パス存在）
}
```

### 3.7 「監査を開始」ボタンの活性条件

```
活性 ⇔ before_path が有効なディレクトリ ∧ after_path が有効なディレクトリ
        ∧ (definition_path が空 OR 有効なファイル)
        ∧ (ignore_path     が空 OR 有効なファイル)
        ∧ is_loading == false
```

---

## 4. プログラム設計

### 4.1 `views/opening.rs` 関数構造

```
pub fn view(app: &App) -> Element
├── welcome_section()                              # ヘッダー（タイトル + ガイド文）
├── required_folders_section(app: &App)            # 必須 2 カード
│   ├── folder_picker_card(label, status, msg)
│   └── folder_picker_card(label, status, msg)
├── optional_settings_section(app: &App)           # 折りたたみ
│   └── if expanded:
│       ├── file_picker_row(label, value, msg)    # audit.yaml
│       └── file_picker_row(label, value, msg)    # .aaaiignore
├── start_audit_button(app: &App)                  # 中央配置
└── recent_projects_section(app: &App)             # プロファイルリスト
    └── recent_project_row(name, paths, msg) ...
```

### 4.2 app.rs の関連箇所

| 箇所 | 変更内容 |
|---|---|
| `App` struct | `optional_settings_expanded: bool` 追加 |
| `App::default()` | `optional_settings_expanded: false` |
| `Message` enum | 8 つの新メッセージ追加（3.2 節） |
| `App::update()` | 8 つの match arm 追加 |

### 4.3 i18n キー追加（RFC 016 完了後に行う）

```yaml
opening:
  guide:            "監査するための 2 つのフォルダを選んでください"
  before_card:      "比較元 (Before)"
  after_card:       "比較先 (After)"
  unselected:       "未選択"
  pick_folder:      "フォルダを選ぶ"
  change_folder:    "フォルダを変更"
  optional_section: "オプション設定（監査定義ファイル / .aaaiignore）"
  optional_hint:    "折りたたみ。指定しない場合は新規作成として開始されます"
  definition_label: "監査定義ファイル"
  ignore_label:     ".aaaiignore"
  pick_file:        "ファイルを選ぶ"
  recent_section:   "または最近使ったプロジェクト"
  open_recent:      "開く"
```

### 4.4 テスト計画

| 項目 | 検証方法 |
|---|---|
| folder picker の dialog 起動 | 手動テスト（rfd は単体テスト困難） |
| キャンセル時に状態が変わらない | 手動テスト |
| 必須 2 つで活性化 | `validate_opening()` のユニットテストを追加 |
| プロファイル復元 | 既存テスト + 統合シナリオ |
| i18n キー存在 | RFC 016 のキー検証スクリプト |

---

## 5. 依存・前提

- **RFC 016 (i18n 修復)** が先行完了していないと UI 文字列がリテラル表示される
- `rfd` v0.17 を新規依存に追加

## 6. オープン事項

- ドラッグ＆ドロップ受付は FR-8 (任意) として将来 RFC に分離
- 最近使ったプロジェクトの並び順は「最終使用日時順」とするが、これは既存プロファイル機能の改修要否を別途検討
- 監査定義ファイルが指定されない場合、内部で空の AuditDefinition を生成する挙動は既に存在するため変更なし
