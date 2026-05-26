# JSentinel Desktop UI

Tauri + React + TypeScript scaffold for the JSentinel desktop application.

This UI routes actions through the Tauri/backend policy boundary. It does not directly execute privileged actions, read system registries, block network traffic, quarantine files, or make network requests. Package 4C enables only confirmed single-PID process termination through the backend action framework.

## Manual Setup

If dependencies are not installed yet:

```powershell
npm install
npm run dev
```

For Tauri development after Rust/Tauri prerequisites are installed:

```powershell
npm run tauri dev
```

## Runtime Rules

- Static sponsor placeholder only.
- Locale data is bundled locally.
- No telemetry.
- No tracking IDs.
- No analytics.
- No ad SDK.
- No cookies.
- No fingerprinting.
- No forced cloud.

## Local Storage in Package 1

Package 1 uses a safe development fallback SQLite path under `.jsentinel-dev/jsentinel.sqlite3` relative to the Tauri process working directory. This avoids writing to system directories while the final app data path policy is still being designed.
