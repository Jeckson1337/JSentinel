# Security

JSentinel should be designed around conservative local control.

## Required Safety Rules

- UI does not execute privileged actions directly.
- Dangerous actions require explicit confirmation.
- Reversible actions are preferred.
- No force-delete in v1.
- No kernel driver in v1.
- No delete-on-reboot in v1.
- No process kill in v1 foundation.
- No firewall/blocking in v1 foundation.
- OS-specific code stays outside the UI and platform-neutral core.

## Reporting Security Issues

Until a public policy is finalized, report security concerns through the project's GitHub issue tracker without posting sensitive exploit details publicly.
