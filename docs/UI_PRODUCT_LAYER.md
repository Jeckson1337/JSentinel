# UI Product Layer

Package 2 turns the initial scaffold into a usable desktop product layer over local/mock events. Package 3 adds live/fallback badges and read-only Windows data where the backend is available.

The UI still does not perform destructive or privileged actions.

## Screens

- Dashboard: summary cards, recent events, local/privacy boundaries, navigation shortcuts, and Windows backend capability status.
- Timeline: searchable/filterable local event list with a detail panel.
- Processes: read-only Windows process list when available, otherwise mock/event-backed process summary table and detail panel.
- Network: read-only Windows TCP/UDP snapshot when available, otherwise mock/event-backed network event view.
- Files: mock/event-backed file and locked-file activity view plus an honest unsupported file-locker detection contract.
- Startup: read-only Windows startup entries when available, otherwise mock/event-backed startup event view.
- Settings: language switch, privacy notes, sponsor explanation, local storage notice, demo seed action.
- About: project identity, current build status, platform direction, and repository/docs/release references.

## Data Source

The UI reads events through existing Tauri commands when available. In browser/Vite mode, it falls back to bundled frontend mock events. Mock data is always labeled as mock/demo.

For Windows read-only data, screens show one of three modes:

- Live Windows data.
- Demo/mock fallback.
- Unsupported platform.

## Disabled Actions

The UI shows disabled placeholders for risky or future actions:

- Kill process.
- Open process location.
- Quarantine.
- Block/unblock network.
- Reveal file.
- Detect lockers / check locker support.
- Delete on reboot.
- Disable/restore startup entries.
- Open startup source.

These controls are intentionally disabled because JSentinel has no privileged action backend, no policy-confirmation flow for real actions, and no OS-specific write implementation.

## Safety Boundary

The UI does not execute privileged actions directly. It does not write the registry, change firewall rules, control processes, quarantine files, start file watchers, monitor microphone/camera usage, or perform network scanning.

Backend packages should keep OS-specific read-only collection behind the core provider boundary and keep actions behind policy and confirmation.
