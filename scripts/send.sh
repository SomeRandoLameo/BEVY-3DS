#!/usr/bin/env bash
#
# Build a `dove.3dsx` and send it to a 3DS over 3dslink (netload).
#
# Usage:
#   ./scripts/send.sh [--release] [--server] [--ip <addr>] [extra args]
#
# Examples:
#   ./scripts/send.sh                       # debug build -> 3DS
#   ./scripts/send.sh --release             # release build -> 3DS
#   ./scripts/send.sh --release --server    # release, then stream stdout/FPS back
#   DOVE_3DS_IP=192.168.2.50 ./scripts/send.sh
#   ./scripts/send.sh --ip 192.168.2.50 --release
#
# The 3DS must be running the Homebrew Launcher with "Receive from network"
# active (or any netload-listening app) when this runs.
#
# Default IP: $DOVE_3DS_IP, else 192.168.2.181.

set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

ip="${DOVE_3DS_IP:-192.168.2.181}"
profile="debug"
cargo_args=()        # passthrough to `cargo run` (must follow --release/-p)
server_flag=()

while [ "$#" -gt 0 ]; do
    case "$1" in
        --release) profile="release"; cargo_args+=(--release); shift ;;
        --server|-s) server_flag=(--server); shift ;;
        --ip) ip="${2:?--ip needs an address}"; shift 2 ;;
        --) shift; cargo_args+=("$@"); break ;;
        *) cargo_args+=("$1"); shift ;;
    esac
done

echo >&2 "==> send: dove ($profile)  ->  3DS @ $ip"
echo >&2 "    (3DS must be in the Homebrew Launcher waiting for netload)"

# cargo-3ds flags (--address / --retries / --server) MUST come before the first
# cargo passthrough arg (e.g. --release), otherwise cargo-3ds hands --address to
# `cargo build` and it errors out.
exec cargo 3ds run \
    --address "$ip" \
    --retries 15 \
    "${server_flag[@]}" \
    -p dove \
    "${cargo_args[@]}"
