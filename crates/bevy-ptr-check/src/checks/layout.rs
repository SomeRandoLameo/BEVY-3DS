//! Memory layout of `bevy_ptr`'s public types on `armv6k-nintendo-3ds`
//! (records the actual `size_of`/`align_of`; the pointer types should all be
//! exactly pointer-sized — no niche/tag overhead, `repr(transparent)` over
//! `NonNull<u8>`).

use super::report;
use bevy_ptr::{ConstNonNull, OwningPtr, Ptr, PtrMut, ThinSlicePtr};
use core::mem::{align_of, size_of};

fn rec<T>(name: &str) {
    report::ok(name, "", "size / align", &format!("{} / {}", size_of::<T>(), align_of::<T>()), true);
}

#[test]
fn type_layout() {
    rec::<Ptr>("size/align Ptr");
    rec::<PtrMut>("size/align PtrMut");
    rec::<OwningPtr>("size/align OwningPtr");
    rec::<ConstNonNull<u32>>("size/align ConstNonNull<u32>");
    rec::<ThinSlicePtr<u32>>("size/align ThinSlicePtr<u32> (has a debug-only len field)");

    let ptr_size = size_of::<*const u8>();
    report::eq("Ptr/PtrMut/OwningPtr/ConstNonNull are exactly pointer-sized", "repr(transparent) over NonNull<u8>", ptr_size, size_of::<Ptr>());
    report::eq("… PtrMut too", "", ptr_size, size_of::<PtrMut>());
    report::eq("… OwningPtr too", "", ptr_size, size_of::<OwningPtr>());
    report::eq("… ConstNonNull<u32> too", "", ptr_size, size_of::<ConstNonNull<u32>>());
}
