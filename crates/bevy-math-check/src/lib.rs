//! Standalone correctness check for `bevy_math` (and its `glam` 0.32) on the 3DS
//! target (`armv6k-nintendo-3ds`).
//!
//! `bevy_math` *compiling* proves little — the interesting question is whether
//! the vector / quaternion / matrix / spline math **computes the right numbers**
//! on-device, since trig / `sqrt` / `exp` come from devkitPro's newlib rather
//! than the host libm. These tests pin known results with an epsilon.
//!
//! Run: `cargo 3ds build -p bevy-math-check`
//!      `./scripts/test-emulator.sh -p bevy-math-check`

#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(test_runner::run_gdb))]

use bevy_math::{Mat4, Quat, Vec3};

/// A representative transform pipeline (what a renderer would actually do):
/// build a model-view-projection matrix and project a point. Forces the common
/// glam paths to monomorphise so "it linked" is meaningful.
pub fn demo() -> Vec3 {
    let model = Mat4::from_scale_rotation_translation(
        Vec3::splat(2.0),
        Quat::from_rotation_y(core::f32::consts::FRAC_PI_2),
        Vec3::new(1.0, 0.0, -5.0),
    );
    let view = Mat4::look_at_rh(Vec3::new(0.0, 2.0, 8.0), Vec3::ZERO, Vec3::Y);
    let proj = Mat4::perspective_rh(60_f32.to_radians(), 400.0 / 240.0, 0.1, 100.0);
    let mvp = proj * view * model;
    mvp.project_point3(Vec3::new(0.5, 0.5, 0.0))
}

#[cfg(test)]
const EPS: f32 = 1.0e-4;

#[cfg(test)]
#[track_caller]
fn close(a: f32, b: f32) {
    assert!((a - b).abs() <= EPS, "expected {b}, got {a}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_math::bounding::{Aabb3d, IntersectsVolume, RayCast3d};
    use bevy_math::cubic_splines::CubicSegment;
    use bevy_math::curve::{Curve, EaseFunction, EasingCurve, FunctionCurve, Interval};
    use bevy_math::primitives::{Cuboid, Measured3d, Sphere};
    use bevy_math::{
        Dir2, Dir3, EulerRot, Isometry3d, Mat3, Ray3d, Rot2, Vec2, Vec3A, Vec4,
    };
    use core::f32::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_6, PI, SQRT_2};

    // --- newlib transcendentals (the actual risk on 3DS) ---------------------

    #[test]
    fn float_ops_match_known_values() {
        use bevy_math::ops;
        close(ops::sin(FRAC_PI_6), 0.5);
        close(ops::cos(FRAC_PI_6), 3.0_f32.sqrt() / 2.0);
        close(ops::tan(FRAC_PI_4), 1.0);
        close(ops::atan2(1.0, 1.0), FRAC_PI_4);
        close(ops::sqrt(2.0), SQRT_2);
        close(ops::powf(2.0, 10.0), 1024.0);
        close(ops::exp(0.0), 1.0);
        close(ops::ln(core::f32::consts::E), 1.0);
        close(ops::cbrt(27.0), 3.0);
        close(ops::hypot(3.0, 4.0), 5.0);
        // and the std methods glam calls internally
        close((FRAC_PI_2).sin(), 1.0);
        close(9.0_f32.sqrt(), 3.0);
    }

    // --- vectors -----------------------------------------------------------

    #[test]
    fn vector_algebra() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        close(a.dot(b), 32.0);
        assert_eq!(a.cross(b), Vec3::new(-3.0, 6.0, -3.0));
        close(Vec3::new(2.0, 3.0, 6.0).length(), 7.0);
        close(a.normalize().length(), 1.0);
        close(a.distance(b), (27.0_f32).sqrt());
        assert_eq!(Vec3::ZERO.lerp(Vec3::splat(10.0), 0.25), Vec3::splat(2.5));

        let v4 = Vec4::new(1.0, 0.0, 0.0, 2.0);
        close(v4.length_squared(), 5.0);
        assert_eq!(Vec2::new(3.0, 4.0) * 2.0, Vec2::new(6.0, 8.0));
    }

    #[test]
    fn vec2_rotation_helpers() {
        let r = Vec2::from_angle(FRAC_PI_2);
        assert!(r.rotate(Vec2::X).abs_diff_eq(Vec2::Y, EPS));
        // from_angle / to_angle round-trip
        close(Vec2::from_angle(0.7).to_angle(), 0.7);
        assert!(Vec2::X.perp().abs_diff_eq(Vec2::Y, EPS));
        close(Vec2::new(1.0, 1.0).angle_to(Vec2::new(-1.0, 1.0)), FRAC_PI_2);
    }

    // --- quaternions -----------------------------------------------------

    #[test]
    fn quaternion_rotation() {
        let q = Quat::from_axis_angle(Vec3::Z, FRAC_PI_2);
        assert!(q.mul_vec3(Vec3::X).abs_diff_eq(Vec3::Y, EPS));
        assert!(Quat::from_rotation_x(FRAC_PI_2)
            .mul_vec3(Vec3::Y)
            .abs_diff_eq(Vec3::Z, EPS));

        // composition: two 45° Z rotations == one 90°
        let half = Quat::from_rotation_z(FRAC_PI_4);
        assert!((half * half).abs_diff_eq(Quat::from_rotation_z(FRAC_PI_2), EPS));

        // normalize an un-normalized quat
        let messy = Quat::from_xyzw(1.0, 2.0, 3.0, 4.0).normalize();
        close(messy.length(), 1.0);

        // slerp midpoint of identity -> 90° == 45°
        let mid = Quat::IDENTITY.slerp(Quat::from_rotation_z(FRAC_PI_2), 0.5);
        assert!(mid.abs_diff_eq(Quat::from_rotation_z(FRAC_PI_4), EPS));

        // euler round-trip
        let e = Quat::from_euler(EulerRot::XYZ, 0.3, -0.4, 0.5);
        let (x, y, z) = e.to_euler(EulerRot::XYZ);
        close(x, 0.3);
        close(y, -0.4);
        close(z, 0.5);
    }

    // --- matrices --------------------------------------------------------

    #[test]
    fn mat4_transforms() {
        let t = Mat4::from_translation(Vec3::new(10.0, 0.0, 0.0));
        assert_eq!(t.transform_point3(Vec3::ZERO), Vec3::new(10.0, 0.0, 0.0));
        assert_eq!(t.transform_vector3(Vec3::Y), Vec3::Y); // vectors ignore translation

        let rz = Mat4::from_rotation_z(FRAC_PI_2);
        assert!(rz.transform_point3(Vec3::X).abs_diff_eq(Vec3::Y, EPS));

        let s = Mat4::from_scale(Vec3::new(2.0, 3.0, 4.0));
        assert_eq!(s.transform_point3(Vec3::ONE), Vec3::new(2.0, 3.0, 4.0));

        let srt = Mat4::from_scale_rotation_translation(
            Vec3::splat(2.0),
            Quat::from_rotation_z(FRAC_PI_2),
            Vec3::new(1.0, 1.0, 0.0),
        );
        assert!(srt
            .transform_point3(Vec3::X)
            .abs_diff_eq(Vec3::new(1.0, 3.0, 0.0), EPS));

        // inverse round-trip
        let inv = srt.inverse();
        assert!((srt * inv).abs_diff_eq(Mat4::IDENTITY, EPS));
        close(rz.determinant(), 1.0);
        close(s.determinant(), 24.0);

        // Mat4 * Vec4
        let v = t * Vec4::new(0.0, 0.0, 0.0, 1.0);
        assert_eq!(v, Vec4::new(10.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn mat3_and_mat4_camera() {
        let m = Mat3::from_scale_angle_translation(Vec2::splat(2.0), FRAC_PI_2, Vec2::new(1.0, 0.0));
        assert!(m
            .transform_point2(Vec2::X)
            .abs_diff_eq(Vec2::new(1.0, 2.0), EPS));

        // look_at: the eye maps to the origin in view space
        let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
        assert!(view
            .transform_point3(Vec3::new(0.0, 0.0, 5.0))
            .abs_diff_eq(Vec3::ZERO, EPS));

        // perspective: a point on the near plane centre projects to ~z=-1 (RH, [-1,1] depth is GL;
        // glam's perspective_rh uses [0,1] -> near maps to 0)
        let proj = Mat4::perspective_rh(FRAC_PI_2, 1.0, 1.0, 100.0);
        let near = proj.project_point3(Vec3::new(0.0, 0.0, -1.0));
        close(near.z, 0.0);
    }

    #[test]
    fn vec3a_layout_and_parity() {
        assert_eq!(core::mem::size_of::<Vec3A>(), 16);
        assert_eq!(core::mem::align_of::<Vec3A>(), 16);
        // Vec3A math agrees with Vec3
        let a = Vec3::new(1.5, -2.0, 3.25);
        let b = Vec3::new(-0.5, 4.0, 1.0);
        let scalar = a.cross(b);
        let wide = Vec3A::from(a).cross(Vec3A::from(b));
        assert!(Vec3::from(wide).abs_diff_eq(scalar, EPS));
    }

    // --- bevy_math primitives -------------------------------------------

    #[test]
    fn directions_normalise_and_reject() {
        let d = Dir3::new(Vec3::new(0.0, 0.0, 5.0)).unwrap();
        assert!(d.as_vec3().abs_diff_eq(Vec3::Z, EPS));
        assert!(Dir3::new(Vec3::ZERO).is_err());
        assert!(Dir2::new(Vec2::new(3.0, 4.0)).unwrap().as_vec2().abs_diff_eq(
            Vec2::new(0.6, 0.8),
            EPS
        ));
    }

    #[test]
    fn rot2_isometry_ray() {
        assert!((Rot2::degrees(90.0) * Vec2::X).abs_diff_eq(Vec2::Y, EPS));

        let iso = Isometry3d::new(Vec3::new(1.0, 2.0, 3.0), Quat::from_rotation_z(FRAC_PI_2));
        let p: Vec3 = iso.transform_point(Vec3::X).into();
        assert!(p.abs_diff_eq(Vec3::new(1.0, 3.0, 3.0), EPS));

        let ray = Ray3d::new(Vec3::ZERO, Dir3::X);
        assert_eq!(ray.get_point(2.5), Vec3::new(2.5, 0.0, 0.0));
    }

    #[test]
    fn bounding_volume_raycast() {
        let aabb = Aabb3d::new(Vec3::new(5.0, 0.0, 0.0), Vec3::splat(1.0));
        let hit = RayCast3d::from_ray(Ray3d::new(Vec3::ZERO, Dir3::X), 100.0);
        let miss = RayCast3d::from_ray(Ray3d::new(Vec3::ZERO, Dir3::Y), 100.0);
        assert!(hit.intersects(&aabb));
        assert!(!miss.intersects(&aabb));
        close(hit.aabb_intersection_at(&aabb).unwrap(), 4.0);
    }

    #[test]
    fn primitive_measurements() {
        close(Sphere::new(2.0).volume(), 4.0 / 3.0 * PI * 8.0);
        close(Sphere::new(2.0).area(), 4.0 * PI * 4.0);
        let c = Cuboid::new(2.0, 3.0, 4.0);
        close(c.volume(), 24.0);
        close(c.area(), 2.0 * (6.0 + 8.0 + 12.0));
    }

    // --- splines / curves ---------------------------------------------

    #[test]
    fn cubic_bezier_segment() {
        let seg = CubicSegment::new_bezier([
            Vec2::ZERO,
            Vec2::new(0.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 0.0),
        ]);
        assert!(seg.position(0.0).abs_diff_eq(Vec2::ZERO, EPS));
        assert!(seg.position(1.0).abs_diff_eq(Vec2::new(1.0, 0.0), EPS));
        // symmetric control points -> midpoint x = 0.5, y = 0.75
        assert!(seg.position(0.5).abs_diff_eq(Vec2::new(0.5, 0.75), EPS));
    }

    #[test]
    fn curve_feature() {
        let lin = EasingCurve::new(0.0_f32, 10.0, EaseFunction::Linear);
        close(lin.sample(0.5).unwrap(), 5.0);

        let quad = EasingCurve::new(0.0_f32, 1.0, EaseFunction::QuadraticIn);
        close(quad.sample(0.5).unwrap(), 0.25);

        // a plain closure-backed curve over [0, 1]
        let f = FunctionCurve::new(Interval::UNIT, |t| t * t + 1.0);
        close(f.sample(0.5).unwrap(), 1.25);
        assert!(f.sample(2.0).is_none(), "outside the interval");
    }

    // --- as a bevy_ecs-free sanity that demo() runs -------------------

    #[test]
    fn demo_pipeline_runs() {
        let p = demo();
        assert!(p.is_finite());
    }
}
