//! `LinearRgba` — the render-facing space: CIE luminance, lightness adjustment,
//! `u32`/`u8` packing, and the vector-space math used with splines.

use super::{chk, near, report, EPS};
use bevy_color::{
    Alpha, ColorToComponents, ColorToPacked, Gray, LinearRgba, Luminance, Mix,
};
use bevy_color::color_difference::EuclideanDistance;
use bevy_math::{Vec3, VectorSpace};

#[test]
fn constants_and_constructors() {
    report::eq("LinearRgba::RED", "", LinearRgba::new(1.0, 0.0, 0.0, 1.0), LinearRgba::RED);
    report::eq("LinearRgba::BLACK", "", LinearRgba::new(0.0, 0.0, 0.0, 1.0), LinearRgba::BLACK);
    report::eq("LinearRgba::WHITE", "", LinearRgba::new(1.0, 1.0, 1.0, 1.0), LinearRgba::WHITE);
    report::eq("LinearRgba::NONE (alpha 0)", "", 0.0, LinearRgba::NONE.alpha);
    report::eq("LinearRgba::default() == WHITE", "", LinearRgba::WHITE, LinearRgba::default());
    report::is_true("LinearRgba::NAN is all-NaN", "", LinearRgba::NAN.red.is_nan() && LinearRgba::NAN.alpha.is_nan());
    report::eq("LinearRgba::rgb sets alpha 1", "", 1.0, LinearRgba::rgb(0.1, 0.2, 0.3).alpha);
    report::eq("LinearRgba::with_red", "", 0.5, LinearRgba::BLACK.with_red(0.5).red);
    report::eq("LinearRgba::with_green", "", 0.5, LinearRgba::BLACK.with_green(0.5).green);
    report::eq("LinearRgba::with_blue", "", 0.5, LinearRgba::BLACK.with_blue(0.5).blue);
}

#[test]
fn luminance_cie_weights() {
    report::f("LinearRgba::luminance (WHITE = .2126+.7152+.0722)", "", 1.0, LinearRgba::WHITE.luminance());
    report::f("LinearRgba::luminance (RED)", "", 0.2126, LinearRgba::RED.luminance());
    report::f("LinearRgba::luminance (GREEN)", "", 0.7152, LinearRgba::GREEN.luminance());
    report::f("LinearRgba::luminance (BLUE)", "", 0.0722, LinearRgba::BLUE.luminance());
    report::f("LinearRgba::with_luminance keeps target", "gray -> 0.5", 0.5, LinearRgba::new(0.3, 0.3, 0.3, 1.0).with_luminance(0.5).luminance());
    report::f("LinearRgba::darker distributive", "", 0.0, LinearRgba::new(0.4, 0.5, 0.6, 1.0).darker(0.1).darker(0.1).distance_squared(&LinearRgba::new(0.4, 0.5, 0.6, 1.0).darker(0.2)));
    report::f("LinearRgba::lighter distributive", "", 0.0, LinearRgba::new(0.4, 0.5, 0.6, 1.0).lighter(0.1).lighter(0.1).distance_squared(&LinearRgba::new(0.4, 0.5, 0.6, 1.0).lighter(0.2)));
}

#[test]
fn packing() {
    report::eq("LinearRgba::as_u32 (blue, opaque)", "A=MSB, R=LSB, LE", 0xFFFF_0000_u32, LinearRgba::new(0.0, 0.0, 1.0, 1.0).as_u32());
    report::eq("LinearRgba::to_u8_array", "BLUE", [0u8, 0, 255, 255], LinearRgba::BLUE.to_u8_array());
    report::eq("LinearRgba::to_u8_array_no_alpha", "cyan", [0u8, 255, 255], LinearRgba::rgb(0.0, 1.0, 1.0).to_u8_array_no_alpha());
    report::eq("LinearRgba::from_u8_array", "[255,0,0,255]", LinearRgba::new(1.0, 0.0, 0.0, 1.0), LinearRgba::from_u8_array([255, 0, 0, 255]));
    report::eq("LinearRgba::from_u8_array_no_alpha", "[255,255,0]", LinearRgba::rgb(1.0, 1.0, 0.0), LinearRgba::from_u8_array_no_alpha([255, 255, 0]));
    report::eq("LinearRgba::to_u8_array clamps", "rgb(0,100,-100)", [0u8, 255, 0], LinearRgba::rgb(0.0, 100.0, -100.0).to_u8_array_no_alpha());
}

#[test]
fn traits() {
    chk("LinearRgba::mix(RED, BLUE, 0.5)", LinearRgba::new(0.5, 0.0, 0.5, 1.0), LinearRgba::RED.mix(&LinearRgba::BLUE, 0.5), near(LinearRgba::RED.mix(&LinearRgba::BLUE, 0.5), LinearRgba::new(0.5, 0.0, 0.5, 1.0), EPS));
    report::f("LinearRgba::distance_squared (BLACK<->WHITE)", "", 3.0, LinearRgba::BLACK.distance_squared(&LinearRgba::WHITE));
    report::f("LinearRgba::with_alpha", "", 0.25, LinearRgba::WHITE.with_alpha(0.25).alpha());
    report::eq("LinearRgba::gray(0.0) == BLACK", "", <LinearRgba as Gray>::BLACK, LinearRgba::gray(0.0));
    report::eq("LinearRgba::gray(1.0) == WHITE", "", <LinearRgba as Gray>::WHITE, LinearRgba::gray(1.0));
    chk("LinearRgba::gray(0.5)", LinearRgba::new(0.5, 0.5, 0.5, 1.0), LinearRgba::gray(0.5), near(LinearRgba::gray(0.5), LinearRgba::new(0.5, 0.5, 0.5, 1.0), EPS));
    report::eq("LinearRgba::to_vec3", "RED", Vec3::new(1.0, 0.0, 0.0), LinearRgba::RED.to_vec3());
    report::eq("LinearRgba::from_vec3 round-trip", "", LinearRgba::rgb(0.2, 0.4, 0.6), LinearRgba::from_vec3(Vec3::new(0.2, 0.4, 0.6)));
    report::eq("LinearRgba + LinearRgba", "RED+GREEN", LinearRgba::new(1.0, 1.0, 0.0, 2.0), LinearRgba::RED + LinearRgba::GREEN);
    report::eq("LinearRgba * f32", "RED*0.5", LinearRgba::new(0.5, 0.0, 0.0, 0.5), LinearRgba::RED * 0.5);
    report::eq("LinearRgba VectorSpace::ZERO", "", LinearRgba::new(0.0, 0.0, 0.0, 0.0), LinearRgba::ZERO);
}
