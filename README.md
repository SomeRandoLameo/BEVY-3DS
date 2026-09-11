# dove

A Nintendo 3DS homebrew app in Rust, built on [`ctru-rs`](https://github.com/rust3ds/ctru-rs)
and [`citro3d`](https://github.com/rust3ds/citro3d-rs).

Scaffolded with `cargo 3ds new` (the modern replacement for the archived
`rust3ds/rust3ds-template`).

Right now it runs a swarm of rainbow triangles, simulated by a real
[`bevy_app`](https://docs.rs/bevy_app) `App` instead of a bare `World` +
`Schedule`: `App::new()` (**not** `App::default()`'s `DefaultPlugins` cousin —
that needs `bevy_render`/`bevy_winit`, which don't exist for this target) plus
exactly one plugin, [`bevy_time`](https://docs.rs/bevy_time)'s `TimePlugin`,
pinned to `TimeUpdateStrategy::ManualDuration` so the clock advances by a fixed
step every `app.update()` instead of calling `Instant::now()` (whether that's
accurate on real 3DS hardware is still unverified — see
`crates/bevy-time-check`). `main()` calls `app.update()` once per frame; that
runs the same `First → PreUpdate → RunFixedMainLoop → Update → PostUpdate →
Last` schedule pipeline already proven on-device by `bevy-transform-check`'s
and `bevy-time-check`'s own `App`-based tests, now driving a real interactive
loop. Every triangle is a [`bevy_ecs`](https://docs.rs/bevy_ecs) entity
`(Body, Spin, Pulse, Tint)` with [`bevy_math`](https://docs.rs/bevy_math)
`Vec2` positions and a [`bevy_color`](https://docs.rs/bevy_color) `Hsla` hue
that spins over time (`Hue::rotate_hue`) and gets baked to `Srgba` corner
colours 120° apart on the wheel; our `drift` → `bounce` + `spin` + `tint`
systems on the `Update` schedule read `Res<Time>` to update them each frame.
The render loop (outside the `App` — citro3d's borrowed render targets aren't
`'static`, so they can't be ECS resources) reads the component state back out
via `app.world()` — on the top screen in stereoscopic 3D (left/right-eye
projections driven by the hardware 3D slider) and on the bottom screen with a
plain centered projection.

Rendering is **batched for throughput**: instead of a draw call per triangle,
every triangle is transformed on the CPU into one shared vertex buffer in linear
memory each frame, then drawn with a single `draw_arrays` per screen (3 total).
The previous frame's buffer is held one extra frame so `C3D_FrameBegin`'s
`SYNCDRAW` can guarantee the GPU is done reading it. Frame rate + triangle count
are printed over `3dslink` (run with `cargo 3ds run --server`) and — when the
bottom-screen console is toggled on with **L** — on the bottom screen itself
(`… fps | … triangles | … verts | 3 draw calls`). Toggling the console swaps the
bottom screen between the `citro3d` render target and a `ctru` text console
(they can't coexist — both own `gfx.bottom_screen`).

Controls: **A/B** ±8 triangles · hold **X/Y** ±32 per frame · **SELECT** reset ·
**L** toggle bottom-screen console · **START** exit. On New 3DS the 804 MHz clock
is enabled at startup.

The PICA200 vertex shader lives in `src/vshader.pica` and is compiled at build
time by `citro3d`'s `include_shader!` macro (which shells out to devkitPro's
`picasso`). `bevy_ecs`, `bevy_math`, `bevy_color`, `bevy_time` and `bevy_app`
are all `default-features = false, features = ["std"]` — no `bevy_reflect` on
any of them, no `rand`/`curve` on `bevy_math`, no `serialize` on `bevy_color` —
see `crates/bevy-ecs-check`/`bevy-math-check`/`bevy-color-check`/
`bevy-time-check`/`bevy-transform-check` for why that works on the 3DS.
`bevy_math` pulls its own `glam` 0.32 alongside `citro3d`'s `glam` 0.30; the
two coexist and we only touch `bevy_math`'s.

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

# Send to a 3DS over 3dslink (netload) — 3DS must be in the Homebrew Launcher
# waiting for network transfer:
./scripts/send.sh                    # debug build
./scripts/send.sh --release          # release build
./scripts/send.sh --release --server # release, then stream stdout/FPS back
```

`send.sh` targets `$DOVE_3DS_IP` (default `192.168.2.181`); override with
`--ip <addr>` or the env var. It wraps `cargo 3ds run` with the cargo-3ds flags
in the order cargo-3ds requires (`--address`/`--retries` before `--release`).

Put any files the app loads at runtime in `romfs/`.

## Workspace

- `.` — the `dove` app.
- `crates/bevy-ecs-check`, `crates/bevy-math-check`, `crates/bevy-transform-check`,
  `crates/bevy-color-check`, `crates/bevy-time-check`, `crates/bevy-ptr-check` —
  standalone probes that pull in one Bevy crate (no `citro3d` in the tree) and
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
  | `bevy-ecs-check` | `bevy_ecs` (`std`, no reflect/threads) | 0.19.1 | 135/135 checks ✅ (whole public API: World, Commands, Queries, Change Detection, Resources, Components/Bundles, Relationships, Messages, Observers, Schedules) |
  | `bevy-math-check` | `bevy_math` + `glam` 0.32 + `rand` (`std`, `curve`, `rand`) | 0.19.1 | 488/488 checks ✅ (≈ whole public API) |
  | `bevy-transform-check` | `bevy_transform` (+`bevy_ecs`/`bevy_app`, `std`, `bevy-support`) | 0.19.1 | 129/129 checks ✅ (whole public API + `App::update()`) |
  | `bevy-color-check` | `bevy_color` (`std`, no reflect/serialize) | 0.19.1 | 381/381 checks ✅ (every color space + conversion graph) |
  | `bevy-time-check` | `bevy_time` (+`bevy_app`/`bevy_ecs`/`bevy_platform`, `std`) | 0.19.1 | 161/161 checks ✅ (whole public API, deterministic — see caveat below) |
  | `bevy-ptr-check` | `bevy_ptr` (no deps, no features) | 0.19.1 | 69/69 checks ✅ (whole public API, incl. a real unaligned-read check on ARMv6) |

  `crates/all-checks` bundles every check crate's `#[test]`s into a **single**
  on-device test binary (300 test functions total as of this writing) via
  `#[path]`, so `./scripts/send-tests.sh all` sends the whole suite as one app
  instead of one per crate.

  Findings, caveats and the full port classification live in `port.md`
  (`bevy_ecs` §B1–§B4/§B14, `bevy_math` §B9, `bevy_transform`
  §B10, `bevy_color` §B11, `bevy_time` §B12, `bevy_ptr` §B13). `bevy-time-check`
  drives every clock deterministically (`TimeUpdateStrategy`/`advance_by`)
  rather than the real wall clock — whether `Instant::now()` behaves on real
  3DS **hardware** stays an open question (§B6).

- `crates/test-console` — an interactive `#![test_runner]` for the 3DS. Build a
  check crate's tests with `--features console` and the **test list appears on
  the bottom screen** — the first row runs every test in sequence, the rest are
  the individual tests (Up/Dn move, L/R page, `A` run the highlighted row, `X`
  run all, `START` exit); results + panic messages scroll on the top screen. The
  `#[test]`
  functions are **unchanged** — the `console` feature just swaps
  `test_runner::run_gdb` for `test_console::run`.

  ```sh
  ./scripts/send-tests.sh bevy-math-check          # build + 3dslink to a 3DS
  ./scripts/send-tests.sh bevy-transform-check --ip 192.168.2.50
  ./scripts/send-tests.sh all                      # every check crate, one app
  cargo 3ds test --no-run -p bevy-ecs-check --features console   # just build the .3dsx
  ```

  Default (no `--features console`) is still the GDB runner that
  `scripts/test-emulator.sh` drives.

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
