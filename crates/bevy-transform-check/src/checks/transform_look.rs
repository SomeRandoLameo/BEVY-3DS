//! `Transform` look-at / alignment: `look_at`, `looking_at`, `look_to`,
//! `looking_to`, `align`, `aligned_by`.

use super::{near3, report};
use bevy_math::{Dir3, Vec3};
use bevy_transform::prelude::*;

#[test]
fn look_at_and_to() {
    let t = Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);
    report::approx("Transform::looking_at(origin) — forward points at target", "eye (0,0,5)", Vec3::NEG_Z, t.forward().as_vec3(), near3(t.forward().as_vec3(), Vec3::NEG_Z));
    report::approx("Transform::looking_at — up preserved", "", Vec3::Y, t.up().as_vec3(), near3(t.up().as_vec3(), Vec3::Y));

    let mut m = Transform::IDENTITY;
    m.look_at(Vec3::new(0.0, 0.0, -1.0), Vec3::Y);
    report::approx("Transform::look_at (mut) — forward", "target -Z", Vec3::NEG_Z, m.forward().as_vec3(), near3(m.forward().as_vec3(), Vec3::NEG_Z));

    let lt = Transform::IDENTITY.looking_to(Dir3::NEG_X, Vec3::Y);
    report::approx("Transform::looking_to(-X) — forward", "", Vec3::NEG_X, lt.forward().as_vec3(), near3(lt.forward().as_vec3(), Vec3::NEG_X));

    let mut lm = Transform::IDENTITY;
    lm.look_to(Dir3::Y, Vec3::Z);
    report::approx("Transform::look_to(+Y) — forward", "", Vec3::Y, lm.forward().as_vec3(), near3(lm.forward().as_vec3(), Vec3::Y));
}

#[test]
fn align() {
    // From the bevy docs: align(X, Z, X, Y) => rotation that maps X->Z.
    let mut t = Transform::IDENTITY;
    t.align(Dir3::X, Dir3::Z, Dir3::X, Dir3::Y);
    report::approx("Transform::align(X, Z, X, Y) — X maps to Z", "", Vec3::Z, t.rotation * Vec3::X, near3(t.rotation * Vec3::X, Vec3::Z));

    let a = Transform::IDENTITY.aligned_by(Dir3::X, Dir3::Y, Vec3::new(1.0, 1.0, 0.0), Dir3::Z);
    report::approx("Transform::aligned_by(X, Y, (1,1,0), Z) — main axis image ≈ Y", "", Vec3::Y, (a.rotation * Vec3::X).normalize(), near3((a.rotation * Vec3::X).normalize(), Vec3::Y));
}
