# Threat Model

JSentinel is a local visibility and control utility. It is not an antivirus, endpoint detection platform, sandbox, firewall, or exploit prevention system.

## Assets to Protect

- User privacy.
- Local event history.
- Local configuration.
- Integrity of reversible actions.
- Trust boundary between UI and privileged backend.

## Key Risks

- UI accidentally gaining privileged behavior.
- Dangerous action without confirmation.
- Local event storage leaking sensitive data.
- Future update or release pipeline compromise.
- Misleading claims that users interpret as antivirus protection.

## Mitigations

- No telemetry, accounts, tracking, ad SDK, or forced cloud.
- Clear release artifacts with checksums.
- No kernel driver in v1.
- Dangerous actions require confirmation.
- Prefer quarantine/restore over irreversible deletion.
- Keep OS-specific backend separated from UI and core.
- Document limitations prominently.
