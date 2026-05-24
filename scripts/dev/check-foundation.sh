#!/usr/bin/env sh
set -u

failures=0
SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)

report_error() {
  echo "ERROR: $1" >&2
  failures=$((failures + 1))
}

report_warning() {
  echo "WARNING: $1" >&2
}

require_path() {
  if [ ! -e "$REPO_ROOT/$1" ]; then
    report_error "Missing required path: $1"
  else
    echo "OK: $1"
  fi
}

echo "Current directory: $(pwd)"
echo "Repository root: $REPO_ROOT"
cd "$REPO_ROOT" || exit 1

for path in \
  "Cargo.toml" \
  "README.md" \
  "apps/desktop-ui" \
  "crates" \
  "docs" \
  "scripts/release"
do
  require_path "$path"
done

for path in "JSentinel" "sentinel-local" "SentinelLocal"; do
  if [ -e "$REPO_ROOT/$path" ]; then
    report_error "Unexpected nested project directory exists: $path"
  else
    echo "OK: nested project directory absent: $path"
  fi
done

crate_names="
jsentinel-core
jsentinel-db
jsentinel-events
jsentinel-policy
jsentinel-windows
jsentinel-linux
jsentinel-quarantine
jsentinel-network
jsentinel-files
jsentinel-startup
jsentinel-device-access
"

for crate in $crate_names; do
  require_path "crates/$crate/Cargo.toml"
  require_path "crates/$crate/src/lib.rs"

  if [ -f "$REPO_ROOT/crates/$crate/Cargo.toml" ]; then
    if ! grep -Eq "name[[:space:]]*=[[:space:]]*\"$crate\"" "$REPO_ROOT/crates/$crate/Cargo.toml"; then
      report_error "Crate package name mismatch in crates/$crate/Cargo.toml"
    fi
  fi

  if ! grep -Fq "crates/$crate" "$REPO_ROOT/Cargo.toml"; then
    report_error "Workspace does not list member: crates/$crate"
  fi
done

if command -v cargo >/dev/null 2>&1; then
  echo "Running cargo check --workspace"
  if ! cargo check --workspace; then
    report_error "cargo check --workspace failed"
  fi
else
  report_warning "cargo is not available; skipping Rust workspace build check."
fi

if command -v npm >/dev/null 2>&1; then
  cd "$REPO_ROOT/apps/desktop-ui" || exit 1
  if [ ! -d "node_modules" ] && [ "${SKIP_NPM_INSTALL:-0}" != "1" ]; then
    echo "Running npm install in apps/desktop-ui"
    if ! npm install; then
      report_error "npm install failed"
    fi
  fi

  if [ "$failures" -eq 0 ]; then
    echo "Running npm run build in apps/desktop-ui"
    if ! npm run build; then
      report_error "npm run build failed"
    fi
  fi
else
  report_warning "npm is not available; skipping UI dependency and build checks."
fi

if [ "$failures" -gt 0 ]; then
  echo "Foundation validation failed with $failures structural/build error(s)." >&2
  exit 1
fi

echo "Foundation validation passed."
exit 0
