param(
    [string]$PngPath = "apps\desktop-ui\src-tauri\icons\128x128.png",
    [string]$IcoPath = "apps\desktop-ui\src-tauri\icons\icon.ico"
)

$ErrorActionPreference = "Stop"
$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
$resolvedPng = Join-Path $repoRoot $PngPath
$resolvedIco = Join-Path $repoRoot $IcoPath

if (-not (Test-Path $resolvedPng)) {
    throw "PNG source not found: $PngPath"
}

$pngBytes = [System.IO.File]::ReadAllBytes($resolvedPng)
$pngSignature = [byte[]](0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A)

for ($index = 0; $index -lt $pngSignature.Length; $index++) {
    if ($pngBytes[$index] -ne $pngSignature[$index]) {
        throw "Source file is not a PNG: $PngPath"
    }
}

$header = New-Object byte[] 22

# ICONDIR
[BitConverter]::GetBytes([UInt16]0).CopyTo($header, 0) # reserved
[BitConverter]::GetBytes([UInt16]1).CopyTo($header, 2) # type: icon
[BitConverter]::GetBytes([UInt16]1).CopyTo($header, 4) # count

# ICONDIRENTRY
$header[6] = 128 # width
$header[7] = 128 # height
$header[8] = 0 # color count
$header[9] = 0 # reserved
[BitConverter]::GetBytes([UInt16]1).CopyTo($header, 10) # planes
[BitConverter]::GetBytes([UInt16]32).CopyTo($header, 12) # bit count
[BitConverter]::GetBytes([UInt32]$pngBytes.Length).CopyTo($header, 14) # bytes in resource
[BitConverter]::GetBytes([UInt32]22).CopyTo($header, 18) # image offset

$output = New-Object byte[] ($header.Length + $pngBytes.Length)
$header.CopyTo($output, 0)
$pngBytes.CopyTo($output, $header.Length)

$targetDirectory = Split-Path -Parent $resolvedIco
New-Item -ItemType Directory -Force -Path $targetDirectory | Out-Null
[System.IO.File]::WriteAllBytes($resolvedIco, $output)

Write-Host "Generated valid ICO: $IcoPath"
Write-Host "Embedded PNG bytes: $($pngBytes.Length)"
