//! `Srgba` — the full public surface: constructors, hex parsing, gamma
//! correction, the color-op traits, and componentwise vector-space math.

use super::{chk, near, report, EPS};
use bevy_color::{
    Alpha, ColorToComponents, ColorToPacked, Gray, HexColorError, LinearRgba, Luminance, Mix, Srgba,
};
use bevy_color::color_difference::EuclideanDistance;
use bevy_math::{StableInterpolate, Vec4, VectorSpace};

#[test]
fn constants_and_constructors() {
    report::eq("Srgba::RED", "", Srgba::new(1.0, 0.0, 0.0, 1.0), Srgba::RED);
    report::eq("Srgba::GREEN", "", Srgba::new(0.0, 1.0, 0.0, 1.0), Srgba::GREEN);
    report::eq("Srgba::BLUE", "", Srgba::new(0.0, 0.0, 1.0, 1.0), Srgba::BLUE);
    report::eq("Srgba::BLACK", "", Srgba::new(0.0, 0.0, 0.0, 1.0), Srgba::BLACK);
    report::eq("Srgba::WHITE", "", Srgba::new(1.0, 1.0, 1.0, 1.0), Srgba::WHITE);
    report::eq("Srgba::NONE (alpha 0)", "", 0.0, Srgba::NONE.alpha);
    report::eq("Srgba::rgb sets alpha 1", "rgb(0.2,0.4,0.6)", 1.0, Srgba::rgb(0.2, 0.4, 0.6).alpha);
    report::eq("Srgba::default() == WHITE", "", Srgba::WHITE, Srgba::default());
    report::eq("Srgba::with_red", "WHITE.with_red(0)", 0.0, Srgba::WHITE.with_red(0.0).red);
    report::eq("Srgba::with_green", "WHITE.with_green(0)", 0.0, Srgba::WHITE.with_green(0.0).green);
    report::eq("Srgba::with_blue", "WHITE.with_blue(0)", 0.0, Srgba::WHITE.with_blue(0.0).blue);
}

#[test]
fn hex_parsing() {
    report::eq("Srgba::hex(\"FFF\")", "", Ok(Srgba::WHITE), Srgba::hex("FFF"));
    report::eq("Srgba::hex(\"FFFF\")", "", Ok(Srgba::WHITE), Srgba::hex("FFFF"));
    report::eq("Srgba::hex(\"FFFFFF\")", "", Ok(Srgba::WHITE), Srgba::hex("FFFFFF"));
    report::eq("Srgba::hex(\"FFFFFFFF\")", "", Ok(Srgba::WHITE), Srgba::hex("FFFFFFFF"));
    report::eq("Srgba::hex(\"#FFFFFF\") strips '#'", "", Ok(Srgba::WHITE), Srgba::hex("#FFFFFF"));
    report::eq("Srgba::hex(\"000\")", "", Ok(Srgba::BLACK), Srgba::hex("000"));
    report::eq("Srgba::hex(\"03a9f4\")", "", Ok(Srgba::rgb_u8(3, 169, 244)), Srgba::hex("03a9f4"));
    report::eq("Srgba::hex(\"#f2a\") (RGB nibble expand)", "", Ok(Srgba::rgb_u8(255, 34, 170)), Srgba::hex("#f2a"));
    report::eq("Srgba::hex(\"12345678\")", "", Ok(Srgba::rgba_u8(18, 52, 86, 120)), Srgba::hex("12345678"));
    report::eq("Srgba::hex(\"yy\") -> Length err", "", Err(HexColorError::Length), Srgba::hex("yy"));
    report::eq("Srgba::hex(\"#ff\") -> Length err", "", Err(HexColorError::Length), Srgba::hex("#ff"));
    report::ok("Srgba::hex(\"yyy\") -> Parse err", "", "Parse(_)", "Parse(_)", matches!(Srgba::hex("yyy"), Err(HexColorError::Parse(_))));
}

#[test]
fn hex_roundtrip_and_u8() {
    report::ok("Srgba::to_hex (opaque)", "rgb_u8(3,169,244)", "#03A9F4", &Srgba::rgb_u8(3, 169, 244).to_hex(), Srgba::rgb_u8(3, 169, 244).to_hex() == "#03A9F4");
    report::ok("Srgba::to_hex (with alpha)", "rgba_u8(1,2,3,4)", "#01020304", &Srgba::rgba_u8(1, 2, 3, 4).to_hex(), Srgba::rgba_u8(1, 2, 3, 4).to_hex() == "#01020304");
    report::eq("Srgba::rgb_u8(255,0,0) == RED", "", Srgba::RED, Srgba::rgb_u8(255, 0, 0));
    report::eq("Srgba::to_u8_array", "BLUE", [0u8, 0, 255, 255], Srgba::BLUE.to_u8_array());
    report::eq("Srgba::to_u8_array_no_alpha", "GREEN", [0u8, 255, 0], Srgba::GREEN.to_u8_array_no_alpha());
    report::eq("Srgba::from_u8_array", "[255,0,0,255]", Srgba::RED, Srgba::from_u8_array([255, 0, 0, 255]));
    report::eq("Srgba::to_u8_array clamps out-of-range", "rgb(2,-1,0.5)", [255u8, 0, 128], Srgba::rgb(2.0, -1.0, 0.5).to_u8_array_no_alpha());
}

#[test]
fn gamma_correction() {
    report::f("Srgba::gamma_function(0.0)", "", 0.0, Srgba::gamma_function(0.0));
    report::f("Srgba::gamma_function(1.0)", "", 1.0, Srgba::gamma_function(1.0));
    report::f("Srgba::gamma_function_inverse(1.0)", "", 1.0, Srgba::gamma_function_inverse(1.0));
    report::f("Srgba::gamma_function(0.5) (gamma branch)", "", 0.21404114, Srgba::gamma_function(0.5));
    let rt = Srgba::gamma_function_inverse(Srgba::gamma_function(0.37));
    report::f("gamma_function round-trip", "0.37", 0.37, rt);
    report::f("Srgba::gamma_function(0.03) (linear branch)", "", 0.03 / 12.92, Srgba::gamma_function(0.03));
}

#[test]
fn srgba_linear_roundtrip() {
    let s = Srgba::new(0.0, 0.5, 1.0, 1.0);
    let lin: LinearRgba = s.into();
    report::f("Srgba->LinearRgba green (0.5 -> ~0.214)", "", 0.21404114, lin.green);
    report::f("Srgba->LinearRgba keeps alpha", "", 1.0, lin.alpha);
    let back: Srgba = lin.into();
    chk("Srgba->LinearRgba->Srgba round-trip", s, back, near(s, back, EPS));
}

#[test]
fn color_op_traits() {
    chk("Srgba::mix(RED, BLUE, 0.5)", Srgba::new(0.5, 0.0, 0.5, 1.0), Srgba::RED.mix(&Srgba::BLUE, 0.5), near(Srgba::RED.mix(&Srgba::BLUE, 0.5), Srgba::new(0.5, 0.0, 0.5, 1.0), EPS));
    report::f("Srgba::luminance (WHITE)", "", 1.0, Srgba::WHITE.luminance());
    report::f("Srgba::luminance (BLACK)", "", 0.0, Srgba::BLACK.luminance());
    report::f("Srgba::darker(0.1).darker(0.1) ~ darker(0.2)", "distributive", 0.0, Srgba::rgb(0.4, 0.5, 0.6).darker(0.1).darker(0.1).distance_squared(&Srgba::rgb(0.4, 0.5, 0.6).darker(0.2)));
    report::f("Srgba::with_alpha / alpha()", "0.25", 0.25, Srgba::WHITE.with_alpha(0.25).alpha());
    report::is_true("Srgba::is_fully_opaque (WHITE)", "", Srgba::WHITE.is_fully_opaque());
    report::is_true("Srgba::is_fully_transparent (NONE)", "", Srgba::NONE.is_fully_transparent());
    let mut m = Srgba::WHITE;
    m.set_alpha(0.0);
    report::f("Srgba::set_alpha", "WHITE -> 0", 0.0, m.alpha);
    report::f("Srgba::distance_squared (BLACK<->WHITE)", "", 3.0, Srgba::BLACK.distance_squared(&Srgba::WHITE));
    report::f("Srgba::distance (BLACK<->WHITE)", "", 3.0_f32.sqrt(), Srgba::BLACK.distance(&Srgba::WHITE));
    report::eq("Srgba::gray(0.0) == BLACK", "", <Srgba as Gray>::BLACK, Srgba::gray(0.0));
    report::eq("Srgba::gray(1.0) == WHITE", "", <Srgba as Gray>::WHITE, Srgba::gray(1.0));
}

#[test]
fn components_and_vector_space() {
    report::eq("Srgba::to_f32_array", "RED", [1.0, 0.0, 0.0, 1.0], Srgba::RED.to_f32_array());
    report::eq("Srgba::to_f32_array_no_alpha", "RED", [1.0, 0.0, 0.0], Srgba::RED.to_f32_array_no_alpha());
    report::eq("Srgba::to_vec4", "RED", Vec4::new(1.0, 0.0, 0.0, 1.0), Srgba::RED.to_vec4());
    report::eq("Srgba::from_vec4 round-trip", "RED", Srgba::RED, Srgba::from_vec4(Srgba::RED.to_vec4()));
    report::eq("Srgba::from_f32_array_no_alpha sets alpha 1", "", 1.0, Srgba::from_f32_array_no_alpha([0.1, 0.2, 0.3]).alpha);
    // impl_componentwise_vector_space: Add/Sub/Neg/Mul<f32>/Div/AddAssign act on all 4 channels.
    report::eq("Srgba + Srgba (incl. alpha)", "RED+GREEN", Srgba::new(1.0, 1.0, 0.0, 2.0), Srgba::RED + Srgba::GREEN);
    report::eq("Srgba - Srgba", "WHITE-RED", Srgba::new(0.0, 1.0, 1.0, 0.0), Srgba::WHITE - Srgba::RED);
    report::eq("Srgba * f32", "RED*0.5", Srgba::new(0.5, 0.0, 0.0, 0.5), Srgba::RED * 0.5);
    report::eq("f32 * Srgba", "0.5*RED", Srgba::new(0.5, 0.0, 0.0, 0.5), 0.5 * Srgba::RED);
    report::eq("Srgba / f32", "RED/2", Srgba::new(0.5, 0.0, 0.0, 0.5), Srgba::RED / 2.0);
    report::eq("-Srgba (Neg)", "-RED", Srgba::new(-1.0, 0.0, 0.0, -1.0), -Srgba::RED);
    report::eq("Srgba VectorSpace::ZERO", "", Srgba::new(0.0, 0.0, 0.0, 0.0), Srgba::ZERO);
    let mut a = Srgba::RED;
    a += Srgba::GREEN;
    report::eq("Srgba::add_assign", "RED+=GREEN", Srgba::new(1.0, 1.0, 0.0, 2.0), a);
    chk("Srgba::interpolate_stable(BLACK, WHITE, 0.5)", Srgba::new(0.5, 0.5, 0.5, 1.0), Srgba::BLACK.interpolate_stable(&Srgba::WHITE, 0.5), near(Srgba::BLACK.interpolate_stable(&Srgba::WHITE, 0.5), Srgba::new(0.5, 0.5, 0.5, 1.0), EPS));
}
