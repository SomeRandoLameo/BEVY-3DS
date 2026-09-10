//! 2D primitives: construction, `Measured2d` (perimeter/area), `Bounded2d`.

use super::report;
use bevy_math::bounding::{Bounded2d, BoundingVolume};
use bevy_math::primitives::{
    Annulus, Capsule2d, Circle, CircularSector, Ellipse, Measured2d, Rectangle, RegularPolygon,
    Rhombus, Triangle2d,
};
use bevy_math::{Isometry2d, Vec2};
use core::f32::consts::PI;

#[test]
fn measured_2d() {
    report::f("Circle::new(2).area()", "r=2", PI * 4.0, Circle::new(2.0).area());
    report::f("Circle::new(2).perimeter()", "r=2", 2.0 * PI * 2.0, Circle::new(2.0).perimeter());
    report::f("Ellipse::new(3, 2).area()", "a=3 b=2", PI * 6.0, Ellipse::new(3.0, 2.0).area());
    report::f("Rectangle::new(4, 2).area()", "", 8.0, Rectangle::new(4.0, 2.0).area());
    report::f("Rectangle::new(4, 2).perimeter()", "", 12.0, Rectangle::new(4.0, 2.0).perimeter());
    report::f("Rhombus::new(4, 2).area()", "diagonals 4,2", 4.0, Rhombus::new(4.0, 2.0).area());
    report::f("Triangle2d area (3-4-5)", "(0,0)(3,0)(0,4)", 6.0, Triangle2d::new(Vec2::ZERO, Vec2::new(3.0, 0.0), Vec2::new(0.0, 4.0)).area());
    report::f("Triangle2d perimeter (3-4-5)", "", 12.0, Triangle2d::new(Vec2::ZERO, Vec2::new(3.0, 0.0), Vec2::new(0.0, 4.0)).perimeter());
    report::f("Annulus::new(1, 2).area()", "π(4-1)", PI * 3.0, Annulus::new(1.0, 2.0).area());
    report::f("RegularPolygon::new(1, 4).area()", "square, circumradius 1", 2.0, RegularPolygon::new(1.0, 4).area());
    report::f("CircularSector::from_turns quarter area", "r=2, quarter", PI, CircularSector::new(2.0, PI / 4.0).area());
    report::f("Capsule2d::new(1, 2).area()", "r=1, len=2", 2.0 * 1.0 * 2.0 + PI * 1.0, Capsule2d::new(1.0, 2.0).area());
}

#[test]
fn bounded_2d() {
    let id = Isometry2d::IDENTITY;
    let c = Circle::new(3.0).aabb_2d(id);
    report::approx("Circle::new(3).aabb_2d — half_size", "identity", Vec2::splat(3.0), c.half_size(), super::near2(c.half_size(), Vec2::splat(3.0)));
    let r = Rectangle::new(4.0, 2.0).aabb_2d(id);
    report::approx("Rectangle::new(4,2).aabb_2d — half_size", "identity", Vec2::new(2.0, 1.0), r.half_size(), super::near2(r.half_size(), Vec2::new(2.0, 1.0)));
    let bc = Circle::new(3.0).bounding_circle(id);
    report::f("Circle::new(3).bounding_circle — radius", "identity", 3.0, bc.radius());
    let t = Triangle2d::new(Vec2::new(-1.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 2.0)).aabb_2d(id);
    report::approx("Triangle2d.aabb_2d — half_size", "(-1,0)(1,0)(0,2)", Vec2::new(1.0, 1.0), t.half_size(), super::near2(t.half_size(), Vec2::new(1.0, 1.0)));
    report::approx("Triangle2d.aabb_2d — center", "same", Vec2::new(0.0, 1.0), t.center(), super::near2(t.center(), Vec2::new(0.0, 1.0)));
}

#[test]
fn primitive_2d_helpers() {
    report::f("Circle::diameter", "r=2.5", 5.0, Circle::new(2.5).diameter());
    report::approx("Circle::closest_point (inside -> unchanged)", "r=2, point (1,0)", Vec2::new(1.0, 0.0), Circle::new(2.0).closest_point(Vec2::new(1.0, 0.0)), super::near2(Circle::new(2.0).closest_point(Vec2::new(1.0, 0.0)), Vec2::new(1.0, 0.0)));
    report::f("RegularPolygon::circumradius", "new(2, 6)", 2.0, RegularPolygon::new(2.0, 6).circumradius());
    report::eq("RegularPolygon::sides", "new(2, 6)", 6u32, RegularPolygon::new(2.0, 6).sides);
}
