# Security

JSentinel should be designed around conservative local control.

## Required Safety Rules

- UI does not execute privileged actions directly.
- Dangerous actions require explicit confirmation.
- Actions must be policy-planned before execution.
- Action attempts that reach the backend execution path should be audited locally.
- Package 4B safe actions are limited to opening local Explorer paths and allowlisted Windows Settings pages.
- JSentinel must not execute arbitrary commands or open arbitrary external URLs from action input.
- Safe action targets must be validated again in the backend before execution, even after UI planning.
- Reversible actions are preferred.
- No force-delete in v1.
- No kernel driver in v1.
- No delete-on-reboot in v1.
- No process kill in v1 foundation.
- No firewall/blocking in v1 foundation.
- No registry writes.
- OS-specific code stays outside the UI and platform-neutral core.
- No automatic mitigation without a later explicit design and review.

## Reporting Security Issues

Until a public policy is finalized, report security concerns through the project's GitHub issue tracker without posting sensitive exploit details publicly.
