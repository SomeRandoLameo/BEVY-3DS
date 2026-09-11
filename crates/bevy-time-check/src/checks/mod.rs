//! One submodule per `bevy_time` area. Every `#[test]` fn emits `ROW|` lines
//! (see `report`), so the emulator output transcribes straight into `README.md`.
//!
//! Kept self-contained (helpers + a private `mod report;`, no `crate::` refs) so
//! `crates/all-checks` can `#[path]`-include this tree unchanged.

mod report;

mod conditions;
mod delayed_commands;
mod fixed_time;
mod layout;
mod plugin;
mod real_time;
mod stopwatch;
mod time_generic;
mod timer;
mod virtual_time;
