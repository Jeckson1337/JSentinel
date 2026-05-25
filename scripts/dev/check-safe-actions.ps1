param()

$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
$actionsFile = Join-Path $repoRoot "crates\jsentinel-core\src\actions.rs"

Write-Host "Safe actions check: $actionsFile"

if (-not (Test-Path $actionsFile)) {
    Write-Error "Missing safe action executor: $actionsFile"
    exit 1
}

$content = Get-Content $actionsFile -Raw
$errors = @()

foreach ($pattern in @(
    'Command::new\("cmd',
    'Command::new\("powershell',
    'Command::new\("pwsh',
    'cmd /C',
    'cmd.exe /C'
)) {
    if ($content -match $pattern) {
        $errors += "Forbidden command execution pattern found: $pattern"
    }
}

foreach ($required in @(
    'Command::new("explorer.exe")',
    'is_allowed_windows_settings_uri',
    'validate_reveal_path',
    'starts_with("http://")',
    'starts_with("https://")',
    'starts_with("file://")',
    'starts_with("shell:")',
    'starts_with("cmd:")',
    'starts_with("powershell:")',
    'starts_with("javascript:")',
    'starts_with("\\\\")',
    "contains('\0')"
)) {
    if (-not $content.Contains($required)) {
        $errors += "Expected safe action guard not found: $required"
    }
}

if ($errors.Count -gt 0) {
    foreach ($errorMessage in $errors) {
        Write-Error $errorMessage
    }
    exit 1
}

Write-Host "Safe actions static check passed."
