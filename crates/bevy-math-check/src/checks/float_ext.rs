//! glam `FloatExt` + bevy_math common traits (`VectorSpace`, `ScalarField`,
//! `NormedVectorSpace`).

use super::{near3, report};
use bevy_math::{FloatExt, Vec3};

#[test]
fn glam_float_ext() {
    report::f("f32::lerp (FloatExt)", "0..10 @ 0.25", 2.5, 0.0_f32.lerp(10.0, 0.25));
    report::f("f32::inverse_lerp", "0..10, v=2.5", 0.25, f32::inverse_lerp(0.0, 10.0, 2.5));
    report::f("f32::remap", "[0,1]->[10,20] @ 0.5", 15.0, 0.5_f32.remap(0.0, 1.0, 10.0, 20.0));
}

#[test]
fn bevy_math_common_traits() {
    use bevy_math::{NormedVectorSpace, ScalarField, VectorSpace};

    report::approx(
        "VectorSpace::lerp on Vec3",
        "0..(4,4,4) @ 0.5",
        Vec3::splat(2.0),
        VectorSpace::lerp(Vec3::ZERO, Vec3::splat(4.0), 0.5),
        near3(VectorSpace::lerp(Vec3::ZERO, Vec3::splat(4.0), 0.5), Vec3::splat(2.0)),
    );
    report::f("ScalarField::recip on f32", "4", 0.25, ScalarField::recip(4.0_f32));
    report::f("NormedVectorSpace::norm on Vec3", "(2,3,6)", 7.0, NormedVectorSpace::norm(Vec3::new(2.0, 3.0, 6.0)));
    report::f(
        "NormedVectorSpace::distance on Vec3",
        "(0,0,0)->(0,3,4)",
        5.0,
        NormedVectorSpace::distance(Vec3::ZERO, Vec3::new(0.0, 3.0, 4.0)),
    );
}
