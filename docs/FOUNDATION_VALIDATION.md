# Foundation Validation

This document describes how to validate the Package 0 scaffold before building Package 1.

## What to Check

The repository root must contain:

- `Cargo.toml`
- `README.md`
- `apps/desktop-ui`
- `crates/`
- `docs/`
- `scripts/release/`

The repository root must not contain accidental nested project folders:

- `JSentinel/`
- `sentinel-local/`
- `SentinelLocal/`

## Validation Commands

Windows:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\dev\check-foundation.ps1
```

Linux/macOS:

```sh
sh scripts/dev/check-foundation.sh
```

If you want to avoid dependency installation during a quick check:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\dev\check-foundation.ps1 -SkipNpmInstall
```

```sh
SKIP_NPM_INSTALL=1 sh scripts/dev/check-foundation.sh
```

## Pass Criteria

A validation pass means:

- Required root files and directories exist.
- No accidental nested project directory exists.
- Every expected Rust crate exists.
- Every crate has `Cargo.toml` and `src/lib.rs`.
- Workspace members point to real crate paths.
- Crate package names use the `jsentinel-*` naming pattern.
- `cargo check --workspace` passes when Cargo is installed.
- `npm run build` passes when npm and frontend dependencies are available.

## Fail Criteria

A validation failure means:

- A required file or directory is missing.
- An accidental nested project directory exists.
- A workspace member points to a missing path.
- A crate is missing `Cargo.toml` or `src/lib.rs`.
- A crate package name does not match its expected `jsentinel-*` name.
- Cargo or npm build checks fail after the relevant toolchain is available.

## Missing Toolchains

If Cargo, npm, or Tauri prerequisites are not installed, validation scripts should print a warning and continue with structural checks. Missing local toolchains are not treated as scaffold failure.

Install prerequisites locally for full validation:

- Rust and Cargo.
- Node.js and npm.
- Tauri v2 prerequisites for your OS.

Do not install dependencies globally unless your platform setup explicitly requires it.

## Intentionally Not Implemented

Package 0.5 still does not implement:

- Real Windows or Linux backend calls.
- Process termination.
- Firewall or network blocking.
- Registry access.
- Quarantine or restore.
- Delete-on-reboot.
- Event database.
- Telemetry, analytics, tracking, ad SDKs, accounts, or forced cloud features.

JSentinel remains a local visibility and control center scaffold, not an antivirus.
