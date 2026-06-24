# Windows Packaging — aaai

This directory contains everything needed to produce a Windows MSIX package
suitable for the Microsoft Store.

## Overview

The MSIX package ships as **one Store product** with **two executables**:

| Binary | Role | Start menu |
|---|---|---|
| `aaai-gui.exe` | Desktop review app | Visible — named **aaai** |
| `aaai.exe` | CLI for automation | Hidden — alias `aaai.exe` in Terminal |

See RFC 091 for the full packaging model decision.

## Files

| File | Purpose |
|---|---|
| `Package.appxmanifest.template` | MSIX manifest template — fill in version before use |
| `make-msix.ps1` | Builds the MSIX candidate from a staged binary directory |
| `validate-msix.ps1` | Validates the staged package layout before submission |
| `assets/` | Store icon assets — **must be replaced with real artwork** before submission |

## Requirements

- Windows SDK `makeappx.exe` or MSIX Packaging Tool
- PowerShell 5.1+
- Both `aaai.exe` and `aaai-gui.exe` built for the target architecture

## Quick start (CI)

CI uses `windows-msix.yaml` which calls `make-msix.ps1` automatically.

## Quick start (manual)

```powershell
# 1. Build both binaries
cargo build --release --target x86_64-pc-windows-msvc -p aaai-cli
cargo build --release --target x86_64-pc-windows-msvc -p aaai-gui

# 2. Stage inputs
New-Item -ItemType Directory -Force msix-input
Copy-Item target\x86_64-pc-windows-msvc\release\aaai.exe     msix-input\
Copy-Item target\x86_64-pc-windows-msvc\release\aaai-gui.exe msix-input\
Copy-Item packaging\windows\assets msix-input\assets -Recurse

# 3. Build candidate
.\packaging\windows\make-msix.ps1 `
  -Version "0.33.0" `
  -Target  "x86_64-pc-windows-msvc" `
  -InputDir "msix-input" `
  -OutDir   "dist-msix"

# 4. Validate
.\packaging\windows\validate-msix.ps1 -PackageDir "dist-msix"
```

## Assets

Replace every file in `assets/` with real artwork before Store submission.

| File | Required dimensions | Usage |
|---|---|---|
| `Square44x44Logo.png` | 44 × 44 px | Taskbar, Start menu small tile |
| `Square150x150Logo.png` | 150 × 150 px | Start menu medium tile |
| `Wide310x150Logo.png` | 310 × 150 px | Start menu wide tile |
| `StoreLogo.png` | 50 × 50 px | Store listing icon |

Provide all four sizes or the Store submission will be rejected.

## Signing

For **Store submission**: Microsoft signs the package during the certification
process. CI may upload an unsigned candidate.

For **direct distribution** outside the Store: a separate signing certificate
and process is required. See `make-msix.ps1` for the unsigned output path.

## Rollout stages

Per RFC 091 §13:

| Stage | Action |
|---|---|
| 1 | Build direct Windows zip artifacts only (**current**) |
| 2 | Build unsigned MSIX candidate as CI artifact |
| 3 | Manually validate local install and CLI alias |
| 4 | Manually submit to Store |
| 5 | Add `workflow_dispatch` Store packaging job |

## References

- [MSIX packaging documentation](https://learn.microsoft.com/en-us/windows/msix/)
- [Microsoft Store publishing overview](https://learn.microsoft.com/en-us/windows/apps/publish/)
- [Packaging CLI as MSIX](https://learn.microsoft.com/en-us/windows/apps/dev-tools/winapp-cli/guides/packaging-cli)
