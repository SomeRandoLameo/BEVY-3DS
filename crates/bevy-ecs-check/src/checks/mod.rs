//! One submodule per `bevy_ecs` area. Every `#[test]` fn emits `ROW|` lines
//! (see `report`), so the emulator output transcribes straight into `README.md`.
//!
//! Kept self-contained (helpers + a private `mod report;`, no `crate::` refs) so
//! `crates/all-checks` can `#[path]`-include this tree unchanged.

mod report;

mod change_detection;
mod commands;
mod components_bundles;
mod entity_ref;
mod layout;
mod messages;
mod observers;
mod query_basic;
mod query_combinations;
mod query_filters;
mod relationships;
mod resources;
mod schedule_and_conditions;
mod system_params;
#[cfg(feature = "multi_threaded")]
mod threading;
mod world;
