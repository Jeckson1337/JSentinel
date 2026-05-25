# Safe Actions v1

Package 4B enables the first real non-destructive actions. These actions are policy-gated, require confirmation, and are written to the local SQLite audit log.

## Enabled Actions

### `reveal_path`

Opens a local filesystem path in Windows Explorer.

Safety rules:

- target must be a non-empty local filesystem path;
- `http://`, `https://`, `file://`, `ms-settings:`, `shell:`, `cmd:`, `powershell:`, and `javascript:` targets are rejected;
- raw command names such as `cmd.exe`, `powershell.exe`, and `pwsh.exe` are rejected;
- embedded null characters and leading/trailing whitespace are rejected;
- UNC/network paths are rejected until explicitly designed later;
- target must already exist;
- JSentinel does not create, delete, move, rename, execute, or change permissions on the target;
- files may be shown in their containing folder, folders may be opened directly;
- non-Windows platforms return unsupported in Package 4B.

### `open_windows_settings`

Opens only allowlisted Windows Settings pages.

Allowlist:

- `ms-settings:privacy`
- `ms-settings:privacy-microphone`
- `ms-settings:privacy-webcam`
- `ms-settings:appsfeatures`
- `ms-settings:startupapps`
- `ms-settings:network-status`
- `ms-settings:windowsdefender`

Safety rules:

- arbitrary URLs are rejected;
- external browser links are not opened;
- raw user-provided URLs are not trusted;
- malformed or whitespace-padded settings values are rejected;
- command execution is not built from concatenated shell strings;
- non-Windows platforms return unsupported.

## Confirmation And Audit

The UI asks `PolicyEngine` for an `ActionPlan` before execution. The confirmation dialog shows expected effects and warnings. Every backend execution attempt writes a local `action_history` record with `succeeded`, `failed`, `denied`, or `unsupported`.

Audit data stays local. There is no upload, telemetry, account, ad SDK, or external service.

## Still Disabled

Package 4B still does not implement:

- process kill;
- firewall block/unblock;
- startup disable/restore;
- quarantine move/restore;
- delete-on-reboot;
- force unlock;
- registry writes;
- file watcher;
- mic/camera monitor;
- automatic mitigation.

Future dangerous actions must keep the same policy-confirmation-audit path and require a separate design package.
