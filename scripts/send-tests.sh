#!/usr/bin/env bash
#
# Build the **interactive on-device test runner** for a crate and 3dslink it to
# a 3DS. On the 3DS: the test list shows on the bottom screen; the first row
# runs every test in sequence, the rest are individual tests (Up/Dn move, L/R
# page, A run the highlighted row, X run all, START exit); results on top.
#
# Usage:
#   ./scripts/send-tests.sh <crate> [--server] [--ip <addr>] [extra cargo args]
#
# Examples:
#   ./scripts/send-tests.sh bevy-math-check
#   ./scripts/send-tests.sh bevy-transform-check --server
#   ./scripts/send-tests.sh dove --ip 192.168.2.50
#   ./scripts/send-tests.sh all            # every check crate in one app
#
# Crates: dove, bevy-ecs-check, bevy-math-check, bevy-transform-check,
#   bevy-color-check, bevy-time-check, all-checks.
#   "all" is an alias for "all-checks" — the aggregate crate that bundles every
#   bevy-*-check crate's #[test]s into a single test .3dsx.
# Default IP: $DOVE_3DS_IP, else 192.168.2.181.
#
# The 3DS must be in the Homebrew Launcher waiting for netload.

set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

crate="${1:?usage: send-tests.sh <crate|all> [--server] [--ip <addr>]}"
shift
[ "$crate" = all ] && crate=all-checks

ip="${DOVE_3DS_IP:-192.168.2.181}"
server=()
passthrough=()
while [ "$#" -gt 0 ]; do
    case "$1" in
        --ip) ip="${2:?--ip needs an address}"; shift 2 ;;
        --server|-s) server=(--server); shift ;;
        --) shift; passthrough+=("$@"); break ;;
        *) passthrough+=("$1"); shift ;;
    esac
done

echo >&2 "==> interactive tests: $crate  ->  3DS @ $ip"
echo >&2 "    bottom screen = test list · row 0 runs all · A run · X run all · START exit"

# cargo-3ds `test` flags (--address/--retries/--server) go before the first
# passthrough arg (-p / --features), same rule as `cargo 3ds run`.
exec cargo 3ds test \
    --address "$ip" \
    --retries 15 \
    "${server[@]}" \
    -p "$crate" \
    --features console \
    "${passthrough[@]}"
