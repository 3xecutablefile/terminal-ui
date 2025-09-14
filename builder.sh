#!/usr/bin/env bash
set -e

platform="$1"

if [ -z "$platform" ]; then
  echo "Usage: $0 <darwin|linux|linux-arm64|windows>"
  exit 1
fi

if [ ! -d node_modules ]; then
  npm install
fi

case "$platform" in
  darwin|linux|windows)
    npm run prebuild-$platform
    npm run build-$platform
    ;;
  linux-arm64)
    TARGET_ARCH=arm64 npm run prebuild-linux
    TARGET_ARCH=arm64 npm run build-linux
    ;;
  *)
    echo "Unknown platform: $platform"
    exit 1
    ;;
esac
