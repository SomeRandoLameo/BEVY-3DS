//! `bevy_math` rectangles + isometries + rays.

use super::{near2, near3, report};
use bevy_math::{IRect, IVec2, Isometry2d, Isometry3d, Quat, Ray2d, Ray3d, Rect, Rot2, URect, Vec2, Vec3};
use core::f32::consts::FRAC_PI_2;

#[test]
fn rect_family() {
    let r = Rect::new(0.0, 0.0, 4.0, 2.0);
    report::f("Rect::width", "(0,0)-(4,2)", 4.0, r.width());
    report::f("Rect::height", "(0,0)-(4,2)", 2.0, r.height());
    report::f("Rect::area", "(0,0)-(4,2)", 8.0, r.area());
    report::eq("Rect::center", "(0,0)-(4,2)", Vec2::new(2.0, 1.0), r.center());
    report::eq("Rect::size", "(0,0)-(4,2)", Vec2::new(4.0, 2.0), r.size());
    report::is_true("Rect::contains", "(0,0)-(4,2) has (1,1)", r.contains(Vec2::new(1.0, 1.0)));
    report::eq("Rect::from_center_size", "c=(0,0), s=(2,2)", Rect::new(-1.0, -1.0, 1.0, 1.0), Rect::from_center_size(Vec2::ZERO, Vec2::splat(2.0)));
    report::eq("Rect::union_point", "(0,0)-(1,1) ∪ (3,3)", Rect::new(0.0, 0.0, 3.0, 3.0), Rect::new(0.0, 0.0, 1.0, 1.0).union_point(Vec2::splat(3.0)));
    report::eq("Rect::intersect", "(0,0)-(2,2) ∩ (1,1)-(3,3)", Rect::new(1.0, 1.0, 2.0, 2.0), Rect::new(0.0, 0.0, 2.0, 2.0).intersect(Rect::new(1.0, 1.0, 3.0, 3.0)));
    report::eq("Rect::inflate", "(0,0)-(2,2) by 1", Rect::new(-1.0, -1.0, 3.0, 3.0), Rect::new(0.0, 0.0, 2.0, 2.0).inflate(1.0));
    report::is_true("Rect::is_empty", "(1,1)-(1,1)", Rect::new(1.0, 1.0, 1.0, 1.0).is_empty());
    report::eq("Rect::as_irect", "(0.2,0.9)-(3.7,4.1)", IRect::new(0, 0, 3, 4), Rect::new(0.2, 0.9, 3.7, 4.1).as_irect());
}

#[test]
fn irect_urect() {
    report::eq("IRect::from_corners", "(3,1),(0,4)", IRect::new(0, 1, 3, 4), IRect::from_corners(IVec2::new(3, 1), IVec2::new(0, 4)));
    report::eq("IRect::width/height", "(0,0)-(5,3)", (5i32, 3i32), (IRect::new(0, 0, 5, 3).width(), IRect::new(0, 0, 5, 3).height()));
    report::eq("URect::center", "(0,0)-(4,4)", bevy_math::UVec2::new(2, 2), URect::new(0, 0, 4, 4).center());
}

#[test]
fn isometries_and_rays() {
    let iso = Isometry2d::new(Vec2::new(1.0, 2.0), Rot2::degrees(90.0));
    report::approx("Isometry2d::transform_point", "T=(1,2), R=90°, point X", Vec2::new(1.0, 3.0), iso.transform_point(Vec2::X), near2(iso.transform_point(Vec2::X), Vec2::new(1.0, 3.0)));
    report::approx("Isometry2d::inverse_transform_point round-trip", "", Vec2::X, iso.inverse_transform_point(iso.transform_point(Vec2::X)), near2(iso.inverse_transform_point(iso.transform_point(Vec2::X)), Vec2::X));
    report::approx("Isometry2d * Isometry2d compose", "", iso.transform_point(iso.transform_point(Vec2::X)), (iso * iso).transform_point(Vec2::X), near2((iso * iso).transform_point(Vec2::X), iso.transform_point(iso.transform_point(Vec2::X))));

    let iso3 = Isometry3d::new(Vec3::new(1.0, 2.0, 3.0), Quat::from_rotation_z(FRAC_PI_2));
    let p: Vec3 = iso3.transform_point(Vec3::X).into();
    report::approx("Isometry3d::transform_point", "T=(1,2,3), Rz=PI/2, point X", Vec3::new(1.0, 3.0, 3.0), p, near3(p, Vec3::new(1.0, 3.0, 3.0)));
    let back: Vec3 = iso3.inverse().transform_point(iso3.transform_point(Vec3::X)).into();
    report::approx("Isometry3d::inverse round-trip", "", Vec3::X, back, near3(back, Vec3::X));

    report::eq("Ray2d::get_point", "origin 0, dir Y, d=3", Vec2::new(0.0, 3.0), Ray2d::new(Vec2::ZERO, bevy_math::Dir2::Y).get_point(3.0));
    report::eq("Ray3d::get_point", "origin 0, dir X, d=2.5", Vec3::new(2.5, 0.0, 0.0), Ray3d::new(Vec3::ZERO, bevy_math::Dir3::X).get_point(2.5));
}
