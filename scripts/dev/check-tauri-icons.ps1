$ErrorActionPreference = "Continue"
$failures = 0
$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
$iconsRoot = Join-Path $repoRoot "apps\desktop-ui\src-tauri\icons"
$icoPath = Join-Path $iconsRoot "icon.ico"

function Report-Error {
    param([string]$Message)
    Write-Host "ERROR: $Message" -ForegroundColor Red
    $script:failures += 1
}

function Check-File {
    param([string]$Path)
    if (-not (Test-Path $Path)) {
        Report-Error "Missing icon file: $Path"
    } else {
        Write-Host "OK: $Path"
    }
}

Check-File (Join-Path $iconsRoot "icon.ico")
Check-File (Join-Path $iconsRoot "icon.png")
Check-File (Join-Path $iconsRoot "32x32.png")
Check-File (Join-Path $iconsRoot "128x128.png")
Check-File (Join-Path $iconsRoot "128x128@2x.png")

if (Test-Path $icoPath) {
    $bytes = [System.IO.File]::ReadAllBytes($icoPath)

    if ($bytes.Length -le 22) {
        Report-Error "icon.ico is too small to contain ICONDIR and ICONDIRENTRY."
    } else {
        $reserved = [BitConverter]::ToUInt16($bytes, 0)
        $type = [BitConverter]::ToUInt16($bytes, 2)
        $count = [BitConverter]::ToUInt16($bytes, 4)
        $entryReserved = $bytes[9]
        $bytesInResource = [BitConverter]::ToUInt32($bytes, 14)
        $imageOffset = [BitConverter]::ToUInt32($bytes, 18)

        if ($reserved -ne 0) {
            Report-Error "ICONDIR reserved must be 0, got $reserved."
        }
        if ($type -ne 1) {
            Report-Error "ICONDIR type must be 1 for icon, got $type."
        }
        if ($count -lt 1) {
            Report-Error "ICONDIR count must be at least 1, got $count."
        }
        if ($entryReserved -ne 0) {
            Report-Error "ICONDIRENTRY reserved byte must be 0, got $entryReserved."
        }
        if ($imageOffset -ge $bytes.Length) {
            Report-Error "ICONDIRENTRY image offset points outside icon.ico."
        }
        if (($imageOffset + $bytesInResource) -gt $bytes.Length) {
            Report-Error "ICONDIRENTRY image byte range points outside icon.ico."
        }

        if (($imageOffset + 4) -le $bytes.Length) {
            $isPng = $bytes[$imageOffset] -eq 0x89 `
                -and $bytes[$imageOffset + 1] -eq 0x50 `
                -and $bytes[$imageOffset + 2] -eq 0x4E `
                -and $bytes[$imageOffset + 3] -eq 0x47
            $isDib = ($bytes.Length - $imageOffset) -ge 4 `
                -and [BitConverter]::ToUInt32($bytes, [int]$imageOffset) -ge 40

            if (-not ($isPng -or $isDib)) {
                Report-Error "Embedded ICO image is neither PNG nor a plausible BMP/DIB header."
            }
        } else {
            Report-Error "ICONDIRENTRY image offset leaves fewer than 4 bytes for an image header."
        }
    }
}

if ($failures -gt 0) {
    Write-Host "Tauri icon validation failed with $failures error(s)." -ForegroundColor Red
    exit 1
}

Write-Host "Tauri icon validation passed." -ForegroundColor Green
exit 0
