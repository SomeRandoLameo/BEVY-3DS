//! 3D primitives: construction, `Measured3d` (area/volume), `Bounded3d`.

use super::report;
use bevy_math::bounding::{Bounded3d, BoundingVolume};
use bevy_math::primitives::{
    Capsule3d, Cone, Cuboid, Cylinder, Measured3d, Sphere, Tetrahedron, Torus,
};
use bevy_math::{Isometry3d, Vec3};
use core::f32::consts::PI;

#[test]
fn measured_3d() {
    report::f("Sphere::new(2).volume()", "r=2", 4.0 / 3.0 * PI * 8.0, Sphere::new(2.0).volume());
    report::f("Sphere::new(2).area()", "r=2", 4.0 * PI * 4.0, Sphere::new(2.0).area());
    report::f("Cuboid::new(2, 3, 4).volume()", "", 24.0, Cuboid::new(2.0, 3.0, 4.0).volume());
    report::f("Cuboid::new(2, 3, 4).area()", "", 2.0 * (6.0 + 8.0 + 12.0), Cuboid::new(2.0, 3.0, 4.0).area());
    report::f("Cylinder::new(2, 4).volume()", "r=2 h=4", PI * 4.0 * 4.0, Cylinder::new(2.0, 4.0).volume());
    report::f("Cylinder::new(2, 4).area()", "r=2 h=4", 2.0 * PI * 4.0 + 2.0 * PI * 2.0 * 4.0, Cylinder::new(2.0, 4.0).area());
    report::f("Cone::new(2, 3).volume()", "r=2 h=3", PI * 4.0 * 3.0 / 3.0, Cone::new(2.0, 3.0).volume());
    report::f("Capsule3d::new(1, 2).volume()", "r=1 len=2", PI * 1.0 * 2.0 + 4.0 / 3.0 * PI, Capsule3d::new(1.0, 2.0).volume());
    // Torus::new(inner, outer) -> minor=(3-1)/2=1, major_radius=3-1=2; V = 2π²·major·minor²
    report::f("Torus::new(1, 3).volume()", "inner=1 outer=3 -> minor=1 major=2", 2.0 * PI * PI * 2.0 * 1.0, Torus::new(1.0, 3.0).volume());
    report::f(
        "Tetrahedron unit volume",
        "(0,0,0)(1,0,0)(0,1,0)(0,0,1)",
        1.0 / 6.0,
        Tetrahedron::new(Vec3::ZERO, Vec3::X, Vec3::Y, Vec3::Z).volume(),
    );
}

#[test]
fn bounded_3d() {
    let id = Isometry3d::IDENTITY;
    let s = Sphere::new(3.0).aabb_3d(id);
    report::approx("Sphere::new(3).aabb_3d — half_size", "identity", Vec3::splat(3.0), s.half_size().into(), super::near3(s.half_size().into(), Vec3::splat(3.0)));
    let c = Cuboid::new(4.0, 2.0, 6.0).aabb_3d(id);
    report::approx("Cuboid::new(4,2,6).aabb_3d — half_size", "identity", Vec3::new(2.0, 1.0, 3.0), c.half_size().into(), super::near3(c.half_size().into(), Vec3::new(2.0, 1.0, 3.0)));
    let bs = Sphere::new(3.0).bounding_sphere(id);
    report::f("Sphere::new(3).bounding_sphere — radius", "identity", 3.0, bs.radius());
    let bs2 = Cuboid::new(2.0, 2.0, 2.0).bounding_sphere(id);
    report::f("Cuboid::new(2,2,2).bounding_sphere — radius", "half-diagonal", 3.0_f32.sqrt(), bs2.radius());
}
