//! Memory layout (`size_of` / `align_of`) of every public `bevy_math` type on
//! `armv6k-nintendo-3ds`. These rows *record* the actual layout (they always
//! "pass") so a target-specific `#[repr]` / padding surprise is visible.

use super::report;
use core::mem::{align_of, size_of};

fn rec<T>(name: &str) {
    report::ok(
        name,
        "",
        "size / align",
        &format!("{} / {}", size_of::<T>(), align_of::<T>()),
        true,
    );
}

#[test]
fn glam_type_layout() {
    use bevy_math::{
        Affine2, Affine3A, DMat4, DQuat, DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, Mat2, Mat3,
        Mat3A, Mat4, Quat, UVec3, Vec2, Vec3, Vec3A, Vec4,
    };
    rec::<Vec2>("size/align Vec2");
    rec::<Vec3>("size/align Vec3");
    rec::<Vec3A>("size/align Vec3A");
    rec::<Vec4>("size/align Vec4");
    rec::<IVec2>("size/align IVec2");
    rec::<IVec3>("size/align IVec3");
    rec::<IVec4>("size/align IVec4");
    rec::<UVec3>("size/align UVec3");
    rec::<DVec2>("size/align DVec2");
    rec::<DVec3>("size/align DVec3");
    rec::<DVec4>("size/align DVec4");
    rec::<Mat2>("size/align Mat2");
    rec::<Mat3>("size/align Mat3");
    rec::<Mat3A>("size/align Mat3A");
    rec::<Mat4>("size/align Mat4");
    rec::<DMat4>("size/align DMat4");
    rec::<Quat>("size/align Quat");
    rec::<DQuat>("size/align DQuat");
    rec::<Affine2>("size/align Affine2");
    rec::<Affine3A>("size/align Affine3A");
}

#[test]
fn bevy_math_type_layout() {
    use bevy_math::{
        bounding::{Aabb2d, Aabb3d, BoundingCircle, BoundingSphere},
        primitives::{Circle, Cuboid, Sphere},
        Dir2, Dir3, Dir3A, Isometry2d, Isometry3d, Ray2d, Ray3d, Rot2,
    };
    rec::<Dir2>("size/align Dir2");
    rec::<Dir3>("size/align Dir3");
    rec::<Dir3A>("size/align Dir3A");
    rec::<Rot2>("size/align Rot2");
    rec::<Isometry2d>("size/align Isometry2d");
    rec::<Isometry3d>("size/align Isometry3d");
    rec::<Ray2d>("size/align Ray2d");
    rec::<Ray3d>("size/align Ray3d");
    rec::<Circle>("size/align Circle");
    rec::<Sphere>("size/align Sphere");
    rec::<Cuboid>("size/align Cuboid");
    rec::<Aabb2d>("size/align Aabb2d");
    rec::<Aabb3d>("size/align Aabb3d");
    rec::<BoundingCircle>("size/align BoundingCircle");
    rec::<BoundingSphere>("size/align BoundingSphere");
}
