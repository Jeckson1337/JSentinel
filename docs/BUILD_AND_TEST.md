# Build and Test

Package 1.5 focuses on build verification and compile hardening. It does not add real Windows or Linux system backends.

## Prerequisites on Windows

Install:

- Rust stable with Cargo.
- Node.js with npm.
- Tauri v2 prerequisites for Windows desktop builds.

Rust is usually installed with `rustup`. Node.js can be installed from the official Node.js installer or a trusted package manager. Do not install global project dependencies unless a platform prerequisite explicitly requires it.

## Main Checks

From the repository root:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\dev\check-foundation.ps1 -SkipNpmInstall
powershell -ExecutionPolicy Bypass -File scripts\dev\check-rust.ps1
powershell -ExecutionPolicy Bypass -File scripts\dev\check-frontend.ps1
powershell -ExecutionPolicy Bypass -File scripts\dev\check-all.ps1
```

Manual equivalents:

```powershell
cargo check --workspace
cargo test --workspace
cd apps\desktop-ui
npm install
npm run build
```

The root Rust workspace includes the Tauri Rust package under `apps/desktop-ui/src-tauri`, so `cargo check --workspace` validates both the shared crates and the Tauri command layer.

Tauri icon assets can be checked locally with:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\dev\check-tauri-icons.ps1
```

## Linux/macOS Shell Scripts

```sh
sh scripts/dev/check-foundation.sh
sh scripts/dev/check-rust.sh
sh scripts/dev/check-frontend.sh
sh scripts/dev/check-all.sh
```

## If Cargo or npm Is Missing

The dev scripts print a warning and skip the unavailable toolchain. Missing toolchains are not treated as structural scaffold failure, but a package should not be considered fully build-verified until the checks run on a machine with Rust/Cargo and Node/npm installed.

## CI

GitHub Actions runs:

- `cargo check --workspace`
- `cargo test --workspace`
- frontend dependency install
- `npm run build` in `apps/desktop-ui`

CI does not publish releases, upload build artifacts, enable telemetry, or run an auto-updater.

## Package 1.5 Non-Goals

Package 1.5 does not implement:

- Windows process listing.
- Registry reads or writes.
- Firewall changes.
- Process kill.
- Quarantine or restore.
- Delete-on-reboot.
- Real file watching.
- Real microphone/camera monitoring.
- Network scanning.
- Telemetry, analytics, ad SDKs, or runtime external network calls.
