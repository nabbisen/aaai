# CI Handoff — Windows Store MSIX Build for aaai

**Status:** Draft implementation handoff  
**Depends on:** RFC 091 — Windows Store Packaging Model  
**Target release:** v0.32.0 or later  
**Project baseline:** aaai v0.31.0  
**Primary goal:** Build and package the GUI and CLI together for Microsoft Store distribution while preserving direct GitHub release artifacts.

---

## 1. Objective

Update aaai release engineering so Windows builds produce both executables:

```text
aaai.exe      # CLI
aaai-gui.exe  # GUI
```

Then stage them into a Windows package layout suitable for an MSIX package:

```text
aaai-v{version}-windows-x64.msix
```

The package should expose:

- one visible Start menu app: `aaai`, launching the GUI,
- one terminal command: `aaai.exe`, launching the CLI,
- no separate visible CLI Start menu app.

---

## 2. Current Baseline

The attached `aaai-0.31.0` project has this workspace shape:

```text
crates/aaai-core
crates/aaai-cli
crates/aaai-gui
```

The CLI crate declares:

```toml
[[bin]]
name = "aaai"
path = "src/main.rs"
```

The GUI crate declares:

```toml
[[bin]]
name = "aaai-gui"
path = "src/main.rs"
```

The current release workflow builds only the CLI:

```text
cargo build --release --target ${{ matrix.target }} -p aaai-cli
```

The current CI build step builds only:

```text
cargo build -p aaai-cli -p aaai-core
```

This handoff requires both CI and release workflows to include `aaai-gui`.

---

## 3. Implementation Principles

1. Keep `aaai-core` shared.
2. Keep `aaai-cli` and `aaai-gui` as separate binaries.
3. Do not introduce a combined mode-switching executable.
4. Do not create a separate Microsoft Store product for the CLI.
5. Keep Store packaging isolated under `packaging/windows/`.
6. Keep GitHub release artifacts independent of Microsoft Store submission.
7. Make every generated artifact name include the version and target.
8. Fail CI early if either Windows binary does not build.

---

## 4. Required Repository Additions

Add:

```text
packaging/
└── windows/
    ├── README.md
    ├── Package.appxmanifest.template
    ├── make-msix.ps1
    ├── validate-msix.ps1
    └── assets/
        ├── Square44x44Logo.png
        ├── Square150x150Logo.png
        ├── Wide310x150Logo.png
        └── StoreLogo.png
```

Notes:

- The asset files may initially be placeholders, but they must be real files before Store submission.
- `Package.appxmanifest.template` should not contain local machine paths.
- `make-msix.ps1` should accept version, target, and input binary directory arguments.
- `validate-msix.ps1` should verify the staged output before upload.

---

## 5. Cargo Build Changes

### 5.1 CI build step

Replace the current build command:

```bash
cargo build -p aaai-cli -p aaai-core
```

with:

```bash
cargo build -p aaai-core -p aaai-cli -p aaai-gui
```

If GUI build dependencies make Linux/macOS CI unstable, split GUI build into a separate job and keep it required for Windows.

Recommended explicit Windows GUI build job:

```yaml
windows-gui-build:
  name: Windows GUI build
  runs-on: windows-latest
  steps:
    - uses: actions/checkout@v6
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        components: rustfmt, clippy
    - uses: Swatinem/rust-cache@v2
    - name: Build GUI
      run: cargo build -p aaai-gui
```

### 5.2 MSRV check

Current MSRV check only checks core and CLI:

```bash
cargo check -p aaai-core -p aaai-cli
```

Recommended:

```bash
cargo check -p aaai-core -p aaai-cli -p aaai-gui
```

If GUI dependency MSRV makes this fail, record that explicitly and decide one of:

1. raise workspace MSRV,
2. exclude GUI from MSRV check with a documented reason,
3. gate GUI behind a separate MSRV policy.

Do not silently omit GUI.

---

## 6. Release Workflow Changes

### 6.1 Build both binaries

For every release target, build both packages where supported:

```bash
cargo build --release --target $TARGET -p aaai-cli
cargo build --release --target $TARGET -p aaai-gui
```

For Windows:

```powershell
cargo build --release --target x86_64-pc-windows-msvc -p aaai-cli
cargo build --release --target x86_64-pc-windows-msvc -p aaai-gui
```

### 6.2 Windows release artifact layout

Stage Windows release binaries like this:

```text
dist/windows-x64/
├── cli/
│   └── aaai.exe
├── gui/
│   └── aaai-gui.exe
└── full/
    ├── aaai.exe
    └── aaai-gui.exe
```

Generate these direct release archives:

```text
aaai-cli-v{version}-x86_64-pc-windows-msvc.zip
aaai-gui-v{version}-x86_64-pc-windows-msvc.zip
aaai-full-v{version}-x86_64-pc-windows-msvc.zip
```

Generate this Store package candidate:

```text
aaai-v{version}-x86_64-pc-windows-msvc.msix
```

If unsigned:

```text
aaai-v{version}-x86_64-pc-windows-msvc-unsigned.msix
```

---

## 7. Proposed Release Workflow Patch Shape

This is not a copy-paste final workflow. It shows the intended shape.

```yaml
jobs:
  build:
    name: Build (${{ matrix.target }})
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            archive_ext: tar.gz
          - os: macos-latest
            target: aarch64-apple-darwin
            archive_ext: tar.gz
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            archive_ext: zip

    steps:
      - uses: actions/checkout@v6

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - uses: Swatinem/rust-cache@v2

      - name: Build CLI release binary
        run: cargo build --release --target ${{ matrix.target }} -p aaai-cli

      - name: Build GUI release binary
        run: cargo build --release --target ${{ matrix.target }} -p aaai-gui

      - name: Create release archives (Windows)
        if: runner.os == 'Windows'
        shell: pwsh
        run: |
          $TAG = $env:GITHUB_REF_NAME
          $TARGET = "${{ matrix.target }}"
          New-Item -ItemType Directory -Force -Path "dist\cli", "dist\gui", "dist\full"

          Copy-Item "target\$TARGET\release\aaai.exe" "dist\cli\"
          Copy-Item "target\$TARGET\release\aaai-gui.exe" "dist\gui\"
          Copy-Item "target\$TARGET\release\aaai.exe" "dist\full\"
          Copy-Item "target\$TARGET\release\aaai-gui.exe" "dist\full\"

          Compress-Archive -Path "dist\cli\*"  -DestinationPath "dist\aaai-cli-$TAG-$TARGET.zip"
          Compress-Archive -Path "dist\gui\*"  -DestinationPath "dist\aaai-gui-$TAG-$TARGET.zip"
          Compress-Archive -Path "dist\full\*" -DestinationPath "dist\aaai-full-$TAG-$TARGET.zip"

          echo "ASSET_CLI=dist\aaai-cli-$TAG-$TARGET.zip" | Out-File -FilePath $env:GITHUB_ENV -Append
          echo "ASSET_GUI=dist\aaai-gui-$TAG-$TARGET.zip" | Out-File -FilePath $env:GITHUB_ENV -Append
          echo "ASSET_FULL=dist\aaai-full-$TAG-$TARGET.zip" | Out-File -FilePath $env:GITHUB_ENV -Append
```

The existing upload step must upload all three Windows archives, not only one `ASSET`.

Possible upload shape:

```yaml
      - name: Upload Windows artifacts
        if: runner.os == 'Windows'
        uses: actions/upload-artifact@v4
        with:
          name: aaai-${{ matrix.target }}
          path: |
            dist/aaai-cli-*.zip
            dist/aaai-gui-*.zip
            dist/aaai-full-*.zip
```

For Linux/macOS, decide whether GUI archives should also be emitted. This handoff is Windows-focused, but building both binaries consistently is recommended.

---

## 8. MSIX Packaging Job

Add a Windows-only packaging job after the Windows release build succeeds.

Conceptual job:

```yaml
  package-msix:
    name: Package MSIX (Windows x64)
    runs-on: windows-latest
    needs: build
    steps:
      - uses: actions/checkout@v6

      - name: Download Windows build artifact
        uses: actions/download-artifact@v4
        with:
          name: aaai-x86_64-pc-windows-msvc
          path: dist/

      - name: Prepare MSIX input
        shell: pwsh
        run: |
          New-Item -ItemType Directory -Force -Path msix-input
          Expand-Archive "dist\aaai-full-${{ github.ref_name }}-x86_64-pc-windows-msvc.zip" -DestinationPath msix-input
          Copy-Item packaging\windows\assets msix-input\assets -Recurse

      - name: Build MSIX
        shell: pwsh
        run: |
          packaging\windows\make-msix.ps1 `
            -Version "${{ github.ref_name }}" `
            -Target "x86_64-pc-windows-msvc" `
            -InputDir "msix-input" `
            -OutDir "dist-msix"

      - name: Validate MSIX layout
        shell: pwsh
        run: |
          packaging\windows\validate-msix.ps1 `
            -PackageDir "dist-msix"

      - name: Upload MSIX candidate
        uses: actions/upload-artifact@v4
        with:
          name: aaai-msix-x64
          path: dist-msix/*.msix
```

Implementation may use one of these packaging approaches:

1. `winapp` CLI.
2. MSIX Packaging Tool command line.
3. `makeappx.exe` from the Windows SDK.

Use whichever is most stable in CI, but isolate it behind `packaging/windows/make-msix.ps1` so the workflow does not depend on tool-specific details.

---

## 9. MSIX Manifest Requirements

The package manifest must express two application entries.

### 9.1 GUI entry

Required behavior:

```text
Display name: aaai
Executable: aaai-gui.exe
Start menu: visible
```

### 9.2 CLI entry

Required behavior:

```text
Executable: aaai.exe
Start menu: hidden
Execution alias: aaai.exe
```

The CLI entry must not create a second visible app called `aaai CLI` in the Start menu.

### 9.3 Conceptual manifest snippet

This snippet is intentionally conceptual. The implementation must validate namespaces and schema versions against the chosen MSIX tooling.

```xml
<Applications>
  <Application
      Id="AaaiGui"
      Executable="aaai-gui.exe"
      EntryPoint="Windows.FullTrustApplication">
    <uap:VisualElements
        DisplayName="aaai"
        Description="aaai"
        BackgroundColor="transparent"
        Square44x44Logo="assets\Square44x44Logo.png"
        Square150x150Logo="assets\Square150x150Logo.png" />
  </Application>

  <Application
      Id="AaaiCli"
      Executable="aaai.exe"
      EntryPoint="Windows.FullTrustApplication">
    <uap:VisualElements
        DisplayName="aaai command"
        Description="aaai command"
        AppListEntry="none"
        BackgroundColor="transparent"
        Square44x44Logo="assets\Square44x44Logo.png"
        Square150x150Logo="assets\Square150x150Logo.png" />
    <Extensions>
      <uap5:Extension Category="windows.appExecutionAlias">
        <uap5:AppExecutionAlias>
          <uap5:ExecutionAlias Alias="aaai.exe" />
        </uap5:AppExecutionAlias>
      </uap5:Extension>
    </Extensions>
  </Application>
</Applications>
```

Validation must confirm that:

- the GUI starts from the Start menu,
- the CLI does not appear as a normal Start menu app,
- `aaai.exe --help` works from Terminal after installation.

---

## 10. `make-msix.ps1` Required Behavior

`packaging/windows/make-msix.ps1` should accept:

```powershell
param(
  [Parameter(Mandatory=$true)] [string] $Version,
  [Parameter(Mandatory=$true)] [string] $Target,
  [Parameter(Mandatory=$true)] [string] $InputDir,
  [Parameter(Mandatory=$true)] [string] $OutDir
)
```

Required checks:

```powershell
Test-Path "$InputDir\aaai.exe"
Test-Path "$InputDir\aaai-gui.exe"
Test-Path "packaging\windows\Package.appxmanifest.template"
Test-Path "packaging\windows\assets\Square44x44Logo.png"
Test-Path "packaging\windows\assets\Square150x150Logo.png"
```

Required output:

```text
$OutDir\aaai-$Version-$Target.msix
```

If unsigned:

```text
$OutDir\aaai-$Version-$Target-unsigned.msix
```

The script must fail with a clear message when required files are missing.

---

## 11. `validate-msix.ps1` Required Behavior

The validation script should check at least:

```text
- MSIX file exists
- MSIX file name includes version and target
- Staged payload included aaai.exe
- Staged payload included aaai-gui.exe
- Manifest contains one visible GUI application entry
- Manifest contains CLI app execution alias aaai.exe
- Manifest hides CLI app from Start menu
```

If local install validation is available in the CI environment, also check:

```powershell
aaai.exe --help
```

Do not make interactive Store submission part of this script.

---

## 12. Signing Policy

### 12.1 Microsoft Store submission

For Store submission, Microsoft signs the package as part of the Store certification and distribution path. CI may therefore produce an unsigned Store candidate artifact if the submission process expects that.

### 12.2 Direct distribution

For direct distribution outside the Store, unsigned MSIX packages are not suitable for ordinary users.

If direct MSIX distribution is desired, define a separate signing policy:

```text
- signing certificate source
- secret storage
- timestamping
- verification command
- renewal process
```

This handoff does not require direct signed MSIX distribution.

---

## 13. Store Submission Gate

Do not upload to Microsoft Store on every tag automatically at first.

Recommended rollout:

| Stage | Action |
|---|---|
| Stage 1 | Build direct Windows zip artifacts only |
| Stage 2 | Build unsigned MSIX candidate as CI artifact |
| Stage 3 | Manually validate local install and CLI alias |
| Stage 4 | Manually submit to Store |
| Stage 5 | Add optional `workflow_dispatch` Store packaging job |
| Stage 6 | Consider semi-automated Store submission only after repeated success |

Initial automation should stop at artifact creation.

---

## 14. Verification Checklist

### 14.1 Build verification

```text
[ ] cargo build -p aaai-core -p aaai-cli -p aaai-gui succeeds on Windows
[ ] cargo test -p aaai-core --lib succeeds
[ ] cargo test -p aaai-cli -- --test-threads=1 succeeds
[ ] i18n key audit remains passing
[ ] mdBook build remains passing
```

### 14.2 Release artifact verification

```text
[ ] aaai-cli-v{version}-x86_64-pc-windows-msvc.zip exists
[ ] aaai-gui-v{version}-x86_64-pc-windows-msvc.zip exists
[ ] aaai-full-v{version}-x86_64-pc-windows-msvc.zip exists
[ ] aaai.exe exists in CLI and full archives
[ ] aaai-gui.exe exists in GUI and full archives
[ ] aaai.exe --help works after extraction
[ ] aaai-gui.exe launches after extraction
```

### 14.3 MSIX verification

```text
[ ] MSIX candidate exists
[ ] MSIX candidate includes aaai.exe
[ ] MSIX candidate includes aaai-gui.exe
[ ] Start menu shows only aaai, not aaai CLI
[ ] Terminal can run aaai.exe --help
[ ] GUI starts from Start menu
[ ] GUI can open folder picker
[ ] CLI can read files in normal user-accessible folders
[ ] Package uninstall removes the app cleanly
```

### 14.4 UX verification

```text
[ ] Store listing describes aaai as a folder-change review app
[ ] Store listing does not lead with CLI/YAML/CI terminology
[ ] Start menu name is aaai
[ ] No duplicate Start menu entry confuses users
[ ] GUI is the obvious path for ordinary users
```

---

## 15. Failure Conditions

The CI/release implementation must fail if:

- `aaai-cli` does not build.
- `aaai-gui` does not build on Windows.
- a Windows full archive lacks either executable.
- MSIX packaging is requested but either executable is missing.
- the manifest template is missing.
- required Store icon assets are missing.
- validation detects that the CLI would appear as a visible Start menu entry.

The implementation may allow the GitHub release to proceed without MSIX during early rollout if MSIX packaging is explicitly marked experimental and non-blocking.

---

## 16. Recommended Workflow Split

Keep the existing `release.yaml`, but add a separate workflow during the first phase:

```text
.github/workflows/windows-msix.yaml
```

Reason:

- It isolates MSIX instability from normal releases.
- It allows manual `workflow_dispatch` runs.
- It reduces risk before Store packaging is proven.

Suggested triggers:

```yaml
on:
  workflow_dispatch:
  push:
    tags:
      - "v[0-9]+.[0-9]+.[0-9]+"
```

For the first implementation, make the tag trigger build an artifact but not submit to Store.

---

## 17. Suggested `windows-msix.yaml` Outline

```yaml
name: Windows MSIX Candidate

on:
  workflow_dispatch:
  push:
    tags:
      - "v[0-9]+.[0-9]+.[0-9]+"

permissions:
  contents: read

jobs:
  msix:
    name: Build MSIX candidate
    runs-on: windows-latest

    steps:
      - uses: actions/checkout@v6

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: x86_64-pc-windows-msvc

      - uses: Swatinem/rust-cache@v2

      - name: Build CLI
        run: cargo build --release --target x86_64-pc-windows-msvc -p aaai-cli

      - name: Build GUI
        run: cargo build --release --target x86_64-pc-windows-msvc -p aaai-gui

      - name: Stage package input
        shell: pwsh
        run: |
          New-Item -ItemType Directory -Force -Path msix-input
          Copy-Item target\x86_64-pc-windows-msvc\release\aaai.exe msix-input\
          Copy-Item target\x86_64-pc-windows-msvc\release\aaai-gui.exe msix-input\
          Copy-Item packaging\windows\assets msix-input\assets -Recurse

      - name: Build MSIX candidate
        shell: pwsh
        run: |
          packaging\windows\make-msix.ps1 `
            -Version "${{ github.ref_name }}" `
            -Target "x86_64-pc-windows-msvc" `
            -InputDir "msix-input" `
            -OutDir "dist-msix"

      - name: Validate MSIX candidate
        shell: pwsh
        run: |
          packaging\windows\validate-msix.ps1 -PackageDir "dist-msix"

      - name: Upload MSIX candidate
        uses: actions/upload-artifact@v4
        with:
          name: aaai-msix-candidate-x64
          path: dist-msix/*
```

---

## 18. Documentation Updates

Update these docs after implementation:

```text
README.md
CHANGELOG.md
ROADMAP.md
docs/src/cli-setup.md
docs/src/getting-started.md
docs/src/compatibility.md
docs/src/ci-integration.md
```

Add or update Windows-specific documentation:

```text
docs/src/windows-store.md
```

Suggested content:

```text
- Install aaai from Microsoft Store
- Open aaai from the Start menu
- Use aaai from Terminal
- How to check whether the command is available
- Where to get direct release archives
```

Do not make the README large. Add only a short Windows Store note and link to mdBook.

---

## 19. Release Notes Template

For the release that adds this packaging model:

```markdown
## Windows distribution

- The Windows release now builds both the desktop app and the command-line tool.
- The Microsoft Store package is prepared as one aaai product with the desktop app as the visible entry point.
- The command-line tool remains available as `aaai` for advanced automation use.
- Direct GitHub release archives are still provided separately for CLI-only, GUI-only, and full Windows downloads.
```

---

## 20. Developer Task List

### Task A — Build coverage

```text
[ ] Add aaai-gui to CI build command
[ ] Add aaai-gui to MSRV decision path
[ ] Add Windows GUI build verification
```

### Task B — Release archives

```text
[ ] Build both CLI and GUI in release workflow
[ ] Produce CLI-only archive
[ ] Produce GUI-only archive
[ ] Produce full archive
[ ] Upload all three archives
```

### Task C — Packaging directory

```text
[ ] Add packaging/windows/README.md
[ ] Add Package.appxmanifest.template
[ ] Add make-msix.ps1
[ ] Add validate-msix.ps1
[ ] Add required Store icon assets
```

### Task D — MSIX candidate workflow

```text
[ ] Add .github/workflows/windows-msix.yaml
[ ] Stage aaai.exe and aaai-gui.exe
[ ] Generate MSIX candidate
[ ] Validate package shape
[ ] Upload candidate artifact
```

### Task E — Manual validation

```text
[ ] Install MSIX locally
[ ] Confirm one Start menu entry
[ ] Confirm GUI launch
[ ] Confirm aaai.exe --help from Terminal
[ ] Confirm uninstall
```

### Task F — Documentation

```text
[ ] Update README short note
[ ] Add docs/src/windows-store.md
[ ] Update docs/src/cli-setup.md
[ ] Update CHANGELOG.md
[ ] Update ROADMAP.md
[ ] Add RFC 091 to rfcs/README.md when accepted
```

---

## 21. Acceptance Criteria

The handoff is implemented when:

- `cargo build -p aaai-core -p aaai-cli -p aaai-gui` is part of CI.
- Windows release builds produce both `aaai.exe` and `aaai-gui.exe`.
- GitHub release artifacts include CLI-only, GUI-only, and full Windows archives.
- A Windows MSIX candidate can be produced in CI.
- The MSIX candidate includes both executables.
- The GUI is the only visible Start menu app.
- The CLI is available as `aaai.exe` in Terminal.
- The Store positioning remains GUI-first and non-technical.
- No combined GUI/CLI executable is introduced.
- No separate CLI Store product is introduced.

---

## 22. Final Engineering Direction

Implement Windows Store packaging as an additive release-engineering layer:

```text
Do not rewrite the product.
Do not merge the binaries.
Do not split the Store identity.
Package the existing architecture correctly.
```

The existing workspace structure is already right. The remaining work is CI, packaging, validation, and Store-facing presentation.
