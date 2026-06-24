# Windows — インストールと設定

aaai は Microsoft Store からインストールできるほか、GitHub Releases から
直接ダウンロードすることもできます。

---

## Microsoft Store からのインストール

1. Windows の Microsoft Store を開く。
2. **aaai** を検索する。
3. **入手** または **インストール** をクリックする。

インストール後に利用できるもの:

- **aaai** がスタートメニューに表示される — クリックするとデスクトップアプリが起動します。
- **aaai** がターミナルコマンドとして使える — ターミナル・PowerShell・コマンドプロンプトのいずれでも使用できます。

---

## デスクトップアプリ

スタートメニューから **aaai** を開きます。フォルダ選択画面が表示されます。
比較したい古いフォルダと新しいフォルダを選んで **変更をチェック** をクリックします。

詳細な手順は [はじめに](getting-started.md) を参照してください。

---

## ターミナルコマンド

Store からインストールすると、PATH を変更せずに任意のターミナルセッションで
`aaai` コマンドが使えます（Windows App Execution Alias による機能）。

コマンドが使えるか確認するには:

```powershell
aaai --help
```

インストール直後にコマンドが見つからない場合は、新しいターミナルウィンドウを開いて再度試してください。

### 基本的な CLI の使い方

```sh
# 現在の差分から確認用テンプレートを生成
aaai snap --left .\before --right .\after --out audit.yaml

# 既存の定義ファイルを使って確認を実行
aaai audit --left .\before --right .\after --config audit.yaml
```

コマンドの全一覧は [CLI リファレンス](cli.md) を参照してください。

---

## 直接ダウンロード（GitHub Releases）

Microsoft Store を使わない場合は
[GitHub Releases](https://github.com/nabbisen/aaai/releases) から
リリースアーカイブを直接ダウンロードできます。

リリースごとに 3 種類の Windows アーカイブが提供されます:

| アーカイブ | 内容 |
|---|---|
| `aaai-cli-v{version}-x86_64-pc-windows-msvc.zip` | `aaai.exe` のみ |
| `aaai-gui-v{version}-x86_64-pc-windows-msvc.zip` | `aaai-gui.exe` のみ |
| `aaai-full-v{version}-x86_64-pc-windows-msvc.zip` | 両方の実行ファイル |

任意のフォルダに展開してください。ターミナルから `aaai.exe` を使うには、
そのフォルダを `PATH` に追加してください。

---

## パッケージの構成

aaai は **1 つの Store 製品** として **2 つの実行ファイル** を含む形で提供されます:

| バイナリ | 役割 |
|---|---|
| `aaai-gui.exe` | デスクトップレビューアプリ |
| `aaai.exe` | コマンドラインインターフェース |

CLI は別の Store 製品ではありません。同じ製品の高度な機能として、
ターミナルエイリアス経由で提供されます。

---

## 動作環境

- Windows 10 バージョン 1803 以降（App Execution Alias のサポートに必要）
- x64 プロセッサ（ARM64 サポートは予定中）
