//! `Transform` constructors, builder methods, constants, `Default`.

use super::{near3, nearq, report};
use bevy_math::{Isometry3d, Mat4, Quat, Vec3};
use bevy_transform::prelude::*;
use core::f32::consts::FRAC_PI_2;

#[test]
fn constructors() {
    report::eq("Transform::IDENTITY", "", Transform::IDENTITY, Transform::from_xyz(0.0, 0.0, 0.0));
    report::eq("Transform::default() == IDENTITY", "", Transform::IDENTITY, Transform::default());
    report::eq("Transform::from_xyz", "(1,2,3)", Vec3::new(1.0, 2.0, 3.0), Transform::from_xyz(1.0, 2.0, 3.0).translation);
    report::eq("Transform::from_translation", "(4,5,6)", Vec3::new(4.0, 5.0, 6.0), Transform::from_translation(Vec3::new(4.0, 5.0, 6.0)).translation);
    report::approx("Transform::from_rotation", "rot_z(PI/2)", Vec3::Y, Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2)).transform_point(Vec3::X), near3(Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2)).transform_point(Vec3::X), Vec3::Y));
    report::eq("Transform::from_scale", "(2,3,4)", Vec3::new(2.0, 3.0, 4.0), Transform::from_scale(Vec3::new(2.0, 3.0, 4.0)).scale);

    let m = Mat4::from_scale_rotation_translation(Vec3::splat(2.0), Quat::from_rotation_z(FRAC_PI_2), Vec3::new(1.0, 1.0, 0.0));
    let t = Transform::from_matrix(m);
    report::approx("Transform::from_matrix — translation", "SRT matrix", Vec3::new(1.0, 1.0, 0.0), t.translation, near3(t.translation, Vec3::new(1.0, 1.0, 0.0)));
    report::approx("Transform::from_matrix — scale", "SRT matrix", Vec3::splat(2.0), t.scale, near3(t.scale, Vec3::splat(2.0)));
    report::approx("Transform::from_matrix — rotation", "SRT matrix", Quat::from_rotation_z(FRAC_PI_2), t.rotation, nearq(t.rotation, Quat::from_rotation_z(FRAC_PI_2)));

    let iso = Isometry3d::new(Vec3::new(3.0, 4.0, 5.0), Quat::from_rotation_x(0.5));
    let ti = Transform::from_isometry(iso);
    report::approx("Transform::from_isometry — translation", "", Vec3::new(3.0, 4.0, 5.0), ti.translation, near3(ti.translation, Vec3::new(3.0, 4.0, 5.0)));
    report::approx("Transform::from_isometry — rotation", "", Quat::from_rotation_x(0.5), ti.rotation, nearq(ti.rotation, Quat::from_rotation_x(0.5)));
    report::eq("Transform::from_isometry — scale is ONE", "", Vec3::ONE, ti.scale);
}

#[test]
fn builder_methods() {
    let t = Transform::IDENTITY
        .with_translation(Vec3::new(1.0, 2.0, 3.0))
        .with_rotation(Quat::from_rotation_y(0.3))
        .with_scale(Vec3::splat(5.0));
    report::eq("Transform::with_translation", "IDENTITY.with_translation((1,2,3))", Vec3::new(1.0, 2.0, 3.0), t.translation);
    report::approx("Transform::with_rotation", "…with_rotation(rot_y(0.3))", Quat::from_rotation_y(0.3), t.rotation, nearq(t.rotation, Quat::from_rotation_y(0.3)));
    report::eq("Transform::with_scale", "…with_scale(5)", Vec3::splat(5.0), t.scale);
    report::is_true("Transform::is_finite", "the built transform", t.is_finite());
    report::eq("Transform::is_finite (NaN translation)", "translation.x = NaN", false, Transform::from_xyz(f32::NAN, 0.0, 0.0).is_finite());
}

#[test]
fn transform_from_global() {
    let g = GlobalTransform::from(Transform::from_xyz(7.0, 8.0, 9.0).with_scale(Vec3::splat(2.0)));
    let t: Transform = Transform::from(g);
    report::approx("Transform::from(GlobalTransform) — translation", "G(T(7,8,9), S=2)", Vec3::new(7.0, 8.0, 9.0), t.translation, near3(t.translation, Vec3::new(7.0, 8.0, 9.0)));
    report::approx("Transform::from(GlobalTransform) — scale", "same", Vec3::splat(2.0), t.scale, near3(t.scale, Vec3::splat(2.0)));
}
