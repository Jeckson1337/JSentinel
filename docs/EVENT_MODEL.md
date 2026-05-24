# Event Model

Package 1 introduces a local event model for JSentinel.

The model is intentionally read-only and demo-safe at this stage. It does not read real Windows or Linux system state, inspect running processes, scan files, watch devices, modify startup entries, block network traffic, or perform privileged actions.

## Core Types

Events are represented by `AccessEvent` in `crates/jsentinel-events`.

Fields:

- `id`: stable event identifier.
- `timestamp`: textual local event timestamp.
- `kind`: event category.
- `severity`: event importance.
- `source`: where the event came from.
- `process`: optional process reference.
- `title`: short human-readable title.
- `summary`: longer explanation.
- `target`: optional file/process/endpoint/device/display target.
- `metadata_json`: optional structured metadata.
- `confidence`: optional marker such as `demo_only`.

## Event Kinds

- `process`
- `network`
- `file`
- `startup`
- `device_access`
- `locked_file`
- `security`
- `system`

## Severity

- `info`
- `warning`
- `critical`

## Sources

- `mock`
- `user`
- `core`
- `windows_backend`
- `linux_backend`

All Package 1 seeded/demo events must use `source = mock`.

## Mock Events

The event crate includes helper constructors:

- `mock_process_event`
- `mock_network_event`
- `mock_file_event`
- `mock_startup_event`
- `mock_device_access_event`
- `mock_locked_file_event`
- `mock_security_event`

These constructors are for UI, database, serialization, and timeline validation. They do not access OS APIs and must not be presented as real system observations.

## Future Backend Integration

Future Windows/Linux backends should implement read-only collectors behind the core `ReadOnlySystemProvider` boundary. Those backends should produce `AccessEvent` values without coupling UI code to OS-specific APIs.

Actions remain out of scope for Package 1.
