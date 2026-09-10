//! `Transform` composition + point transformation: `mul_transform`,
//! `Mul<Transform>`, `Mul<GlobalTransform>`, `Mul<Vec3>`, `transform_point`.

use super::{near3, report};
use bevy_math::{Quat, Vec3};
use bevy_transform::prelude::*;
use core::f32::consts::FRAC_PI_2;

#[test]
fn mul_transform_and_operators() {
    let parent = Transform::from_xyz(1.0, 2.0, 3.0);
    let child = Transform::from_xyz(0.0, 0.0, 1.0).with_scale(Vec3::splat(2.0));

    let via_method = parent.mul_transform(child);
    let via_op = parent * child;
    report::approx("Transform::mul_transform — translation", "T(1,2,3) * (T(0,0,1), S=2)", Vec3::new(1.0, 2.0, 4.0), via_method.translation, near3(via_method.translation, Vec3::new(1.0, 2.0, 4.0)));
    report::eq("Transform::mul_transform — scale", "same", Vec3::splat(2.0), via_method.scale);
    report::eq("Transform * Transform == mul_transform", "same", via_method, via_op);

    // parent with rotation: child local +X ends up rotated
    let rp = Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2));
    let rc = Transform::from_xyz(1.0, 0.0, 0.0);
    report::approx("Transform::mul_transform composes rotation", "Rz(PI/2) * T(1,0,0)", Vec3::Y, rp.mul_transform(rc).translation, near3(rp.mul_transform(rc).translation, Vec3::Y));
}

#[test]
fn transform_point_and_vec3_mul() {
    let t = Transform::from_xyz(10.0, 0.0, 0.0).with_scale(Vec3::splat(2.0));
    report::eq("Transform::transform_point", "T(10,0,0) S=2 * point (1,0,0)", Vec3::new(12.0, 0.0, 0.0), t.transform_point(Vec3::new(1.0, 0.0, 0.0)));
    report::eq("Transform * Vec3 == transform_point", "same", t.transform_point(Vec3::X), t * Vec3::X);

    let rot = Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2));
    report::approx("Transform::transform_point with rotation", "Rz(PI/2) * (1,0,0)", Vec3::Y, rot.transform_point(Vec3::X), near3(rot.transform_point(Vec3::X), Vec3::Y));
}

#[test]
fn transform_times_global_transform() {
    // `Transform * GlobalTransform` -> GlobalTransform (LHS acts as the parent,
    // as used in TransformHelper: `ancestor_transform * accumulated_global`).
    let parent = Transform::from_xyz(5.0, 0.0, 0.0);
    let inner_global = GlobalTransform::from(Transform::from_xyz(0.0, 1.0, 0.0));
    let out: GlobalTransform = parent * inner_global;
    report::approx("Transform * GlobalTransform — translation", "T(5,0,0) * G(T(0,1,0))", Vec3::new(5.0, 1.0, 0.0), out.translation(), near3(out.translation(), Vec3::new(5.0, 1.0, 0.0)));
}
