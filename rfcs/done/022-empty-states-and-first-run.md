# RFC 022 — Empty State Guidance & First-Run UX

**Status.** Implemented (v0.20.0)
**Priority.** v1.0 改善
**Tracks.** 設計書 p.2「中心体験」前段 / p.8「初めての人が怖くない」/ p.10 A. 初期画面
**Touches.** `crates/aaai-gui/src/views/main_view.rs` (file_tree placeholder / diff panel placeholder / inspector panel placeholder) · `views/dashboard.rs`

---

## 1. 要件定義

### 1.1 目的

初回利用者または audit_result 不在状態のユーザーに対し、**「次にすべき操作」を画面上で
明示する**。設計書 p.8 「初めての人が怖くない」を満たし、p.10 A. の「最近の組み合わせ再利用」
を最大限活用する。

### 1.2 解決すべき問題

| 問題 | 現状 |
|---|---|
| ファイルツリーが「No audit result. Press "Re-run".」と表示するだけで、何をするか不明 | placeholder text のみ |
| 差分ビューアの空状態が "No data. Start an audit from the Opening screen." とだけ | 同上 |
| インスペクターの空状態が "Select a file to inspect." のみ | 同上 |
| 初回起動時の Opening 画面に「最近使った」が空のとき、何も訴求しない | recent_projects は空配列で hidden |
| 初回起動者が「サンプルで試してみたい」とき、手元にフォルダを用意していないとつまずく | 例示・サンプル動線なし |

### 1.3 機能要件

| ID | 要件 | 必須/任意 |
|---|---|---|
| FR-1 | audit_result 不在時の Main 画面各ペインに、視覚的に区別された空状態 (illustration なし、テキスト + 矢印で誘導) を表示 | 必須 |
| FR-2 | Opening 画面で「最近使った」が空のとき、3〜4 行のオンボーディングテキストを表示 | 必須 |
| FR-3 | オンボーディングテキストには、「Before/After フォルダ選択 → 監査を開始」の流れを 3 ステップ程度で示す | 必須 |
| FR-4 | 「サンプルプロジェクトを試す」(任意・拡張) — `~/.aaai/sample/` を生成し、それを Before/After として読み込む | 任意 (v1.1 以降検討) |
| FR-5 | 空状態の表示は en / ja 両言語対応 | 必須 |

### 1.4 非機能要件

| ID | 要件 |
|---|---|
| NFR-1 | 既存ユーザーには冗長にならないよう、audit_result が存在する場合は空状態テキストを一切出さない |
| NFR-2 | 空状態のテキストは長文化させない (3〜4 行以内) |
| NFR-3 | ABDD: 矢印やアイコンは記号のみ・色だけに依存しない |

---

## 2. 外部設計（基本設計）

### 2.1 Opening 画面の初回オンボーディング

最近使った (recent_projects.profiles) が空のとき、画面下部に以下を追加:

```
┌──────────────────────────────────────────────────────────┐
│  はじめての方へ                                            │
│                                                          │
│  1. 「比較元 (Before)」を選ぶ — 監査の基準となるフォルダ    │
│  2. 「比較先 (After)」を選ぶ — 監査対象のフォルダ           │
│  3. 「監査を開始」をクリック                                │
│                                                          │
│  audit.yaml は監査開始時に自動的に作成されます。           │
└──────────────────────────────────────────────────────────┘
```

### 2.2 Main 画面のファイルツリー空状態

```
                                ┌───┐
                                │ ↑ │  (上向き矢印)
                                └───┘
                          「監査実行」を押すと
                          差分一覧がここに表示されます
```

ABDD: 上向き矢印は文字列「↑」(または iced font icon)。色だけに依存しない。

### 2.3 Main 画面の差分ビューア空状態

ダッシュボード (`dashboard::view`) は audit_result が **存在する** ときの表示なので、
audit_result が **無い** ときの差分ビューアにも別途空状態が必要:

```
        ┌──────────────────────────────┐
        │                              │
        │   監査結果がありません         │
        │                              │
        │   ① ツールバーの             │
        │      「▶ 監査実行」を押す     │
        │   ② または                   │
        │      「□ 開く」から          │
        │      新しい監査を開始         │
        │                              │
        └──────────────────────────────┘
```

### 2.4 インスペクター空状態

```
        ┌──────────────────────────────┐
        │                              │
        │   ファイルを選んでください     │
        │                              │
        │   ← 左のファイル一覧から       │
        │      確認したい項目を選択      │
        │                              │
        └──────────────────────────────┘
```

---

## 3. 内部設計（詳細設計）

### 3.1 i18n キー追加

```yaml
# en.yaml
empty_state:
  onboarding_title: "Getting started"
  onboarding_step1: "Pick the 'Before' folder — the audit baseline"
  onboarding_step2: "Pick the 'After' folder — what you want to audit"
  onboarding_step3: "Click 'Start Audit'"
  onboarding_note: "An audit.yaml file is created automatically when you start."
  file_tree_no_result: "Click '▶ Run audit' to see file differences here"
  diff_no_audit_title: "No audit result yet"
  diff_no_audit_step1: "Click '▶ Run audit' in the toolbar"
  diff_no_audit_step2: "Or click '□ Open' to start a new project"
  inspector_no_selection: "Pick a file to inspect"
  inspector_no_selection_hint: "Choose an entry from the file tree on the left"

# ja.yaml (同様に翻訳)
```

### 3.2 view 側の挿入位置

```rust
// opening.rs::view()
// recent_projects_section の代わりに以下を分岐:
let recent_or_onboarding: Element<_> = if app.profiles.profiles.is_empty() {
    onboarding_section()
} else {
    recent_projects_section(app)
};
```

```rust
// main_view.rs::build_file_tree()
if app.audit_result.is_none() {
    return file_tree_empty_state(); // 新規 helper
}
// ... 既存 OK ロジック
```

```rust
// main_view.rs::build_diff_panel()
// 既存: audit_result が無い場合 "No data..." text のみ
// → 新規 helper diff_no_audit_state() に置換
```

### 3.3 空状態の共通スタイル

`crates/aaai-gui/src/style.rs` に `empty_state_panel_style` を追加:

- 背景: 透明 / 薄いボーダー破線
- 中央寄せ
- パディング: 24px 以上
- 文字色: 中間グレー (`#777`)

### 3.4 RFC 015 との整合性

RFC 015 の `recent_projects_section()` は profiles が空のとき高さ 0 で消える実装。
本 RFC は当該分岐を「空のときは onboarding_section、空でないときは recent_projects」に
置き換える。

---

## 4. プログラム設計

### 4.1 実装手順

| Step | 作業 | 検証 |
|---|---|---|
| 1 | i18n キー追加 (en/ja) | YAML 構文 OK |
| 2 | `opening.rs` に `onboarding_section()` を追加 | コンパイル |
| 3 | `recent_projects_section` を呼ぶ分岐をリファクタ | 視覚確認 (profiles 空/非空) |
| 4 | `main_view.rs` の 3 ペインにそれぞれ空状態関数を追加 | 視覚確認 |
| 5 | `style.rs` に `empty_state_panel_style` を追加 | レンダリング確認 |
| 6 | `docs/src/gui.md` の Opening 章に「初回オンボーディング」節を追加 | レビュー |

### 4.2 影響範囲

| ファイル | 変更行 |
|---|---|
| `crates/aaai-gui/src/views/opening.rs` | 約 30 行 (onboarding_section) |
| `crates/aaai-gui/src/views/main_view.rs` | 約 40 行 (3 空状態関数) |
| `crates/aaai-gui/src/style.rs` | 約 15 行 |
| `crates/aaai-gui/locales/{en,ja}.yaml` | 約 24 行 |
| `docs/src/gui.md` / `docs/ja/src/gui.md` | 約 15 行 |

### 4.3 リスク

| リスク | 対策 |
|---|---|
| 既存ユーザーが冗長と感じる | profiles が空のときのみ表示。再表示しない |
| 翻訳文が冗長で 4 行を超える | 文字数制限を意識 (1 行 ≤ 30 文字目安) |
| 空状態の表示で「監査実行ボタンがどれ？」が分かりにくい | 矢印 + ボタンと同じアイコン (▶) を表示文字に使う |

---

## 5. 完了条件

- [ ] Opening 画面で profiles 空 → オンボーディング 3 ステップが表示される
- [ ] Opening 画面で profiles 非空 → 既存の Recent 一覧が表示される (オンボーディングは出ない)
- [ ] Main 画面 audit_result 不在 → ファイルツリー / 差分ビューア / インスペクターに
      それぞれ空状態が表示される
- [ ] audit_result 存在 → 空状態テキストはどこにも出ない
- [ ] en / ja 両方で正しく翻訳表示される
- [ ] RFC 017 の Visual Verification で「初回起動者の動線が画面上で明示」と判定される

## 6. 依存

- **RFC 017** (verification harness): 視覚検証用
- **RFC 019** (docs refresh): docs/src/gui.md にオンボーディング節を追記する前提

## 7. 後続検討項目 (本 RFC 範囲外)

- 「サンプルプロジェクトを試す」ボタン (v1.1 以降検討)
- オンボーディングを 1 回で済ます (ローカル設定で次回非表示) — 現状はプロファイル
  保存後は自然に表示されなくなるため、明示的なフラグは不要と判断
