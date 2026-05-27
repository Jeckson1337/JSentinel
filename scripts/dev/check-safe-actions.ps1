param()

$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
$actionsFile = Join-Path $repoRoot "crates\jsentinel-core\src\actions.rs"
$windowsFile = Join-Path $repoRoot "crates\jsentinel-windows\src\lib.rs"

Write-Host "Safe actions check: $actionsFile"

if (-not (Test-Path $actionsFile)) {
    Write-Error "Missing safe action executor: $actionsFile"
    exit 1
}

$content = Get-Content $actionsFile -Raw
$windowsContent = if (Test-Path $windowsFile) { Get-Content $windowsFile -Raw } else { "" }
$errors = @()

foreach ($pattern in @(
    'Command::new\("cmd',
    'Command::new\("powershell',
    'Command::new\("pwsh',
    'cmd /C',
    'cmd.exe /C',
    'powershell -',
    'pwsh -'
)) {
    if ($content -match $pattern) {
        $errors += "Forbidden command execution pattern found in safe action executor: $pattern"
    }
}

foreach ($pattern in @(
    'taskkill',
    'Stop-Process',
    'kill /f',
    'TerminateJobObject',
    'SeDebugPrivilege',
    'AdjustTokenPrivileges',
    'CreateToolhelp32Snapshot',
    'Process32First',
    'Process32Next',
    'netsh',
    'New-NetFirewallRule',
    'Set-NetFirewallRule',
    'Remove-NetFirewallRule',
    'Set-ItemProperty',
    'New-ItemProperty',
    'Remove-ItemProperty',
    'Disable-ScheduledTask',
    'Enable-ScheduledTask',
    'Unregister-ScheduledTask',
    'Set-Service',
    'Stop-Service',
    'Start-Service'
)) {
    if (($content -match $pattern) -or ($windowsContent -match $pattern)) {
        $errors += "Forbidden dangerous-action pattern found in source: $pattern"
    }
}

foreach ($required in @(
    'TerminateProcess',
    'OpenProcess',
    'precheck_kill_process',
    'is_self_or_parent_process',
    'current_process_id'
)) {
    if (-not $windowsContent.Contains($required)) {
        $errors += "Expected kill-process guard/API not found: $required"
    }
}

foreach ($required in @(
    'is_windows_directory_path',
    'Process path is unavailable',
    'Command line is intentionally not stored'
)) {
    if (-not $content.Contains($required)) {
        $errors += "Expected kill-process hardening guard not found: $required"
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
