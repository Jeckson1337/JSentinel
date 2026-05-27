# Database

Package 1 adds a local SQLite storage layer in `crates/jsentinel-db`.

The database is local-only. It does not sync, call remote services, create accounts, send telemetry, or publish events.

## Development Path

The current Tauri integration uses a safe development fallback path:

```text
.jsentinel-dev/jsentinel.sqlite3
```

This path is relative to the Tauri process working directory. It avoids system directories while the final app data path policy is still being designed.

## Tables

### events

- `id TEXT PRIMARY KEY`
- `timestamp TEXT NOT NULL`
- `kind TEXT NOT NULL`
- `severity TEXT NOT NULL`
- `source TEXT NOT NULL`
- `process_pid INTEGER NULL`
- `process_name TEXT NULL`
- `process_path TEXT NULL`
- `title TEXT NOT NULL`
- `summary TEXT NOT NULL`
- `target TEXT NULL`
- `metadata_json TEXT NULL`
- `created_at TEXT NOT NULL`

### settings

- `key TEXT PRIMARY KEY`
- `value TEXT NOT NULL`
- `updated_at TEXT NOT NULL`

### schema_migrations

- `version INTEGER PRIMARY KEY`
- `applied_at TEXT NOT NULL`

### action_history

Local action audit records:

- `id TEXT PRIMARY KEY`
- `timestamp TEXT NOT NULL`
- `action_type TEXT NOT NULL`
- `target TEXT NOT NULL`
- `risk_level TEXT NOT NULL`
- `result TEXT NOT NULL`
- `error TEXT NULL`
- `message TEXT NULL`
- `started_at TEXT NULL`
- `finished_at TEXT NULL`
- `metadata_json TEXT NULL`

### startup_backups

Prepared in Package 4D for future Startup Guard restore planning only:

- `backup_id TEXT PRIMARY KEY`
- `entry_id TEXT NOT NULL`
- `created_at TEXT NOT NULL`
- `source TEXT NOT NULL`
- `original_name TEXT NOT NULL`
- `original_command TEXT NOT NULL`
- `original_path TEXT NULL`
- `original_enabled_state TEXT NOT NULL`
- `restore_strategy TEXT NOT NULL`
- `metadata_json TEXT NULL`

Package 4D stores metadata only. It does not disable, restore, delete, or modify startup entries.

## Repository Functions

- `init_db(path)`
- `insert_event(event)`
- `list_events(query)`
- `get_event(id)`
- `seed_mock_events()`
- `count_events()`
- `dashboard_summary()`
- `insert_action_history(result)`
- `list_action_history(query)`
- `get_action_history(id)`
- `insert_startup_backup(record)`
- `list_startup_backups(query)`
- `get_startup_backup(backup_id)`
- `find_startup_backup_by_entry(entry_id)`

`EventQuery` supports optional kind filter, optional severity filter, text search over title/summary/process/target, limit, and newest-first ordering.

## Mock Seed

`seed_mock_events()` inserts demo events from `jsentinel-events`. Every seeded event is explicitly marked `source = mock`.

## Non-Goals

Package 1 does not implement:

- Real Windows/Linux backend collection.
- Registry reads or writes.
- Process termination.
- Firewall or network blocking.
- Quarantine or restore.
- Delete-on-reboot.
- File watching.
- Device monitoring.
- Event upload or telemetry.
