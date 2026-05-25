# UI Product Layer

Package 2 turns the initial scaffold into a usable desktop product layer over local/mock events.

It does not add real Windows or Linux system backends.

## Screens

- Dashboard: summary cards, recent events, local/privacy boundaries, and navigation shortcuts.
- Timeline: searchable/filterable local event list with a detail panel.
- Processes: mock/event-backed process summary table and detail panel.
- Network: mock/event-backed network event view and managed rules placeholder.
- Files: mock/event-backed file and locked-file activity view.
- Startup: mock/event-backed startup event view.
- Settings: language switch, privacy notes, sponsor explanation, local storage notice, demo seed action.
- About: project identity, current build status, platform direction, and repository/docs/release references.

## Data Source

The UI reads events through existing Tauri commands when available. In browser/Vite mode, it falls back to bundled frontend mock events. Mock data is always labeled as mock/demo.

## Disabled Actions

The UI shows disabled placeholders for risky or future actions:

- Kill process.
- Open process location.
- Quarantine.
- Block/unblock network.
- Reveal file.
- Detect lockers.
- Delete on reboot.
- Disable/restore startup entries.
- Open startup source.

These controls are intentionally disabled because Package 2 has no privileged backend, no policy-confirmation flow for real actions, and no OS-specific read/write implementation.

## Safety Boundary

The UI does not execute privileged actions directly. It does not access the registry, firewall, process control APIs, file watchers, microphone/camera monitors, or network scanning APIs.

Future backend packages should keep OS-specific read-only collection behind the core provider boundary and keep actions behind policy and confirmation.
