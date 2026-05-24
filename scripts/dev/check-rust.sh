#!/usr/bin/env sh
set -u

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)

cd "$REPO_ROOT" || exit 1
echo "Repository root: $REPO_ROOT"

if [ ! -f "Cargo.toml" ]; then
  echo "ERROR: Missing root Cargo.toml" >&2
  exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "WARNING: cargo is not available; skipping cargo check/test." >&2
  exit 0
fi

echo "Running cargo check --workspace"
cargo check --workspace || exit 1

echo "Running cargo test --workspace"
cargo test --workspace || exit 1

echo "Rust checks passed."
