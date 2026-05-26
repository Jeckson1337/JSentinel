# User Guide

JSentinel is intended to help users understand what is happening on their PC.

It can show local visibility areas such as processes, network activity, files, startup entries, events, and settings as the backend grows.

It is not an antivirus. It does not replace dedicated protection tools, and OS limitations apply.

## Current Build

The current build includes local/mock events, SQLite storage, a usable desktop UI layer, and a Windows read-only backend for best-effort process, network, and startup snapshots.

Package 4B adds the first safe actions. Action buttons ask the policy layer for a plan, show a confirmation or disabled reason, execute only allowlisted non-destructive actions, and record local audit results.

Currently enabled safe actions are revealing an existing local path in Windows Explorer and opening allowlisted Windows Settings pages such as Privacy, Startup Apps, and Network Status. JSentinel does not open arbitrary URLs from action input.

Package 4C adds a controlled Kill Process action. It can terminate one non-protected process by PID after confirmation and live backend safety checks. Unsaved work in that process may be lost. JSentinel refuses its own process, system/protected names, Windows-directory targets, missing verified details, process trees, and name-only kills.

Other dangerous actions are not implemented. JSentinel does not change firewall rules, write registry keys, quarantine files, force unlock files, disable startup entries, kill process trees, kill by name, or delete files on reboot.

## Privacy Promise

The project is designed around no telemetry, no accounts, no tracking, no ad SDK, and no forced cloud.
