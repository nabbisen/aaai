# Setup & Tooling Commands

Commands for project initialisation, configuration, and shell integration.

## aaai init

対話型プロジェクト初期設定ウィザードです。

```sh
aaai init [--dir <DIR>] [--non-interactive]
```

`--non-interactive` でプロンプトをスキップしてデフォルト値で `.aaai.yaml` を生成します。

---

## aaai config

プロジェクト設定ファイル `.aaai.yaml` を表示または生成します。

```sh
aaai config [--init] [--dir <DIR>] [--show]
```

---

## aaai version

バージョン情報を表示します。

```sh
aaai version [--json-output]
```

---

## aaai completions

シェル補完スクリプトを生成します。

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
