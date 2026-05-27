# What JSentinel Can and Cannot Do

## Can Do

- Provide a local-first control center architecture.
- Present UI placeholders for Dashboard, Timeline, Processes, Network, Files, Startup, Settings, and About.
- Document privacy, security, release, and architecture principles.
- Prepare safe Rust crate boundaries for future implementation.
- Store and display local/mock audit-style events.
- On Windows, query read-only process, network, and startup snapshots best-effort.
- Show explicit live/fallback/unsupported status in UI.
- Refresh read-only snapshots manually.
- Show local read-only diagnostic counts without upload.
- Plan future actions through a policy layer.
- Show confirmation dialogs and disabled reasons for actions.
- Store local action audit records for denied, dry-run, unsupported, or completed action paths.
- Open existing local filesystem paths in Windows Explorer after confirmation.
- Open allowlisted Windows Settings pages after confirmation.
- Terminate one non-protected, non-system process by PID after confirmation, live backend verification, and safety checks.
- Store local Startup Guard backup metadata for future restore planning.
- Show planned/disabled startup disable/restore previews without changing the OS.

## Cannot Do Yet

- Provide real-time monitoring.
- Guarantee complete process, network, startup, or file visibility.
- Parse scheduled tasks in the Windows startup backend.
- Detect file lockers with Restart Manager.
- Force-unlock files.
- Export or upload private diagnostic dumps automatically.
- Quarantine files.
- Restore files.
- Delete files.
- Block network access.
- Modify firewall rules.
- Write the Windows registry.
- Disable or restore startup entries.
- Modify scheduled tasks or services.
- Delete startup entries.
- Install services or daemons.
- Execute destructive actions other than the Package 4C single-PID kill action.
- Run automatic mitigation in the background.
- Open arbitrary external URLs from action input.
- Execute arbitrary shell commands from action input.
- Kill process trees.
- Kill processes by name.
- Automatically kill processes.
- Kill JSentinel itself, its parent process when detectable, protected/system processes, or Windows-directory processes.

## Will Not Claim

JSentinel will not claim to be an antivirus or guarantee complete detection of malware or suspicious behavior.
