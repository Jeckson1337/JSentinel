# Release Process

JSentinel releases are distributed through GitHub Releases.

## Planned Artifacts

Windows:

- `JSentinel-Setup-x64.exe`
- `JSentinel-Portable-x64.zip` if portable mode is possible

Linux:

- `JSentinel-x86_64.AppImage`
- `JSentinel-amd64.deb`

## Required Release Notes

Each release must include:

- Changelog.
- SHA256 checksums.
- Known limitations.
- Privacy statement.
- Supported platforms.
- Clear warning that JSentinel is not an antivirus.

## Auto-Update Policy

The first version should not include an auto-updater if it requires a server, telemetry, tracking, or unclear network behavior. GitHub Releases are the primary distribution channel.
