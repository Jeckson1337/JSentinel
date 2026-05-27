# Startup Guard Action Model

Package 4D prepares the model for future startup disable/restore actions. It does not modify startup configuration.

## Current Read-Only Sources

The Windows backend can currently read, best-effort:

- `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`;
- `HKLM\Software\Microsoft\Windows\CurrentVersion\Run`;
- `HKLM\Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Run`;
- current user Startup folder;
- all users Startup folder, if readable.

Scheduled Tasks and Services remain unsupported/planned for Startup Guard actions.

## Backup Metadata

Package 4D adds local SQLite metadata for future reversible startup changes:

- `backup_id`;
- `entry_id`;
- `created_at`;
- `source`;
- `original_name`;
- `original_command`;
- `original_path`;
- `original_enabled_state`;
- `restore_strategy`;
- optional `metadata_json`.

This is metadata only. Creating a record does not disable, delete, move, or restore any startup item.

## Planned Actions

`disable_startup` is planned as a caution-level action because it changes launch behavior but should be reversible when implemented correctly. It will require confirmation and backup metadata before any future execution.

`restore_startup` is planned as a caution-level action. It will require a trusted matching backup record before any future execution.

In Package 4D both actions return planned/disabled status. No real executor is added.

## Safety Rules Before Real Disable

Before any future Package 4E implementation, the project must define:

- exact source-specific restore strategies;
- backup verification rules;
- scope and admin behavior;
- user confirmation text;
- local audit behavior;
- rollback behavior;
- tests that do not mutate the host machine.

## Explicit Non-Goals

Package 4D does not implement:

- registry writes;
- startup folder edits;
- scheduled task modification;
- service modification;
- startup entry deletion;
- auto-disable;
- background mitigation;
- network calls;
- telemetry;
- analytics;
- ad SDKs.
