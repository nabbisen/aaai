# CI/CD 統合

aaai は、予測可能な終了コードと機械可読な出力を備え、  
CI/CD パイプラインでの利用を想定して設計されています。

---

## 終了コード

| コード | 意味 |
|---|---|
| 0 | PASSED — 全エントリ OK または Ignored |
| 1 | FAILED — 1 件以上の監査失敗 |
| 2 | PENDING — 未承認エントリあり（`--allow-pending` で 0 に） |
| 3 | ERROR — ファイル読み取りエラー |
| 4 | CONFIG_ERROR — 定義ファイルの構文エラー |

---

## GitHub Actions の例

```yaml
- name: リリース成果物を監査する
  run: |
    aaai audit \
      --left ./dist-before \
      --right ./dist-after \
      --config ./audit/release.yaml \
      --no-history
```

---

## SARIF アノテーション

SARIF 出力を生成して GitHub のプルリクエストにインライン注釈を表示できます。

```yaml
- name: aaai 監査を実行
  run: |
    aaai report \
      --left ./before \
      --right ./after \
      --config ./audit.yaml \
      --format sarif \
      --out results.sarif

- name: SARIF をアップロード
  uses: github/codeql-action/upload-sarif@v3
  with:
    sarif_file: results.sarif
```

---

## ドラフトモードでの Pending 許容

初期セットアップ時など、エントリが Pending でも CI を通したい場合:

```sh
aaai audit --left ./before --right ./after \
           --config ./audit.yaml \
           --allow-pending --no-history
```

---

## ローカル開発向け Watch モード

```sh
aaai watch --left ./before --right ./after --config ./audit.yaml
```

---

## プロジェクト設定 (.aaai.yaml)

リポジトリルートに `.aaai.yaml` を置くとフラグの繰り返しを省略できます。

```yaml
version: "1"
default_definition: "audit/audit.yaml"
default_ignore: "audit/.aaaiignore"
approver_name: "ci-bot"
mask_secrets: true
suppress_warnings:
  - no-approver
```

---

## 警告の抑制

```sh
# コマンドラインで抑制
aaai audit --left ./before --right ./after --config ./audit.yaml \
           --suppress-warnings no-approver,no-strategy
```

---

## シェル補完のインストール

```sh
# Bash
aaai completions bash >> ~/.bash_completion

# Zsh
aaai completions zsh > ~/.zfunc/_aaai

# Fish
aaai completions fish > ~/.config/fish/completions/aaai.fish
```
