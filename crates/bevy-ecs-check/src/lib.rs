//! Standalone check for `bevy_ecs` on the 3DS target (`armv6k-nintendo-3ds`).
//!
//! No `bevy_render`, no `App` — just core ECS: spawn/despawn, `Query` iteration
//! (incl. `&mut`), a `Resource`, and a `Schedule` with several systems. Confirms
//! the ECS actually *behaves* on-device.
//!
//! Each check prints a Markdown table row (function | input | expected | got |
//! status); see `README.md`. Run: `./scripts/test-emulator.sh -p bevy-ecs-check`.

#![cfg_attr(test, feature(custom_test_frameworks))]
// Default: GDB-reporting runner (used by scripts/test-emulator.sh).
// `--features console`: interactive on-device runner (bottom-screen menu).
#![cfg_attr(all(test, not(feature = "console")), test_runner(test_runner::run_gdb))]
#![cfg_attr(all(test, feature = "console"), test_runner(test_console::run))]

// All check logic lives in `checks/` so the aggregate `all-checks` crate can
// pull this crate's `#[test]`s in verbatim with `#[path]` (see `crates/all-checks`).
#[cfg(test)]
mod checks;
