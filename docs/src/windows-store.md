# Windows — Installation and Setup

The current supported Windows installation route is to build aaai from source.

> **Distribution status:** Microsoft Store/MSIX distribution is deferred and
> is not currently available. Do not search for or install an unofficial Store
> listing.

## Build from source

Install [Git](https://git-scm.com/download/win) and
[Rust 1.91 or newer](https://rustup.rs/), then run:

```powershell
git clone https://github.com/nabbisen/aaai.git
cd aaai
cargo build --release -p aaai-cli -p aaai-gui
```

The build produces `target\release\aaai.exe` and
`target\release\aaai-gui.exe`.

---

## Planned direct download (v1 target)

The following Windows archives are planned v1 artifacts, contingent on the
C1/R1 release gates. They are not currently available from
[GitHub Releases](https://github.com/nabbisen/aaai/releases).

| Archive | Contents |
|---|---|
| `aaai-cli-v{version}-x86_64-pc-windows-msvc.zip` | `aaai.exe` only |
| `aaai-gui-v{version}-x86_64-pc-windows-msvc.zip` | `aaai-gui.exe` only |
| `aaai-full-v{version}-x86_64-pc-windows-msvc.zip` | Both executables |

When these archives become available, extract one to a folder of your choice.

---

## Desktop app

After building from source, run `target\release\aaai-gui.exe`. The desktop app
opens to the folder selection screen. Choose the older and newer folders you
want to compare, then click **Check changes**.

Future direct archives will provide the same executable in the extracted
folder.

See [Getting Started](getting-started.md) for a full walkthrough.

---

## Terminal command

After building from source, run `target\release\aaai.exe`. To use `aaai` from
any terminal, add `target\release` to your `PATH`. Future direct archives will
provide the same executable in the extracted folder, which can likewise be
added to `PATH`.

### Basic CLI usage

```powershell
# Generate a review template from the current diff
.\target\release\aaai.exe snap --left .\before --right .\after --out audit.yaml

# Run a review against an existing definition
.\target\release\aaai.exe audit --left .\before --right .\after --config audit.yaml
```

See the [CLI Reference](cli.md) for the full command list.

---

## Deferred Microsoft Store package model

RFC 091 retains a future design for **one Store product** containing **two
executables**:

| Binary | Role |
|---|---|
| `aaai-gui.exe` | Desktop review application |
| `aaai.exe` | Command-line interface |

If Store distribution is implemented later, the GUI will be the visible app
and the CLI will remain an advanced capability of the same product. This model
is design guidance only: no Store listing, MSIX installation, or terminal alias
is currently supported.

---

## Requirements

- x64 Windows
- Git
- Rust 1.91 or newer
- ARM64 packages are deferred
