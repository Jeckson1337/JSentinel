# jsentinel-windows

Windows-specific backend boundary.

Windows 10/11 is the first target platform. Package 3 adds read-only snapshot support for:

- process inventory;
- process details;
- TCP/UDP connection inventory;
- startup entries from Registry Run keys and Startup folders.

This crate must not implement process kill, firewall changes, registry writes, startup disable/restore, quarantine, delete-on-reboot, force unlock, telemetry, analytics, ad SDKs, or runtime external network calls.

File locker detection is currently an explicit unsupported contract. It does not inspect or close handles.
