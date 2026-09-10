//! `glam::Quat` — the full public surface.

use super::{near3, nearq, report, EPS};
use bevy_math::{EulerRot, Mat3, Mat4, Quat, Vec3};
use core::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

#[test]
fn quat_constructors() {
    report::approx("Quat::from_axis_angle(Z, PI/2) * X", "", Vec3::Y, Quat::from_axis_angle(Vec3::Z, FRAC_PI_2).mul_vec3(Vec3::X), near3(Quat::from_axis_angle(Vec3::Z, FRAC_PI_2).mul_vec3(Vec3::X), Vec3::Y));
    report::approx("Quat::from_rotation_x(PI/2) * Y", "", Vec3::Z, Quat::from_rotation_x(FRAC_PI_2).mul_vec3(Vec3::Y), near3(Quat::from_rotation_x(FRAC_PI_2).mul_vec3(Vec3::Y), Vec3::Z));
    report::approx("Quat::from_rotation_y(PI/2) * Z", "", Vec3::X, Quat::from_rotation_y(FRAC_PI_2).mul_vec3(Vec3::Z), near3(Quat::from_rotation_y(FRAC_PI_2).mul_vec3(Vec3::Z), Vec3::X));
    report::approx("Quat::from_scaled_axis(Z*PI/2) * X", "", Vec3::Y, Quat::from_scaled_axis(Vec3::Z * FRAC_PI_2).mul_vec3(Vec3::X), near3(Quat::from_scaled_axis(Vec3::Z * FRAC_PI_2).mul_vec3(Vec3::X), Vec3::Y));
    report::approx("Quat::from_mat3(Mat3::from_rotation_z(PI/2))", "", Quat::from_rotation_z(FRAC_PI_2), Quat::from_mat3(&Mat3::from_rotation_z(FRAC_PI_2)), nearq(Quat::from_mat3(&Mat3::from_rotation_z(FRAC_PI_2)), Quat::from_rotation_z(FRAC_PI_2)));
    report::approx("Quat::from_mat4(Mat4::from_rotation_z(PI/2))", "", Quat::from_rotation_z(FRAC_PI_2), Quat::from_mat4(&Mat4::from_rotation_z(FRAC_PI_2)), nearq(Quat::from_mat4(&Mat4::from_rotation_z(FRAC_PI_2)), Quat::from_rotation_z(FRAC_PI_2)));
    report::approx("Quat::from_rotation_arc(X, Y) * X", "", Vec3::Y, Quat::from_rotation_arc(Vec3::X, Vec3::Y).mul_vec3(Vec3::X), near3(Quat::from_rotation_arc(Vec3::X, Vec3::Y).mul_vec3(Vec3::X), Vec3::Y));
    report::eq("Quat::from_xyzw / from_array round-trip", "(0,0,0,1)", Quat::IDENTITY, Quat::from_array([0.0, 0.0, 0.0, 1.0]));
}

#[test]
fn quat_operations() {
    let half = Quat::from_rotation_z(FRAC_PI_4);
    report::approx("Quat * Quat (compose)", "rot_z(PI/4)^2", Quat::from_rotation_z(FRAC_PI_2), half * half, nearq(half * half, Quat::from_rotation_z(FRAC_PI_2)));
    report::approx("Quat::mul_quat", "rot_z(PI/4).mul_quat(rot_z(PI/4))", Quat::from_rotation_z(FRAC_PI_2), half.mul_quat(half), nearq(half.mul_quat(half), Quat::from_rotation_z(FRAC_PI_2)));
    report::f("Quat::dot", "IDENTITY·IDENTITY", 1.0, Quat::IDENTITY.dot(Quat::IDENTITY));
    report::f("Quat::length (normalized)", "rot_z(PI/4)", 1.0, half.length());
    report::f("Quat::length_squared", "rot_z(PI/4)", 1.0, half.length_squared());
    report::approx("Quat::conjugate", "rot_z(PI/2).conjugate() * Y", Vec3::X, Quat::from_rotation_z(FRAC_PI_2).conjugate().mul_vec3(Vec3::Y), near3(Quat::from_rotation_z(FRAC_PI_2).conjugate().mul_vec3(Vec3::Y), Vec3::X));
    report::approx("Quat::inverse", "rot_z(PI/2).inverse() * Y", Vec3::X, Quat::from_rotation_z(FRAC_PI_2).inverse().mul_vec3(Vec3::Y), near3(Quat::from_rotation_z(FRAC_PI_2).inverse().mul_vec3(Vec3::Y), Vec3::X));
    report::f("Quat::normalize().length()", "from_xyzw(1,2,3,4)", 1.0, Quat::from_xyzw(1.0, 2.0, 3.0, 4.0).normalize().length());
    report::is_true("Quat::is_normalized", "rot_z(PI/4)", half.is_normalized());
    report::is_true("Quat::is_near_identity", "rot_z(1e-5)", Quat::from_rotation_z(1e-5).is_near_identity());
    report::is_true("Quat::is_finite", "IDENTITY", Quat::IDENTITY.is_finite());
}

#[test]
fn quat_interpolation_and_decompose() {
    let mid = Quat::IDENTITY.slerp(Quat::from_rotation_z(FRAC_PI_2), 0.5);
    report::approx("Quat::slerp(IDENTITY, rot_z(PI/2), 0.5)", "", Quat::from_rotation_z(FRAC_PI_4), mid, nearq(mid, Quat::from_rotation_z(FRAC_PI_4)));
    let lmid = Quat::IDENTITY.lerp(Quat::from_rotation_z(FRAC_PI_2), 0.5).normalize();
    report::approx("Quat::lerp(IDENTITY, rot_z(PI/2), 0.5).normalize()", "", Quat::from_rotation_z(FRAC_PI_4), lmid, nearq(lmid, Quat::from_rotation_z(FRAC_PI_4)));
    report::f("Quat::angle_between", "IDENTITY, rot_z(PI/2)", FRAC_PI_2, Quat::IDENTITY.angle_between(Quat::from_rotation_z(FRAC_PI_2)));

    let (axis, angle) = Quat::from_axis_angle(Vec3::Z, 1.0).to_axis_angle();
    report::approx("Quat::to_axis_angle — axis", "from_axis_angle(Z, 1.0)", Vec3::Z, axis, near3(axis, Vec3::Z));
    report::f("Quat::to_axis_angle — angle", "from_axis_angle(Z, 1.0)", 1.0, angle);
    report::approx("Quat::to_scaled_axis", "from_scaled_axis(Z*1.2)", Vec3::Z * 1.2, Quat::from_scaled_axis(Vec3::Z * 1.2).to_scaled_axis(), near3(Quat::from_scaled_axis(Vec3::Z * 1.2).to_scaled_axis(), Vec3::Z * 1.2));

    let (x, y, z) = Quat::from_euler(EulerRot::XYZ, 0.3, -0.4, 0.5).to_euler(EulerRot::XYZ);
    report::approx("Quat::from_euler XYZ -> to_euler XYZ", "(0.3,-0.4,0.5)", (0.3_f32, -0.4_f32, 0.5_f32), (x, y, z), (x - 0.3).abs() <= EPS && (y + 0.4).abs() <= EPS && (z - 0.5).abs() <= EPS);
    let (zx, zy, zz) = Quat::from_euler(EulerRot::ZYX, 0.2, 0.1, -0.3).to_euler(EulerRot::ZYX);
    report::approx("Quat::from_euler ZYX -> to_euler ZYX", "(0.2,0.1,-0.3)", (0.2_f32, 0.1_f32, -0.3_f32), (zx, zy, zz), (zx - 0.2).abs() <= EPS && (zy - 0.1).abs() <= EPS && (zz + 0.3).abs() <= EPS);

    report::eq("Quat::xyz of rot_z(PI/2)", "≈(0,0,0.707)", true, (Quat::from_rotation_z(FRAC_PI_2).xyz() - Vec3::Z * (PI / 4.0).sin()).length() < EPS);
    report::eq("Quat::to_array", "IDENTITY", [0.0_f32, 0.0, 0.0, 1.0], Quat::IDENTITY.to_array());
}
