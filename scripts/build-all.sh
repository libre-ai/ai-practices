#!/usr/bin/env sh
# Build rumble-ai-practices for every platform from the single Rust codebase.
#
# One core, four render targets (ADR 0036 / multiplatform.md):
#   web  -> installable PWA (the v1 shippable)
#   macos/windows/linux -> native desktop (wry webview)
#   ios / android -> native mobile shells
#
# Prereqs (this machine has them): dioxus-cli 0.7.9, wasm32 target; for native
# mobile: Xcode (iOS) and the Android SDK+NDK (Android). Release web needs the
# size-tuned profile (wasm budget <= 450 KiB gzip).
#
# Usage:  sh scripts/build-all.sh [web|macos|ios|android|all]

set -eu
cd "$(dirname "$0")/../apps/web"
DX="${DX_BIN:-$HOME/.cargo/bin/dx}"
what="${1:-all}"

build_web() {
  echo "== web (release PWA) =="
  "$DX" build --platform web --release
}
build_macos() {
  echo "== macos (desktop .app) =="
  "$DX" build --platform macos
}
build_ios() {
  echo "== ios (.app) =="
  "$DX" build --platform ios
}
build_android() {
  echo "== android (app bundle) =="
  # dx needs the SDK/NDK on PATH; default macOS install location:
  export ANDROID_HOME="${ANDROID_HOME:-$HOME/Library/Android/sdk}"
  if [ -z "${ANDROID_NDK_HOME:-}" ]; then
    ANDROID_NDK_HOME="$(ls -d "$ANDROID_HOME"/ndk/* 2>/dev/null | sort -V | tail -1)"
    export ANDROID_NDK_HOME
    export NDK_HOME="$ANDROID_NDK_HOME"
  fi
  "$DX" build --platform android
}

case "$what" in
  web) build_web ;;
  macos) build_macos ;;
  ios) build_ios ;;
  android) build_android ;;
  all) build_web; build_macos; build_ios; build_android ;;
  *) echo "unknown target: $what (web|macos|ios|android|all)"; exit 1 ;;
esac

echo "done. artifacts under target/dx/rumble-ai-practices-web-app/*/"
