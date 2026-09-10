# dove

A Nintendo 3DS homebrew app in Rust, built on [`ctru-rs`](https://github.com/rust3ds/ctru-rs)
and [`citro3d`](https://github.com/rust3ds/citro3d-rs).

Scaffolded with `cargo 3ds new` (the modern replacement for the archived
`rust3ds/rust3ds-template`).

Right now it runs a little swarm of RGB triangles: every triangle is a
[`bevy_ecs`](https://docs.rs/bevy_ecs) entity `(Body, Spin, Pulse)`, a `Schedule`
of `drift` / `bounce` / `spin` systems updates them each frame, and the render
loop reads the component state back out and draws one `citro3d` triangle per
entity — on the top screen in stereoscopic 3D (left/right-eye projections driven
by the hardware 3D slider) and on the bottom screen with a plain centered
projection.

Controls: **A** spawns a triangle, **B** despawns one, **START** exits.

The PICA200 vertex shader lives in `src/vshader.pica` and is compiled at build
time by `citro3d`'s `include_shader!` macro (which shells out to devkitPro's
`picasso`). `bevy_ecs` is core-only (`default-features = false`,
`features = ["std"]`) — see `crates/bevy-ecs-check` for why that works on the 3DS.

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
- `crates/bevy-ecs-check` — a standalone probe that pulls in `bevy_ecs` (core ECS
  only: `default-features = false`, `features = ["std"]`, no `bevy_render` /
  `bevy_app`) to confirm it compiles, links, **and runs** on `armv6k-nintendo-3ds`.

  ```sh
  cargo 3ds build -p bevy-ecs-check          # compile + link
  ./scripts/test-emulator.sh -p bevy-ecs-check   # run the ECS tests on-device
  ```

  Result: it works — spawn / `Query<&mut T>` / `Resource` / `Schedule` all
  behave correctly in the emulator (3/3 tests pass). The 3DS has no 64-bit
  atomics, but `bevy_platform` turns on `portable-atomic`'s self-contained
  `fallback`, so no `critical-section` impl is needed. `bevy_tasks` is a
  non-optional dep of `bevy_ecs` and builds fine in its single-threaded form.

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
