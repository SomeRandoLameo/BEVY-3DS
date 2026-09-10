//! `compass` (`CompassQuadrant`, `CompassOctant`), `FloatOrd`, `AspectRatio`.

use super::report;
use bevy_math::{AspectRatio, CompassOctant, CompassQuadrant, Dir2, FloatOrd, Vec2};
use core::cmp::Ordering;

#[test]
fn compass() {
    report::eq("Dir2 -> CompassQuadrant (North)", "Dir2::Y", CompassQuadrant::North, CompassQuadrant::from(Dir2::Y));
    report::eq("Dir2 -> CompassQuadrant (East)", "Dir2::X", CompassQuadrant::East, CompassQuadrant::from(Dir2::X));
    report::eq("CompassQuadrant::opposite", "North", CompassQuadrant::South, CompassQuadrant::North.opposite());
    report::approx("CompassQuadrant -> Dir2 (North)", "", Vec2::Y, Dir2::from(CompassQuadrant::North).as_vec2(), super::near2(Dir2::from(CompassQuadrant::North).as_vec2(), Vec2::Y));
    report::is_true("CompassQuadrant::is_in_direction", "East, origin 0, candidate (5,0.1)", CompassQuadrant::East.is_in_direction(Vec2::ZERO, Vec2::new(5.0, 0.1)));

    report::eq("Dir2 -> CompassOctant (NorthEast)", "Dir2::from_xy(1,1)", CompassOctant::NorthEast, CompassOctant::from(Dir2::new(Vec2::ONE).unwrap()));
    report::eq("CompassOctant::opposite", "NorthEast", CompassOctant::SouthWest, CompassOctant::NorthEast.opposite());
    report::approx("CompassOctant -> Dir2 (East)", "", Vec2::X, Dir2::from(CompassOctant::East).as_vec2(), super::near2(Dir2::from(CompassOctant::East).as_vec2(), Vec2::X));
}

#[test]
fn float_ord() {
    report::eq("FloatOrd::cmp", "1.0 vs 2.0", Ordering::Less, FloatOrd(1.0).cmp(&FloatOrd(2.0)));
    report::eq("FloatOrd::cmp — NaN sorts as least", "NaN vs 1e9", Ordering::Less, FloatOrd(f32::NAN).cmp(&FloatOrd(1e9)));
    report::eq("FloatOrd::cmp — NaN == NaN", "", Ordering::Equal, FloatOrd(f32::NAN).cmp(&FloatOrd(f32::NAN)));
    report::eq("FloatOrd == FloatOrd (NaN)", "", true, FloatOrd(f32::NAN) == FloatOrd(f32::NAN));
    let mut v = [FloatOrd(3.0), FloatOrd(f32::NAN), FloatOrd(-1.0), FloatOrd(0.5)];
    v.sort();
    report::is_true("sort [3, NaN, -1, 0.5] — NaN first", "", v[0].0.is_nan());
    report::eq("...then [-1, 0.5, 3]", "", vec![-1.0_f32, 0.5, 3.0], vec![v[1].0, v[2].0, v[3].0]);
    report::eq("FloatOrd::neg", "-FloatOrd(2.5)", FloatOrd(-2.5), -FloatOrd(2.5));
}

#[test]
fn aspect_ratio() {
    report::f("AspectRatio::try_new(16, 9).ratio()", "", 16.0 / 9.0, AspectRatio::try_new(16.0, 9.0).unwrap().ratio());
    report::f("AspectRatio::try_from_pixels(1920, 1080).ratio()", "", 16.0 / 9.0, AspectRatio::try_from_pixels(1920, 1080).unwrap().ratio());
    report::f("AspectRatio::inverse", "16:9 -> 9:16", 9.0 / 16.0, AspectRatio::SIXTEEN_NINE.inverse().ratio());
    report::is_true("AspectRatio::is_landscape", "16:9", AspectRatio::SIXTEEN_NINE.is_landscape());
    report::is_true("AspectRatio::is_portrait", "9:16", AspectRatio::SIXTEEN_NINE.inverse().is_portrait());
    report::is_true("AspectRatio::is_square", "try_new(5,5)", AspectRatio::try_new(5.0, 5.0).unwrap().is_square());
    report::ok("AspectRatio::try_new(1, 0)", "zero height", "Err", if AspectRatio::try_new(1.0, 0.0).is_err() { "Err" } else { "Ok" }, AspectRatio::try_new(1.0, 0.0).is_err());
}
