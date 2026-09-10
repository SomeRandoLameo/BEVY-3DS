//! `bevy_math::sampling` (`rand` feature): `ShapeSample`, `FromRng`.
//! No `OsRng` on the 3DS (`getrandom` has no backend), so a deterministic
//! `Xorshift` `RngCore` is used.

use super::{report, Xorshift};
use bevy_math::primitives::{Circle, Cuboid, Rectangle, Sphere};
use bevy_math::{ShapeSample, Vec2, Vec3};

#[test]
fn shape_sample_2d() {
    let mut rng = Xorshift(0x1234_5678_9abc_def0);
    let rect = Rectangle::new(2.0, 4.0);
    let mut ok_interior = true;
    for _ in 0..500 {
        let p = rect.sample_interior(&mut rng);
        ok_interior &= p.x.abs() <= 1.0 + 1e-4 && p.y.abs() <= 2.0 + 1e-4;
    }
    report::is_true("Rectangle::sample_interior — 500 points inside [-1,1]x[-2,2]", "seed 0x1234…", ok_interior);

    let mut ok_boundary = true;
    for _ in 0..500 {
        let p = rect.sample_boundary(&mut rng);
        let on_edge = (p.x.abs() - 1.0).abs() < 1e-3 || (p.y.abs() - 2.0).abs() < 1e-3;
        ok_boundary &= on_edge && p.x.abs() <= 1.0 + 1e-3 && p.y.abs() <= 2.0 + 1e-3;
    }
    report::is_true("Rectangle::sample_boundary — 500 points on the perimeter", "", ok_boundary);

    let circle = Circle::new(3.0);
    let mut ok_circle = true;
    for _ in 0..500 {
        ok_circle &= circle.sample_interior(&mut rng).length() <= 3.0 + 1e-3;
    }
    report::is_true("Circle::sample_interior — 500 points within radius 3", "", ok_circle);
    let mut ok_circ_b = true;
    for _ in 0..500 {
        ok_circ_b &= (circle.sample_boundary(&mut rng).length() - 3.0).abs() < 1e-3;
    }
    report::is_true("Circle::sample_boundary — 500 points on radius 3", "", ok_circ_b);
}

#[test]
fn shape_sample_3d() {
    let mut rng = Xorshift(0xdead_beef_cafe_babe);
    let cuboid = Cuboid::new(2.0, 4.0, 6.0);
    let mut ok = true;
    for _ in 0..500 {
        let p = cuboid.sample_interior(&mut rng);
        ok &= p.x.abs() <= 1.0 + 1e-4 && p.y.abs() <= 2.0 + 1e-4 && p.z.abs() <= 3.0 + 1e-4;
    }
    report::is_true("Cuboid::sample_interior — 500 points inside half-extents (1,2,3)", "", ok);

    let sphere = Sphere::new(5.0);
    let mut ok_s = true;
    for _ in 0..500 {
        ok_s &= sphere.sample_interior(&mut rng).length() <= 5.0 + 1e-3;
    }
    report::is_true("Sphere::sample_interior — 500 points within radius 5", "", ok_s);
    let mut ok_sb = true;
    for _ in 0..500 {
        ok_sb &= (sphere.sample_boundary(&mut rng).length() - 5.0).abs() < 1e-2;
    }
    report::is_true("Sphere::sample_boundary — 500 points on radius 5", "", ok_sb);
}

#[test]
fn sampling_is_deterministic() {
    let mut a = Xorshift(42);
    let mut b = Xorshift(42);
    let s = Sphere::new(1.0);
    let pa: Vec3 = s.sample_interior(&mut a);
    let pb: Vec3 = s.sample_interior(&mut b);
    report::eq("Same seed -> same sample (Sphere::sample_interior)", "seed 42", pa, pb);

    let r = Rectangle::new(2.0, 2.0);
    let mut c = Xorshift(7);
    let mut d = Xorshift(7);
    let ca: Vec2 = r.sample_boundary(&mut c);
    let da: Vec2 = r.sample_boundary(&mut d);
    report::eq("Same seed -> same sample (Rectangle::sample_boundary)", "seed 7", ca, da);
}
