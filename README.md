# JSentinel

JSentinel is a local-first privacy and security control center for Windows 10/11 and Linux.

It is designed to help regular users understand what is happening on their own computer: active processes, network activity, startup entries, events, and items that may deserve attention.

JSentinel is not an antivirus. It does not replace dedicated protection tools, and OS limitations apply.

## Foundation Principles

- Local-first by default.
- No telemetry.
- No accounts.
- No advertising SDKs.
- No tracking.
- No forced cloud.
- No kernel driver in v1.
- No irreversible force-delete in v1.
- Dangerous actions require explicit confirmation.
- Quarantine and restore are preferred over irreversible deletion.
- The UI must not execute privileged actions directly.
- OS-specific backends stay separated from UI and core logic.
- Windows 10/11 is the primary first platform.
- Linux support is planned architecturally and may start as beta.

## Workspace

This repository is a Rust workspace with a Tauri + React + TypeScript desktop UI scaffold.

Core Rust crates live under `crates/`.
The desktop UI lives under `apps/desktop-ui/`.
Services, installers, release scripts, and documentation are intentionally separated.

## Current Status

Package 0: foundation/docs/scaffold.

This package contains only safe placeholders and architecture documentation. Real Windows/Linux system integrations are intentionally not implemented yet.
