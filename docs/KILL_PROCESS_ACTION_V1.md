# Kill Process Action v1

Package 4C adds the first controlled destructive action: terminate one running process by PID.

This is not malware removal, not quarantine, not firewall control, and not startup cleanup. It is a narrow user-confirmed process termination action with policy, precheck, and audit.

## Enabled Action

`kill_process`

Rules:

- requires explicit user confirmation;
- requires PID metadata;
- re-queries process details before termination;
- terminates only one PID;
- does not kill a process tree;
- does not kill by process name;
- does not retry in a loop;
- does not elevate to admin;
- does not close file handles;
- does not unlock files;
- does not delete files;
- does not remove startup entries;
- does not block network access.

## Hard Deny

Package 4C denies:

- PID `0`;
- JSentinel's own process;
- the parent desktop process when detectable;
- unknown or unverified process names;
- `System`;
- `Idle`;
- `Registry`;
- `smss.exe`;
- `csrss.exe`;
- `wininit.exe`;
- `winlogon.exe`;
- `services.exe`;
- `lsass.exe`;
- `lsm.exe`;
- `svchost.exe`;
- `fontdrvhost.exe`;
- `dwm.exe`;
- `conhost.exe`;
- `explorer.exe`;
- processes under `C:\Windows\System32`;
- processes under `C:\Windows\SysWOW64`.

If the process cannot be opened because of permissions or OS protection, JSentinel reports failure/denied and does not attempt elevation.

## Confirmation

The confirmation dialog must show the action kind, target, display name, dangerous risk, expected effects, and warnings.

Expected effects include:

- terminates one running process by PID;
- unsaved work in that process may be lost;
- does not delete files;
- does not remove startup entries;
- does not block network access.

## Audit

Every backend execution attempt writes local `action_history`.

Statuses:

- `succeeded`: one allowed PID termination was requested successfully;
- `denied`: policy or safety precheck denied the request;
- `failed`: OS call failed;
- `unsupported`: platform does not support this action.

Audit remains local-only. There is no upload, telemetry, analytics, account, ad SDK, or external service.

## Still Disabled

Package 4C still does not implement:

- firewall block/unblock;
- startup disable/restore;
- quarantine move/restore;
- delete-on-reboot;
- force unlock;
- registry writes;
- file watcher;
- mic/camera monitor;
- automatic mitigation.
