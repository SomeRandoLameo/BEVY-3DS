//! Misc `color_ops`/`color_range` surface not already covered per-type:
//! `Alpha for f32`, `Gray` across every space, and `ColorRange`.

use super::{chk, report};
use bevy_color::{
    Alpha, ColorRange, Gray, Hsla, Hsva, Hwba, Laba, Lcha, LinearRgba, Oklaba, Oklcha, Srgba, Xyza,
};

#[test]
fn alpha_for_f32() {
    report::f("f32::alpha() is itself", "0.5", 0.5, 0.5_f32.alpha());
    report::f("f32::with_alpha() replaces the value", "0.5 -> 0.9", 0.9, 0.5_f32.with_alpha(0.9));
    let mut a = 0.2_f32;
    a.set_alpha(0.8);
    report::f("f32::set_alpha", "", 0.8, a);
    report::is_true("f32::is_fully_opaque (1.0)", "", 1.0_f32.is_fully_opaque());
    report::is_true("f32::is_fully_transparent (0.0)", "", 0.0_f32.is_fully_transparent());
}

/// `Gray::gray(0.0)/gray(1.0)` must equal `BLACK`/`WHITE` in every color space —
/// mirrors `bevy_color`'s own generic `verify_gray::<Col>()` test.
#[test]
fn gray_black_and_white_every_space() {
    fn verify<C: Gray + core::fmt::Debug + PartialEq + Copy>(name: &str) {
        report::eq(&format!("{name}::gray(0.0) == BLACK"), "", C::BLACK, C::gray(0.0));
        report::eq(&format!("{name}::gray(1.0) == WHITE"), "", C::WHITE, C::gray(1.0));
    }
    verify::<Srgba>("Srgba");
    verify::<LinearRgba>("LinearRgba");
    verify::<Hsla>("Hsla");
    verify::<Hsva>("Hsva");
    verify::<Hwba>("Hwba");
    verify::<Laba>("Laba");
    verify::<Lcha>("Lcha");
    verify::<Oklaba>("Oklaba");
    verify::<Oklcha>("Oklcha");
    verify::<Xyza>("Xyza");
}

#[test]
fn color_range() {
    let range = Srgba::RED..Srgba::BLUE;
    report::eq("(RED..BLUE).at(-0.5) clamps to start", "", Srgba::RED, range.at(-0.5));
    report::eq("(RED..BLUE).at(0.0)", "", Srgba::RED, range.at(0.0));
    report::eq("(RED..BLUE).at(0.5)", "", Srgba::new(0.5, 0.0, 0.5, 1.0), range.at(0.5));
    report::eq("(RED..BLUE).at(1.0)", "", Srgba::BLUE, range.at(1.0));
    report::eq("(RED..BLUE).at(1.5) clamps to end", "", Srgba::BLUE, range.at(1.5));

    let lrange = LinearRgba::from(Srgba::RED)..LinearRgba::from(Srgba::BLUE);
    chk(
        "(LinearRgba RED..BLUE).at(0.5)",
        LinearRgba::new(0.5, 0.0, 0.5, 1.0),
        lrange.at(0.5),
        super::near(lrange.at(0.5), LinearRgba::new(0.5, 0.0, 0.5, 1.0), super::EPS),
    );
}
