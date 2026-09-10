//! Memory layout of `bevy_transform`'s public types on `armv6k-nintendo-3ds`
//! (records the actual `size_of` / `align_of`).

use super::report;
use core::mem::{align_of, size_of};

fn rec<T>(name: &str) {
    report::ok(name, "", "size / align", &format!("{} / {}", size_of::<T>(), align_of::<T>()), true);
}

#[test]
fn type_layout() {
    use bevy_transform::components::TransformTreeChanged;
    use bevy_transform::prelude::*;
    rec::<Transform>("size/align Transform");
    rec::<GlobalTransform>("size/align GlobalTransform");
    rec::<TransformTreeChanged>("size/align TransformTreeChanged");
}
