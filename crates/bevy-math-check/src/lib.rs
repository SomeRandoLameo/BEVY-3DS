//! Standalone correctness check for `bevy_math` (and its `glam` 0.32) on the 3DS
//! target (`armv6k-nintendo-3ds`).
//!
//! `bevy_math` *compiling* proves little — the question is whether the vector /
//! quaternion / matrix / spline math **computes the right numbers** on-device,
//! since trig / `sqrt` / `exp` come from devkitPro's newlib rather than the host
//! libm.
//!
//! Each check prints a Markdown table row (function | input | expected | got |
//! status); see `README.md`. Run: `./scripts/test-emulator.sh -p bevy-math-check`.

#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(test_runner::run_gdb))]

use bevy_math::{Mat4, Quat, Vec3};

/// A representative transform pipeline (build an MVP matrix, project a point).
/// Forces the common glam paths to monomorphise.
pub fn demo() -> Vec3 {
    let model = Mat4::from_scale_rotation_translation(
        Vec3::splat(2.0),
        Quat::from_rotation_y(core::f32::consts::FRAC_PI_2),
        Vec3::new(1.0, 0.0, -5.0),
    );
    let view = Mat4::look_at_rh(Vec3::new(0.0, 2.0, 8.0), Vec3::ZERO, Vec3::Y);
    let proj = Mat4::perspective_rh(60_f32.to_radians(), 400.0 / 240.0, 0.1, 100.0);
    (proj * view * model).project_point3(Vec3::new(0.5, 0.5, 0.0))
}

#[cfg(test)]
mod report;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report;
    use bevy_math::bounding::{Aabb3d, IntersectsVolume, RayCast3d};
    use bevy_math::cubic_splines::CubicSegment;
    use bevy_math::curve::{Curve, EaseFunction, EasingCurve, FunctionCurve, Interval};
    use bevy_math::primitives::{Cuboid, Measured3d, Sphere};
    use bevy_math::{Dir2, Dir3, EulerRot, Isometry3d, Mat3, Ray3d, Rot2, Vec2, Vec3A, Vec4};
    use core::f32::consts::{E, FRAC_PI_2, FRAC_PI_4, FRAC_PI_6, PI, SQRT_2};

    const EPS: f32 = report::EPS;
    fn near2(a: Vec2, b: Vec2) -> bool { a.abs_diff_eq(b, EPS) }
    fn near3(a: Vec3, b: Vec3) -> bool { a.abs_diff_eq(b, EPS) }
    fn nearq(a: Quat, b: Quat) -> bool { a.abs_diff_eq(b, EPS) }

    // --- newlib transcendentals (the actual risk on 3DS) ------------------

    #[test]
    fn float_ops_match_known_values() {
        use bevy_math::ops;
        report::f("bevy_math::ops::sin", "PI/6", 0.5, ops::sin(FRAC_PI_6));
        report::f("bevy_math::ops::cos", "PI/6", 3.0_f32.sqrt() / 2.0, ops::cos(FRAC_PI_6));
        report::f("bevy_math::ops::tan", "PI/4", 1.0, ops::tan(FRAC_PI_4));
        report::f("bevy_math::ops::atan2", "(1, 1)", FRAC_PI_4, ops::atan2(1.0, 1.0));
        report::f("bevy_math::ops::sqrt", "2", SQRT_2, ops::sqrt(2.0));
        report::f("bevy_math::ops::powf", "2^10", 1024.0, ops::powf(2.0, 10.0));
        report::f("bevy_math::ops::exp", "0", 1.0, ops::exp(0.0));
        report::f("bevy_math::ops::ln", "E", 1.0, ops::ln(E));
        report::f("bevy_math::ops::cbrt", "27", 3.0, ops::cbrt(27.0));
        report::f("bevy_math::ops::hypot", "(3, 4)", 5.0, ops::hypot(3.0, 4.0));
        report::f("f32::sin", "PI/2", 1.0, (FRAC_PI_2).sin());
        report::f("f32::sqrt", "9", 3.0, 9.0_f32.sqrt());
    }

    // --- vectors --------------------------------------------------------

    #[test]
    fn vector_algebra() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        report::f("Vec3::dot", "(1,2,3)·(4,5,6)", 32.0, a.dot(b));
        report::eq("Vec3::cross", "(1,2,3)x(4,5,6)", Vec3::new(-3.0, 6.0, -3.0), a.cross(b));
        report::f("Vec3::length", "(2,3,6)", 7.0, Vec3::new(2.0, 3.0, 6.0).length());
        report::f("Vec3::normalize().length()", "(1,2,3)", 1.0, a.normalize().length());
        report::f("Vec3::distance", "(1,2,3)->(4,5,6)", 27.0_f32.sqrt(), a.distance(b));
        report::eq("Vec3::lerp", "0 -> (10,10,10) @ t=0.25", Vec3::splat(2.5), Vec3::ZERO.lerp(Vec3::splat(10.0), 0.25));
        report::f("Vec4::length_squared", "(1,0,0,2)", 5.0, Vec4::new(1.0, 0.0, 0.0, 2.0).length_squared());
        report::eq("Vec2 * f32", "(3,4) * 2", Vec2::new(6.0, 8.0), Vec2::new(3.0, 4.0) * 2.0);
    }

    #[test]
    fn vec2_rotation_helpers() {
        let r = Vec2::from_angle(FRAC_PI_2);
        report::approx("Vec2::from_angle(a).rotate(X)", "a = PI/2", Vec2::Y, r.rotate(Vec2::X), near2(r.rotate(Vec2::X), Vec2::Y));
        report::f("Vec2::from_angle(a).to_angle()", "a = 0.7", 0.7, Vec2::from_angle(0.7).to_angle());
        report::approx("Vec2::perp", "X", Vec2::Y, Vec2::X.perp(), near2(Vec2::X.perp(), Vec2::Y));
        report::f("Vec2::angle_to", "(1,1) -> (-1,1)", FRAC_PI_2, Vec2::new(1.0, 1.0).angle_to(Vec2::new(-1.0, 1.0)));
    }

    // --- quaternions --------------------------------------------------

    #[test]
    fn quaternion_rotation() {
        let q = Quat::from_axis_angle(Vec3::Z, FRAC_PI_2);
        report::approx("Quat::from_axis_angle(Z, PI/2).mul_vec3(X)", "", Vec3::Y, q.mul_vec3(Vec3::X), near3(q.mul_vec3(Vec3::X), Vec3::Y));
        let rx = Quat::from_rotation_x(FRAC_PI_2);
        report::approx("Quat::from_rotation_x(PI/2).mul_vec3(Y)", "", Vec3::Z, rx.mul_vec3(Vec3::Y), near3(rx.mul_vec3(Vec3::Y), Vec3::Z));
        let half = Quat::from_rotation_z(FRAC_PI_4);
        report::approx("Quat::from_rotation_z(PI/4) squared", "", Quat::from_rotation_z(FRAC_PI_2), half * half, nearq(half * half, Quat::from_rotation_z(FRAC_PI_2)));
        report::f("Quat::from_xyzw(1,2,3,4).normalize().length()", "", 1.0, Quat::from_xyzw(1.0, 2.0, 3.0, 4.0).normalize().length());
        let mid = Quat::IDENTITY.slerp(Quat::from_rotation_z(FRAC_PI_2), 0.5);
        report::approx("Quat::slerp(IDENTITY, rot_z(PI/2), 0.5)", "", Quat::from_rotation_z(FRAC_PI_4), mid, nearq(mid, Quat::from_rotation_z(FRAC_PI_4)));
        let (x, y, z) = Quat::from_euler(EulerRot::XYZ, 0.3, -0.4, 0.5).to_euler(EulerRot::XYZ);
        report::approx("Quat::from_euler -> to_euler (XYZ)", "(0.3, -0.4, 0.5)", (0.3_f32, -0.4_f32, 0.5_f32), (x, y, z),
            (x - 0.3).abs() <= EPS && (y + 0.4).abs() <= EPS && (z - 0.5).abs() <= EPS);
    }

    // --- matrices ---------------------------------------------------

    #[test]
    fn mat4_transforms() {
        let t = Mat4::from_translation(Vec3::new(10.0, 0.0, 0.0));
        report::eq("Mat4::from_translation.transform_point3", "T(10,0,0) * (0,0,0)", Vec3::new(10.0, 0.0, 0.0), t.transform_point3(Vec3::ZERO));
        report::eq("Mat4::from_translation.transform_vector3", "T(10,0,0) * vec Y", Vec3::Y, t.transform_vector3(Vec3::Y));
        let rz = Mat4::from_rotation_z(FRAC_PI_2);
        report::approx("Mat4::from_rotation_z(PI/2).transform_point3(X)", "", Vec3::Y, rz.transform_point3(Vec3::X), near3(rz.transform_point3(Vec3::X), Vec3::Y));
        let s = Mat4::from_scale(Vec3::new(2.0, 3.0, 4.0));
        report::eq("Mat4::from_scale.transform_point3(ONE)", "S(2,3,4)", Vec3::new(2.0, 3.0, 4.0), s.transform_point3(Vec3::ONE));
        let srt = Mat4::from_scale_rotation_translation(Vec3::splat(2.0), Quat::from_rotation_z(FRAC_PI_2), Vec3::new(1.0, 1.0, 0.0));
        report::approx("Mat4::from_scale_rotation_translation.transform_point3(X)", "S=2, Rz=PI/2, T=(1,1,0)", Vec3::new(1.0, 3.0, 0.0), srt.transform_point3(Vec3::X), near3(srt.transform_point3(Vec3::X), Vec3::new(1.0, 3.0, 0.0)));
        let ident = srt * srt.inverse();
        report::approx("Mat4::inverse round-trip (M * M^-1)", "srt above", Mat4::IDENTITY, ident, ident.abs_diff_eq(Mat4::IDENTITY, EPS));
        report::f("Mat4::from_rotation_z.determinant", "PI/2", 1.0, rz.determinant());
        report::f("Mat4::from_scale.determinant", "S(2,3,4)", 24.0, s.determinant());
        report::eq("Mat4 * Vec4", "T(10,0,0) * (0,0,0,1)", Vec4::new(10.0, 0.0, 0.0, 1.0), t * Vec4::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn mat3_and_mat4_camera() {
        let m = Mat3::from_scale_angle_translation(Vec2::splat(2.0), FRAC_PI_2, Vec2::new(1.0, 0.0));
        report::approx("Mat3::from_scale_angle_translation.transform_point2(X)", "S=2, a=PI/2, T=(1,0)", Vec2::new(1.0, 2.0), m.transform_point2(Vec2::X), near2(m.transform_point2(Vec2::X), Vec2::new(1.0, 2.0)));
        let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
        report::approx("Mat4::look_at_rh — eye maps to origin", "eye=(0,0,5)", Vec3::ZERO, view.transform_point3(Vec3::new(0.0, 0.0, 5.0)), near3(view.transform_point3(Vec3::new(0.0, 0.0, 5.0)), Vec3::ZERO));
        let proj = Mat4::perspective_rh(FRAC_PI_2, 1.0, 1.0, 100.0);
        report::f("Mat4::perspective_rh — near plane depth", "fov=PI/2, near=1, point z=-1", 0.0, proj.project_point3(Vec3::new(0.0, 0.0, -1.0)).z);
    }

    #[test]
    fn vec3a_layout_and_parity() {
        report::eq("core::mem::size_of::<Vec3A>()", "", 16usize, core::mem::size_of::<Vec3A>());
        report::eq("core::mem::align_of::<Vec3A>()", "", 16usize, core::mem::align_of::<Vec3A>());
        let a = Vec3::new(1.5, -2.0, 3.25);
        let b = Vec3::new(-0.5, 4.0, 1.0);
        let wide = Vec3::from(Vec3A::from(a).cross(Vec3A::from(b)));
        report::approx("Vec3A::cross == Vec3::cross", "(1.5,-2,3.25) x (-0.5,4,1)", a.cross(b), wide, near3(wide, a.cross(b)));
    }

    // --- bevy_math primitives ------------------------------------

    #[test]
    fn directions_normalise_and_reject() {
        let d = Dir3::new(Vec3::new(0.0, 0.0, 5.0)).unwrap();
        report::approx("Dir3::new normalises", "(0,0,5)", Vec3::Z, d.as_vec3(), near3(d.as_vec3(), Vec3::Z));
        report::ok("Dir3::new rejects zero vector", "(0,0,0)", "Err", if Dir3::new(Vec3::ZERO).is_err() { "Err" } else { "Ok" }, Dir3::new(Vec3::ZERO).is_err());
        let d2 = Dir2::new(Vec2::new(3.0, 4.0)).unwrap().as_vec2();
        report::approx("Dir2::new normalises", "(3,4)", Vec2::new(0.6, 0.8), d2, near2(d2, Vec2::new(0.6, 0.8)));
    }

    #[test]
    fn rot2_isometry_ray() {
        let r = Rot2::degrees(90.0) * Vec2::X;
        report::approx("Rot2::degrees(90) * Vec2::X", "", Vec2::Y, r, near2(r, Vec2::Y));
        let iso = Isometry3d::new(Vec3::new(1.0, 2.0, 3.0), Quat::from_rotation_z(FRAC_PI_2));
        let p: Vec3 = iso.transform_point(Vec3::X).into();
        report::approx("Isometry3d::new(T,Rz).transform_point(X)", "T=(1,2,3), Rz=PI/2", Vec3::new(1.0, 3.0, 3.0), p, near3(p, Vec3::new(1.0, 3.0, 3.0)));
        report::eq("Ray3d::new(0, Dir3::X).get_point(2.5)", "", Vec3::new(2.5, 0.0, 0.0), Ray3d::new(Vec3::ZERO, Dir3::X).get_point(2.5));
    }

    #[test]
    fn bounding_volume_raycast() {
        let aabb = Aabb3d::new(Vec3::new(5.0, 0.0, 0.0), Vec3::splat(1.0));
        let hit = RayCast3d::from_ray(Ray3d::new(Vec3::ZERO, Dir3::X), 100.0);
        let miss = RayCast3d::from_ray(Ray3d::new(Vec3::ZERO, Dir3::Y), 100.0);
        report::ok("RayCast3d(+X) intersects Aabb3d @ x=5", "", "true", &hit.intersects(&aabb).to_string(), hit.intersects(&aabb));
        report::ok("RayCast3d(+Y) misses Aabb3d @ x=5", "", "false", &miss.intersects(&aabb).to_string(), !miss.intersects(&aabb));
        report::f("RayCast3d::aabb_intersection_at", "ray +X, aabb center 5 half 1", 4.0, hit.aabb_intersection_at(&aabb).unwrap());
    }

    #[test]
    fn primitive_measurements() {
        report::f("Sphere::new(2).volume()", "r=2", 4.0 / 3.0 * PI * 8.0, Sphere::new(2.0).volume());
        report::f("Sphere::new(2).area()", "r=2", 4.0 * PI * 4.0, Sphere::new(2.0).area());
        report::f("Cuboid::new(2,3,4).volume()", "", 24.0, Cuboid::new(2.0, 3.0, 4.0).volume());
        report::f("Cuboid::new(2,3,4).area()", "", 2.0 * (6.0 + 8.0 + 12.0), Cuboid::new(2.0, 3.0, 4.0).area());
    }

    // --- splines / curves --------------------------------------

    #[test]
    fn cubic_bezier_segment() {
        let seg = CubicSegment::new_bezier([Vec2::ZERO, Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0), Vec2::new(1.0, 0.0)]);
        report::approx("CubicSegment::new_bezier(..).position(0)", "control pts (0,0)(0,1)(1,1)(1,0)", Vec2::ZERO, seg.position(0.0), near2(seg.position(0.0), Vec2::ZERO));
        report::approx("CubicSegment::position(1)", "same", Vec2::new(1.0, 0.0), seg.position(1.0), near2(seg.position(1.0), Vec2::new(1.0, 0.0)));
        report::approx("CubicSegment::position(0.5)", "same", Vec2::new(0.5, 0.75), seg.position(0.5), near2(seg.position(0.5), Vec2::new(0.5, 0.75)));
    }

    #[test]
    fn curve_feature() {
        let lin = EasingCurve::new(0.0_f32, 10.0, EaseFunction::Linear);
        report::f("EasingCurve(Linear, 0..10).sample(0.5)", "", 5.0, lin.sample(0.5).unwrap());
        let quad = EasingCurve::new(0.0_f32, 1.0, EaseFunction::QuadraticIn);
        report::f("EasingCurve(QuadraticIn, 0..1).sample(0.5)", "", 0.25, quad.sample(0.5).unwrap());
        let f = FunctionCurve::new(Interval::UNIT, |t| t * t + 1.0);
        report::f("FunctionCurve(|t| t^2+1 over [0,1]).sample(0.5)", "", 1.25, f.sample(0.5).unwrap());
        report::ok("FunctionCurve::sample(2.0) outside interval", "t = 2.0", "None", if f.sample(2.0).is_none() { "None" } else { "Some" }, f.sample(2.0).is_none());
    }

    #[test]
    fn demo_pipeline_runs() {
        let p = demo();
        report::ok("demo() — proj·view·model.project_point3", "MVP of (0.5, 0.5, 0)", "finite", if p.is_finite() { "finite" } else { "non-finite" }, p.is_finite());
    }
}
