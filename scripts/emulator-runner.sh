#!/usr/bin/env bash
#
# Cargo runner for the `armv6k-nintendo-3ds` target: boots a built executable in
# a 3DS emulator (Azahar / Citra / Lime3DS) and drives it with GDB.
#
# It is not meant to be called directly. `scripts/test-emulator.sh` points
# `CARGO_TARGET_ARMV6K_NINTENDO_3DS_RUNNER` at it so `cargo 3ds test` runs the
# test executables here instead of over `3dslink`.
#
# How it works (mirrors rust3ds/test-runner's CI image):
#   1. Find the `.3dsx` that `cargo-3ds` built next to the ELF cargo hands us.
#   2. Launch the emulator with its GDB stub enabled; it halts the CPU and waits
#      for a debugger before running any code.
#   3. arm-none-eabi-gdb attaches, resumes execution, breaks at libctru's
#      `__ctru_exit`, and quits with the process exit code (r0) -- so a failing
#      test (which exits non-zero) fails the command.
#
# The tests print output over GDB's File-I/O channel; that needs the
# `test-runner` dev-dependency + `#![test_runner(test_runner::run_gdb)]`.
#
# Env overrides:
#   EMULATOR    emulator executable (default: first of azahar/citra-qt/citra/lime3ds on PATH)
#   DEVKITPRO   devkitPro install prefix (default: /opt/devkitpro)
#   GDB         arm-none-eabi-gdb path (default: $DEVKITPRO/devkitARM/bin/arm-none-eabi-gdb)
#   GDB_PORT    GDB stub port (default: 4000)
#   EMU_TIMEOUT seconds to wait for the emulator's GDB stub to come up (default: 60)
#   EMU_HEADLESS set to 1 to run the emulator under xvfb-run (needs xvfb)

set -euo pipefail

DEVKITPRO="${DEVKITPRO:-/opt/devkitpro}"
GDB="${GDB:-$DEVKITPRO/devkitARM/bin/arm-none-eabi-gdb}"
GDB_PORT="${GDB_PORT:-4000}"
EMU_TIMEOUT="${EMU_TIMEOUT:-60}"

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
    echo >&2 "emulator-runner: no 3DS emulator found (tried azahar, citra-qt, citra, lime3ds)."
    echo >&2 "                 Install one or set \$EMULATOR."
    exit 127
fi

emu_launcher=("$emulator")
if [ "${EMU_HEADLESS:-0}" = "1" ]; then
    emu_launcher=(xvfb-run --auto-servernum "$emulator")
fi

exe_elf="$1"
exe_noext="$(dirname "$exe_elf")/$(basename "$exe_elf" .elf)"
exe_3dsx="$exe_noext.3dsx"

if [ ! -f "$exe_3dsx" ]; then
    # cargo-3ds only emits the .3dsx on a `--no-run` pass (and never when a
    # custom runner is set, e.g. plain `cargo 3ds run`). Fall back to a bare
    # .3dsx straight from the ELF -- no SMDH, no RomFS.
    echo >&2 "emulator-runner: $exe_3dsx missing; building a bare one from the ELF."
    echo >&2 "                 Run 'cargo 3ds test --no-run' first if tests need RomFS."
    3dsxtool "$exe_elf" "$exe_3dsx"
fi

emu_log="$(mktemp)"
gdb_script="$(mktemp)"
emu_pid=""
cleanup() {
    if [ -n "$emu_pid" ] && kill -0 "$emu_pid" 2>/dev/null; then
        kill "$emu_pid" 2>/dev/null || true
        for _ in $(seq 1 10); do
            kill -0 "$emu_pid" 2>/dev/null || break
            sleep 0.5
        done
        kill -9 "$emu_pid" 2>/dev/null || true
    fi
    # Some emulator builds fork/re-exec, so $emu_pid may already be gone while
    # the real process lingers. Match the exact command line we launched.
    pkill -9 -f -- "-g $GDB_PORT $exe_3dsx" 2>/dev/null || true
    rm -f "$emu_log" "$gdb_script"
}
trap cleanup EXIT

cat >"$gdb_script" <<EOF
set tcp connect-timeout $EMU_TIMEOUT
break __ctru_exit
commands
    # libctru leaves the process exit code in r0; propagate it as GDB's.
    quit \$r0
end
target extended-remote :$GDB_PORT
continue
quit
EOF

echo >&2 "emulator-runner: booting $(basename "$exe_3dsx") in $emulator (GDB stub :$GDB_PORT)"
"${emu_launcher[@]}" -g "$GDB_PORT" "$exe_3dsx" >"$emu_log" 2>&1 &
emu_pid=$!

# Wait for the emulator to report that its GDB stub is listening. Note $emu_pid
# is unreliable (some builds fork/re-exec on startup), so we key off the log and
# the overall timeout rather than the process being alive.
stub_re='Waiting for gdb|Starting GDB server|GDB server.*port'
deadline=$((SECONDS + EMU_TIMEOUT))
until grep -qiE "$stub_re" "$emu_log"; do
    if [ "$SECONDS" -ge "$deadline" ]; then
        echo >&2 "emulator-runner: timed out after ${EMU_TIMEOUT}s waiting for the GDB stub. Log:"
        sed 's/^/  | /' >&2 <"$emu_log"
        exit 1
    fi
    sleep 0.5
done

set +e
"$GDB" --nx --batch-silent --command "$gdb_script" "$exe_elf"
status=$?
set -e

if [ "$status" -eq 0 ]; then
    echo >&2 "emulator-runner: tests passed"
else
    echo >&2 "emulator-runner: tests failed (exit $status)"
fi
exit "$status"
