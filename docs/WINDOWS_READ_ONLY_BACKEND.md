# Windows Read-Only Backend

Package 3 introduces the first Windows backend surface for JSentinel. It is intentionally read-only.

## Supported Data Sources

- Process inventory: best-effort `Win32_Process` snapshot.
- Process details: filtered view of the same read-only process snapshot.
- Network connections: point-in-time TCP/UDP endpoint snapshot with best-effort owning PID mapping.
- Startup entries:
  - `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`
  - `HKLM\Software\Microsoft\Windows\CurrentVersion\Run`
  - `HKLM\Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Run`
  - current user Startup folder
  - all users Startup folder, if readable

Scheduled task parsing is not implemented yet.

Package 4D adds local backup metadata and planning DTOs for future Startup Guard actions, but this backend remains read-only for startup sources.

## Best-Effort / Unsupported

File locker detection is represented as an explicit contract, but Package 3 returns `unsupported`.

The read-only snapshot backend does not use Restart Manager, inspect open handles, close handles, unlock files, or schedule delete-on-reboot. Package 4C adds a separate action-layer `kill_process` path for one PID after policy, safety precheck, confirmation, and audit; it is not part of snapshot collection.

## Capabilities And Limitations

Every read-only result includes:

- platform;
- provider;
- capability id and label;
- supported/unsupported status;
- partial/best-effort status where visibility is incomplete;
- whether admin is expected;
- data source;
- `read_only = true`;
- limitation message when data is unavailable or partial.

The backend should not silently pretend that unsupported data is available.

## Error Model

Read-only commands return structured diagnostics instead of panicking. Errors are serialized with one of these categories:

- unsupported platform;
- permission denied;
- unavailable;
- parse error;
- OS error;
- unknown.

The UI may show fallback data, but it must label fallback data as mock/demo and must not present it as live system state.

## Refresh Model

Refresh controls repeat the same read-only snapshot queries. Refresh does not modify the OS, does not start real-time monitoring, does not export logs, and does not upload data.

## What Is Not Implemented

The read-only backend does not implement:

- firewall rules or network blocking;
- registry writes;
- startup disable/restore;
- quarantine;
- file deletion;
- delete-on-reboot;
- force unlock;
- real-time file watching;
- microphone or camera monitoring;
- telemetry, analytics, ad SDKs, or runtime external network calls.

## UI Behavior

The desktop UI tries the Tauri read-only commands first. If the commands are unavailable, fail, or report unsupported capability, the UI falls back to local mock/event-backed data and shows an explicit badge:

- Live Windows data;
- Demo/mock fallback;
- Unsupported platform.

Potentially dangerous actions remain policy-gated. Package 4C enables only guarded single-PID process termination from process context; firewall, startup, quarantine, delete, and unlock actions remain disabled/planned.

Startup Guard buttons may show planned/disabled state and backup availability. They do not write registry values, edit Startup folders, modify Scheduled Tasks, modify Services, or auto-disable entries.
