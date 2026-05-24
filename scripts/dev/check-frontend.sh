#!/usr/bin/env sh
set -u

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)
UI_ROOT="$REPO_ROOT/apps/desktop-ui"

if [ ! -d "$UI_ROOT" ]; then
  echo "ERROR: Missing apps/desktop-ui" >&2
  exit 1
fi

cd "$UI_ROOT" || exit 1
echo "Frontend root: $UI_ROOT"

if ! command -v npm >/dev/null 2>&1; then
  echo "WARNING: npm is not available; skipping frontend install/build." >&2
  exit 0
fi

if [ "${USE_NPM_CI:-0}" = "1" ] && [ -f "package-lock.json" ]; then
  echo "Running npm ci"
  npm ci || exit 1
else
  echo "Running npm install"
  npm install || exit 1
fi

echo "Running npm run build"
npm run build || exit 1

echo "Frontend checks passed."
