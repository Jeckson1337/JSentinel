$ErrorActionPreference = "Continue"
$failures = 0
$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")

function Report-Error {
    param([string]$Message)
    Write-Host "ERROR: $Message" -ForegroundColor Red
    $script:failures += 1
}

function Report-Warning {
    param([string]$Message)
    Write-Host "WARNING: $Message" -ForegroundColor Yellow
}

Set-Location $repoRoot
Write-Host "Repository root: $repoRoot"

if (-not (Test-Path "Cargo.toml")) {
    Report-Error "Missing root Cargo.toml"
}

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Report-Warning "cargo is not available; skipping cargo check/test."
    if ($failures -gt 0) { exit 1 }
    exit 0
}

Write-Host "Running cargo check --workspace"
cargo check --workspace
if ($LASTEXITCODE -ne 0) {
    Report-Error "cargo check --workspace failed"
}

Write-Host "Running cargo test --workspace"
cargo test --workspace
if ($LASTEXITCODE -ne 0) {
    Report-Error "cargo test --workspace failed"
}

if ($failures -gt 0) {
    Write-Host "Rust checks failed with $failures error(s)." -ForegroundColor Red
    exit 1
}

Write-Host "Rust checks passed." -ForegroundColor Green
exit 0
