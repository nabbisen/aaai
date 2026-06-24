# RFC 091 — Windows Store Packaging Model

**Status:** Implemented (v0.33.0 — RFC 091)  
**Target release:** v0.32.0 or later  
**Related area:** Windows distribution, Microsoft Store, release engineering, CLI/GUI packaging  
**Prepared for:** aaai v0.31.0 project structure  
**Authors:** nabbisen / project maintainers  

---

## 1. Summary

aaai should be published to the Microsoft Store as **one product** containing **two executables**:

```text
Microsoft Store listing:
  aaai

Visible app entry:
  aaai → launches the GUI executable

Terminal command:
  aaai.exe → launches the CLI executable

Internal package payload:
  aaai-gui.exe
  aaai.exe
```

The project should **not** merge the GUI and CLI into a single mode-switching executable.  
The project should **not** publish the GUI and CLI as separate Microsoft Store apps.

The current aaai workspace already supports this direction:

```text
crates/aaai-core  → shared domain logic
crates/aaai-cli   → CLI package, binary name: aaai
crates/aaai-gui   → GUI package, binary name: aaai-gui
```

The RFC therefore preserves the current architectural separation and changes only the Windows packaging and release model.

---

## 2. Decision

Adopt the following Windows Store packaging model:

| Concern | Decision |
|---|---|
| Store product count | One Store product: `aaai` |
| Visible Start menu app | GUI only |
| CLI distribution inside Store package | Yes |
| CLI Start menu entry | No |
| CLI access method | Terminal command alias: `aaai.exe` |
| GUI binary | Keep `aaai-gui.exe`, or rename intentionally to `aaai-desktop.exe` |
| CLI binary | Keep `aaai.exe` |
| Shared logic | Remains in `aaai-core` |
| Combined GUI/CLI executable | Rejected |
| Separate GUI Store app + CLI Store app | Rejected |

The product experience should be:

```text
Install aaai from Microsoft Store.
Open aaai from the Start menu for the desktop app.
Advanced users may run aaai from Terminal or PowerShell.
```

---

## 3. Motivation

The aaai product serves two kinds of users:

1. People who want a safe, visible desktop review flow.
2. Developers or release engineers who want command-line automation.

Publishing two Store products would force users to choose between “aaai” and “aaai CLI”. That is not a good experience for general users because the CLI is not a separate product. It is an advanced capability of the same product.

Merging GUI and CLI into one executable would also create avoidable complexity:

- GUI launch and terminal launch have different user expectations.
- Windows console behavior differs between console and GUI subsystems.
- Store packaging becomes harder to explain.
- Testing a mode-switching binary becomes less clear.
- The existing workspace separation would be weakened.

The strongest model is therefore:

```text
One product.
Two executables.
One shared core.
One user-facing Store identity.
```

---

## 4. Current Project Baseline

This RFC is based on the attached `aaai-0.31.0` project state.

Observed workspace:

```toml
[workspace]
members = [
    "crates/aaai-core",
    "crates/aaai-cli",
    "crates/aaai-gui",
]
```

Observed CLI binary:

```toml
[[bin]]
name = "aaai"
path = "src/main.rs"
```

Observed GUI binary:

```toml
[[bin]]
name = "aaai-gui"
path = "src/main.rs"
```

Observed release workflow issue:

```text
.github/workflows/release.yaml currently builds and archives only aaai-cli.
```

Observed CI issue:

```text
.github/workflows/ci.yaml currently builds aaai-cli and aaai-core, but does not build aaai-gui in the main Build step.
```

This RFC requires the Windows release path to build both binaries.

---

## 5. Naming Decision

### 5.1 Store-facing names

Use simple product naming:

| Surface | Name |
|---|---|
| Store product | `aaai` |
| Start menu entry | `aaai` |
| Window title | `aaai` or `aaai — Review changes` |
| CLI command | `aaai` |

### 5.2 Internal binary names

Accepted choices:

| Binary | Recommended name | Notes |
|---|---|---|
| CLI | `aaai.exe` | Already correct |
| GUI | `aaai-gui.exe` | Current name; acceptable |
| GUI alternative | `aaai-desktop.exe` | Clearer, but requires rename |

Avoid `aaai-guit.exe`.

Reason:

```text
`aaai-guit` looks like a typo, is hard to explain, and is not suitable for Store-era packaging.
```

If a rename is desired, prefer `aaai-desktop.exe`. If minimizing change is more important, keep `aaai-gui.exe`.

---

## 6. Microsoft Store Package Model

The Microsoft Store package should be an MSIX package with one visible app entry and one hidden CLI entry.

Conceptual layout:

```text
aaai.msix
├── aaai-gui.exe
├── aaai.exe
├── assets/
│   ├── Square44x44Logo.png
│   ├── Square150x150Logo.png
│   ├── Wide310x150Logo.png
│   └── StoreLogo.png
├── locales/
└── Package.appxmanifest
```

The GUI application entry should be visible in the Start menu.

The CLI application entry should be hidden from the Start menu and exposed through an app execution alias.

Conceptual manifest behavior:

```text
Application: aaai-desktop or aaai-gui
  Executable: aaai-gui.exe
  Start menu: visible
  Display name: aaai

Application: aaai-cli
  Executable: aaai.exe
  Start menu: hidden
  Execution alias: aaai.exe
```

This gives users one visible app while still making the command-line workflow available.

---

## 7. Store Listing Positioning

The Microsoft Store listing should describe the GUI-first product experience.

Recommended short description:

```text
aaai helps you compare two folders and save a clear reason for every accepted change.
```

Recommended feature list:

```text
- Compare two folders
- Review changed files one by one
- Save a reason for every accepted change
- Re-check the same review later
- Save a readable review report
- Includes an optional terminal command for automation
```

Avoid leading with developer-oriented terms such as:

```text
CLI
YAML
CI/CD
exit codes
regex
checksum
```

Those can appear in documentation and advanced sections, but not as the primary Store message.

---

## 8. Alternatives Considered

### 8.1 Separate Store apps for GUI and CLI

Rejected.

Problems:

- Confuses ordinary users.
- Makes the CLI look like a separate product.
- Splits reviews, ratings, and discoverability.
- Creates duplicate release management.
- Increases support burden.

### 8.2 Single integrated mode-switching executable

Rejected.

Problems:

- GUI and CLI subsystem expectations differ on Windows.
- Terminal output and Start menu launch behavior become harder to reason about.
- Testing becomes more complicated.
- It weakens the current Cargo workspace split.
- A boot parameter is invisible to Store users and not useful for the normal app launch path.

### 8.3 Store GUI only, CLI distributed only through GitHub

Accepted only as a temporary fallback.

This can work for an early Store submission if MSIX CLI alias work is deferred. However, the target model should still be one Store package containing both GUI and CLI.

### 8.4 CLI-only Store product

Rejected.

aaai’s primary non-technical value is the guided desktop review workflow. A CLI-only Store product does not match the product strategy.

---

## 9. Scope

This RFC includes:

- Windows Store packaging model.
- One-product/two-executable decision.
- GUI visibility and CLI alias behavior.
- Release artifact naming expectations.
- Required CI/release workflow direction.
- Store listing positioning.

---

## 10. Non-Scope

This RFC does not define:

- Full Store artwork.
- Store screenshots.
- Store pricing.
- Final legal text.
- Partner Center submission procedure.
- Installer for non-Store Windows distribution.
- Winget submission.
- macOS or Linux packaging.
- New CLI commands.
- GUI redesign.

---

## 11. Required Project Changes

### 11.1 CI build coverage

The normal CI build must include the GUI package:

```text
cargo build -p aaai-core -p aaai-cli -p aaai-gui
```

The GUI does not need full visual verification in this RFC, but it must compile on supported platforms.

### 11.2 Release build coverage

The release workflow must build both Windows binaries:

```text
cargo build --release --target x86_64-pc-windows-msvc -p aaai-cli
cargo build --release --target x86_64-pc-windows-msvc -p aaai-gui
```

If the project later supports ARM64 Windows, add:

```text
aarch64-pc-windows-msvc
```

### 11.3 Windows packaging files

Add:

```text
packaging/windows/
├── README.md
├── Package.appxmanifest.template
├── make-msix.ps1
└── assets/
```

### 11.4 Artifact names

For GitHub direct release artifacts:

```text
aaai-cli-v{version}-windows-x64.zip
aaai-gui-v{version}-windows-x64.zip
aaai-full-v{version}-windows-x64.zip
```

For Store packaging:

```text
aaai-v{version}-windows-x64.msix
```

If MSIX signing is not performed in CI, the CI artifact name should make that explicit:

```text
aaai-v{version}-windows-x64-unsigned.msix
```

---

## 12. CI and Release Implications

This RFC requires a dedicated release-engineering handoff. At minimum, it must define:

- Windows target triple.
- Build commands for both binaries.
- Artifact staging layout.
- MSIX package generation.
- Manifest validation.
- App execution alias validation.
- Direct release zip generation.
- Store artifact upload policy.
- Signing policy.

The companion document is:

```text
CI-HANDOFF-windows-store-msix-build.md
```

---

## 13. Risks and Mitigations

| Risk | Mitigation |
|---|---|
| CLI alias does not work after Store install | Add manual validation checklist; keep GitHub CLI zip as fallback |
| CLI appears as a Start menu app | Manifest review must verify hidden CLI entry |
| GUI executable name changes break scripts | Prefer keeping `aaai-gui.exe` unless a rename is explicitly approved |
| Store packaging blocks release | Keep direct GitHub release archives independent of Store packaging |
| MSIX generation tooling changes | Keep packaging scripts isolated in `packaging/windows/` |
| Store review rejects metadata | Keep Store text GUI-first and non-technical |

---

## 14. Rollback Plan

If MSIX packaging or Store alias behavior blocks release:

1. Publish the GUI-only Store package.
2. Continue distributing the CLI through GitHub release archives.
3. Keep the RFC decision open until CLI-in-MSIX is validated.
4. Do not merge GUI and CLI binaries as a shortcut.

Rollback must not change the core architecture.

---

## 15. Acceptance Criteria

This RFC is complete when all of the following are true:

- Windows release builds both `aaai.exe` and `aaai-gui.exe`.
- CI checks that `aaai-gui` compiles.
- A Windows package layout can be produced with both executables.
- The Store package has one visible Start menu app named `aaai`.
- The CLI entry does not appear as a normal Start menu app.
- `aaai.exe` is available as a terminal command after package install.
- The Store listing describes the GUI-first product clearly.
- Direct GitHub release archives remain available.
- No mode-switching executable is introduced.
- No separate CLI Store product is introduced.

---

## 16. Open Questions

1. Should the GUI binary remain `aaai-gui.exe`, or be renamed to `aaai-desktop.exe`?
2. Should Store packaging be introduced before or after v1.0.0?
3. Should Windows ARM64 be supported in the first Store submission?
4. Should the Store package include only GUI + CLI, or also sample files/templates?
5. Should direct release MSIX artifacts be signed in CI, or only signed by the Store submission process?

---

## 17. References

Official references to check during implementation:

- Microsoft Store publishing overview: `https://learn.microsoft.com/en-us/windows/apps/publish/`
- Publish your first Windows app: `https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/publish-first-app`
- Packaging a CLI executable as MSIX: `https://learn.microsoft.com/en-us/windows/apps/dev-tools/winapp-cli/guides/packaging-cli`
- MSIX Packaging Tool overview: `https://learn.microsoft.com/en-us/windows/msix/packaging-tool/tool-overview`
- MSIX package command-line creation: `https://learn.microsoft.com/en-us/windows/msix/packaging-tool/package-conversion-command-line`
- Multiple MSIX application entries / Start menu behavior: `https://learn.microsoft.com/en-us/windows/msix/packaging-tool/create-start-group`
- MSIX troubleshooting guide: `https://learn.microsoft.com/en-us/windows/msix/msix-troubleshooting-guide`

---

## 18. Final Decision Statement

aaai should enter the Microsoft Store as one coherent product:

```text
One Store listing.
One visible desktop app.
One optional terminal command.
Two executables.
One shared core.
```

This protects the user experience while preserving a clean Rust workspace and a strong automation story.
