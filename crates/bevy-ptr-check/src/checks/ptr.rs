//! `Ptr<'a, A>` — type-erased, `&'a T`-like shared borrow.

use super::report;
use bevy_ptr::{OwningPtr, Ptr};
use core::mem::ManuallyDrop;

#[test]
fn from_ref_and_deref_roundtrip() {
    let x = 123_i32;
    let p: Ptr = Ptr::from(&x);
    // SAFETY: `p` erases a `&i32`, so `i32` is the correct pointee type.
    let back: &i32 = unsafe { p.deref() };
    report::eq("Ptr::from(&i32) -> deref::<i32>() round-trips", "", 123_i32, *back);
}

#[test]
fn as_ptr_matches_source_address() {
    let x = 1_u8;
    let addr_before = &x as *const u8 as usize;
    let p: Ptr = Ptr::from(&x);
    report::eq("Ptr::as_ptr() keeps the original address", "", addr_before, p.as_ptr() as usize);
}

#[test]
fn byte_offset_and_byte_add_index_into_an_array() {
    let arr = [10_i32, 20, 30];
    let p: Ptr = Ptr::from(&arr);
    // SAFETY: offsetting by one/two `i32`s stays within `arr`'s allocation and
    // keeps 4-byte alignment (the array itself is aligned, elements are 4B).
    let second = unsafe { p.byte_add(4) };
    let third = unsafe { p.byte_offset(8) };
    report::eq("Ptr::byte_add(4) lands on arr[1]", "", 20_i32, *unsafe { second.deref::<i32>() });
    report::eq("Ptr::byte_offset(8) lands on arr[2]", "", 30_i32, *unsafe { third.deref::<i32>() });
}

#[test]
fn to_unaligned_preserves_the_address() {
    let x = 9_u32;
    let p: Ptr = Ptr::from(&x);
    let addr = p.as_ptr();
    let unaligned = p.to_unaligned();
    report::eq("Ptr::to_unaligned() only changes the type parameter", "", addr, unaligned.as_ptr());
}

#[test]
fn assert_unique_promotes_a_genuinely_exclusive_ptr_to_ptr_mut() {
    // Build exclusivity honestly: an owned value nobody else can see, matching
    // how `OwningPtr::make`'s own internals justify `assert_unique`/`promote`.
    let mut val = ManuallyDrop::new(55_i64);
    // SAFETY: `val` is not observed elsewhere, so treating its borrow as
    // exclusive (and later as owning) is sound.
    let owning: OwningPtr = unsafe { OwningPtr::new(core::ptr::NonNull::from(&mut *val).cast()) };
    let shared: Ptr = owning.as_ref();
    // SAFETY: `shared` still has exclusive access via `owning`'s chain, and no
    // other `PtrMut` is created for it before this one is used.
    let unique = unsafe { shared.assert_unique() };
    // SAFETY: `i64` is the erased pointee type.
    let r: &mut i64 = unsafe { unique.deref_mut() };
    report::eq("Ptr::assert_unique() -> PtrMut::deref_mut() sees the value", "", 55_i64, *r);
}
