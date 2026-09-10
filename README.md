# dove

A Nintendo 3DS homebrew app in Rust, built on [`ctru-rs`](https://github.com/rust3ds/ctru-rs).

Scaffolded with `cargo 3ds new` (the modern replacement for the archived
`rust3ds/rust3ds-template`).

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
cargo 3ds run                # run in Citra (if installed)
cargo 3ds run -- --address <3DS-IP>   # send to hardware via 3dslink
```

Put any files the app loads at runtime in `romfs/`.
