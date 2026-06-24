# セットアップ・ツールコマンド

プロジェクト初期設定・設定管理・シェル統合のためのコマンド。

## aaai init

対話型のプロジェクト初期設定ウィザード。

```sh
aaai init [--dir <DIR>] [--non-interactive]
```

`--non-interactive` でプロンプトをスキップしてデフォルト値で
`.aaai.yaml` を生成する。人間の入力が無い CI/CD スクリプトでの
利用に便利。

---

## aaai config

プロジェクト設定ファイル `.aaai.yaml` を表示・生成する。

```sh
aaai config [--init] [--dir <DIR>] [--show]
```

- `--init` はスターター `.aaai.yaml` を書き出す（既存ファイルが
  あれば上書きせず拒否）。
- `--show` は現在の有効設定を stdout に表示。

---

## aaai version

`aaai` のバージョンを表示。

```sh
aaai version [--json-output]
```

`--json-output` を付けると `version` と `git_commit` を含む JSON
を出力する。CI スクリプトからのパースに便利。

---

## aaai exit-codes

標準終了コード一覧を表示する。

```sh
aaai exit-codes
```

| コード | 意味 |
|---|---|
| 0 | PASSED — 監査完了、すべて許容内 |
| 1 | FAILED — Failed エントリ 1 件以上 |
| 2 | PENDING — レビューが必要な Pending エントリ 1 件以上 |
| 3 | ERROR — 実行時エラーで完了不能 |
| 4 | CONFIG_ERROR — 定義または CLI 引数が無効 |

これらは v1.x で安定 — SemVer 解釈の詳細は
[互換性方針](compatibility.md) を参照。CI スクリプトは
パッチ／マイナーリリースを跨いでこの数値に依拠してよい。

---

## aaai completions

シェル補完スクリプトを生成する。

```sh
aaai completions <bash|zsh|fish|powershell>
```

**インストール例:**

```sh
# Bash
aaai completions bash >> ~/.bash_completion

# Zsh
aaai completions zsh > ~/.zfunc/_aaai
echo 'fpath=(~/.zfunc $fpath)' >> ~/.zshrc

# Fish
aaai completions fish > ~/.config/fish/completions/aaai.fish
```

---

## 次の操作を見つける

すべての aaai サブコマンドの `--help` 出力末尾に「Next steps:」
ブロックがあり、その操作の典型的な次のコマンドを示す。例:

```sh
$ aaai snap --help
...
Next steps:
  Review the generated audit.yaml and fill in reasons.
  Re-run with `aaai audit` to verify.
```

トップレベルの `aaai --help` にも「Getting started:」ブロックが
あり、初心者を `init → snap → audit → report` の流れに導く。
