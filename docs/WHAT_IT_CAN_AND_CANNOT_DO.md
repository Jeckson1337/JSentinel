# What JSentinel Can and Cannot Do

## Can Do

- Provide a local-first control center architecture.
- Present UI placeholders for Dashboard, Timeline, Processes, Network, Files, Startup, Settings, and About.
- Document privacy, security, release, and architecture principles.
- Prepare safe Rust crate boundaries for future implementation.
- Store and display local/mock audit-style events.
- On Windows, query read-only process, network, and startup snapshots best-effort.
- Show explicit live/fallback/unsupported status in UI.

## Cannot Do Yet

- Provide real-time monitoring.
- Guarantee complete process, network, startup, or file visibility.
- Parse scheduled tasks in the Windows startup backend.
- Detect file lockers with Restart Manager.
- Quarantine files.
- Restore files.
- Delete files.
- Kill processes.
- Block network access.
- Modify firewall rules.
- Write the Windows registry.
- Install services or daemons.

## Will Not Claim

JSentinel will not claim to be an antivirus or guarantee complete detection of malware or suspicious behavior.
