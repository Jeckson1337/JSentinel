# Codex Rules for JSentinel

Work directly in the repository root. Do not create nested project directories such as `JSentinel/`, `sentinel-local/`, `SentinelLocal/`, `app/`, or `project/`.

## Hard Rules

- Do not add telemetry.
- Do not add advertising SDKs.
- Do not add tracking.
- Do not add forced cloud dependencies.
- Do not implement kernel drivers.
- Do not implement process kill.
- Do not implement firewall or network blocking.
- Do not implement quarantine until the policy and restore model are designed.
- Do not implement delete-on-reboot.
- Do not read or write Windows registry during foundation work.
- Do not request real OS permissions during foundation work.
- Do not make antivirus claims.
- Do not mix UI and privileged operations into one monolith.
- Do not touch files outside the current JSentinel workspace.

## Preferred Direction

Create small, safe, extensible stubs. Document boundaries clearly. Favor read-only visibility first and route future actions through policy, confirmation, and OS-specific backends.

## Build Discipline

Before each feature package, run `scripts/dev/check-all.ps1` or the equivalent shell script when the local toolchain is available.

If the build is red, fix build/test issues before adding new features. Do not remove Package 1 event/database/core functionality just to make checks green.
