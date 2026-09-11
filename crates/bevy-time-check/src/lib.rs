//! Standalone check for **`bevy_time`** on `armv6k-nintendo-3ds` — clocks,
//! `Timer`/`Stopwatch`, fixed timestep, run conditions, delayed commands, and
//! `TimePlugin` under a real `bevy_app::App`. Entirely deterministic (driven
//! via `TimeUpdateStrategy`/`advance_by`, never real wall-clock time) — see
//! `README.md` for what that leaves untested.
//!
//! Each check prints a Markdown table row (function | input | expected | got
//! | status); see `README.md`. Run: `./scripts/test-emulator.sh -p bevy-time-check`.

#![cfg_attr(test, feature(custom_test_frameworks))]
// Default: GDB-reporting runner (scripts/test-emulator.sh). `--features console`:
// interactive on-device runner with a bottom-screen menu.
#![cfg_attr(all(test, not(feature = "console")), test_runner(test_runner::run_gdb))]
#![cfg_attr(all(test, feature = "console"), test_runner(test_console::run))]

// All check logic lives in `checks/` so the aggregate `all-checks` crate can
// pull this crate's `#[test]`s in verbatim with `#[path]` (see `crates/all-checks`).
#[cfg(test)]
mod checks;
