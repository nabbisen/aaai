# RFC 019 — Documentation Refresh for v0.15–v0.19 Realities

**Status.** Implemented (v0.20.0)
**Priority.** v1.0 blocker
**Tracks.** 設計書 全ページ / docs/src/gui.md・docs/ja/src/gui.md の現実同期
**Touches.** `docs/src/gui.md` · `docs/ja/src/gui.md` · `docs/src/testing.md` · `docs/ja/src/testing.md` · `README.md` · `crates/aaai-gui/README.md`

---

## 1. 要件定義

### 1.1 目的

RFC 007〜016 で実装された GUI の現実と、`docs/src/gui.md` および
`docs/ja/src/gui.md` の記述が大きく乖離している。読者を惑わせるドキュメント
記述を、v0.20.0 出荷時点の現実と一致させる。

### 1.2 確認された乖離

| 章 | 現状の記述 (古い) | 現実 (v0.19.0) | 影響 |
|---|---|---|---|
| 2-1. ファイルツリー | "+ 緑 / - 赤 / ~ 黄" のテキストバッジ | 行頭=`status_icon (✓⚠✗!—)`, 右端=`diff_type_tag (+−~T)` | 利用者が画面と一致しない凡例を学ぶ |
| 2-1. バッチ選択 | "「Batch Approve」ボタンで一括承認" | RFC 007 で削除済み | 存在しないボタンを探させる |
| 2-3. 差分ビューア | 「左右並列の差分」のみ | タブ (`左右差分 / 統合 / 変更のみ`) | RFC 011 が見えない |
| 2-3. 差分ハイライト | (未記載) | 凡例 (Removed / Added) を下部に表示 | RFC 010 が見えない |
| 2-4. インスペクター | 「承認して適用」 | 「承認して保存」 (RFC 008) | 文言不一致 |
| 2-4. インスペクター: 理由 | 単行入力との認識を促す表現 | 複数行 textarea (RFC 009) | 入力イメージ不一致 |
| 2-4. インスペクター: LineMatch | (未記載) | 色付きコードブロック表示 (RFC 012) | RFC 012 が見えない |
| 5. レポート出力 | "Export MD / Export JSON ボタン" | 単一の「レポート出力」ボタン (RFC 007/014) | 存在しないボタン |
| ボトムバー | (未記載) | 「承認して保存」「選択中」「未解決件数」(RFC 008) | RFC 008 が見えない |
| Opening 画面 | "4 つのテキストボックスにパス入力" | 「フォルダを選ぶ」OS ネイティブダイアログ (RFC 015) | RFC 015 が見えない |

### 1.3 機能要件

| ID | 要件 | 必須/任意 |
|---|---|---|
| FR-1 | docs/src/gui.md が v0.20.0 の現実と整合する | 必須 |
| FR-2 | docs/ja/src/gui.md が同等内容を日本語で提供する | 必須 |
| FR-3 | docs/src/testing.md / docs/ja/src/testing.md のテストケースに RFC 007〜016 の検証手順を含める | 必須 |
| FR-4 | README.md トップに記載されている機能リストが現実と一致する | 必須 |
| FR-5 | crates/aaai-gui/README.md の機能一覧も同期する | 必須 |
| FR-6 | 設計書 p.X への明示的な参照を docs/src/gui.md に追加する | 任意 (推奨) |

### 1.4 非機能要件

| ID | 要件 |
|---|---|
| NFR-1 | 英語版と日本語版は構造（目次・章番号）を一致させる |
| NFR-2 | 表現は冗長を避け、設計書原則「ミニマル & フォーカス」(PDF p.3) に準拠 |
| NFR-3 | スクリーンショットを使う場合は実機キャプチャ。架空イラストは使用しない |

---

## 2. 外部設計（基本設計）

### 2.1 docs/src/gui.md の新章構成（提案）

```
1. Opening 画面
   1-1. 必須 2 フォルダの選択
   1-2. オプション設定（折りたたみ）
   1-3. 最近使ったプロジェクト
   1-4. 「監査を開始」ボタンの活性条件
2. メイン画面（3 ペイン）
   2-1. ツールバー（開く / 保存 / 監査実行 / レポート出力 + ステータス）
   2-2. フィルター・検索バー
   2-3. ファイルツリー（行頭=status_icon / 右端=diff_type_tag）
   2-4. 差分ビューア（タブ: 左右差分 / 統合 / 変更のみ + 凡例）
   2-5. インスペクター（理由必須・戦略別フォーム・LineMatch 色ブロック）
   2-6. ボトムアクションバー（承認して保存 / 選択中 / 未解決件数）
3. 状態モデル
   3-1. ステータス (OK / Pending / Failed / Error / Ignored)
   3-2. 表示・色・記号 (ABDD 準拠)
4. キーボードショートカット
5. 言語切替
6. レポート出力
7. 典型的なワークフロー（Opening → 監査 → 確認 → 承認 → 保存 → 再監査 → 出力）
8. 設計書との対応
```

### 2.2 README.md の機能テーブル更新

| 既存表記 | 修正案 |
|---|---|
| `GUI \| 3-pane resizable desktop UI with dark/light theme` | `GUI \| 3-pane main screen with toolbar / bottom action bar; resizable panes; dark/light theme; ABDD-compliant` |

### 2.3 docs/src/testing.md への追加項目

| 既存 | 追加 |
|---|---|
| 2. Main Screen — File Tree | 行頭アイコン (✓⚠✗!—) と右端タグ (+−~T) の確認手順 |
| 3. Diff Viewer | タブ切替 (左右差分 / 統合 / 変更のみ) の確認手順 / 凡例の有無 |
| 4. Inspector | reason textarea が複数行であることの確認 / LineMatch 色ブロック表示の確認 |
| 5. Save / Re-run | 「承認して保存」がボトムバーにあることの確認 |

---

## 3. 内部設計（詳細設計）

### 3.1 ドキュメント更新の作業単位

| 単位 | 推定差分 (行) |
|---|---|
| docs/src/gui.md 全面書き直し | 250 行（既存約 130 行 → 約 250 行） |
| docs/ja/src/gui.md 同上 | 250 行 |
| docs/src/testing.md 章 2-5 追記 | 30 行 |
| docs/ja/src/testing.md 同上 | 30 行 |
| README.md 機能テーブル微調整 | 5 行 |
| crates/aaai-gui/README.md 機能リスト追記 | 10 行 |

### 3.2 設計書ページ参照の貼付方針

各章末に「**設計書参照**: p.5 / p.7 / p.8」のような短い注記を入れる。
読者が設計書 PDF を持っている前提で、深堀りしたい場合の動線となる。

### 3.3 旧記述のクリーンアップ

以下の語句を全削除または書き換え:

- `Batch Approve`（廃止済み）
- `Export MD` / `Export JSON`（単一の「レポート出力」に統合）
- `承認して適用`（→「承認して保存」）
- ファイルツリーの "+ 緑 / - 赤 / ~ 黄" 凡例（→ status_icon / diff_type_tag 二段）

### 3.4 多言語整合性チェック

英語版・日本語版で章番号・章タイトル・図表の数が一致することを目視確認する。
必要に応じて簡易スクリプト (`diff <(grep -c '^##' docs/src/gui.md) <(grep -c '^##' docs/ja/src/gui.md)`) で
章数を確認。

---

## 4. プログラム設計

### 4.1 実装手順

| Step | 作業 | 検証 |
|---|---|---|
| 1 | docs/src/gui.md を新章構成で全面書き直し | mdbook build が通る |
| 2 | docs/ja/src/gui.md を同等構造で書き直し | mdbook build (ja) が通る |
| 3 | docs/src/testing.md に RFC 007〜016 のテストケースを追記 | 既存番号との重複なし |
| 4 | docs/ja/src/testing.md 同上 | — |
| 5 | README.md の機能テーブル更新 | レビュー |
| 6 | crates/aaai-gui/README.md の機能一覧更新 | レビュー |
| 7 | mdbook build を実行し、両 SUMMARY.md からの参照が壊れていないこと | エラーなし |

### 4.2 影響範囲

- Rust ソースコード: なし
- ドキュメント: 上記 6 ファイル
- ユーザー機能: 変更なし（説明の精度向上のみ）

### 4.3 リスク

| リスク | 対策 |
|---|---|
| 視覚検証 (RFC 017) の前に書き直すと、内容自体が誤りである可能性 | RFC 017 の P0 検証完了後に着手する |
| 英語版と日本語版で章構造が分かれる | 章追加・削除は両言語で同じコミット内に行う |
| README.md と docs/src/gui.md で重複説明が増える | README は要点 4〜5 行、詳細は docs に寄せる |

---

## 5. 完了条件

- [ ] docs/src/gui.md が v0.20.0 の現実を反映している（RFC 007〜016 のすべてを記述）
- [ ] docs/ja/src/gui.md が英語版と章構造一致
- [ ] docs/src/testing.md と docs/ja/src/testing.md が RFC 007〜016 のテストケースを含む
- [ ] README.md の機能テーブルが現状を反映
- [ ] crates/aaai-gui/README.md の機能一覧が現状を反映
- [ ] mdbook build が両言語で成功する
- [ ] 旧用語 (Batch Approve / Export MD,JSON / 承認して適用 / 単行 reason 等) が
      ドキュメント全体から検索しても 0 件

## 6. 依存

- **RFC 017** (Visual Verification Harness): 視覚検証の結果が docs の根拠となるため、
  P0 検証完了後に着手するのが望ましい

## 7. 後続への影響

- 将来の RFC 020 以降で UI を変更した場合は、docs/src/gui.md と docs/ja/src/gui.md を
  当該 RFC のスコープ内で更新することを「完了条件」に含める運用とする
