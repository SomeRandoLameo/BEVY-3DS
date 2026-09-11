//! `UnsafeCellDeref` — helper methods for going from `&UnsafeCell<T>` to
//! `&T`/`&mut T`/a copy of `T`, without the ceremony of raw `.get()` calls.

use super::report;
use bevy_ptr::UnsafeCellDeref;
use core::cell::UnsafeCell;

#[test]
fn deref_reads_the_current_value() {
    let cell = UnsafeCell::new(11_i32);
    // SAFETY: no `&mut` is derived from `cell` while this borrow is live.
    let r: &i32 = unsafe { (&cell).deref() };
    report::eq("UnsafeCellDeref::deref()", "", 11_i32, *r);
}

#[test]
fn deref_mut_allows_mutation() {
    let cell = UnsafeCell::new(1_i32);
    {
        // SAFETY: this is the only reference (mutable or otherwise) to `cell`'s
        // contents for its duration.
        let r: &mut i32 = unsafe { (&cell).deref_mut() };
        *r = 5;
    }
    // SAFETY: no other reference is live.
    report::eq("UnsafeCellDeref::deref_mut() mutation is visible", "", 5_i32, unsafe { *(&cell).deref() });
}

#[test]
fn read_copies_out_without_borrowing() {
    let cell = UnsafeCell::new(3_i32);
    // SAFETY: no `&mut` to `cell`'s contents exists.
    let copy: i32 = unsafe { (&cell).read() };
    report::eq("UnsafeCellDeref::read() (Copy types)", "", 3_i32, copy);
    // The cell itself is untouched — still readable afterwards.
    // SAFETY: no `&mut` is live.
    report::eq("… the cell is unchanged after read()", "", 3_i32, unsafe { *(&cell).deref() });
}
