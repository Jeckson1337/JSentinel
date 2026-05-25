# Safe Action Framework

Package 4A introduces the action planning, confirmation, and audit foundation for future control features. It does not implement destructive OS actions.

## Model

Actions are described as data first:

- `ActionKind`: requested operation, such as `reveal_path`, `kill_process`, `block_network`, or `quarantine_file`.
- `ActionRiskLevel`: `safe`, `caution`, or `dangerous`.
- `ActionRequest`: what the UI asked for, including target, source screen, and metadata.
- `ActionPlan`: policy decision, availability, confirmation text, warnings, expected effects, and disabled reason.
- `ActionResult`: local audit record for completed, denied, cancelled, unsupported, or dry-run execution paths.

All DTOs are serializable so Rust, Tauri, SQLite, and the UI can share one contract.

## Policy

`jsentinel-policy` owns risk classification and action planning. In Package 4A the policy is intentionally strict:

- `reveal_path`: safe, confirmation-gated, dry-run only.
- `open_windows_settings`: safe, confirmation-gated, dry-run only.
- `detect_file_lockers`: caution, unsupported/planned.
- `kill_process`, `block_network`, `unblock_network`, `disable_startup`, `restore_startup`, `quarantine_file`, `restore_quarantine`, and `schedule_delete_on_reboot`: dangerous and planned/disabled.

Dangerous actions return a disabled reason: the framework is prepared, but implementation belongs to a later package.

## Confirmation

The UI asks the backend for an `ActionPlan` before showing confirmation. Confirmation copy comes from policy output, not from ad-hoc screen logic. A confirmed Package 4A safe action is recorded as `dry_run`; no OS mutation happens.

## Audit Log

Action results are stored locally in SQLite `action_history`. The log supports filtering by action kind, status, text search, limit, and newest-first ordering.

The audit log is local-only. There is no upload, telemetry, account, or external service.

## Explicit Non-Goals

Package 4A does not implement:

- process kill;
- firewall block/unblock;
- startup disable/restore;
- quarantine move/restore;
- delete-on-reboot;
- force unlock;
- registry writes;
- automatic mitigation;
- background auto-actions.

Future action packages must keep the same policy-confirmation-audit path and add real actions only under explicit safety rules.
