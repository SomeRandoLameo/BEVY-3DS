//! `GlobalTransform` — every public method + operator.

use super::{near3, nearq, report};
use bevy_math::{Isometry3d, Mat4, Quat, Vec3, Vec3A};
use bevy_transform::prelude::*;
use core::f32::consts::FRAC_PI_2;

#[test]
fn constructors_and_accessors() {
    report::eq("GlobalTransform::from_xyz", "(1,2,3)", Vec3::new(1.0, 2.0, 3.0), GlobalTransform::from_xyz(1.0, 2.0, 3.0).translation());
    report::eq("GlobalTransform::from_translation", "(4,5,6)", Vec3::new(4.0, 5.0, 6.0), GlobalTransform::from_translation(Vec3::new(4.0, 5.0, 6.0)).translation());
    report::approx("GlobalTransform::from_rotation", "rot_z(PI/2) * X", Vec3::Y, GlobalTransform::from_rotation(Quat::from_rotation_z(FRAC_PI_2)).transform_point(Vec3::X), near3(GlobalTransform::from_rotation(Quat::from_rotation_z(FRAC_PI_2)).transform_point(Vec3::X), Vec3::Y));
    report::eq("GlobalTransform::from_scale", "(2,3,4)", Vec3::new(2.0, 3.0, 4.0), GlobalTransform::from_scale(Vec3::new(2.0, 3.0, 4.0)).scale());
    report::eq("GlobalTransform::default() == from(IDENTITY)", "", GlobalTransform::from(Transform::IDENTITY), GlobalTransform::default());

    let g = GlobalTransform::from(Transform::from_xyz(5.0, 6.0, 7.0).with_scale(Vec3::splat(3.0)).with_rotation(Quat::from_rotation_z(FRAC_PI_2)));
    report::approx("GlobalTransform::translation", "T(5,6,7) S=3 Rz", Vec3::new(5.0, 6.0, 7.0), g.translation(), near3(g.translation(), Vec3::new(5.0, 6.0, 7.0)));
    report::eq("GlobalTransform::translation_vec3a", "same", Vec3A::new(5.0, 6.0, 7.0), g.translation_vec3a());
    report::approx("GlobalTransform::scale", "same", Vec3::splat(3.0), g.scale(), near3(g.scale(), Vec3::splat(3.0)));
    report::approx("GlobalTransform::rotation", "same", Quat::from_rotation_z(FRAC_PI_2), g.rotation(), nearq(g.rotation(), Quat::from_rotation_z(FRAC_PI_2)));
}

#[test]
fn decompose_and_convert() {
    let src = Transform::from_xyz(-2.0, 4.0, 1.0).with_scale(Vec3::splat(2.0)).with_rotation(Quat::from_rotation_y(0.5));
    let g = GlobalTransform::from(src);

    let (s, r, t) = g.to_scale_rotation_translation();
    report::approx("GlobalTransform::to_scale_rotation_translation — translation", "", src.translation, t, near3(t, src.translation));
    report::approx("… — scale", "", src.scale, s, near3(s, src.scale));
    report::is_true("… — rotation", "", r.abs_diff_eq(src.rotation, super::EPS));

    report::approx("GlobalTransform::compute_transform() round-trip — translation", "", src.translation, g.compute_transform().translation, near3(g.compute_transform().translation, src.translation));
    report::approx("GlobalTransform::to_matrix().transform_point3 == transform_point", "point X", g.transform_point(Vec3::X), g.to_matrix().transform_point3(Vec3::X), near3(g.to_matrix().transform_point3(Vec3::X), g.transform_point(Vec3::X)));
    report::approx("GlobalTransform::affine().transform_point3 == transform_point", "point Y", g.transform_point(Vec3::Y), g.affine().transform_point3(Vec3::Y), near3(g.affine().transform_point3(Vec3::Y), g.transform_point(Vec3::Y)));

    // to_isometry needs a scale-1 transform
    let g1 = GlobalTransform::from(Transform::from_xyz(1.0, 2.0, 3.0).with_rotation(Quat::from_rotation_z(FRAC_PI_2)));
    let iso: Isometry3d = g1.to_isometry();
    let bp: Vec3 = iso.transform_point(Vec3::X).into();
    report::approx("GlobalTransform::to_isometry().transform_point == transform_point", "point X", g1.transform_point(Vec3::X), bp, near3(bp, g1.transform_point(Vec3::X)));
}

#[test]
fn from_mat4_and_operators() {
    let m = Mat4::from_translation(Vec3::new(9.0, 0.0, 0.0));
    report::eq("GlobalTransform::from(Mat4)", "T(9,0,0)", Vec3::new(9.0, 0.0, 0.0), GlobalTransform::from(m).translation());

    let a = GlobalTransform::from(Transform::from_xyz(1.0, 0.0, 0.0));
    let b = GlobalTransform::from(Transform::from_xyz(0.0, 2.0, 0.0));
    report::approx("GlobalTransform * GlobalTransform — translation", "G(1,0,0) * G(0,2,0)", Vec3::new(1.0, 2.0, 0.0), (a * b).translation(), near3((a * b).translation(), Vec3::new(1.0, 2.0, 0.0)));
    report::approx("GlobalTransform * Transform — translation", "G(1,0,0) * T(0,2,0)", Vec3::new(1.0, 2.0, 0.0), (a * Transform::from_xyz(0.0, 2.0, 0.0)).translation(), near3((a * Transform::from_xyz(0.0, 2.0, 0.0)).translation(), Vec3::new(1.0, 2.0, 0.0)));
    report::eq("GlobalTransform * Vec3 == transform_point", "point Z", a.transform_point(Vec3::Z), a * Vec3::Z);
    report::approx("GlobalTransform::mul_transform", "same as *", (a * Transform::from_xyz(0.0, 2.0, 0.0)).translation(), a.mul_transform(Transform::from_xyz(0.0, 2.0, 0.0)).translation(), near3(a.mul_transform(Transform::from_xyz(0.0, 2.0, 0.0)).translation(), (a * Transform::from_xyz(0.0, 2.0, 0.0)).translation()));
}

#[test]
fn reparented_to_and_radius() {
    // child global at (10, 0, 0), parent global at (4, 0, 0) -> local (6, 0, 0)
    let child = GlobalTransform::from(Transform::from_xyz(10.0, 0.0, 0.0));
    let parent = GlobalTransform::from(Transform::from_xyz(4.0, 0.0, 0.0));
    let local: Transform = child.reparented_to(&parent);
    report::approx("GlobalTransform::reparented_to — local translation", "child (10,0,0) under parent (4,0,0)", Vec3::new(6.0, 0.0, 0.0), local.translation, near3(local.translation, Vec3::new(6.0, 0.0, 0.0)));

    // radius_vec3a: for a pure translation the "radius" of a unit extent is 1
    let g = GlobalTransform::from(Transform::IDENTITY);
    report::f("GlobalTransform::radius_vec3a(unit extents)", "IDENTITY", 3.0_f32.sqrt(), g.radius_vec3a(Vec3A::ONE));
}
