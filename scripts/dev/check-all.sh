#!/usr/bin/env sh
set -u

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)

cd "$REPO_ROOT" || exit 1

echo "== Foundation validation =="
sh scripts/dev/check-foundation.sh || exit 1

echo "== Rust checks =="
sh scripts/dev/check-rust.sh || exit 1

echo "== Frontend checks =="
sh scripts/dev/check-frontend.sh || exit 1

echo "check-all passed."
