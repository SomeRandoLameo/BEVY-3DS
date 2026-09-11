//! `ConstNonNull` — the `*const T` counterpart to `NonNull<T>`'s `*mut T`.

use super::report;
use bevy_ptr::ConstNonNull;
use core::ptr::NonNull;

#[test]
fn new_rejects_null_accepts_non_null() {
    let x = 42_u32;
    report::is_true("ConstNonNull::new(&x) is Some", "", ConstNonNull::new(&x as *const u32).is_some());
    report::is_true("ConstNonNull::new(null) is None", "", ConstNonNull::<u32>::new(core::ptr::null()).is_none());
}

#[test]
fn new_unchecked_and_as_ref_read_through() {
    let x = 7_i64;
    // SAFETY: `&x` is non-null.
    let ptr = unsafe { ConstNonNull::new_unchecked(&x as *const i64) };
    // SAFETY: `ptr` points to a live, initialized `i64`.
    let r: &i64 = unsafe { ptr.as_ref() };
    report::eq("ConstNonNull::new_unchecked + as_ref reads through", "&x = 7", 7_i64, *r);
}

#[test]
fn from_conversions() {
    let x = 5_u32;
    let from_ref: ConstNonNull<u32> = ConstNonNull::from(&x);
    // SAFETY: `from_ref` was built from a live `&x`.
    report::eq("ConstNonNull::from(&T)", "", 5_u32, unsafe { *from_ref.as_ref() });

    let mut y = 6_u32;
    let from_mut_ref: ConstNonNull<u32> = ConstNonNull::from(&mut y);
    // SAFETY: `from_mut_ref` was built from a live `&mut y`.
    report::eq("ConstNonNull::from(&mut T)", "", 6_u32, unsafe { *from_mut_ref.as_ref() });

    let mut z = 9_u32;
    let non_null = NonNull::from(&mut z);
    let from_non_null: ConstNonNull<u32> = ConstNonNull::from(non_null);
    // SAFETY: `from_non_null` was built from a live `NonNull`.
    report::eq("ConstNonNull::from(NonNull<T>)", "", 9_u32, unsafe { *from_non_null.as_ref() });
}

#[test]
fn is_copy_and_clone() {
    let x = 1_u32;
    let ptr = ConstNonNull::from(&x);
    let cloned = ptr.clone();
    let copied: ConstNonNull<u32> = ptr; // Copy, not moved out from under `cloned`.
    // SAFETY: both point to the still-live `x`.
    report::eq("ConstNonNull is Copy — original still usable", "", 1_u32, unsafe { *copied.as_ref() });
    report::eq("ConstNonNull::clone() reads the same value", "", 1_u32, unsafe { *cloned.as_ref() });
}
