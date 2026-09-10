//! `bevy_transform::traits::TransformPoint`.

use super::{near3, report};
use bevy_math::{Affine3A, Mat4, Quat, Vec3};
use bevy_transform::prelude::*;
use core::f32::consts::FRAC_PI_2;

#[test]
fn transform_point_trait_impls() {
    let expect = Vec3::new(10.0, 0.0, 0.0);

    let t = Transform::from_xyz(10.0, 0.0, 0.0);
    report::eq("TransformPoint on Transform", "T(10,0,0) * (0,0,0)", expect, TransformPoint::transform_point(&t, Vec3::ZERO));

    let g = GlobalTransform::from(t);
    report::eq("TransformPoint on GlobalTransform", "G(T(10,0,0)) * (0,0,0)", expect, TransformPoint::transform_point(&g, Vec3::ZERO));

    let m = Mat4::from_translation(Vec3::new(10.0, 0.0, 0.0));
    report::eq("TransformPoint on Mat4", "T(10,0,0) * (0,0,0)", expect, TransformPoint::transform_point(&m, Vec3::ZERO));

    let a = Affine3A::from_translation(Vec3::new(10.0, 0.0, 0.0));
    report::eq("TransformPoint on Affine3A", "T(10,0,0) * (0,0,0)", expect, TransformPoint::transform_point(&a, Vec3::ZERO));

    // with rotation, generic over impl
    fn tp<T: TransformPoint>(x: &T, p: Vec3) -> Vec3 {
        x.transform_point(p)
    }
    let rt = Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2));
    report::approx("TransformPoint (generic) on rotated Transform", "Rz(PI/2) * X", Vec3::Y, tp(&rt, Vec3::X), near3(tp(&rt, Vec3::X), Vec3::Y));
}
