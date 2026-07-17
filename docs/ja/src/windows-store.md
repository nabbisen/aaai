# Windows — インストールと設定

現在サポートされている Windows でのインストール方法は、aaai を
ソースからビルドする方法です。

> **配布状況:** Microsoft Store / MSIX での配布は延期されており、
> 現在は利用できません。非公式の Store 掲載を検索したり、
> インストールしたりしないでください。

## ソースからビルド

[Git](https://git-scm.com/download/win) と
[Rust 1.91 以降](https://rustup.rs/) をインストールし、次を実行します:

```powershell
git clone https://github.com/nabbisen/aaai.git
cd aaai
cargo build --release -p aaai-cli -p aaai-gui
```

ビルドにより `target\release\aaai.exe` と
`target\release\aaai-gui.exe` が生成されます。

---

## 予定されている直接ダウンロード（v1 目標）

次の Windows 用アーカイブは、C1/R1 リリースゲートの達成を条件とする
v1 の予定成果物です。現在は
[GitHub Releases](https://github.com/nabbisen/aaai/releases) から
利用できません。

| アーカイブ | 内容 |
|---|---|
| `aaai-cli-v{version}-x86_64-pc-windows-msvc.zip` | `aaai.exe` のみ |
| `aaai-gui-v{version}-x86_64-pc-windows-msvc.zip` | `aaai-gui.exe` のみ |
| `aaai-full-v{version}-x86_64-pc-windows-msvc.zip` | 両方の実行ファイル |

これらが利用可能になった後は、任意のフォルダに展開して使用します。

---

## デスクトップアプリ

ソースからビルドした後、`target\release\aaai-gui.exe` を実行します。
フォルダ選択画面が表示されます。比較したい古いフォルダと新しい
フォルダを選んで **変更をチェック** をクリックします。

将来の直接配布アーカイブでも、展開先のフォルダに同じ実行ファイルが
含まれる予定です。

詳細な手順は [はじめに](getting-started.md) を参照してください。

---

## ターミナルコマンド

ソースからビルドした後、`target\release\aaai.exe` を実行します。
任意のターミナルで `aaai` を使うには、`target\release` を `PATH` に
追加してください。将来の直接配布アーカイブでも、展開先のフォルダに
同じ実行ファイルが含まれ、そのフォルダを同様に `PATH` に追加できます。

### 基本的な CLI の使い方

```powershell
# 現在の差分から確認用テンプレートを生成
.\target\release\aaai.exe snap --left .\before --right .\after --out audit.yaml

# 既存の定義ファイルを使って確認を実行
.\target\release\aaai.exe audit --left .\before --right .\after --config audit.yaml
```

コマンドの全一覧は [CLI リファレンス](cli.md) を参照してください。

---

## 延期された Microsoft Store パッケージモデル

RFC 091 では、将来の設計として **1 つの Store 製品** に **2 つの
実行ファイル** を含めるモデルを維持しています:

| バイナリ | 役割 |
|---|---|
| `aaai-gui.exe` | デスクトップレビューアプリ |
| `aaai.exe` | コマンドラインインターフェース |

将来 Store 配布を実装する場合、GUI を表示アプリとし、CLI は同じ製品の
高度な機能として扱います。このモデルは設計指針にすぎません。
現在、Store 掲載、MSIX インストール、ターミナルエイリアスは
サポートされていません。

---

## 動作環境

- x64 Windows
- Git
- Rust 1.91 以降
- ARM64 パッケージは延期されています
