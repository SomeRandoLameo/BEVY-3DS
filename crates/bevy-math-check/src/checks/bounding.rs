//! `bevy_math::bounding`: `Aabb2d`/`Aabb3d`, `BoundingCircle`/`BoundingSphere`,
//! `BoundingVolume` + `IntersectsVolume`, and the ray/shape casts.

use super::report;
use bevy_math::bounding::{
    Aabb2d, Aabb3d, AabbCast2d, BoundingCircle, BoundingCircleCast, BoundingSphere,
    BoundingSphereCast, BoundingVolume, IntersectsVolume, RayCast2d, RayCast3d,
};
use bevy_math::{Dir2, Dir3, Ray2d, Ray3d, Vec2, Vec3};

#[test]
fn aabb2d_bounding_volume() {
    let a = Aabb2d::new(Vec2::ZERO, Vec2::new(2.0, 1.0));
    report::eq("Aabb2d::center", "c=0 h=(2,1)", Vec2::ZERO, a.center());
    report::eq("Aabb2d::half_size", "c=0 h=(2,1)", Vec2::new(2.0, 1.0), a.half_size());
    report::f("Aabb2d::visible_area (= area = w·h)", "h=(2,1) -> 4x2", 8.0, a.visible_area());
    report::is_true("Aabb2d::contains (inner)", "h=(2,1) contains h=(1,0.5)", a.contains(&Aabb2d::new(Vec2::ZERO, Vec2::new(1.0, 0.5))));
    report::eq("Aabb2d::merge", "h=(1,1)@0 + h=(1,1)@(4,0)", Vec2::new(3.0, 1.0), Aabb2d::new(Vec2::ZERO, Vec2::ONE).merge(&Aabb2d::new(Vec2::new(4.0, 0.0), Vec2::ONE)).half_size());
    report::eq("Aabb2d::grow", "h=(2,1) by (1,1)", Vec2::new(3.0, 2.0), a.grow(Vec2::ONE).half_size());
    report::eq("Aabb2d::shrink", "h=(2,1) by (0.5,0.5)", Vec2::new(1.5, 0.5), a.shrink(Vec2::splat(0.5)).half_size());
    report::eq("Aabb2d::closest_point (outside)", "h=(2,1), p=(5,0)", Vec2::new(2.0, 0.0), a.closest_point(Vec2::new(5.0, 0.0)));
}

#[test]
fn aabb3d_and_sphere() {
    let a = Aabb3d::new(Vec3::ZERO, Vec3::splat(2.0));
    report::approx("Aabb3d::center", "c=0 h=2", Vec3::ZERO, a.center().into(), super::near3(a.center().into(), Vec3::ZERO));
    report::approx("Aabb3d::half_size", "c=0 h=2", Vec3::splat(2.0), a.half_size().into(), super::near3(a.half_size().into(), Vec3::splat(2.0)));
    let merged = Aabb3d::new(Vec3::ZERO, Vec3::ONE).merge(&Aabb3d::new(Vec3::X * 4.0, Vec3::ONE));
    report::approx("Aabb3d::merge half_size", "h=1@0 + h=1@(4,0,0)", Vec3::new(3.0, 1.0, 1.0), merged.half_size().into(), super::near3(merged.half_size().into(), Vec3::new(3.0, 1.0, 1.0)));
    let s = BoundingSphere::new(Vec3::ZERO, 3.0);
    report::f("BoundingSphere::radius", "r=3", 3.0, s.radius());
    report::f("BoundingSphere::visible_area", "r=3", 2.0 * core::f32::consts::PI * 9.0, s.visible_area());
    report::is_true("BoundingSphere::contains", "r=3 contains r=1", s.contains(&BoundingSphere::new(Vec3::ZERO, 1.0)));
}

#[test]
fn intersections_2d() {
    let a = Aabb2d::new(Vec2::ZERO, Vec2::ONE);
    let b = Aabb2d::new(Vec2::new(1.5, 0.0), Vec2::ONE);
    let far = Aabb2d::new(Vec2::new(10.0, 0.0), Vec2::ONE);
    report::is_true("Aabb2d ∩ Aabb2d (overlap)", "", a.intersects(&b));
    report::eq("Aabb2d ∩ Aabb2d (disjoint)", "", false, a.intersects(&far));
    let c = BoundingCircle::new(Vec2::new(1.0, 0.0), 0.5);
    report::is_true("Aabb2d ∩ BoundingCircle", "", a.intersects(&c));
    report::is_true("BoundingCircle ∩ BoundingCircle", "", BoundingCircle::new(Vec2::ZERO, 1.0).intersects(&BoundingCircle::new(Vec2::new(1.5, 0.0), 1.0)));
}

#[test]
fn intersections_and_casts_3d() {
    let aabb = Aabb3d::new(Vec3::new(5.0, 0.0, 0.0), Vec3::ONE);
    let hit = RayCast3d::from_ray(Ray3d::new(Vec3::ZERO, Dir3::X), 100.0);
    let miss = RayCast3d::from_ray(Ray3d::new(Vec3::ZERO, Dir3::Y), 100.0);
    report::is_true("RayCast3d(+X) intersects Aabb3d @ x=5", "", hit.intersects(&aabb));
    report::eq("RayCast3d(+Y) misses Aabb3d @ x=5", "", false, miss.intersects(&aabb));
    report::f("RayCast3d::aabb_intersection_at", "ray +X, aabb center 5 half 1", 4.0, hit.aabb_intersection_at(&aabb).unwrap());
    let sphere = BoundingSphere::new(Vec3::new(0.0, 6.0, 0.0), 2.0);
    report::f("RayCast3d::sphere_intersection_at", "ray +Y, sphere @ y=6 r=2", 4.0, RayCast3d::from_ray(Ray3d::new(Vec3::ZERO, Dir3::Y), 100.0).sphere_intersection_at(&sphere).unwrap());
    report::is_true("Aabb3d ∩ Aabb3d", "", Aabb3d::new(Vec3::ZERO, Vec3::ONE).intersects(&Aabb3d::new(Vec3::new(1.5, 0.0, 0.0), Vec3::ONE)));
    report::is_true("BoundingSphere ∩ Aabb3d", "", BoundingSphere::new(Vec3::new(1.0, 0.0, 0.0), 0.6).intersects(&Aabb3d::new(Vec3::ZERO, Vec3::ONE)));

    let moving = AabbCast2d::from_ray(Aabb2d::new(Vec2::ZERO, Vec2::splat(0.5)), Ray2d::new(Vec2::ZERO, Dir2::X), 100.0);
    report::f("AabbCast2d::aabb_collision_at", "moving box +X toward box @ x=5", 4.0, moving.aabb_collision_at(Aabb2d::new(Vec2::new(5.0, 0.0), Vec2::splat(0.5))).unwrap());
    let bcc = BoundingCircleCast::from_ray(BoundingCircle::new(Vec2::ZERO, 0.5), Ray2d::new(Vec2::ZERO, Dir2::X), 100.0);
    report::f("BoundingCircleCast::circle_collision_at", "moving circle +X toward circle @ x=5", 4.0, bcc.circle_collision_at(BoundingCircle::new(Vec2::new(5.0, 0.0), 0.5)).unwrap());
    let bsc = BoundingSphereCast::from_ray(BoundingSphere::new(Vec3::ZERO, 0.5), Ray3d::new(Vec3::ZERO, Dir3::Z), 100.0);
    report::f("BoundingSphereCast::sphere_collision_at", "moving sphere +Z toward sphere @ z=5", 4.0, bsc.sphere_collision_at(BoundingSphere::new(Vec3::new(0.0, 0.0, 5.0), 0.5)).unwrap());

    let r2 = RayCast2d::from_ray(Ray2d::new(Vec2::ZERO, Dir2::X), 100.0);
    report::f("RayCast2d::circle_intersection_at", "ray +X, circle @ x=5 r=1", 4.0, r2.circle_intersection_at(&BoundingCircle::new(Vec2::new(5.0, 0.0), 1.0)).unwrap());
}
