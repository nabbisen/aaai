# UI/UX テスト仕様書

本ドキュメントは `aaai-gui` と `aaai` CLI の手動テストケースを定義します。  
リリース前検証を行うテスター向けです。

CLI テストは自動実行できます。

```sh
cargo test -p aaai-cli -- --test-threads=1
```

以下の GUI テストケースは手動で実行してください。

---

## 環境セットアップ

```sh
cargo build --release -p aaai-cli -p aaai-gui

# テスト用ディレクトリを準備
mkdir -p /tmp/aaai-test/{before,after}
echo 'port = 80'   > /tmp/aaai-test/before/config.toml
echo 'port = 8080' > /tmp/aaai-test/after/config.toml
echo 'v1' > /tmp/aaai-test/before/version.txt
echo 'v2' > /tmp/aaai-test/after/version.txt

aaai snap --left /tmp/aaai-test/before --right /tmp/aaai-test/after \
          --out /tmp/aaai-test/audit.yaml
```

---

## 1. Opening 画面

| # | 操作 | 期待結果 |
|---|---|---|
| 1-1 | `aaai-gui` を起動 | Opening 画面が表示される。エラーなし |
| 1-2 | 全フィールドが空の状態で「監査を開始」を押す | ボタンが無効化されているか検証エラーが表示される |
| 1-3 | 存在しないパスを入力して「監査を開始」を押す | エラーメッセージが表示される |
| 1-4 | Before / After のみ入力し定義ファイルなしで開始 | 空の定義で監査が実行され、全エントリが Pending |
| 1-5 | 有効なパスを全て入力して「監査を開始」を押す | ローディングスピナーが表示された後、メイン画面が開く |
| 1-6 | `.aaaiignore` のパスを入力して開始 | 除外されたファイルがファイルツリーに表示されない |
| 1-7 | プロファイルを保存して再読み込み | フィールドが正しく復元される |

---

## 2. メイン画面 — ファイルツリー

| # | 操作 | 期待結果 |
|---|---|---|
| 2-1 | メイン画面を開く | ファイルツリーに変更ファイルが表示され、ダッシュボードが見える |
| 2-2 | 「変更のみ」フィルターをクリック | Unchanged エントリが非表示になる |
| 2-3 | 「未承認のみ」フィルターをクリック | Pending エントリのみ表示される |
| 2-4 | 検索バーにパスの一部を入力 | ファイルツリーがリアルタイムで絞り込まれる |
| 2-5 | ディレクトリヘッダー（▼/▶）をクリック | 配下エントリが折りたたみ / 展開される |
| 2-6 | Pending エントリを選択 | 差分ビューアとインスペクターが更新される |
| 2-7 | ↓ キーを押す | 次のエントリが選択される |
| 2-8 | ↑ キーを押す | 前のエントリが選択される |
| 2-9 | `AuditWarning` があるエントリ | 行に `⚠N` バッジが表示される |

---

## 3. 差分ビューア

| # | 操作 | 期待結果 |
|---|---|---|
| 3-1 | Modified のテキストファイルを選択 | +/− の色分けで左右並列差分が表示される |
| 3-2 | Added ファイルを選択 | 右ペインにコンテンツ、左ペインは空 |
| 3-3 | Removed ファイルを選択 | 左ペインにコンテンツ、右ペインは空 |
| 3-4 | バイナリファイルを選択 | SHA-256 ハッシュを含むバイナリパネルが表示される |
| 3-5 | Modified ファイルを選択 | 統計バーに `+N lines` / `−N lines` が表示される |
| 3-6 | ファイルを選択しない（ダッシュボード状態） | サマリーカードと要注意リストが表示される |

---

## 4. インスペクター

| # | 操作 | 期待結果 |
|---|---|---|
| 4-1 | エントリを選択 | パス・差分種別・ステータスバッジが表示される |
| 4-2 | `AuditWarning` があるエントリ | divider 下に黄色の警告ブロックが表示される |
| 4-3 | 理由を空欄のまま「承認して適用」を押す | ボタンが無効または検証エラーが表示される |
| 4-4 | 理由を入力して「承認して適用」を押す | エントリが OK に変わり、ファイルツリーのバッジが更新される |
| 4-5 | ticket と approved_by を入力して承認 | 値が定義 YAML に保存される |
| 4-6 | expires_at に不正な形式を入力 | 検証エラーが表示される |
| 4-7 | テンプレートを適用 | 戦略フィールドがテンプレートの値で埋まる |
| 4-8 | 承認後に Ctrl+Z を押す | 最後の承認が取り消され、エントリが Pending に戻る |

---

## 5. 保存と再実行

| # | 操作 | 期待結果 |
|---|---|---|
| 5-1 | エントリを承認 | フッターに「未保存の変更があります」が表示される |
| 5-2 | Ctrl+S を押す | 定義ファイルが保存され、未保存インジケーターが消える |
| 5-3 | Ctrl+R を押す | 監査が再実行され、結果が更新される |
| 5-4 | Before / After ファイルを外部で変更して Ctrl+R | 更新された差分が表示される |

---

## 6. テーマとフッター

| # | 操作 | 期待結果 |
|---|---|---|
| 6-1 | フッターで Dark テーマに切り替え | UI が即座にダークパレットに切り替わる |
| 6-2 | `aaai-gui` を再起動 | 前回のテーマが復元される |
| 6-3 | 言語を English に切り替え | ラベルが英語に変わる |
| 6-4 | メイン画面のフッター | ショートカット凡例（`Ctrl+S`, `Ctrl+R` など）が表示されている |

---

## 7. エクスポートとレポート

| # | 操作 | 期待結果 |
|---|---|---|
| 7-1 | 「Export MD」ボタンを押す | カレントディレクトリに `aaai-report.md` が作成される |
| 7-2 | 「Export JSON」ボタンを押す | `aaai-report.json` が作成され、有効な JSON |
| 7-3 | `aaai report --format html` | サマリーカード付きの有効な HTML |
| 7-4 | `aaai report --format sarif` | SARIF 2.1.0 準拠の有効な JSON |

---

## 8. CLI スモークテスト

```sh
# 監査 — Pending のため exit 2
aaai audit --left /tmp/aaai-test/before --right /tmp/aaai-test/after \
           --config /tmp/aaai-test/audit.yaml --no-history

# reason を記入して再実行 — PASSED で exit 0
sed -i 's/reason: .*/reason: "テスト変更"/' /tmp/aaai-test/audit.yaml
aaai audit --left /tmp/aaai-test/before --right /tmp/aaai-test/after \
           --config /tmp/aaai-test/audit.yaml --no-history

# ダッシュボード
aaai dashboard --left /tmp/aaai-test/before --right /tmp/aaai-test/after \
               --config /tmp/aaai-test/audit.yaml

# リント
aaai lint /tmp/aaai-test/audit.yaml
# 期待: exit 0、"No issues found"

# 履歴統計（過去の実行履歴が必要）
aaai history --stats
```

---

## 9. リリース受け入れ基準

以下をすべて満たした時点でリリース可能です。

- [ ] Opening 画面ケース（1-1 〜 1-7）全パス
- [ ] ファイルツリーケース（2-1 〜 2-9）全パス
- [ ] 差分ビューアケース（3-1 〜 3-6）全パス
- [ ] インスペクターケース（4-1 〜 4-8）全パス
- [ ] 保存/再実行ケース（5-1 〜 5-4）全パス
- [ ] テーマケース（6-1 〜 6-4）全パス
- [ ] エクスポートケース（7-1 〜 7-4）全パス
- [ ] CLI スモークテスト全コマンドが期待通りの終了コードで終了
- [ ] `cargo check --all-targets` 警告ゼロ
- [ ] `cargo test -p aaai-core --lib` — 92 件パス
- [ ] `cargo test -p aaai-cli -- --test-threads=1` — 30 件パス
