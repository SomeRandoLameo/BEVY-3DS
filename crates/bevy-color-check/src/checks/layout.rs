//! Memory layout of `bevy_color`'s public types on `armv6k-nintendo-3ds`
//! (records the actual `size_of`/`align_of`; every `*a` type is 4 `f32`s).

use super::report;
use bevy_color::{Color, Hsla, Hsva, Hwba, Laba, Lcha, LinearRgba, Oklaba, Oklcha, Srgba, Xyza};
use core::mem::{align_of, size_of};

fn rec<T>(name: &str) {
    report::ok(name, "", "size / align", &format!("{} / {}", size_of::<T>(), align_of::<T>()), true);
}

#[test]
fn type_layout() {
    rec::<Srgba>("size/align Srgba");
    rec::<LinearRgba>("size/align LinearRgba");
    rec::<Hsla>("size/align Hsla");
    rec::<Hsva>("size/align Hsva");
    rec::<Hwba>("size/align Hwba");
    rec::<Laba>("size/align Laba");
    rec::<Lcha>("size/align Lcha");
    rec::<Oklaba>("size/align Oklaba");
    rec::<Oklcha>("size/align Oklcha");
    rec::<Xyza>("size/align Xyza");
    rec::<Color>("size/align Color (enum)");
    report::eq("every *a color type is 4 x f32", "", 16, size_of::<Srgba>());
}
