//! `Transform` conversions: `to_matrix`, `compute_affine`, `to_isometry`,
//! round-trips.

use super::{near3, nearq, report, EPS};
use bevy_math::{Quat, Vec3};
use bevy_transform::prelude::*;
use core::f32::consts::FRAC_PI_2;

#[test]
fn to_matrix_and_affine() {
    let t = Transform {
        translation: Vec3::new(3.0, -1.0, 2.0),
        rotation: Quat::from_rotation_y(0.6),
        scale: Vec3::splat(1.5),
    };
    report::approx(
        "Transform::to_matrix().transform_point3 == Transform::transform_point",
        "T=(3,-1,2), Ry=0.6, S=1.5, point X",
        t.transform_point(Vec3::X),
        t.to_matrix().transform_point3(Vec3::X),
        near3(t.to_matrix().transform_point3(Vec3::X), t.transform_point(Vec3::X)),
    );
    report::approx(
        "Transform::compute_affine().transform_point3 == transform_point",
        "same",
        t.transform_point(Vec3::Y),
        t.compute_affine().transform_point3(Vec3::Y),
        near3(t.compute_affine().transform_point3(Vec3::Y), t.transform_point(Vec3::Y)),
    );
}

#[test]
fn to_isometry_round_trip() {
    // Isometry has no scale, so use a scale-1 transform.
    let t = Transform::from_xyz(1.0, 2.0, 3.0).with_rotation(Quat::from_rotation_z(FRAC_PI_2));
    let iso = t.to_isometry();
    let back = Transform::from_isometry(iso);
    report::approx("Transform::to_isometry -> from_isometry — translation", "T(1,2,3), Rz(PI/2)", t.translation, back.translation, near3(back.translation, t.translation));
    report::approx("Transform::to_isometry -> from_isometry — rotation", "same", t.rotation, back.rotation, nearq(back.rotation, t.rotation));
    let p: Vec3 = iso.transform_point(Vec3::X).into();
    report::approx("to_isometry().transform_point == transform_point", "point X", t.transform_point(Vec3::X), p, near3(p, t.transform_point(Vec3::X)));
}

#[test]
fn matrix_round_trip() {
    let t = Transform::from_xyz(5.0, 6.0, 7.0)
        .with_rotation(Quat::from_rotation_x(0.4))
        .with_scale(Vec3::new(2.0, 2.0, 2.0));
    let back = Transform::from_matrix(t.to_matrix());
    report::approx("Transform -> to_matrix -> from_matrix — translation", "", t.translation, back.translation, near3(back.translation, t.translation));
    report::approx("… — scale", "", t.scale, back.scale, near3(back.scale, t.scale));
    report::is_true("… — rotation", "", back.rotation.abs_diff_eq(t.rotation, EPS));
}
