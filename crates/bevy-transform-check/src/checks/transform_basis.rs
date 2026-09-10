//! `Transform` local axes / direction accessors (all return `Dir3`).

use super::{near3, report};
use bevy_math::{Quat, Vec3};
use bevy_transform::prelude::*;
use core::f32::consts::FRAC_PI_2;

#[test]
fn identity_basis() {
    let t = Transform::IDENTITY;
    report::approx("Transform::local_x (identity)", "", Vec3::X, t.local_x().as_vec3(), near3(t.local_x().as_vec3(), Vec3::X));
    report::approx("Transform::local_y (identity)", "", Vec3::Y, t.local_y().as_vec3(), near3(t.local_y().as_vec3(), Vec3::Y));
    report::approx("Transform::local_z (identity)", "", Vec3::Z, t.local_z().as_vec3(), near3(t.local_z().as_vec3(), Vec3::Z));
    report::approx("Transform::forward (identity) = -Z", "", Vec3::NEG_Z, t.forward().as_vec3(), near3(t.forward().as_vec3(), Vec3::NEG_Z));
    report::approx("Transform::back (identity) = +Z", "", Vec3::Z, t.back().as_vec3(), near3(t.back().as_vec3(), Vec3::Z));
    report::approx("Transform::right (identity) = +X", "", Vec3::X, t.right().as_vec3(), near3(t.right().as_vec3(), Vec3::X));
    report::approx("Transform::left (identity) = -X", "", Vec3::NEG_X, t.left().as_vec3(), near3(t.left().as_vec3(), Vec3::NEG_X));
    report::approx("Transform::up (identity) = +Y", "", Vec3::Y, t.up().as_vec3(), near3(t.up().as_vec3(), Vec3::Y));
    report::approx("Transform::down (identity) = -Y", "", Vec3::NEG_Y, t.down().as_vec3(), near3(t.down().as_vec3(), Vec3::NEG_Y));
}

#[test]
fn rotated_basis() {
    // 90° about Y: local_x -> -Z, local_z -> +X
    let t = Transform::from_rotation(Quat::from_rotation_y(FRAC_PI_2));
    report::approx("Transform::local_x after rot_y(PI/2)", "", Vec3::NEG_Z, t.local_x().as_vec3(), near3(t.local_x().as_vec3(), Vec3::NEG_Z));
    report::approx("Transform::local_z after rot_y(PI/2)", "", Vec3::X, t.local_z().as_vec3(), near3(t.local_z().as_vec3(), Vec3::X));
    report::approx("Transform::forward after rot_y(PI/2)", "", Vec3::NEG_X, t.forward().as_vec3(), near3(t.forward().as_vec3(), Vec3::NEG_X));
}
