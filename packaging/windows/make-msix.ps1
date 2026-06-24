<#
.SYNOPSIS
    Builds an MSIX candidate for the aaai Windows Store package.

.DESCRIPTION
    RFC 091 — Windows Store Packaging Model.
    Stages binaries and assets, substitutes version into the manifest template,
    then calls makeappx.exe to produce the MSIX candidate.

    Output: $OutDir\aaai-$Version-$Target[-unsigned].msix

.PARAMETER Version
    Semantic version string (e.g. "0.33.0"). Must not include the "v" prefix.

.PARAMETER Target
    Rust target triple (e.g. "x86_64-pc-windows-msvc").

.PARAMETER InputDir
    Directory containing aaai.exe and aaai-gui.exe.

.PARAMETER OutDir
    Directory where the MSIX file will be written.

.PARAMETER Unsigned
    When set, appends "-unsigned" to the output filename.

.EXAMPLE
    .\make-msix.ps1 -Version "0.33.0" -Target "x86_64-pc-windows-msvc" `
                    -InputDir "msix-input" -OutDir "dist-msix"
#>
param(
    [Parameter(Mandatory=$true)]  [string] $Version,
    [Parameter(Mandatory=$true)]  [string] $Target,
    [Parameter(Mandatory=$true)]  [string] $InputDir,
    [Parameter(Mandatory=$true)]  [string] $OutDir,
    [Parameter(Mandatory=$false)] [switch] $Unsigned
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

# ── Helper ────────────────────────────────────────────────────────────────────
function Require-File($path, $desc) {
    if (-not (Test-Path $path)) {
        Write-Error "Missing required file: $path ($desc)"
        exit 1
    }
}

# ── Pre-flight checks ─────────────────────────────────────────────────────────
Write-Host "make-msix.ps1 — aaai v$Version ($Target)"

Require-File "$InputDir\aaai.exe"                                      "CLI binary"
Require-File "$InputDir\aaai-gui.exe"                                  "GUI binary"
Require-File "packaging\windows\Package.appxmanifest.template"         "manifest template"
Require-File "packaging\windows\assets\Square44x44Logo.png"            "Store asset"
Require-File "packaging\windows\assets\Square150x150Logo.png"          "Store asset"
Require-File "packaging\windows\assets\Wide310x150Logo.png"            "Store asset"
Require-File "packaging\windows\assets\StoreLogo.png"                  "Store asset"

# Locate makeappx.exe (Windows SDK)
$makeappx = Get-Command makeappx.exe -ErrorAction SilentlyContinue
if (-not $makeappx) {
    # Try common SDK paths
    $sdk_paths = @(
        "${env:ProgramFiles(x86)}\Windows Kits\10\bin\x64",
        "${env:ProgramFiles(x86)}\Windows Kits\10\bin\x86"
    )
    foreach ($p in $sdk_paths) {
        $candidate = Join-Path $p "makeappx.exe"
        if (Test-Path $candidate) { $makeappx = $candidate; break }
    }
    if (-not $makeappx) {
        Write-Error "makeappx.exe not found. Install the Windows SDK or add it to PATH."
        exit 1
    }
}
Write-Host "  makeappx: $makeappx"

# ── Stage package directory ───────────────────────────────────────────────────
$staged = Join-Path $env:TEMP "aaai-msix-staged-$Version"
if (Test-Path $staged) { Remove-Item $staged -Recurse -Force }
New-Item -ItemType Directory -Force -Path $staged | Out-Null

# Binaries
Copy-Item "$InputDir\aaai.exe"     "$staged\"
Copy-Item "$InputDir\aaai-gui.exe" "$staged\"

# Store assets
Copy-Item "packaging\windows\assets" "$staged\assets" -Recurse

# Manifest — substitute {{VERSION}} placeholder
$manifestTemplate = Get-Content "packaging\windows\Package.appxmanifest.template" -Raw
$manifest = $manifestTemplate -replace '\{\{VERSION\}\}', $Version
$manifest | Set-Content "$staged\AppxManifest.xml" -Encoding UTF8

Write-Host "  staged: $staged"
Write-Host "    aaai.exe     $(Get-Item "$staged\aaai.exe" | Select -Expand Length) bytes"
Write-Host "    aaai-gui.exe $(Get-Item "$staged\aaai-gui.exe" | Select -Expand Length) bytes"

# ── Build MSIX ────────────────────────────────────────────────────────────────
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null

$suffix  = if ($Unsigned) { "-unsigned" } else { "" }
$outFile = Join-Path $OutDir "aaai-$Version-$Target$suffix.msix"

Write-Host "  building: $outFile"

& $makeappx pack /d "$staged" /p "$outFile" /overwrite /nv
if ($LASTEXITCODE -ne 0) {
    Write-Error "makeappx pack failed (exit $LASTEXITCODE)"
    exit 1
}

Write-Host "  done: $outFile ($(Get-Item $outFile | Select -Expand Length) bytes)"
