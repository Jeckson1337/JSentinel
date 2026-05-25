# Architecture

JSentinel is split into UI, core domain crates, OS-specific backend crates, services/daemons, installer assets, scripts, and documentation.

## Boundaries

- `apps/desktop-ui`: Tauri + React + TypeScript UI shell.
- `crates/jsentinel-core`: platform-neutral shared domain types and principles.
- `crates/jsentinel-events`: local event model and future timeline types.
- `crates/jsentinel-db`: local persistence boundary.
- `crates/jsentinel-policy`: policy evaluation and action safety boundary.
- `crates/jsentinel-windows`: Windows-specific backend boundary.
- `crates/jsentinel-linux`: Linux-specific backend boundary.
- `crates/jsentinel-quarantine`: future reversible quarantine/restore boundary.
- `crates/jsentinel-network`: network visibility model.
- `crates/jsentinel-files`: file visibility model.
- `crates/jsentinel-startup`: startup visibility model.
- `crates/jsentinel-device-access`: device access visibility model.
- `services/windows-service`: future Windows service placeholder.
- `services/linux-daemon`: future Linux daemon placeholder.

## Package 1 Event Flow

Package 1 introduces a local event flow:

1. Mock/demo events are created in `jsentinel-events`.
2. `jsentinel-db` stores and queries events in local SQLite.
3. `jsentinel-core` exposes `EventService` and a read-only provider boundary.
4. Tauri commands expose local event reads and mock seeding to the UI.
5. React screens render Dashboard summary and Timeline records.

This flow is local-only and does not use real OS backends yet.

## Package 2 UI Layer

Package 2 adds a product-oriented desktop UI over the Package 1 event model. Dashboard, Timeline, Processes, Network, Files, Startup, Settings, and About are now usable screens backed by local SQLite events or explicit frontend mock fallback data.

Risky controls are visible only as disabled placeholders. The UI still does not execute privileged actions or call OS-specific backends directly.

## Package 3 Windows Read-Only Backend

Package 3 adds the first real OS backend surface for Windows, but only for read-only snapshots. Shared DTOs live in `jsentinel-core`, Windows-specific collection lives in `jsentinel-windows`, and Tauri exposes safe commands for capabilities, process inventory, network connections, startup entries, and the file-locker detection contract.

The UI tries live Windows data first and then falls back to mock/event-backed views when commands are unavailable or unsupported. File locker detection remains an explicit unsupported/best-effort contract in this package.

Package 3 still does not implement process kill, firewall changes, registry writes, startup disable/restore, quarantine, delete-on-reboot, force unlock, real-time file watching, device monitoring, telemetry, analytics, ad SDKs, or runtime external network calls.

Package 3.5 hardens this layer with structured capability status, serialized read-only backend errors, manual refresh controls, local diagnostic counts, and clearer live/partial/unsupported/mock UI states. Refresh remains read-only snapshot collection.

## Package 4A Safe Action Framework

Package 4A adds action DTOs, policy planning, confirmation UI, and a local SQLite audit log. The action boundary is data-first: UI creates an `ActionRequest`, `jsentinel-policy` returns an `ActionPlan`, the UI shows confirmation or a disabled reason, and only the backend command path can record an `ActionResult`.

In this package safe actions are dry-run only, and dangerous actions are planned/disabled. No process kill, firewall change, registry write, startup mutation, quarantine, delete-on-reboot, or force unlock is implemented.

## Privilege Model

The UI must not execute privileged actions directly. Future privileged operations should flow through:

1. UI request.
2. Policy evaluation.
3. Explicit user confirmation for risky actions.
4. Backend command boundary.
5. OS-specific implementation.
6. Local audit event.

## Platform Model

Windows 10/11 is the first primary platform. Linux backend crates and daemon placeholders exist so the architecture does not become Windows-only, but Linux may ship later or as beta.

## v1 Non-Goals

- No kernel driver.
- No force-delete.
- No kill-process action.
- No firewall/block action.
- No delete-on-reboot action.
- No direct registry mutation.
- No UI-to-privileged-action shortcut.
