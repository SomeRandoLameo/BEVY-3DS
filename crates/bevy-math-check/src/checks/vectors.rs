//! glam vectors: `Vec2`, `Vec3`, `Vec3A`, `Vec4`.

use super::{near2, near3, near3a, near4, report, EPS};
use bevy_math::{Vec2, Vec3, Vec3A, Vec4};
use core::f32::consts::FRAC_PI_2;

#[test]
fn vec2_arithmetic_and_products() {
    report::eq("Vec2 + Vec2", "(1,2)+(3,4)", Vec2::new(4.0, 6.0), Vec2::new(1.0, 2.0) + Vec2::new(3.0, 4.0));
    report::eq("Vec2 - Vec2", "(3,4)-(1,2)", Vec2::new(2.0, 2.0), Vec2::new(3.0, 4.0) - Vec2::new(1.0, 2.0));
    report::eq("Vec2 * f32", "(3,4)*2", Vec2::new(6.0, 8.0), Vec2::new(3.0, 4.0) * 2.0);
    report::eq("Vec2 / f32", "(6,8)/2", Vec2::new(3.0, 4.0), Vec2::new(6.0, 8.0) / 2.0);
    report::f("Vec2::dot", "(1,2)·(3,4)", 11.0, Vec2::new(1.0, 2.0).dot(Vec2::new(3.0, 4.0)));
    report::f("Vec2::perp_dot", "(1,0)⟂·(0,1)", 1.0, Vec2::X.perp_dot(Vec2::Y));
    report::eq("Vec2::perp", "(1,0)", Vec2::new(0.0, 1.0), Vec2::X.perp());
}

#[test]
fn vec2_geometry() {
    report::f("Vec2::length", "(3,4)", 5.0, Vec2::new(3.0, 4.0).length());
    report::f("Vec2::length_squared", "(3,4)", 25.0, Vec2::new(3.0, 4.0).length_squared());
    report::f("Vec2::length_recip", "(3,4)", 0.2, Vec2::new(3.0, 4.0).length_recip());
    report::f("Vec2::distance", "(0,0)->(3,4)", 5.0, Vec2::ZERO.distance(Vec2::new(3.0, 4.0)));
    report::f("Vec2::distance_squared", "(0,0)->(3,4)", 25.0, Vec2::ZERO.distance_squared(Vec2::new(3.0, 4.0)));
    report::f("Vec2::normalize().length()", "(3,4)", 1.0, Vec2::new(3.0, 4.0).normalize().length());
    report::is_true("Vec2::is_normalized", "(0.6,0.8)", Vec2::new(0.6, 0.8).is_normalized());
    report::eq("Vec2::normalize_or_zero", "(0,0)", Vec2::ZERO, Vec2::ZERO.normalize_or_zero());
    report::approx("Vec2::project_onto", "(2,2) onto X", Vec2::new(2.0, 0.0), Vec2::new(2.0, 2.0).project_onto(Vec2::X), near2(Vec2::new(2.0, 2.0).project_onto(Vec2::X), Vec2::new(2.0, 0.0)));
    report::approx("Vec2::reject_from", "(2,2) from X", Vec2::new(0.0, 2.0), Vec2::new(2.0, 2.0).reject_from(Vec2::X), near2(Vec2::new(2.0, 2.0).reject_from(Vec2::X), Vec2::new(0.0, 2.0)));
    report::approx("Vec2::lerp", "0..(4,4) @ 0.25", Vec2::splat(1.0), Vec2::ZERO.lerp(Vec2::splat(4.0), 0.25), near2(Vec2::ZERO.lerp(Vec2::splat(4.0), 0.25), Vec2::splat(1.0)));
    report::approx("Vec2::midpoint", "(0,0),(4,2)", Vec2::new(2.0, 1.0), Vec2::ZERO.midpoint(Vec2::new(4.0, 2.0)), near2(Vec2::ZERO.midpoint(Vec2::new(4.0, 2.0)), Vec2::new(2.0, 1.0)));
    report::approx("Vec2::move_towards", "(0,0)->(10,0) by 3", Vec2::new(3.0, 0.0), Vec2::ZERO.move_towards(Vec2::new(10.0, 0.0), 3.0), near2(Vec2::ZERO.move_towards(Vec2::new(10.0, 0.0), 3.0), Vec2::new(3.0, 0.0)));
    report::f("Vec2::angle_to", "(1,0)->(0,1)", FRAC_PI_2, Vec2::X.angle_to(Vec2::Y));
    report::f("Vec2::to_angle", "(0,1)", FRAC_PI_2, Vec2::Y.to_angle());
    report::approx("Vec2::from_angle", "PI/2", Vec2::Y, Vec2::from_angle(FRAC_PI_2), near2(Vec2::from_angle(FRAC_PI_2), Vec2::Y));
    report::approx("Vec2::rotate", "from_angle(PI/2).rotate(X)", Vec2::Y, Vec2::from_angle(FRAC_PI_2).rotate(Vec2::X), near2(Vec2::from_angle(FRAC_PI_2).rotate(Vec2::X), Vec2::Y));
}

#[test]
fn vec2_componentwise() {
    report::eq("Vec2::abs", "(-1,2)", Vec2::new(1.0, 2.0), Vec2::new(-1.0, 2.0).abs());
    report::eq("Vec2::signum", "(-3,0.5)", Vec2::new(-1.0, 1.0), Vec2::new(-3.0, 0.5).signum());
    report::eq("Vec2::floor", "(1.7,-1.2)", Vec2::new(1.0, -2.0), Vec2::new(1.7, -1.2).floor());
    report::eq("Vec2::ceil", "(1.2,-1.7)", Vec2::new(2.0, -1.0), Vec2::new(1.2, -1.7).ceil());
    report::eq("Vec2::round", "(1.5,2.4)", Vec2::new(2.0, 2.0), Vec2::new(1.5, 2.4).round());
    report::eq("Vec2::fract", "(1.75,-0.25)", Vec2::new(0.75, -0.25), Vec2::new(1.75, -0.25).fract());
    report::eq("Vec2::trunc", "(1.9,-1.9)", Vec2::new(1.0, -1.0), Vec2::new(1.9, -1.9).trunc());
    report::eq("Vec2::recip", "(2,4)", Vec2::new(0.5, 0.25), Vec2::new(2.0, 4.0).recip());
    report::eq("Vec2::min", "(1,4),(3,2)", Vec2::new(1.0, 2.0), Vec2::new(1.0, 4.0).min(Vec2::new(3.0, 2.0)));
    report::eq("Vec2::max", "(1,4),(3,2)", Vec2::new(3.0, 4.0), Vec2::new(1.0, 4.0).max(Vec2::new(3.0, 2.0)));
    report::eq("Vec2::clamp", "(5,-5) in [-1,1]", Vec2::new(1.0, -1.0), Vec2::new(5.0, -5.0).clamp(Vec2::splat(-1.0), Vec2::splat(1.0)));
    report::f("Vec2::min_element", "(3,-2)", -2.0, Vec2::new(3.0, -2.0).min_element());
    report::f("Vec2::max_element", "(3,-2)", 3.0, Vec2::new(3.0, -2.0).max_element());
    report::f("Vec2::element_sum", "(3,4)", 7.0, Vec2::new(3.0, 4.0).element_sum());
    report::f("Vec2::element_product", "(3,4)", 12.0, Vec2::new(3.0, 4.0).element_product());
    report::eq("Vec2::extend", "(1,2) z=3", Vec3::new(1.0, 2.0, 3.0), Vec2::new(1.0, 2.0).extend(3.0));
    report::eq("Vec2::to_array", "(1,2)", [1.0_f32, 2.0], Vec2::new(1.0, 2.0).to_array());
    report::eq("Vec2::from_array", "[1,2]", Vec2::new(1.0, 2.0), Vec2::from_array([1.0, 2.0]));
    report::eq("Vec2::with_x", "(1,2).with_x(9)", Vec2::new(9.0, 2.0), Vec2::new(1.0, 2.0).with_x(9.0));
}

#[test]
fn vec3_core() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);
    report::f("Vec3::dot", "(1,2,3)·(4,5,6)", 32.0, a.dot(b));
    report::eq("Vec3::cross", "(1,2,3)x(4,5,6)", Vec3::new(-3.0, 6.0, -3.0), a.cross(b));
    report::f("Vec3::length", "(2,3,6)", 7.0, Vec3::new(2.0, 3.0, 6.0).length());
    report::f("Vec3::length_squared", "(1,2,2)", 9.0, Vec3::new(1.0, 2.0, 2.0).length_squared());
    report::f("Vec3::length_recip", "(0,0,4)", 0.25, Vec3::new(0.0, 0.0, 4.0).length_recip());
    report::f("Vec3::normalize().length()", "(1,2,3)", 1.0, a.normalize().length());
    report::f("Vec3::distance", "(1,2,3)->(4,5,6)", 27.0_f32.sqrt(), a.distance(b));
    report::f("Vec3::distance_squared", "(1,2,3)->(4,5,6)", 27.0, a.distance_squared(b));
    report::f("Vec3::angle_between", "X,Y", FRAC_PI_2, Vec3::X.angle_between(Vec3::Y));
    report::approx("Vec3::project_onto", "(1,1,1) onto X", Vec3::X, Vec3::ONE.project_onto(Vec3::X), near3(Vec3::ONE.project_onto(Vec3::X), Vec3::X));
    report::approx("Vec3::reject_from", "(1,1,1) from X", Vec3::new(0.0, 1.0, 1.0), Vec3::ONE.reject_from(Vec3::X), near3(Vec3::ONE.reject_from(Vec3::X), Vec3::new(0.0, 1.0, 1.0)));
    report::approx("Vec3::lerp", "0..(4,4,4) @ 0.25", Vec3::splat(1.0), Vec3::ZERO.lerp(Vec3::splat(4.0), 0.25), near3(Vec3::ZERO.lerp(Vec3::splat(4.0), 0.25), Vec3::splat(1.0)));
    report::approx("Vec3::reflect", "(1,-1,0) about Y", Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, -1.0, 0.0).reflect(Vec3::Y), near3(Vec3::new(1.0, -1.0, 0.0).reflect(Vec3::Y), Vec3::new(1.0, 1.0, 0.0)));
    report::approx("Vec3::slerp", "X..Y @ 0.5", Vec3::new(0.70710677, 0.70710677, 0.0), Vec3::X.slerp(Vec3::Y, 0.5), near3(Vec3::X.slerp(Vec3::Y, 0.5), Vec3::new(0.70710677, 0.70710677, 0.0)));
}

#[test]
fn vec3_componentwise_and_convert() {
    report::eq("Vec3::abs", "(-1,2,-3)", Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, 2.0, -3.0).abs());
    report::eq("Vec3::signum", "(-2,0,3)", Vec3::new(-1.0, 1.0, 1.0), Vec3::new(-2.0, 0.0, 3.0).signum());
    report::eq("Vec3::min", "(1,5,3),(4,2,6)", Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 5.0, 3.0).min(Vec3::new(4.0, 2.0, 6.0)));
    report::eq("Vec3::max", "(1,5,3),(4,2,6)", Vec3::new(4.0, 5.0, 6.0), Vec3::new(1.0, 5.0, 3.0).max(Vec3::new(4.0, 2.0, 6.0)));
    report::eq("Vec3::clamp", "(9,-9,0.5) in [-1,1]", Vec3::new(1.0, -1.0, 0.5), Vec3::new(9.0, -9.0, 0.5).clamp(Vec3::splat(-1.0), Vec3::splat(1.0)));
    report::f("Vec3::min_element", "(3,-2,5)", -2.0, Vec3::new(3.0, -2.0, 5.0).min_element());
    report::f("Vec3::max_element", "(3,-2,5)", 5.0, Vec3::new(3.0, -2.0, 5.0).max_element());
    report::f("Vec3::element_sum", "(1,2,3)", 6.0, Vec3::new(1.0, 2.0, 3.0).element_sum());
    report::f("Vec3::element_product", "(1,2,3)", 6.0, Vec3::new(1.0, 2.0, 3.0).element_product());
    report::eq("Vec3::extend", "(1,2,3) w=4", Vec4::new(1.0, 2.0, 3.0, 4.0), Vec3::new(1.0, 2.0, 3.0).extend(4.0));
    report::eq("Vec3::truncate", "(1,2,3)", Vec2::new(1.0, 2.0), Vec3::new(1.0, 2.0, 3.0).truncate());
    report::eq("Vec3::to_array", "(1,2,3)", [1.0_f32, 2.0, 3.0], Vec3::new(1.0, 2.0, 3.0).to_array());
    let (u, v) = Vec3::Z.any_orthonormal_pair();
    report::is_true("Vec3::any_orthonormal_pair — orthonormal", "Z", u.is_normalized() && v.is_normalized() && u.dot(v).abs() < EPS && u.cross(v).abs_diff_eq(Vec3::Z, 1e-3));
    report::is_true("Vec3::is_finite", "(1,2,3)", Vec3::new(1.0, 2.0, 3.0).is_finite());
    report::is_true("Vec3::is_nan", "(NaN,0,0)", Vec3::new(f32::NAN, 0.0, 0.0).is_nan());
}

#[test]
fn vec3a_specifics() {
    report::eq("Vec3A: size_of", "", 16usize, core::mem::size_of::<Vec3A>());
    report::eq("Vec3A: align_of", "", 16usize, core::mem::align_of::<Vec3A>());
    let a = Vec3::new(1.5, -2.0, 3.25);
    let b = Vec3::new(-0.5, 4.0, 1.0);
    report::approx(
        "Vec3A::cross matches Vec3::cross",
        "(1.5,-2,3.25) x (-0.5,4,1)",
        a.cross(b),
        Vec3::from(Vec3A::from(a).cross(Vec3A::from(b))),
        near3(Vec3::from(Vec3A::from(a).cross(Vec3A::from(b))), a.cross(b)),
    );
    report::f("Vec3A::dot matches Vec3::dot", "same", a.dot(b), Vec3A::from(a).dot(Vec3A::from(b)));
    report::approx("Vec3::to_vec3a round-trip", "(1.5,-2,3.25)", a, Vec3::from(a.to_vec3a()), near3(Vec3::from(a.to_vec3a()), a));
    report::approx("Vec3A::lerp", "0..(4,4,4)@0.5", Vec3A::splat(2.0), Vec3A::ZERO.lerp(Vec3A::splat(4.0), 0.5), near3a(Vec3A::ZERO.lerp(Vec3A::splat(4.0), 0.5), Vec3A::splat(2.0)));
}

#[test]
fn vec4_specifics() {
    let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
    report::f("Vec4::dot", "(1,2,3,4)·self", 30.0, v.dot(v));
    report::f("Vec4::length", "(0,0,3,4)", 5.0, Vec4::new(0.0, 0.0, 3.0, 4.0).length());
    report::f("Vec4::length_squared", "(1,2,3,4)", 30.0, v.length_squared());
    report::eq("Vec4::truncate", "(1,2,3,4)", Vec3::new(1.0, 2.0, 3.0), v.truncate());
    report::eq("Vec4::min_element", "(1,2,3,4)", 1.0_f32, v.min_element());
    report::eq("Vec4::max_element", "(1,2,3,4)", 4.0_f32, v.max_element());
    report::f("Vec4::element_sum", "(1,2,3,4)", 10.0, v.element_sum());
    report::eq("Vec4::abs", "(-1,2,-3,4)", Vec4::new(1.0, 2.0, 3.0, 4.0), Vec4::new(-1.0, 2.0, -3.0, 4.0).abs());
    report::approx("Vec4::lerp", "0..(4,4,4,4)@0.25", Vec4::splat(1.0), Vec4::ZERO.lerp(Vec4::splat(4.0), 0.25), near4(Vec4::ZERO.lerp(Vec4::splat(4.0), 0.25), Vec4::splat(1.0)));
    report::eq("Vec4::to_array", "(1,2,3,4)", [1.0_f32, 2.0, 3.0, 4.0], v.to_array());
}
