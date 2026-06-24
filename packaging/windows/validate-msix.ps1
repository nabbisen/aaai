<#
.SYNOPSIS
    Validates an aaai MSIX candidate before Store submission.

.DESCRIPTION
    RFC 091 — CI Handoff §11.
    Checks that the MSIX package:
      - exists and contains a manifest
      - includes both aaai.exe and aaai-gui.exe
      - declares a visible GUI application entry
      - hides the CLI from the Start menu (AppListEntry="none")
      - declares a CLI app execution alias

.PARAMETER PackageDir
    Directory containing the .msix file produced by make-msix.ps1.

.EXAMPLE
    .\validate-msix.ps1 -PackageDir "dist-msix"
#>
param(
    [Parameter(Mandatory=$true)] [string] $PackageDir
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$pass  = 0
$fail  = 0

function Pass($msg) { Write-Host "  [PASS] $msg" -ForegroundColor Green; $script:pass++ }
function Fail($msg) { Write-Host "  [FAIL] $msg" -ForegroundColor Red;   $script:fail++ }

Write-Host "`nvalidate-msix.ps1 — aaai MSIX validation`n"

# ── Locate the MSIX file ──────────────────────────────────────────────────────
$msix_files = Get-ChildItem $PackageDir -Filter "*.msix" -ErrorAction SilentlyContinue
if ($msix_files.Count -eq 0) {
    Write-Error "No .msix file found in $PackageDir"
    exit 1
}
$msix = $msix_files[0].FullName
Write-Host "  Package: $msix`n"

# ── Check filename conventions ────────────────────────────────────────────────
$name = [System.IO.Path]::GetFileNameWithoutExtension($msix)
if ($name -match 'aaai-[\d.]+-') { Pass "Filename includes version: $name" }
else                              { Fail "Filename does not include version: $name" }

if ($name -match 'x86_64|aarch64') { Pass "Filename includes architecture: $name" }
else                                { Fail "Filename does not include architecture: $name" }

# ── Extract and inspect payload ───────────────────────────────────────────────
$expanded = Join-Path $env:TEMP "aaai-msix-validate-$$"
if (Test-Path $expanded) { Remove-Item $expanded -Recurse -Force }
New-Item -ItemType Directory -Force -Path $expanded | Out-Null

# MSIX is a ZIP archive
try {
    Expand-Archive $msix -DestinationPath $expanded -Force
    Pass "Package extracts cleanly"
} catch {
    Fail "Package extraction failed: $_"
    exit 1
}

# ── Binary payload ────────────────────────────────────────────────────────────
if (Test-Path "$expanded\aaai.exe")     { Pass "aaai.exe present in payload" }
else                                    { Fail "aaai.exe MISSING from payload" }

if (Test-Path "$expanded\aaai-gui.exe") { Pass "aaai-gui.exe present in payload" }
else                                    { Fail "aaai-gui.exe MISSING from payload" }

# ── Store assets ──────────────────────────────────────────────────────────────
foreach ($asset in @("Square44x44Logo.png","Square150x150Logo.png","StoreLogo.png")) {
    if (Test-Path "$expanded\assets\$asset") { Pass "Asset present: $asset" }
    else                                     { Fail "Asset MISSING: $asset" }
}

# ── Manifest checks ───────────────────────────────────────────────────────────
$manifest_path = "$expanded\AppxManifest.xml"
if (-not (Test-Path $manifest_path)) {
    Fail "AppxManifest.xml MISSING from package"
    Write-Host "`n  $fail check(s) failed." -ForegroundColor Red
    exit 1
}
Pass "AppxManifest.xml present"

[xml]$manifest = Get-Content $manifest_path

# Namespace manager for the MSIX schemas
$ns = New-Object System.Xml.XmlNamespaceManager($manifest.NameTable)
$ns.AddNamespace("pkg",  "http://schemas.microsoft.com/appx/manifest/foundation/windows10")
$ns.AddNamespace("uap",  "http://schemas.microsoft.com/appx/manifest/uap/windows10")
$ns.AddNamespace("uap5", "http://schemas.microsoft.com/appx/manifest/uap/windows10/5")

# Visible GUI application entry
$gui_apps = $manifest.SelectNodes("//pkg:Application[@Executable='aaai-gui.exe']", $ns)
if ($gui_apps.Count -gt 0) { Pass "GUI application entry (aaai-gui.exe) declared" }
else                        { Fail "No GUI application entry for aaai-gui.exe" }

# GUI must NOT have AppListEntry="none"
$hidden_gui = $manifest.SelectNodes(
    "//pkg:Application[@Executable='aaai-gui.exe']//uap:VisualElements[@AppListEntry='none']", $ns)
if ($hidden_gui.Count -eq 0) { Pass "GUI is not hidden from Start menu" }
else                          { Fail "GUI application has AppListEntry='none' — it will not appear in Start menu" }

# CLI application hidden from Start menu
$cli_hidden = $manifest.SelectNodes(
    "//pkg:Application[@Executable='aaai.exe']//uap:VisualElements[@AppListEntry='none']", $ns)
if ($cli_hidden.Count -gt 0) { Pass "CLI is hidden from Start menu (AppListEntry=none)" }
else                          { Fail "CLI application is NOT hidden — it may appear as a second Start menu entry" }

# CLI execution alias
$alias_nodes = $manifest.SelectNodes(
    "//uap5:ExecutionAlias[@Alias='aaai.exe']", $ns)
if ($alias_nodes.Count -gt 0) { Pass "CLI app execution alias 'aaai.exe' declared" }
else                           { Fail "CLI app execution alias 'aaai.exe' NOT declared" }

# ── Summary ───────────────────────────────────────────────────────────────────
Write-Host "`n  $pass passed, $fail failed`n"
if ($fail -gt 0) {
    Write-Error "MSIX validation FAILED — $fail check(s) did not pass."
    exit 1
} else {
    Write-Host "  MSIX validation PASSED." -ForegroundColor Green
}
