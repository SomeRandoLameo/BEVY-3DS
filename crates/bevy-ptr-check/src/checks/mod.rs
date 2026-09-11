//! One submodule per `bevy_ptr` area. Every `#[test]` fn emits `ROW|` lines
//! (see `report`), so the emulator output transcribes straight into `README.md`.
//!
//! Kept self-contained (helpers + a private `mod report;`, no `crate::` refs) so
//! `crates/all-checks` can `#[path]`-include this tree unchanged.

mod report;

use core::cell::Cell;

/// Increments a shared counter on `Drop`, so tests can prove `bevy_ptr`'s
/// owning-pointer operations actually run (or correctly skip) destructors.
struct DropTracker<'a>(&'a Cell<u32>);

impl Drop for DropTracker<'_> {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

mod alignment;
mod const_non_null;
mod layout;
mod moving_ptr;
mod owning_ptr;
mod ptr;
mod ptr_mut;
mod thin_slice_ptr;
mod unsafe_cell_deref;
