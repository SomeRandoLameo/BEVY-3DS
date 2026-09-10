# dove

A Nintendo 3DS homebrew app in Rust, built on [`ctru-rs`](https://github.com/rust3ds/ctru-rs)
and [`citro3d`](https://github.com/rust3ds/citro3d-rs).

Scaffolded with `cargo 3ds new` (the modern replacement for the archived
`rust3ds/rust3ds-template`).

Right now it runs a swarm of RGB triangles: every triangle is a
[`bevy_ecs`](https://docs.rs/bevy_ecs) entity `(Body, Spin, Pulse)` with
[`bevy_math`](https://docs.rs/bevy_math) `Vec2` positions, a `Schedule` of
`drift` → `bounce` + `spin` systems updates them each frame, and the render loop
reads the component state back out — on the top screen in stereoscopic 3D
(left/right-eye projections driven by the hardware 3D slider) and on the bottom
screen with a plain centered projection.

Rendering is **batched for throughput**: instead of a draw call per triangle,
every triangle is transformed on the CPU into one shared vertex buffer in linear
memory each frame, then drawn with a single `draw_arrays` per screen (3 total).
The previous frame's buffer is held one extra frame so `C3D_FrameBegin`'s
`SYNCDRAW` can guarantee the GPU is done reading it. Frame rate + triangle count
are printed over `3dslink` (run with `cargo 3ds run --server`).

Controls: **A/B** ±8 triangles · hold **X/Y** ±32 per frame · **SELECT** reset ·
**START** exit. On New 3DS the 804 MHz clock is enabled at startup.

The PICA200 vertex shader lives in `src/vshader.pica` and is compiled at build
time by `citro3d`'s `include_shader!` macro (which shells out to devkitPro's
`picasso`). `bevy_ecs` and `bevy_math` are both core-only
(`default-features = false`, `features = ["std"]` — no `bevy_reflect`, no `rand`,
no `curve`) — see `crates/bevy-ecs-check` for why that works on the 3DS.
`bevy_math` pulls its own `glam` 0.32 alongside `citro3d`'s `glam` 0.30; the two
coexist and we only touch `bevy_math`'s.

## Prerequisites

- [devkitPro](https://devkitpro.org/wiki/Getting_Started) with the `3ds-dev` group
  (`devkitARM` + `libctru`), installed at `/opt/devkitpro`
- Nightly Rust with the `rust-src` component (pinned via `rust-toolchain.toml`)
- `cargo-3ds`:

  ```sh
  cargo install cargo-3ds
  ```

### Toolchain note

Arch's `arm-none-eabi-gcc` package ships a `/usr/bin/arm-none-eabi-gcc` that
lacks devkitPro's `3dsx.specs`. `.cargo/config.toml` pins the linker to
`/opt/devkitpro/devkitARM/bin/arm-none-eabi-gcc` so the build works without
touching `PATH`. Alternatively, put `$DEVKITARM/bin` ahead of `/usr/bin` on
`PATH` and delete that override.

## Build & run

```sh
cargo 3ds build              # -> target/armv6k-nintendo-3ds/debug/dove.3dsx
cargo 3ds build --release
cargo 3ds run -- --address <3DS-IP>   # send to hardware via 3dslink
```

Put any files the app loads at runtime in `romfs/`.

## Workspace

- `.` — the `dove` app.
- `crates/bevy-ecs-check`, `crates/bevy-math-check`, `crates/bevy-transform-check`
  — standalone probes that pull in one Bevy crate (no `citro3d` in the tree) and
  run assertion tests on-device to confirm it works on `armv6k-nintendo-3ds`.

  ```sh
  cargo 3ds build -p bevy-ecs-check                 # compile + link
  ./scripts/test-emulator.sh -p bevy-ecs-check      # run the tests in the emulator
  ```

  Each check crate has its own **`README.md`** with the Bevy unit + version, the
  date/status, and a full table of **every checked function — input, expected,
  actual output, pass/fail**. Every check prints its result row straight to
  stdout (bypassing libtest's capture) so the tables are transcribed from real
  emulator runs. Summary:

  | crate | Bevy unit | version | result |
  |---|---|---|---|
  | `bevy-ecs-check` | `bevy_ecs` (`std`, no reflect/threads) | 0.19.1 | 9/9 checks ✅ |
  | `bevy-math-check` | `bevy_math` + `glam` 0.32 (`std`, `curve`) | 0.19.1 | 66/66 checks ✅ |
  | `bevy-transform-check` | `bevy_transform` (`std`, `bevy-support`) | 0.19.1 | 26/26 checks ✅ |

  Findings, caveats and the full port classification live in `port.md`
  (`bevy_ecs` §"Was verifiziert ist" / §B1–§B2, `bevy_math` §B9, `bevy_transform`
  §B10).

## Emulator (Azahar / Citra / Lime3DS)

`cargo-3ds` only knows how to deploy over `3dslink`, so emulator support lives in
two wrapper scripts. They auto-detect `azahar`, `citra-qt`, `citra`, or `lime3ds`
on `PATH`; override with `EMULATOR=<exe>`.

```sh
./scripts/run-emulator.sh            # build (with RomFS) + launch the app
./scripts/run-emulator.sh --release

./scripts/test-emulator.sh           # build + run `cargo 3ds test` in the emulator
./scripts/test-emulator.sh --release
./scripts/test-emulator.sh --lib     # any extra args pass through to `cargo 3ds test`
```

### How the test runner works

Standard `cargo test` can't run on the 3DS, so tests use the
[`test-runner`](https://github.com/rust3ds/ctru-rs/tree/master/test-runner) dev
harness (`#![test_runner(test_runner::run_gdb)]` in `src/main.rs`).

`test-emulator.sh` builds the test `.3dsx` (`cargo 3ds test --no-run` — needed
because `cargo-3ds` only emits the `.3dsx` on that pass), then re-runs
`cargo 3ds test` with `CARGO_TARGET_ARMV6K_NINTENDO_3DS_RUNNER` pointed at
`scripts/emulator-runner.sh`. That script boots the `.3dsx` in the emulator with
its GDB stub enabled, attaches `arm-none-eabi-gdb`, and breaks at libctru's
`__ctru_exit` to recover the process exit code (non-zero = test failure). Test
output is streamed over GDB's File-I/O channel.

Useful env vars for `emulator-runner.sh`: `GDB_PORT` (default `4000`),
`EMU_TIMEOUT` (stub-startup wait, default `60`s), `EMU_HEADLESS=1` (run the
emulator under `xvfb-run`).

The Qt frontends (Azahar / Citra / Lime3DS) persist the `-g` GDB-stub flag back
into their config on exit, which would make every later normal launch hang on
"Waiting for gdb to connect". `emulator-runner.sh` resets `use_gdbstub=false` in
`~/.config/*-emu/qt-config.ini` when it finishes to undo that.
