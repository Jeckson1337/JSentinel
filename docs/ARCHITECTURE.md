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
