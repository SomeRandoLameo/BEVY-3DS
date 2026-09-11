//! `Aligned` vs `Unaligned` — the actually platform-relevant part of
//! `bevy_ptr`: does `armv6k` (ARM11, no strict alignment fault for the loads
//! newlib/rustc emit here) round-trip a genuinely misaligned multi-byte read
//! correctly through the `Unaligned` path, and does the `Aligned` path's
//! debug-mode alignment check actually catch a misaligned pointer?

use super::report;
use bevy_ptr::{OwningPtr, Ptr, Unaligned};
use core::mem::size_of;
use core::ptr::NonNull;

/// A byte buffer with a `u32`-sized value written at an offset chosen so the
/// result is *always* misaligned for `u32` (needs 4-byte alignment) —
/// `[u8; N]` locals only need 1-byte alignment, so the buffer's own base
/// address isn't guaranteed aligned or misaligned either way; picking the
/// offset from the actual base address (rather than a fixed `+1`) makes this
/// deterministic instead of true only some of the time.
fn misaligned_u32_buffer(value: u32) -> ([u8; 9], usize) {
    let mut buf = [0_u8; 9];
    let base = buf.as_ptr() as usize;
    let offset = if base % size_of::<u32>() == 0 { 1 } else { 0 };
    // SAFETY: `buf` is 9 bytes, so a 4-byte write at `offset` (0 or 1, so
    // bytes 0..4 or 1..5) is in-bounds; `write_unaligned` doesn't require
    // alignment.
    unsafe { buf.as_mut_ptr().add(offset).cast::<u32>().write_unaligned(value) };
    (buf, offset)
}

#[test]
fn unaligned_read_across_a_misaligned_offset_returns_the_right_value() {
    let (mut buf, offset) = misaligned_u32_buffer(0xDEAD_BEEF);
    let misaligned = buf.as_mut_ptr().wrapping_add(offset);
    report::is_true(
        "test setup: offset+1 really is misaligned for u32",
        "",
        (misaligned as usize) % size_of::<u32>() != 0,
    );

    let nn = NonNull::new(misaligned).unwrap();
    // SAFETY: `misaligned` points to 4 valid, initialized bytes within `buf` —
    // any 4-byte pattern is a valid `u32`, and nothing else accesses `buf`
    // while this pointer is alive.
    let owning: OwningPtr<Unaligned> = unsafe { OwningPtr::new(nn) };
    // SAFETY: `u32` is the type written at `misaligned` by `misaligned_u32_buffer`.
    let read_back: u32 = unsafe { owning.read_unaligned() };
    report::eq("OwningPtr<Unaligned>::read_unaligned::<u32>() on a misaligned ARM load", "", 0xDEAD_BEEF_u32, read_back);
}

#[test]
fn to_unaligned_lets_a_misaligned_ptr_still_be_constructed() {
    let (mut buf, offset) = misaligned_u32_buffer(0x1234_5678);
    let misaligned = buf.as_mut_ptr().wrapping_add(offset);
    let nn = NonNull::new(misaligned).unwrap();
    // SAFETY: `nn` is non-null and points into `buf`; `Ptr::new`'s contract
    // for an `Aligned` `Ptr` isn't violated here because we immediately
    // downgrade to `Unaligned` before ever dereferencing it.
    let p: Ptr = unsafe { Ptr::new(nn) };
    let unaligned = p.to_unaligned();
    report::eq("Ptr::to_unaligned() keeps the (misaligned) address", "", misaligned as usize, unaligned.as_ptr() as usize);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "not aligned")]
fn aligned_deref_panics_in_debug_on_a_misaligned_pointer() {
    let (mut buf, offset) = misaligned_u32_buffer(0);
    let misaligned = buf.as_mut_ptr().wrapping_add(offset);
    let nn = NonNull::new(misaligned).unwrap();
    // SAFETY (of constructing the `Ptr`, not of the `.deref()` below): `nn` is
    // non-null. The whole point of this test is that `deref::<u32>()` below
    // is *not* actually safe to call (misaligned) and `debug_ensure_aligned`
    // catches exactly that with a panic in debug builds.
    let p: Ptr = unsafe { Ptr::new(nn) };
    let _ = unsafe { p.deref::<u32>() };
}
