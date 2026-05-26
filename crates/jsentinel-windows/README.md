# jsentinel-windows

Windows-specific backend boundary.

Windows 10/11 is the first target platform. Package 3 adds read-only snapshot support for:

- process inventory;
- process details;
- TCP/UDP connection inventory;
- startup entries from Registry Run keys and Startup folders.

Package 4C adds a narrow action adapter for single-PID process termination through `OpenProcess`/`TerminateProcess` after policy and safety checks. It must not kill by name, kill process trees, retry, escalate privileges, or terminate protected/system processes.

This crate must not implement firewall changes, registry writes, startup disable/restore, quarantine, delete-on-reboot, force unlock, telemetry, analytics, ad SDKs, or runtime external network calls.

File locker detection is currently an explicit unsupported contract. It does not inspect or close handles.
