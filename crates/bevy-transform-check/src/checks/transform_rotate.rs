//! `Transform` rotation mutators: `rotate*`, `rotate_local*`, `rotate_around`,
//! `translate_around`.

use super::{near3, nearq, report};
use bevy_math::{Dir3, Quat, Vec3};
use bevy_transform::prelude::*;
use core::f32::consts::FRAC_PI_2;

#[test]
fn rotate_world_space() {
    let mut t = Transform::IDENTITY;
    t.rotate(Quat::from_rotation_z(FRAC_PI_2));
    report::approx("Transform::rotate(rot_z(PI/2))", "IDENTITY", Quat::from_rotation_z(FRAC_PI_2), t.rotation, nearq(t.rotation, Quat::from_rotation_z(FRAC_PI_2)));

    let mut tx = Transform::IDENTITY;
    tx.rotate_x(FRAC_PI_2);
    report::approx("Transform::rotate_x(PI/2) * local +Y = +Z", "", Vec3::Z, tx.transform_point(Vec3::Y), near3(tx.transform_point(Vec3::Y), Vec3::Z));
    let mut ty = Transform::IDENTITY;
    ty.rotate_y(FRAC_PI_2);
    report::approx("Transform::rotate_y(PI/2) * local +Z = +X", "", Vec3::X, ty.transform_point(Vec3::Z), near3(ty.transform_point(Vec3::Z), Vec3::X));
    let mut tz = Transform::IDENTITY;
    tz.rotate_z(FRAC_PI_2);
    report::approx("Transform::rotate_z(PI/2) * local +X = +Y", "", Vec3::Y, tz.transform_point(Vec3::X), near3(tz.transform_point(Vec3::X), Vec3::Y));

    let mut ta = Transform::IDENTITY;
    ta.rotate_axis(Dir3::Z, FRAC_PI_2);
    report::approx("Transform::rotate_axis(Z, PI/2)", "", Quat::from_rotation_z(FRAC_PI_2), ta.rotation, nearq(ta.rotation, Quat::from_rotation_z(FRAC_PI_2)));
}

#[test]
fn rotate_local_space() {
    // start rotated 90° about Z; rotate_local_x then acts about the *current* X.
    let mut t = Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2));
    let before = t.rotation;
    t.rotate_local_x(FRAC_PI_2);
    report::is_true("Transform::rotate_local_x changes rotation", "started at rot_z(PI/2)", !nearq(t.rotation, before));

    let mut tl = Transform::IDENTITY;
    tl.rotate_local(Quat::from_rotation_y(FRAC_PI_2));
    report::approx("Transform::rotate_local(rot_y(PI/2)) from IDENTITY", "", Quat::from_rotation_y(FRAC_PI_2), tl.rotation, nearq(tl.rotation, Quat::from_rotation_y(FRAC_PI_2)));

    let mut tla = Transform::IDENTITY;
    tla.rotate_local_axis(Dir3::Y, FRAC_PI_2);
    report::approx("Transform::rotate_local_axis(Y, PI/2) from IDENTITY", "", Quat::from_rotation_y(FRAC_PI_2), tla.rotation, nearq(tla.rotation, Quat::from_rotation_y(FRAC_PI_2)));

    let mut tlz = Transform::IDENTITY;
    tlz.rotate_local_z(FRAC_PI_2);
    report::approx("Transform::rotate_local_z(PI/2) from IDENTITY", "", Quat::from_rotation_z(FRAC_PI_2), tlz.rotation, nearq(tlz.rotation, Quat::from_rotation_z(FRAC_PI_2)));
}

#[test]
fn orbit_around_a_point() {
    // A point at (1,0,0), rotate 90° about Z around the origin -> (0,1,0).
    let mut t = Transform::from_xyz(1.0, 0.0, 0.0);
    t.translate_around(Vec3::ZERO, Quat::from_rotation_z(FRAC_PI_2));
    report::approx("Transform::translate_around(origin, rot_z(PI/2))", "start (1,0,0)", Vec3::new(0.0, 1.0, 0.0), t.translation, near3(t.translation, Vec3::new(0.0, 1.0, 0.0)));

    let mut t2 = Transform::from_xyz(1.0, 0.0, 0.0);
    t2.rotate_around(Vec3::ZERO, Quat::from_rotation_z(FRAC_PI_2));
    report::approx("Transform::rotate_around — translation orbits", "start (1,0,0)", Vec3::new(0.0, 1.0, 0.0), t2.translation, near3(t2.translation, Vec3::new(0.0, 1.0, 0.0)));
    report::approx("Transform::rotate_around — also spins the rotation", "", Quat::from_rotation_z(FRAC_PI_2), t2.rotation, nearq(t2.rotation, Quat::from_rotation_z(FRAC_PI_2)));
}
