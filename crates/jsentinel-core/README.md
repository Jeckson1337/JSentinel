# jsentinel-core

Shared domain types and safety principles for JSentinel.

This crate must stay platform-neutral. It should not call Windows APIs, Linux system interfaces, shell commands, network endpoints, or privileged operations directly.
