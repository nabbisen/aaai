# Windows — Installation and Setup

aaai is available for Windows through the Microsoft Store and as direct
download archives from GitHub Releases.

---

## Install from the Microsoft Store

1. Open the Microsoft Store on Windows.
2. Search for **aaai**.
3. Click **Get** or **Install**.

After installation you will have:

- **aaai** in the Start menu — opens the desktop review app.
- **aaai** as a terminal command — available in Terminal, PowerShell,
  and Command Prompt.

---

## Desktop app

Open **aaai** from the Start menu. The desktop app opens to the folder
selection screen. Choose the older and newer folders you want to compare,
then click **Check changes**.

See [Getting Started](getting-started.md) for a full walkthrough.

---

## Terminal command

After installing from the Store, the `aaai` command is available in any
terminal session without changing PATH. This is provided through a Windows
App Execution Alias.

To confirm the command is available:

```powershell
aaai --help
```

If the command is not found immediately after install, open a new Terminal
window and try again.

### Basic CLI usage

```sh
# Generate a review template from the current diff
aaai snap --left .\before --right .\after --out audit.yaml

# Run a review against an existing definition
aaai audit --left .\before --right .\after --config audit.yaml
```

See the [CLI Reference](cli.md) for the full command list.

---

## Direct download (GitHub Releases)

If you prefer not to use the Microsoft Store, download the release archive
directly from [GitHub Releases](https://github.com/nabbisen/aaai/releases).

Three Windows archives are available per release:

| Archive | Contents |
|---|---|
| `aaai-cli-v{version}-x86_64-pc-windows-msvc.zip` | `aaai.exe` only |
| `aaai-gui-v{version}-x86_64-pc-windows-msvc.zip` | `aaai-gui.exe` only |
| `aaai-full-v{version}-x86_64-pc-windows-msvc.zip` | Both executables |

Extract to a folder of your choice. To use `aaai.exe` from any terminal,
add the folder to your `PATH`.

---

## Package model

aaai is shipped as **one Store product** containing **two executables**:

| Binary | Role |
|---|---|
| `aaai-gui.exe` | Desktop review application |
| `aaai.exe` | Command-line interface |

The CLI is not a separate Store product. It is an advanced capability of
the same product, accessed through the terminal alias.

---

## Requirements

- Windows 10 version 1803 or later (for App Execution Alias support)
- x64 processor (ARM64 support is planned)
