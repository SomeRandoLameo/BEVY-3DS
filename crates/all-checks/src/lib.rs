//! One on-device test binary containing **every** `bevy-*-check` crate's
//! `#[test]`s, so the whole suite ships as a single `.3dsx`.
//!
//! Nothing is re-implemented here: each check crate keeps its logic in a
//! self-contained `src/checks/` tree (helpers + a private `report` module, no
//! `crate::` references), and this crate pulls each tree in with `#[path]`. When
//! a new `bevy-*-check` crate is added, add its `checks/` tree below and mirror
//! its deps/features into `Cargo.toml`.
//!
//! Run: `./scripts/send-tests.sh all` (interactive) or
//! `./scripts/test-emulator.sh -p all-checks` (GDB).

#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(all(test, not(feature = "console")), test_runner(test_runner::run_gdb))]
#![cfg_attr(all(test, feature = "console"), test_runner(test_console::run))]

#[cfg(test)]
#[path = "../../bevy-color-check/src/checks/mod.rs"]
mod bevy_color_check;

#[cfg(test)]
#[path = "../../bevy-ecs-check/src/checks/mod.rs"]
mod bevy_ecs_check;

#[cfg(test)]
#[path = "../../bevy-math-check/src/checks/mod.rs"]
mod bevy_math_check;

#[cfg(test)]
#[path = "../../bevy-ptr-check/src/checks/mod.rs"]
mod bevy_ptr_check;

#[cfg(test)]
#[path = "../../bevy-time-check/src/checks/mod.rs"]
mod bevy_time_check;

#[cfg(test)]
#[path = "../../bevy-transform-check/src/checks/mod.rs"]
mod bevy_transform_check;
