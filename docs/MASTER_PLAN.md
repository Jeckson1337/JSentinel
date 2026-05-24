# JSentinel Master Plan

JSentinel is a local-first privacy and security control center for Windows 10/11 and Linux.

It helps users see what is happening on their own PC: active processes, network activity, startup entries, local events, file attention items, and future device-access signals.

JSentinel is not an antivirus. It must stay honest about best-effort local visibility, audit mode, and OS limitations.

## Package 0: Foundation

Scope:

- Monorepo structure.
- Rust workspace and placeholder crates.
- Tauri + React + TypeScript UI scaffold.
- Documentation for architecture, privacy, security, scope, and release process.
- Safe release script placeholders.

Out of scope:

- Real OS integration.
- Process termination.
- Firewall or network blocking.
- Quarantine implementation.
- Registry access.
- Kernel driver.
- Telemetry or remote services.

## v1 Direction

Windows 10/11 is the primary first platform. Linux support is planned architecturally and can start as beta once the Windows boundaries are stable.

v1 should prioritize read-only visibility, clear explanations, local storage controls, and reversible actions. Dangerous actions must require confirmation and must be routed through backend policy and OS-specific layers, not direct UI calls.
