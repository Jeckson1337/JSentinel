param(
    [switch]$UseNpmCi
)

$ErrorActionPreference = "Continue"
$failures = 0
$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
$uiRoot = Join-Path $repoRoot "apps\desktop-ui"

function Report-Error {
    param([string]$Message)
    Write-Host "ERROR: $Message" -ForegroundColor Red
    $script:failures += 1
}

function Report-Warning {
    param([string]$Message)
    Write-Host "WARNING: $Message" -ForegroundColor Yellow
}

if (-not (Test-Path $uiRoot)) {
    Report-Error "Missing apps\desktop-ui"
}

if ($failures -gt 0) {
    exit 1
}

Set-Location $uiRoot
Write-Host "Frontend root: $uiRoot"

if (-not (Get-Command npm -ErrorAction SilentlyContinue)) {
    Report-Warning "npm is not available; skipping frontend install/build."
    exit 0
}

if ($UseNpmCi -and (Test-Path "package-lock.json")) {
    Write-Host "Running npm ci"
    npm ci
} else {
    Write-Host "Running npm install"
    npm install
}

if ($LASTEXITCODE -ne 0) {
    Report-Error "npm dependency install failed"
}

if ($failures -eq 0) {
    Write-Host "Running npm run build"
    npm run build
    if ($LASTEXITCODE -ne 0) {
        Report-Error "npm run build failed"
    }
}

if ($failures -gt 0) {
    Write-Host "Frontend checks failed with $failures error(s)." -ForegroundColor Red
    exit 1
}

Write-Host "Frontend checks passed." -ForegroundColor Green
exit 0
