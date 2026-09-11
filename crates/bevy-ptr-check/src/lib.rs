//! Standalone check for **`bevy_ptr`** on `armv6k-nintendo-3ds` — the
//! type-erased raw-pointer toolkit `bevy_ecs` builds its dynamic storage on
//! top of. Zero dependencies, no features; aims to touch every public item.
//!
//! Each check prints a Markdown table row (function | input | expected | got
//! | status); see `README.md`. Run: `./scripts/test-emulator.sh -p bevy-ptr-check`.

#![cfg_attr(test, feature(custom_test_frameworks))]
// Default: GDB-reporting runner (scripts/test-emulator.sh). `--features console`:
// interactive on-device runner with a bottom-screen menu.
#![cfg_attr(all(test, not(feature = "console")), test_runner(test_runner::run_gdb))]
#![cfg_attr(all(test, feature = "console"), test_runner(test_console::run))]

// All check logic lives in `checks/` so the aggregate `all-checks` crate can
// pull this crate's `#[test]`s in verbatim with `#[path]` (see `crates/all-checks`).
#[cfg(test)]
mod checks;
