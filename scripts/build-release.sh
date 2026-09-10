#!/usr/bin/env bash
#
# Build the 3DS release and open the directory containing the resulting .3dsx.
#
# Usage:
#   ./scripts/build-release.sh
#

set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

echo >&2 "==> Building 3DS release"
cargo 3ds build --release

release_dir="$PWD/target/armv6k-nintendo-3ds/release"
rom="$release_dir/dove.3dsx"

if [ ! -f "$rom" ]; then
    echo >&2 "build-release: expected 3DSX not found:"
    echo >&2 "                 $rom"
    exit 1
fi

echo >&2 "==> Release built:"
echo >&2 "    $rom"

echo >&2 "==> Opening release directory"

if command -v xdg-open >/dev/null 2>&1; then
    xdg-open "$release_dir" >/dev/null 2>&1 &
elif command -v gio >/dev/null 2>&1; then
    gio open "$release_dir" >/dev/null 2>&1 &
else
    echo >&2 "build-release: no file manager opener found (xdg-open/gio)."
    exit 1
fi