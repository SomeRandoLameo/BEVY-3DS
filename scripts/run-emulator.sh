#!/usr/bin/env bash
#
# Build the app (with RomFS + SMDH) and launch it in a 3DS emulator.
#
# Usage:
#   ./scripts/run-emulator.sh [--release] [extra emulator args]
#
# Examples:
#   ./scripts/run-emulator.sh
#   ./scripts/run-emulator.sh --release
#   EMULATOR=citra-qt ./scripts/run-emulator.sh
#
# `cargo 3ds run` is left alone for 3dslink to hardware
# (`cargo 3ds run -- --address <3DS-IP>`).

set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

profile_dir=debug
build_args=()
if [ "${1:-}" = "--release" ]; then
    profile_dir=release
    build_args=(--release)
    shift
fi

cargo 3ds build "${build_args[@]}"

emulator="${EMULATOR:-}"
if [ -z "$emulator" ]; then
    for candidate in azahar citra-qt citra lime3ds; do
        if command -v "$candidate" >/dev/null 2>&1; then
            emulator="$candidate"
            break
        fi
    done
fi
if [ -z "$emulator" ]; then
    echo >&2 "run-emulator: no 3DS emulator found (tried azahar, citra-qt, citra, lime3ds)."
    echo >&2 "              Install one or set \$EMULATOR."
    exit 127
fi

exec "$emulator" "target/armv6k-nintendo-3ds/$profile_dir/dove.3dsx" "$@"
