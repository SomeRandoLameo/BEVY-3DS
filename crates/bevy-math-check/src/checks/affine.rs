//! `glam::Affine2` / `Affine3A`.

use super::{near2, near3, report, EPS};
use bevy_math::{Affine2, Affine3A, Quat, Vec2, Vec3};
use core::f32::consts::FRAC_PI_2;

#[test]
fn affine2() {
    let a = Affine2::from_scale_angle_translation(Vec2::splat(2.0), FRAC_PI_2, Vec2::new(1.0, 0.0));
    report::approx("Affine2::from_scale_angle_translation.transform_point2(X)", "S=2, a=PI/2, T=(1,0)", Vec2::new(1.0, 2.0), a.transform_point2(Vec2::X), near2(a.transform_point2(Vec2::X), Vec2::new(1.0, 2.0)));
    report::approx("Affine2::transform_vector2 (no translation)", "same, X", Vec2::new(0.0, 2.0), a.transform_vector2(Vec2::X), near2(a.transform_vector2(Vec2::X), Vec2::new(0.0, 2.0)));
    report::approx("Affine2::inverse round-trip", "a.inverse().transform_point2(a * X)", Vec2::X, a.inverse().transform_point2(a.transform_point2(Vec2::X)), near2(a.inverse().transform_point2(a.transform_point2(Vec2::X)), Vec2::X));
    report::approx("Affine2::from_translation", "(3,4) * (0,0)", Vec2::new(3.0, 4.0), Affine2::from_translation(Vec2::new(3.0, 4.0)).transform_point2(Vec2::ZERO), near2(Affine2::from_translation(Vec2::new(3.0, 4.0)).transform_point2(Vec2::ZERO), Vec2::new(3.0, 4.0)));
    report::is_true("Affine2::IDENTITY.is_finite", "", Affine2::IDENTITY.is_finite());
}

#[test]
fn affine3a() {
    let a = Affine3A::from_scale_rotation_translation(Vec3::splat(2.0), Quat::from_rotation_z(FRAC_PI_2), Vec3::new(1.0, 1.0, 0.0));
    report::approx("Affine3A::from_scale_rotation_translation.transform_point3(X)", "S=2, Rz=PI/2, T=(1,1,0)", Vec3::new(1.0, 3.0, 0.0), a.transform_point3(Vec3::X), near3(a.transform_point3(Vec3::X), Vec3::new(1.0, 3.0, 0.0)));
    report::approx("Affine3A::transform_vector3", "same, vec X", Vec3::new(0.0, 2.0, 0.0), a.transform_vector3(Vec3::X), near3(a.transform_vector3(Vec3::X), Vec3::new(0.0, 2.0, 0.0)));
    report::approx("Affine3A::inverse round-trip", "a.inverse() ∘ a on X", Vec3::X, a.inverse().transform_point3(a.transform_point3(Vec3::X)), near3(a.inverse().transform_point3(a.transform_point3(Vec3::X)), Vec3::X));
    let (s, r, t) = a.to_scale_rotation_translation();
    report::approx("Affine3A::to_scale_rotation_translation — scale", "a", Vec3::splat(2.0), s, near3(s, Vec3::splat(2.0)));
    report::approx("Affine3A::to_scale_rotation_translation — translation", "a", Vec3::new(1.0, 1.0, 0.0), t, near3(t, Vec3::new(1.0, 1.0, 0.0)));
    report::is_true("Affine3A::to_scale_rotation_translation — rotation", "a", r.abs_diff_eq(Quat::from_rotation_z(FRAC_PI_2), EPS));
    report::approx("Affine3A::from_mat4(Mat4::from_translation)", "(5,0,0)", Vec3::new(5.0, 0.0, 0.0), Affine3A::from_mat4(bevy_math::Mat4::from_translation(Vec3::X * 5.0)).transform_point3(Vec3::ZERO), near3(Affine3A::from_mat4(bevy_math::Mat4::from_translation(Vec3::X * 5.0)).transform_point3(Vec3::ZERO), Vec3::new(5.0, 0.0, 0.0)));
}
