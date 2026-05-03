# はじめに

## インストール

```sh
# ソースからビルド（Rust 1.81 以降が必要）
cargo build --release -p aaai-cli -p aaai-gui

# バイナリを PATH に追加（例）
cp target/release/aaai     ~/.local/bin/
cp target/release/aaai-gui ~/.local/bin/
```

---

## 初回セットアップ（推奨: `aaai init`）

新しいプロジェクトでは `aaai init` が最も簡単な出発点です。

```sh
cd /your/project
aaai init
```

対話的に以下を設定できます。

- Before / After フォルダパス
- 監査定義ファイルの場所
- 承認者名
- 初回スナップショットの自動生成

`--non-interactive` フラグで CI / スクリプトから無人実行できます。

```sh
aaai init --non-interactive --dir /path/to/project
```

---

## 手動セットアップ（ステップバイステップ）

### 1. 差分テンプレートを生成する

```sh
aaai snap --left ./before --right ./after --out audit.yaml
```

生成された `audit.yaml` の各エントリに `reason` フィールドを記入します。  
空欄のまま監査を実行すると Pending 扱いになります。

### 2. 監査を実行する

```sh
aaai audit --left ./before --right ./after --config audit.yaml
```

| 終了コード | 意味 |
|---|---|
| 0 | **PASSED** — 全エントリが期待通り |
| 1 | **FAILED** — ルール不一致のエントリがある |
| 2 | **PENDING** — reason 未記入のエントリがある（`--allow-pending` で続行可） |
| 3 | **ERROR** — ファイル読み取りエラー |
| 4 | **CONFIG_ERROR** — 定義ファイルの構文エラー |

### 3. 問題を確認して修正する

```sh
# 差分の詳細を確認
aaai diff --left ./before --right ./after --content

# ベストプラクティスチェック
aaai lint audit.yaml
```

### 4. レポートを出力する

```sh
# Markdown レポート
aaai report --left ./before --right ./after --config audit.yaml --out report.md

# HTML レポート（ブラウザで確認可能）
aaai report --left ./before --right ./after --config audit.yaml \
            --format html --out report.html
```

---

## GUI を使う

```sh
aaai-gui
```

Opening 画面で Before / After / 定義ファイルを指定して「監査を開始」をクリックします。  
詳しくは [GUI ガイド](gui.md) を参照してください。

---

## .aaai.yaml でデフォルト設定

プロジェクトルートに `.aaai.yaml` を置くと、よく使うパスと設定を省略できます。

```yaml
version: "1"
default_definition: "audit/audit.yaml"
default_ignore: "audit/.aaaiignore"
approver_name: "your-name"
mask_secrets: true
```

```sh
# 初期テンプレート生成
aaai config --init
```

---

## シェル補完のインストール

```sh
# Zsh の例
aaai completions zsh > ~/.zfunc/_aaai
echo 'fpath=(~/.zfunc $fpath)' >> ~/.zshrc
source ~/.zshrc
```

---

## 次のステップ

- [CLI リファレンス](cli.md) — 全コマンドの詳細
- [監査定義ファイル](audit-definition.md) — YAML フォーマット
- [内容監査戦略](strategies.md) — None / Checksum / LineMatch / Regex / Exact
- [CI/CD 統合](ci-integration.md) — GitHub Actions での使い方
- [GUI ガイド](gui.md) — 3 ペイン画面の操作方法
- [FAQ](faq.md) — よくある質問
