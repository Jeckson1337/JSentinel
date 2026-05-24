param(
    [switch]$SkipNpmInstall
)

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

function Require-Path {
    param([string]$Path)
    $fullPath = Join-Path $repoRoot $Path
    if (-not (Test-Path $fullPath)) {
        Report-Error "Missing required path: $Path"
    } else {
        Write-Host "OK: $Path"
    }
}

Write-Host "Current directory: $(Get-Location)"
Write-Host "Repository root: $repoRoot"
Set-Location $repoRoot

$requiredPaths = @(
    "Cargo.toml",
    "README.md",
    "apps\desktop-ui",
    "crates",
    "docs",
    "scripts\release"
)

foreach ($path in $requiredPaths) {
    Require-Path $path
}

$forbiddenProjectDirs = @(
    "JSentinel",
    "sentinel-local",
    "SentinelLocal"
)

foreach ($path in $forbiddenProjectDirs) {
    if (Test-Path (Join-Path $repoRoot $path)) {
        Report-Error "Unexpected nested project directory exists: $path"
    } else {
        Write-Host "OK: nested project directory absent: $path"
    }
}

$crateNames = @(
    "jsentinel-core",
    "jsentinel-db",
    "jsentinel-events",
    "jsentinel-policy",
    "jsentinel-windows",
    "jsentinel-linux",
    "jsentinel-quarantine",
    "jsentinel-network",
    "jsentinel-files",
    "jsentinel-startup",
    "jsentinel-device-access"
)

foreach ($crate in $crateNames) {
    Require-Path "crates\$crate\Cargo.toml"
    Require-Path "crates\$crate\src\lib.rs"

    $crateToml = Join-Path $repoRoot "crates\$crate\Cargo.toml"
    if (Test-Path $crateToml) {
        $content = Get-Content $crateToml -Raw
        if ($content -notmatch "name\s*=\s*`"$crate`"") {
            Report-Error "Crate package name mismatch in crates\$crate\Cargo.toml"
        }
    }
}

$workspaceToml = Get-Content (Join-Path $repoRoot "Cargo.toml") -Raw
foreach ($crate in $crateNames) {
    $memberPath = "crates/$crate"
    if ($workspaceToml -notmatch [regex]::Escape($memberPath)) {
        Report-Error "Workspace does not list member: $memberPath"
    }
}

$iconCheckScript = Join-Path $repoRoot "scripts\dev\check-tauri-icons.ps1"
if (Test-Path $iconCheckScript) {
    Write-Host "Running Tauri icon validation"
    powershell -ExecutionPolicy Bypass -File $iconCheckScript
    if ($LASTEXITCODE -ne 0) {
        Report-Error "Tauri icon validation failed"
    }
}

if (Get-Command cargo -ErrorAction SilentlyContinue) {
    Write-Host "Running cargo check --workspace"
    cargo check --workspace
    if ($LASTEXITCODE -ne 0) {
        Report-Error "cargo check --workspace failed"
    }
} else {
    Report-Warning "cargo is not available; skipping Rust workspace build check."
}

$uiPath = Join-Path $repoRoot "apps\desktop-ui"
if (Get-Command npm -ErrorAction SilentlyContinue) {
    Push-Location $uiPath
    if ((Test-Path "node_modules") -or (-not $SkipNpmInstall)) {
        if (-not (Test-Path "node_modules")) {
            Write-Host "Running npm install in apps\desktop-ui"
            npm install
            if ($LASTEXITCODE -ne 0) {
                Report-Error "npm install failed"
            }
        }

        if ($failures -eq 0) {
            Write-Host "Running npm run build in apps\desktop-ui"
            npm run build
            if ($LASTEXITCODE -ne 0) {
                Report-Error "npm run build failed"
            }
        }
    } else {
        Report-Warning "npm is available, but node_modules is missing and -SkipNpmInstall was set; skipping UI build."
    }
    Pop-Location
} else {
    Report-Warning "npm is not available; skipping UI dependency and build checks."
}

if ($failures -gt 0) {
    Write-Host "Foundation validation failed with $failures structural/build error(s)." -ForegroundColor Red
    exit 1
}

Write-Host "Foundation validation passed." -ForegroundColor Green
exit 0
