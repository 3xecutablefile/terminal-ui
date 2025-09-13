#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

log()  { printf "[depinstall] %s\n" "$*"; }
die()  { printf "[depinstall] ERROR: %s\n" "$*" >&2; exit 1; }

need() { command -v "$1" >/dev/null 2>&1 || die "Missing required command: $1"; }

need node 
need npm

PLAT="$(uname -s 2>/dev/null || echo unknown)"
ARCH_NODE="$(node -p "process.arch")"

log "Platform: ${PLAT}, Node arch: ${ARCH_NODE}"

# Suggest build tools if missing
if [[ "$PLAT" == "Darwin" ]]; then
  if ! xcode-select -p >/dev/null 2>&1; then
    log "Tip: Install Xcode command line tools for native builds: xcode-select --install"
  fi
elif [[ "$PLAT" == "Linux" ]]; then
  for tool in python3 make g++; do
    command -v "$tool" >/dev/null 2>&1 || log "Tip: Install build tool '$tool' via your package manager (e.g., apt, dnf, pacman)"
  done
fi

pushd "$ROOT_DIR" >/dev/null

log "Installing root dependencies (electron, builder, rebuild)..."
if [[ -f package-lock.json ]]; then
  if ! npm ci; then
    log "npm ci failed (lock out of sync). Falling back to npm install..."
    npm install
  fi
else
  npm install
fi

log "Installing app (src) dependencies (skipping native build scripts)..."
pushd src >/dev/null

if [[ -f package-lock.json ]]; then
  if ! npm ci --ignore-scripts; then
    log "npm ci (src) failed. Falling back to npm install --ignore-scripts..."
    npm install --ignore-scripts
  fi
else
  npm install --ignore-scripts
fi

# Locate electron-rebuild (installed at repo root)
REBUILD="${ROOT_DIR}/node_modules/.bin/electron-rebuild"
if [[ ! -x "$REBUILD" && -x "${REBUILD}.cmd" ]]; then
  REBUILD="${REBUILD}.cmd"
fi

if [[ ! -x "$REBUILD" ]]; then
  die "electron-rebuild not found (expected at ${ROOT_DIR}/node_modules/.bin). Did root npm install succeed?"
fi

log "Rebuilding native modules for Electron (node-pty)..."
RB_ARGS=( -f -w node-pty )

# Help electron-rebuild on ARM where terminals may run under Rosetta
case "$ARCH_NODE" in
  arm64|aarch64) RB_ARGS+=( --arch=arm64 );;
  x64) RB_ARGS+=( --arch=x64 );;
esac

# Determine Electron version installed at the root and force it
ELECTRON_VER=$(node -p "require('${ROOT_DIR}/node_modules/electron/package.json').version")
RB_ARGS+=( -v "$ELECTRON_VER" )

# Workaround for gyp var not defined on some hosts
export GYP_DEFINES="openssl_fips="

"$REBUILD" "${RB_ARGS[@]}"

popd >/dev/null # src

log "Done. Common next steps:"
log "- Start dev: npm start"
log "- Or build:  npm run build-<linux|darwin|windows>"

popd >/dev/null 
