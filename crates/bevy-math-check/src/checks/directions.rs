//! `bevy_math` directions & 2D rotation: `Dir2`, `Dir3`, `Dir3A`, `Rot2`.

use super::{near2, near3, report, EPS};
use bevy_math::{Dir2, Dir3, Quat, Rot2, Vec2, Vec3};
use core::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

#[test]
fn dir2() {
    report::approx("Dir2::new normalises", "(3,4)", Vec2::new(0.6, 0.8), Dir2::new(Vec2::new(3.0, 4.0)).unwrap().as_vec2(), near2(Dir2::new(Vec2::new(3.0, 4.0)).unwrap().as_vec2(), Vec2::new(0.6, 0.8)));
    report::ok("Dir2::new rejects zero", "(0,0)", "Err", if Dir2::new(Vec2::ZERO).is_err() { "Err" } else { "Ok" }, Dir2::new(Vec2::ZERO).is_err());
    report::ok("Dir2::new rejects NaN", "(NaN,0)", "Err", if Dir2::new(Vec2::new(f32::NAN, 0.0)).is_err() { "Err" } else { "Ok" }, Dir2::new(Vec2::new(f32::NAN, 0.0)).is_err());
    report::approx("Dir2::from_angle(PI/2)", "", Vec2::Y, Dir2::from_angle(FRAC_PI_2).as_vec2(), near2(Dir2::from_angle(FRAC_PI_2).as_vec2(), Vec2::Y));
    report::approx("Dir2::slerp(X, Y, 0.5)", "", Vec2::new(FRAC_PI_4.cos(), FRAC_PI_4.sin()), Dir2::X.slerp(Dir2::Y, 0.5).as_vec2(), near2(Dir2::X.slerp(Dir2::Y, 0.5).as_vec2(), Vec2::new(FRAC_PI_4.cos(), FRAC_PI_4.sin())));
    report::f("Dir2::rotation_to(X, Y).as_radians()", "", FRAC_PI_2, Dir2::X.rotation_to(Dir2::Y).as_radians());
    report::approx("-Dir2 (Neg)", "-X", Vec2::NEG_X, (-Dir2::X).as_vec2(), near2((-Dir2::X).as_vec2(), Vec2::NEG_X));
}

#[test]
fn dir3() {
    report::approx("Dir3::new normalises", "(0,0,5)", Vec3::Z, Dir3::new(Vec3::new(0.0, 0.0, 5.0)).unwrap().as_vec3(), near3(Dir3::new(Vec3::new(0.0, 0.0, 5.0)).unwrap().as_vec3(), Vec3::Z));
    report::ok("Dir3::new rejects zero", "(0,0,0)", "Err", if Dir3::new(Vec3::ZERO).is_err() { "Err" } else { "Ok" }, Dir3::new(Vec3::ZERO).is_err());
    report::approx("Dir3::new_unchecked", "(1,0,0)", Vec3::X, Dir3::new_unchecked(Vec3::X).as_vec3(), near3(Dir3::new_unchecked(Vec3::X).as_vec3(), Vec3::X));
    let (d, len) = Dir3::new_and_length(Vec3::new(0.0, 3.0, 4.0)).unwrap();
    report::f("Dir3::new_and_length — length", "(0,3,4)", 5.0, len);
    report::approx("Dir3::new_and_length — dir", "(0,3,4)", Vec3::new(0.0, 0.6, 0.8), d.as_vec3(), near3(d.as_vec3(), Vec3::new(0.0, 0.6, 0.8)));
    report::approx("Dir3::slerp(X, Y, 0.5)", "", Vec3::new(FRAC_PI_4.cos(), FRAC_PI_4.sin(), 0.0), Dir3::X.slerp(Dir3::Y, 0.5).as_vec3(), near3(Dir3::X.slerp(Dir3::Y, 0.5).as_vec3(), Vec3::new(FRAC_PI_4.cos(), FRAC_PI_4.sin(), 0.0)));
    report::approx("Dir3::fast_renormalize", "(1.001,0,0)", Vec3::X, Dir3::new_unchecked(Vec3::new(1.001, 0.0, 0.0)).fast_renormalize().as_vec3(), near3(Dir3::new_unchecked(Vec3::new(1.001, 0.0, 0.0)).fast_renormalize().as_vec3(), Vec3::X));
    report::approx("Quat::from_rotation_arc(X, Z) * X ≈ Z", "", Vec3::Z, Quat::from_rotation_arc(Vec3::X, Vec3::Z).mul_vec3(Vec3::X), near3(Quat::from_rotation_arc(Vec3::X, Vec3::Z).mul_vec3(Vec3::X), Vec3::Z));
}

#[test]
fn rot2() {
    report::approx("Rot2::degrees(90) * X", "", Vec2::Y, Rot2::degrees(90.0) * Vec2::X, near2(Rot2::degrees(90.0) * Vec2::X, Vec2::Y));
    report::approx("Rot2::radians(PI/2) * X", "", Vec2::Y, Rot2::radians(FRAC_PI_2) * Vec2::X, near2(Rot2::radians(FRAC_PI_2) * Vec2::X, Vec2::Y));
    report::f("Rot2::degrees(90).as_radians()", "", FRAC_PI_2, Rot2::degrees(90.0).as_radians());
    report::f("Rot2::radians(PI/2).as_degrees()", "", 90.0, Rot2::radians(FRAC_PI_2).as_degrees());
    report::f("Rot2::radians(PI/2).as_turn_fraction()", "", 0.25, Rot2::radians(FRAC_PI_2).as_turn_fraction());
    report::approx("Rot2 * Rot2 (compose)", "45° * 45°", Rot2::degrees(90.0), Rot2::degrees(45.0) * Rot2::degrees(45.0), (Rot2::degrees(45.0) * Rot2::degrees(45.0)).angle_to(Rot2::degrees(90.0)).abs() < EPS);
    report::approx("Rot2::inverse", "90°.inverse() * Y", Vec2::X, Rot2::degrees(90.0).inverse() * Vec2::Y, near2(Rot2::degrees(90.0).inverse() * Vec2::Y, Vec2::X));
    report::f("Rot2::angle_to", "0° -> 90°", FRAC_PI_2, Rot2::IDENTITY.angle_to(Rot2::degrees(90.0)));
    report::approx("Rot2::slerp(0°, 90°, 0.5)", "", Rot2::degrees(45.0), Rot2::IDENTITY.slerp(Rot2::degrees(90.0), 0.5), Rot2::IDENTITY.slerp(Rot2::degrees(90.0), 0.5).angle_to(Rot2::degrees(45.0)).abs() < EPS);
    report::approx("Rot2::nlerp(0°, 90°, 0.5)", "", Rot2::degrees(45.0), Rot2::IDENTITY.nlerp(Rot2::degrees(90.0), 0.5), Rot2::IDENTITY.nlerp(Rot2::degrees(90.0), 0.5).angle_to(Rot2::degrees(45.0)).abs() < EPS);
    report::is_true("Rot2::is_normalized", "from_sin_cos(sin,cos of 30°)", Rot2::from_sin_cos((PI / 6.0).sin(), (PI / 6.0).cos()).is_normalized());
}
