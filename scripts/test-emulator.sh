#!/usr/bin/env bash
#
# Build the test executables and run them in a 3DS emulator.
#
# Usage:
#   ./scripts/test-emulator.sh [extra `cargo 3ds test` args]
#
# Examples:
#   ./scripts/test-emulator.sh
#   ./scripts/test-emulator.sh --release
#   EMULATOR=citra-qt ./scripts/test-emulator.sh
#
# Two passes are needed: cargo-3ds only emits the `.3dsx` (with RomFS + SMDH) on
# a `--no-run` build, and the custom runner needs that file to exist before
# `cargo 3ds test` invokes it. See https://github.com/rust3ds/cargo-3ds/issues/44

set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

echo >&2 "==> Building test executables (cargo 3ds test --no-run)"
cargo 3ds test --no-run "$@"

echo >&2 "==> Running tests in emulator"
export CARGO_TARGET_ARMV6K_NINTENDO_3DS_RUNNER="$PWD/scripts/emulator-runner.sh"
exec cargo 3ds test "$@"
