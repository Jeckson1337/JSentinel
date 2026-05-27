# Safe Action Framework

Package 4A introduced the action planning, confirmation, and audit foundation for future control features. Package 4B enables only safe non-destructive actions. It still does not implement destructive OS actions.

## Model

Actions are described as data first:

- `ActionKind`: requested operation, such as `reveal_path`, `kill_process`, `block_network`, or `quarantine_file`.
- `ActionRiskLevel`: `safe`, `caution`, or `dangerous`.
- `ActionRequest`: what the UI asked for, including target, source screen, and metadata.
- `ActionPlan`: policy decision, availability, confirmation text, warnings, expected effects, and disabled reason.
- `ActionResult`: local audit record for completed, denied, cancelled, unsupported, failed, or dry-run execution paths.

All DTOs are serializable so Rust, Tauri, SQLite, and the UI can share one contract.

## Policy

`jsentinel-policy` owns risk classification and action planning. In the current Package 4D policy:

- `reveal_path`: safe, available, confirmation-gated, local filesystem only.
- `open_windows_settings`: safe, available, confirmation-gated, Windows Settings URI allowlist only.
- `kill_process`: dangerous, available only with PID metadata, confirmation-gated, and safety-prechecked.
- `disable_startup`: caution, planned/disabled, confirmation text prepared, backup required in the future.
- `restore_startup`: caution, planned/disabled, confirmation text prepared, backup required in the future.
- `detect_file_lockers`: caution, unsupported/planned.
- `block_network`, `unblock_network`, `quarantine_file`, `restore_quarantine`, and `schedule_delete_on_reboot`: dangerous and planned/disabled.

Dangerous actions return a disabled reason: the framework is prepared, but implementation belongs to a later package.

## Confirmation

The UI asks the backend for an `ActionPlan` before showing confirmation. Confirmation copy comes from policy output, not from ad-hoc screen logic. Confirmed Package 4B safe actions may open Explorer or an allowlisted Windows Settings page. No file, registry, firewall, process, startup, or permission mutation happens.

Package 4B.5 hardens this path by rejecting URL-like path targets, shell-like schemes, embedded null characters, UNC paths, malformed settings URIs, and non-allowlisted settings pages before OS launch.

Package 4C adds `kill_process` for one PID only. It re-plans the request in the backend, requires confirmation, denies protected/system/self targets, and writes every attempt to local action history.

Package 4D adds Startup Guard planning DTOs and local backup metadata storage. It does not execute startup disable/restore and does not write the registry, edit Startup folders, modify scheduled tasks, or modify services.

## Audit Log

Action results are stored locally in SQLite `action_history`. The log supports filtering by action kind, status, text search, limit, and newest-first ordering.

The audit log is local-only. There is no upload, telemetry, account, or external service.

## Explicit Non-Goals

Package 4D does not implement:

- firewall block/unblock;
- startup disable/restore;
- quarantine move/restore;
- delete-on-reboot;
- force unlock;
- registry writes;
- automatic mitigation;
- background auto-actions.

Future action packages must keep the same policy-confirmation-audit path and add real actions only under explicit safety rules.
