//! `ThinSlicePtr<'a, T>` — `&'a [T]` minus the length (bounds-checked only in
//! debug builds), plus its `UnsafeCell<T>` specialisation.

use super::report;
use bevy_ptr::ThinSlicePtr;
use core::cell::UnsafeCell;

#[test]
fn from_slice_and_get_unchecked() {
    let data = [2_u32, 4, 8, 16];
    let thin: ThinSlicePtr<u32> = ThinSlicePtr::from(data.as_slice());
    // SAFETY: indices 0..4 are in-bounds of `data`.
    unsafe {
        report::eq("ThinSlicePtr::get_unchecked(0)", "", 2_u32, *thin.get_unchecked(0));
        report::eq("ThinSlicePtr::get_unchecked(1)", "", 4_u32, *thin.get_unchecked(1));
        report::eq("ThinSlicePtr::get_unchecked(3)", "", 16_u32, *thin.get_unchecked(3));
    }
}

#[test]
fn as_slice_unchecked_rebuilds_a_slice() {
    let data = [1_i32, 2, 3, 4, 5];
    let thin: ThinSlicePtr<i32> = ThinSlicePtr::from(data.as_slice());
    // SAFETY: `data` has no other live mutable aliases and `len <= data.len()`.
    let back: &[i32] = unsafe { thin.as_slice_unchecked(data.len()) };
    report::eq("ThinSlicePtr::as_slice_unchecked(len) round-trips the slice", "", &data[..], back);
    // SAFETY: 3 <= data.len().
    let prefix: &[i32] = unsafe { thin.as_slice_unchecked(3) };
    report::eq("… a shorter length gives a prefix", "", &data[..3], prefix);
}

#[test]
fn is_copy_and_clone() {
    let data = [9_u8, 8, 7];
    let thin: ThinSlicePtr<u8> = ThinSlicePtr::from(data.as_slice());
    let cloned = thin.clone();
    let copied = thin; // Copy — `thin` stays usable too.
    // SAFETY: index 0 is in-bounds.
    unsafe {
        report::eq("ThinSlicePtr is Copy", "", 9_u8, *copied.get_unchecked(0));
        report::eq("ThinSlicePtr::clone()", "", 9_u8, *cloned.get_unchecked(0));
    }
}

#[test]
fn unsafe_cell_specialisation_allows_mutation_through_a_shared_slice() {
    let data = [UnsafeCell::new(1_i32), UnsafeCell::new(2), UnsafeCell::new(3)];
    let thin: ThinSlicePtr<UnsafeCell<i32>> = ThinSlicePtr::from(data.as_slice());
    // SAFETY: no other aliases to `data` exist right now, and `len <= data.len()`.
    let mutable: &mut [i32] = unsafe { thin.as_mut_slice_unchecked(data.len()) };
    mutable[1] = 20;
    report::eq("ThinSlicePtr<UnsafeCell<T>>::as_mut_slice_unchecked mutates through", "", 20, unsafe { *data[1].get() });

    let plain: ThinSlicePtr<i32> = thin.cast();
    // SAFETY: index 0 is in-bounds.
    report::eq("ThinSlicePtr<UnsafeCell<T>>::cast() strips the UnsafeCell layer", "", 1_i32, *unsafe { plain.get_unchecked(0) });
}
