# Kill Process Security Review

Package 4C.5 reviews and hardens the `kill_process` action before any other destructive actions are considered.

## Threat Model

`kill_process` can cause data loss if a user terminates an application with unsaved work. It can also destabilize Windows if used against shell, service, protected, or system processes. JSentinel therefore treats process termination as a dangerous action, not as malware removal.

The primary risks are:

- UI or caller attempts to bypass policy;
- stale UI data points to the wrong process;
- a process disappears between precheck and execution;
- a system/protected process is selected;
- PID metadata is missing and a name-only kill is attempted;
- command-line or other sensitive process details are stored unnecessarily.

## Required Path

The only supported path is:

1. UI creates an `ActionRequest` with `kind = kill_process` and explicit PID metadata.
2. `PolicyEngine` classifies the action as dangerous and requires confirmation.
3. The UI shows the confirmation dialog.
4. The Tauri command calls `SafeActionExecutor`.
5. The Windows adapter re-queries process details by PID.
6. The safety check runs immediately before `TerminateProcess`.
7. The backend calls `TerminateProcess` at most once.
8. The result is written to local `action_history`.

The UI-provided process name/path are display context only. The Windows backend does not trust them as the source of truth for execution.

## Hard Deny

Package 4C.5 denies:

- PID `0`;
- JSentinel's current process;
- JSentinel's parent desktop process when detectable;
- missing or unverified process name;
- missing process path;
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
- processes under `C:\Windows`;
- processes under `C:\Windows\System32`;
- processes under `C:\Windows\SysWOW64`.

`explorer.exe` and `svchost.exe` are denied in v1 because terminating them can break the shell/session or service hosting model. This can be revisited only with a more mature protected-process model.

## Explicit Non-Goals

Package 4C.5 still does not implement:

- kill by process name;
- process tree kill;
- child traversal;
- retry loops;
- admin elevation;
- `SeDebugPrivilege`;
- `AdjustTokenPrivileges`;
- handle stealing;
- force unlock;
- firewall changes;
- registry writes;
- startup disable/restore;
- quarantine;
- delete-on-reboot;
- automatic mitigation.

## Audit Behavior

Every backend execution attempt returns an `ActionResult` for local audit:

- `succeeded`: `TerminateProcess` reported success for one allowed PID;
- `denied`: policy or safety denied the request;
- `failed`: the OS call failed, or the process disappeared/became unavailable between precheck and execution;
- `unsupported`: the platform does not support the action.

Audit metadata includes PID and request display context when available. Full command lines are intentionally not stored in action metadata.

There is no upload, telemetry, analytics, account, ad SDK, or remote action history.
