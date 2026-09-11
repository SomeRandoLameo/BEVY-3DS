//! `OwningPtr<'a, A>` — type-erased, `Box`-like owned pointer (doesn't own the
//! *allocation*, just responsible for running `Drop`).

use super::{report, DropTracker};
use bevy_ptr::OwningPtr;
use core::cell::Cell;

#[test]
fn make_gives_a_safe_owning_ptr_for_the_closures_duration() {
    let ran = Cell::new(false);
    let value = 77_i32;
    OwningPtr::make(value, |ptr| {
        // SAFETY: `i32` is the type `make` erased.
        let back: i32 = unsafe { ptr.read() };
        report::eq("OwningPtr::make(77).read::<i32>()", "", 77_i32, back);
        ran.set(true);
    });
    report::is_true("OwningPtr::make actually invoked the closure", "", ran.get());
}

#[test]
fn read_extracts_the_value_without_double_dropping() {
    let counter = Cell::new(0_u32);
    OwningPtr::make(DropTracker(&counter), |ptr| {
        // SAFETY: `DropTracker` is the type `make` erased.
        let tracker: DropTracker = unsafe { ptr.read() };
        report::eq("OwningPtr::read() hasn't dropped the value yet", "", 0_u32, counter.get());
        drop(tracker);
    });
    report::eq("… dropping the read-out value runs Drop exactly once", "", 1_u32, counter.get());
}

#[test]
fn drop_as_runs_the_destructor_in_place() {
    let counter = Cell::new(0_u32);
    OwningPtr::make(DropTracker(&counter), |ptr| {
        // SAFETY: `DropTracker` is the type `make` erased.
        unsafe { ptr.drop_as::<DropTracker>() };
    });
    report::eq("OwningPtr::drop_as::<T>() runs Drop exactly once", "", 1_u32, counter.get());
}

#[test]
fn cast_to_moving_ptr_preserves_the_value() {
    OwningPtr::make(314_i64, |ptr| {
        // SAFETY: `i64` is the type `make` erased.
        let moving = unsafe { ptr.cast::<i64>() };
        report::eq("OwningPtr::cast::<i64>() -> MovingPtr::read()", "", 314_i64, moving.read());
    });
}

#[test]
fn as_ref_and_as_mut_views() {
    OwningPtr::make(1_u32, |mut ptr| {
        {
            let view = ptr.as_ref();
            // SAFETY: `u32` is the type `make` erased.
            report::eq("OwningPtr::as_ref() reads the current value", "", 1_u32, *unsafe { view.deref::<u32>() });
        }
        {
            let view = ptr.as_mut();
            // SAFETY: `u32` is the type `make` erased.
            *unsafe { view.deref_mut::<u32>() } = 2;
        }
        // SAFETY: `u32` is the type `make` erased.
        report::eq("OwningPtr::as_mut() mutation is visible afterwards", "", 2_u32, *unsafe { ptr.as_ref().deref::<u32>() });
        // Consume it so `make`'s internal `ManuallyDrop` doesn't leak.
        // SAFETY: `u32` is the type `make` erased and has no drop glue to skip.
        unsafe { ptr.drop_as::<u32>() };
    });
}
