//! `PtrMut<'a, A>` — type-erased, `&'a mut T`-like exclusive borrow.

use super::report;
use bevy_ptr::{OwningPtr, Ptr, PtrMut};
use core::mem::ManuallyDrop;

#[test]
fn from_mut_and_deref_mut_roundtrip() {
    let mut x = 1_i32;
    {
        let p: PtrMut = PtrMut::from(&mut x);
        // SAFETY: `p` erases a `&mut i32`, so `i32` is the correct pointee type.
        let r: &mut i32 = unsafe { p.deref_mut() };
        *r = 42;
    }
    report::eq("PtrMut::from(&mut i32).deref_mut() mutates the original", "", 42_i32, x);
}

#[test]
fn as_ptr_matches_source_address() {
    let mut x = 1_u8;
    let addr_before = &mut x as *mut u8 as usize;
    let p: PtrMut = PtrMut::from(&mut x);
    report::eq("PtrMut::as_ptr() keeps the original address", "", addr_before, p.as_ptr() as usize);
}

#[test]
fn reborrow_gives_a_shorter_lived_handle_to_the_same_data() {
    let mut x = 5_i32;
    let mut p: PtrMut = PtrMut::from(&mut x);
    {
        let sub = p.reborrow();
        // SAFETY: `i32` is the erased pointee type.
        *unsafe { sub.deref_mut::<i32>() } = 9;
    }
    // `p` is still usable after the reborrow ends.
    // SAFETY: `i32` is the erased pointee type.
    report::eq("PtrMut::reborrow() mutation is visible through the original", "", 9_i32, *unsafe { p.deref_mut::<i32>() });
}

#[test]
fn as_ref_gives_a_read_only_view() {
    let mut x = 7_i32;
    let p: PtrMut = PtrMut::from(&mut x);
    let view: Ptr = p.as_ref();
    // SAFETY: `i32` is the erased pointee type.
    report::eq("PtrMut::as_ref() reads the current value", "", 7_i32, *unsafe { view.deref::<i32>() });
}

#[test]
fn promote_to_owning_ptr_hands_over_ownership() {
    // Build exclusivity honestly: an owned, not-otherwise-observed value —
    // mirrors `OwningPtr::make_internal`'s own justification for `promote`.
    let mut val = ManuallyDrop::new(String::from("owned"));
    let p: PtrMut = PtrMut::from(&mut *val);
    // SAFETY: `val` is not read again after this; `p` is the only handle.
    let owning: OwningPtr = unsafe { p.promote() };
    // SAFETY: `String` is the erased pointee type.
    let s: String = unsafe { owning.read() };
    report::eq("PtrMut::promote() -> OwningPtr::read::<String>()", "owned", "owned".to_string(), s);
}
