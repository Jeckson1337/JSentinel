$ErrorActionPreference = "Continue"
$failures = 0
$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")

function Run-Step {
    param(
        [string]$Name,
        [scriptblock]$Command
    )

    Write-Host "== $Name =="
    & $Command
    if ($LASTEXITCODE -ne 0) {
        Write-Host "ERROR: $Name failed" -ForegroundColor Red
        $script:failures += 1
    }
}

Set-Location $repoRoot

Run-Step "Foundation validation" {
    powershell -ExecutionPolicy Bypass -File scripts\dev\check-foundation.ps1 -SkipNpmInstall
}

Run-Step "Rust checks" {
    powershell -ExecutionPolicy Bypass -File scripts\dev\check-rust.ps1
}

Run-Step "Frontend checks" {
    powershell -ExecutionPolicy Bypass -File scripts\dev\check-frontend.ps1
}

if ($failures -gt 0) {
    Write-Host "check-all failed with $failures step error(s)." -ForegroundColor Red
    exit 1
}

Write-Host "check-all passed." -ForegroundColor Green
exit 0
