//! Standalone check for **`bevy_color`** on `armv6k-nintendo-3ds` — plain
//! color-space math, no rendering. Aims to touch every public color space and
//! its conversions.
//!
//! Every check prints a Markdown table row (function | input | expected | got
//! | status); see `README.md`. Run: `./scripts/test-emulator.sh -p bevy-color-check`.

#![cfg_attr(test, feature(custom_test_frameworks))]
// Default: GDB-reporting runner (scripts/test-emulator.sh). `--features console`:
// interactive on-device runner with a bottom-screen menu.
#![cfg_attr(all(test, not(feature = "console")), test_runner(test_runner::run_gdb))]
#![cfg_attr(all(test, feature = "console"), test_runner(test_console::run))]

// All check logic lives in `checks/` so the aggregate `all-checks` crate can
// pull this crate's `#[test]`s in verbatim with `#[path]` (see `crates/all-checks`).
#[cfg(test)]
mod checks;
