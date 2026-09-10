//! `glam` matrices: `Mat2`, `Mat3`, `Mat3A`, `Mat4`.

use super::{near2, near3, report, EPS};
use bevy_math::{Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec4};
use core::f32::consts::FRAC_PI_2;

#[test]
fn mat2() {
    let r = Mat2::from_angle(FRAC_PI_2);
    report::approx("Mat2::from_angle(PI/2) * X", "", Vec2::Y, r * Vec2::X, near2(r * Vec2::X, Vec2::Y));
    report::f("Mat2::from_angle(PI/2).determinant()", "", 1.0, r.determinant());
    report::approx("Mat2::inverse round-trip", "from_angle(PI/2)", Mat2::IDENTITY, r * r.inverse(), (r * r.inverse()).abs_diff_eq(Mat2::IDENTITY, EPS));
    report::approx("Mat2::from_scale_angle * X", "S=(2,3), a=0", Vec2::new(2.0, 0.0), Mat2::from_scale_angle(Vec2::new(2.0, 3.0), 0.0) * Vec2::X, near2(Mat2::from_scale_angle(Vec2::new(2.0, 3.0), 0.0) * Vec2::X, Vec2::new(2.0, 0.0)));
    report::eq("Mat2::from_diagonal", "(2,3) * (1,1)", Vec2::new(2.0, 3.0), Mat2::from_diagonal(Vec2::new(2.0, 3.0)) * Vec2::ONE);
    report::eq("Mat2::transpose", "cols->rows", Mat2::from_cols(Vec2::new(1.0, 3.0), Vec2::new(2.0, 4.0)), Mat2::from_cols(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)).transpose());
}

#[test]
fn mat3() {
    let m = Mat3::from_scale_angle_translation(Vec2::splat(2.0), FRAC_PI_2, Vec2::new(1.0, 0.0));
    report::approx("Mat3::from_scale_angle_translation.transform_point2(X)", "S=2, a=PI/2, T=(1,0)", Vec2::new(1.0, 2.0), m.transform_point2(Vec2::X), near2(m.transform_point2(Vec2::X), Vec2::new(1.0, 2.0)));
    report::approx("Mat3::transform_vector2 (ignores translation)", "same, vec X", Vec2::new(0.0, 2.0), m.transform_vector2(Vec2::X), near2(m.transform_vector2(Vec2::X), Vec2::new(0.0, 2.0)));
    report::approx("Mat3::from_rotation_z(PI/2) * X (as Vec3)", "", Vec3::Y, Mat3::from_rotation_z(FRAC_PI_2) * Vec3::X, near3(Mat3::from_rotation_z(FRAC_PI_2) * Vec3::X, Vec3::Y));
    report::approx("Mat3::from_quat(rot_z(PI/2)) * X", "", Vec3::Y, Mat3::from_quat(Quat::from_rotation_z(FRAC_PI_2)) * Vec3::X, near3(Mat3::from_quat(Quat::from_rotation_z(FRAC_PI_2)) * Vec3::X, Vec3::Y));
    report::f("Mat3::from_scale((2,3)).determinant()", "", 6.0, Mat3::from_scale(Vec2::new(2.0, 3.0)).determinant());
    report::approx("Mat3::inverse round-trip", "scale+rot", Mat3::IDENTITY, m * m.inverse(), (m * m.inverse()).abs_diff_eq(Mat3::IDENTITY, EPS));
    report::approx("Mat3::transpose", "rot_z(PI/2)", Mat3::from_rotation_z(-FRAC_PI_2), Mat3::from_rotation_z(FRAC_PI_2).transpose(), Mat3::from_rotation_z(FRAC_PI_2).transpose().abs_diff_eq(Mat3::from_rotation_z(-FRAC_PI_2), EPS));
    report::eq("Mat3::col(0)", "from_cols e_i", Vec3::X, Mat3::IDENTITY.col(0));
}

#[test]
fn mat3a() {
    report::approx("Mat3A::from_rotation_z(PI/2) * X", "", Vec3::Y, Mat3A::from_rotation_z(FRAC_PI_2).mul_vec3(Vec3::X), near3(Mat3A::from_rotation_z(FRAC_PI_2).mul_vec3(Vec3::X), Vec3::Y));
    report::f("Mat3A::from_scale((2,3)).determinant()", "", 6.0, Mat3A::from_scale(Vec2::new(2.0, 3.0)).determinant());
    report::eq("Mat3A: size_of", "", 48usize, core::mem::size_of::<Mat3A>());
}

#[test]
fn mat4_constructors_and_transforms() {
    let t = Mat4::from_translation(Vec3::new(10.0, 0.0, 0.0));
    report::eq("Mat4::from_translation.transform_point3", "T(10,0,0)*(0,0,0)", Vec3::new(10.0, 0.0, 0.0), t.transform_point3(Vec3::ZERO));
    report::eq("Mat4::from_translation.transform_vector3", "T(10,0,0)*vec Y", Vec3::Y, t.transform_vector3(Vec3::Y));
    report::approx("Mat4::from_rotation_z(PI/2).transform_point3(X)", "", Vec3::Y, Mat4::from_rotation_z(FRAC_PI_2).transform_point3(Vec3::X), near3(Mat4::from_rotation_z(FRAC_PI_2).transform_point3(Vec3::X), Vec3::Y));
    report::approx("Mat4::from_axis_angle(Z, PI/2).transform_point3(X)", "", Vec3::Y, Mat4::from_axis_angle(Vec3::Z, FRAC_PI_2).transform_point3(Vec3::X), near3(Mat4::from_axis_angle(Vec3::Z, FRAC_PI_2).transform_point3(Vec3::X), Vec3::Y));
    report::eq("Mat4::from_scale.transform_point3(ONE)", "S(2,3,4)", Vec3::new(2.0, 3.0, 4.0), Mat4::from_scale(Vec3::new(2.0, 3.0, 4.0)).transform_point3(Vec3::ONE));
    report::approx("Mat4::from_quat(rot_y(PI/2)).transform_point3(Z)", "", Vec3::X, Mat4::from_quat(Quat::from_rotation_y(FRAC_PI_2)).transform_point3(Vec3::Z), near3(Mat4::from_quat(Quat::from_rotation_y(FRAC_PI_2)).transform_point3(Vec3::Z), Vec3::X));
    let srt = Mat4::from_scale_rotation_translation(Vec3::splat(2.0), Quat::from_rotation_z(FRAC_PI_2), Vec3::new(1.0, 1.0, 0.0));
    report::approx("Mat4::from_scale_rotation_translation.transform_point3(X)", "S=2,Rz=PI/2,T=(1,1,0)", Vec3::new(1.0, 3.0, 0.0), srt.transform_point3(Vec3::X), near3(srt.transform_point3(Vec3::X), Vec3::new(1.0, 3.0, 0.0)));
}

#[test]
fn mat4_algebra_and_camera() {
    let s = Mat4::from_scale(Vec3::new(2.0, 3.0, 4.0));
    report::f("Mat4::determinant", "S(2,3,4)", 24.0, s.determinant());
    report::f("Mat4::from_rotation_z.determinant", "PI/2", 1.0, Mat4::from_rotation_z(FRAC_PI_2).determinant());
    let srt = Mat4::from_scale_rotation_translation(Vec3::splat(2.0), Quat::from_rotation_z(FRAC_PI_2), Vec3::ONE);
    report::approx("Mat4::inverse round-trip (M*M^-1)", "srt", Mat4::IDENTITY, srt * srt.inverse(), (srt * srt.inverse()).abs_diff_eq(Mat4::IDENTITY, EPS));
    report::approx("Mat4::transpose", "rot_z(PI/2)", Mat4::from_rotation_z(-FRAC_PI_2), Mat4::from_rotation_z(FRAC_PI_2).transpose(), Mat4::from_rotation_z(FRAC_PI_2).transpose().abs_diff_eq(Mat4::from_rotation_z(-FRAC_PI_2), EPS));
    report::eq("Mat4 * Vec4", "T(10,0,0) * (0,0,0,1)", Vec4::new(10.0, 0.0, 0.0, 1.0), Mat4::from_translation(Vec3::X * 10.0) * Vec4::new(0.0, 0.0, 0.0, 1.0));
    let (sc, ro, tr) = srt.to_scale_rotation_translation();
    report::approx("Mat4::to_scale_rotation_translation — scale", "srt", Vec3::splat(2.0), sc, near3(sc, Vec3::splat(2.0)));
    report::approx("Mat4::to_scale_rotation_translation — translation", "srt", Vec3::ONE, tr, near3(tr, Vec3::ONE));
    report::is_true("Mat4::to_scale_rotation_translation — rotation ≈ rot_z(PI/2)", "srt", ro.abs_diff_eq(Quat::from_rotation_z(FRAC_PI_2), EPS));

    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    report::approx("Mat4::look_at_rh — eye -> origin", "eye=(0,0,5)", Vec3::ZERO, view.transform_point3(Vec3::new(0.0, 0.0, 5.0)), near3(view.transform_point3(Vec3::new(0.0, 0.0, 5.0)), Vec3::ZERO));
    let proj = Mat4::perspective_rh(FRAC_PI_2, 1.0, 1.0, 100.0);
    report::f("Mat4::perspective_rh — near plane -> z≈0", "fov=PI/2, near=1, z=-1", 0.0, proj.project_point3(Vec3::new(0.0, 0.0, -1.0)).z);
    let ortho = Mat4::orthographic_rh(-1.0, 1.0, -1.0, 1.0, 0.0, 10.0);
    // glam's RH ortho uses a [0,1] depth range; z=-5 with near=0 far=10 -> 0.5.
    report::approx("Mat4::orthographic_rh center (z -5, [0,1] depth)", "near=0 far=10", Vec3::new(0.0, 0.0, 0.5), ortho.project_point3(Vec3::new(0.0, 0.0, -5.0)), near3(ortho.project_point3(Vec3::new(0.0, 0.0, -5.0)), Vec3::new(0.0, 0.0, 0.5)));
}
