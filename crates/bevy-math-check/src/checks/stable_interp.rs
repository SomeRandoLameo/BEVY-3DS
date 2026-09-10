//! `bevy_math::StableInterpolate` for the common types.

use super::{near3, nearq, report, EPS};
use bevy_math::{Dir3, Isometry3d, Quat, Rot2, StableInterpolate, Vec3};
use core::f32::consts::FRAC_PI_2;

#[test]
fn stable_interpolate_impls() {
    report::approx(
        "Vec3::interpolate_stable",
        "0..(4,4,4) @ 0.25",
        Vec3::splat(1.0),
        Vec3::ZERO.interpolate_stable(&Vec3::splat(4.0), 0.25),
        near3(Vec3::ZERO.interpolate_stable(&Vec3::splat(4.0), 0.25), Vec3::splat(1.0)),
    );
    report::approx(
        "Quat::interpolate_stable",
        "IDENTITY..rot_z(PI/2) @ 0.5",
        Quat::from_rotation_z(FRAC_PI_2 / 2.0),
        Quat::IDENTITY.interpolate_stable(&Quat::from_rotation_z(FRAC_PI_2), 0.5),
        nearq(Quat::IDENTITY.interpolate_stable(&Quat::from_rotation_z(FRAC_PI_2), 0.5), Quat::from_rotation_z(FRAC_PI_2 / 2.0)),
    );
    report::approx(
        "Dir3::interpolate_stable",
        "X..Y @ 0.5",
        Dir3::from_xyz(1.0, 1.0, 0.0).unwrap().as_vec3(),
        Dir3::X.interpolate_stable(&Dir3::Y, 0.5).as_vec3(),
        near3(Dir3::X.interpolate_stable(&Dir3::Y, 0.5).as_vec3(), Dir3::from_xyz(1.0, 1.0, 0.0).unwrap().as_vec3()),
    );
    let r = Rot2::IDENTITY.interpolate_stable(&Rot2::degrees(90.0), 0.5);
    report::approx("Rot2::interpolate_stable", "0°..90° @ 0.5", Rot2::degrees(45.0), r, r.angle_to(Rot2::degrees(45.0)).abs() < EPS);

    // Isometry3d composed from its parts (StableInterpolate is impl'd for the parts).
    let ta = Vec3::ZERO.interpolate_stable(&Vec3::new(10.0, 0.0, 0.0), 0.5);
    let ra = Quat::IDENTITY.interpolate_stable(&Quat::from_rotation_z(FRAC_PI_2), 0.5);
    let iso = Isometry3d::new(ta, ra);
    let p: Vec3 = iso.translation.into();
    report::approx("Isometry3d from interpolated parts — translation", "0..(10,0,0) @ 0.5", Vec3::new(5.0, 0.0, 0.0), p, near3(p, Vec3::new(5.0, 0.0, 0.0)));
}

#[test]
fn smooth_nudge() {
    let mut v = Vec3::ZERO;
    for _ in 0..1000 {
        v.smooth_nudge(&Vec3::new(10.0, 0.0, 0.0), 5.0, 1.0 / 60.0);
    }
    report::approx("Vec3::smooth_nudge converges to target", "target (10,0,0), 1000 steps", Vec3::new(10.0, 0.0, 0.0), v, near3(v, Vec3::new(10.0, 0.0, 0.0)));
}
