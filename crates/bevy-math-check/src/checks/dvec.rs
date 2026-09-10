//! `f64` vectors + matrices — exercises newlib's double-precision math.

use super::report;
use bevy_math::{DMat4, DQuat, DVec2, DVec3, DVec4};

#[test]
fn dvec_math() {
    report::d("DVec2::dot", "(1,2)·(3,4)", 11.0, DVec2::new(1.0, 2.0).dot(DVec2::new(3.0, 4.0)));
    report::d("DVec3::length", "(2,3,6)", 7.0, DVec3::new(2.0, 3.0, 6.0).length());
    report::d("DVec3::normalize().length()", "(1,2,3)", 1.0, DVec3::new(1.0, 2.0, 3.0).normalize().length());
    report::d("DVec3::cross.x", "(1,0,0)x(0,1,0)", 0.0, DVec3::X.cross(DVec3::Y).x);
    report::d("DVec3::cross.z", "(1,0,0)x(0,1,0)", 1.0, DVec3::X.cross(DVec3::Y).z);
    report::d("DVec4::length", "(0,0,3,4)", 5.0, DVec4::new(0.0, 0.0, 3.0, 4.0).length());
    report::d("DVec2::distance", "(0,0)->(3,4)", 5.0, DVec2::ZERO.distance(DVec2::new(3.0, 4.0)));
    report::eq("DVec3 -> Vec3 (as_vec3)", "(1.5,-2,3)", bevy_math::Vec3::new(1.5, -2.0, 3.0), DVec3::new(1.5, -2.0, 3.0).as_vec3());
}

#[test]
fn dquat_and_dmat() {
    let q = DQuat::from_rotation_z(core::f64::consts::FRAC_PI_2);
    let r = q.mul_vec3(DVec3::X);
    report::d("DQuat::from_rotation_z(PI/2).mul_vec3(X).y", "", 1.0, r.y);
    report::d("DQuat::length after normalize", "from_xyzw(1,2,3,4)", 1.0, DQuat::from_xyzw(1.0, 2.0, 3.0, 4.0).normalize().length());

    let m = DMat4::from_translation(DVec3::new(10.0, 0.0, 0.0));
    report::d("DMat4::from_translation.transform_point3.x", "T(10,0,0)*(0,0,0)", 10.0, m.transform_point3(DVec3::ZERO).x);
    report::d("DMat4::from_scale.determinant", "S(2,3,4)", 24.0, DMat4::from_scale(DVec3::new(2.0, 3.0, 4.0)).determinant());
    let srt = DMat4::from_scale_rotation_translation(DVec3::splat(2.0), q, DVec3::ONE);
    report::d("DMat4::inverse round-trip diagonal", "srt", 1.0, (srt * srt.inverse()).x_axis.x);
}
