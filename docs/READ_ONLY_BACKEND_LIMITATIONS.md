# Read-Only Backend Limitations

Package 3.5 hardens the read-only backend and UI diagnostics. It does not add mitigation actions.

## Snapshot, Not Real-Time Monitoring

Process, network, and startup data are point-in-time snapshots. Refreshing a screen repeats the read-only query. JSentinel does not yet run a real-time monitor, packet monitor, file watcher, or device access monitor.

## Capability Status

Capabilities can be:

- supported;
- partial / best-effort;
- unsupported.

Each capability should state its data source, read-only status, admin visibility caveat, and limitation message. Admin rights may improve visibility for protected process details, but the basic UI must not require admin to open.

## Unsupported Or Planned

The following remain unsupported or planned:

- process kill;
- firewall modification or network blocking;
- startup modification;
- quarantine and restore;
- delete-on-reboot;
- force unlock;
- file locker detection through Restart Manager;
- scheduled task parsing;
- service inventory;
- real-time file watching;
- microphone or camera monitoring.

## Diagnostics

Diagnostics are local and read-only. They may include:

- app version;
- platform;
- capability statuses;
- snapshot counts.

Diagnostics must not include a private full system dump by default and must not upload anything.
