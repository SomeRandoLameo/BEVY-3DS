//! The perceptual / foundational spaces: `Laba`, `Lcha`, `Oklaba`, `Oklcha`,
//! `Xyza`. Constants, `Luminance`, `EuclideanDistance`, `Hue`, `Mix`, `Gray`.

use super::{chk, near, report, CONV_EPS, EPS};
use bevy_color::{
    Gray, Hue, Laba, Lcha, LinearRgba, Luminance, Mix, Oklaba, Oklcha, Srgba, Xyza,
};
use bevy_color::color_difference::EuclideanDistance;

#[test]
fn laba_surface() {
    report::eq("Laba::lab sets alpha 1", "", 1.0, Laba::lab(0.5, 0.1, -0.1).alpha);
    report::eq("Laba::new", "", Laba::lab(0.5, 0.1, -0.1), Laba::new(0.5, 0.1, -0.1, 1.0));
    report::eq("Laba::with_lightness", "", 0.2, Laba::lab(0.5, 0.0, 0.0).with_lightness(0.2).lightness);
    report::f("Laba::CIE_EPSILON == 216/24389", "", 216.0 / 24389.0, Laba::CIE_EPSILON);
    report::f("Laba::CIE_KAPPA == 24389/27", "", 24389.0 / 27.0, Laba::CIE_KAPPA);
    report::f("Laba::luminance == lightness", "", 0.7, Laba::lab(0.7, 0.2, 0.2).luminance());
    report::eq("Laba::gray(0.0) == BLACK", "", <Laba as Gray>::BLACK, Laba::gray(0.0));
    report::eq("Laba::gray(1.0) == WHITE", "", <Laba as Gray>::WHITE, Laba::gray(1.0));
    chk("Laba::mix @ 0.5", Laba::new(0.5, 0.0, 0.0, 1.0), Laba::BLACK.mix(&Laba::WHITE, 0.5), near(Laba::BLACK.mix(&Laba::WHITE, 0.5), Laba::new(0.5, 0.0, 0.0, 1.0), EPS));
    // red: Laba(0.532408, 0.8009243, 0.6720321) per bevy_color's table
    let lab: Laba = Srgba::RED.into();
    chk("Srgba::RED -> Laba lightness", 0.532408_f32, lab.lightness, (lab.lightness - 0.532408).abs() <= 2.0e-3);
    let back: Srgba = lab.into();
    chk("Srgba -> Laba -> Srgba (red)", Srgba::RED, back, near(back, Srgba::RED, CONV_EPS));
}

#[test]
fn lcha_surface() {
    report::eq("Lcha::lch sets alpha 1", "", 1.0, Lcha::lch(0.5, 0.1, 90.0).alpha);
    report::eq("Lcha::new", "", Lcha::lch(0.5, 0.1, 90.0), Lcha::new(0.5, 0.1, 90.0, 1.0));
    report::eq("Lcha::with_chroma", "", 0.3, Lcha::lch(0.5, 0.1, 90.0).with_chroma(0.3).chroma);
    report::eq("Lcha::with_lightness", "", 0.2, Lcha::lch(0.5, 0.1, 90.0).with_lightness(0.2).lightness);
    report::f("Lcha::luminance == lightness", "", 0.4, Lcha::lch(0.4, 0.2, 30.0).luminance());
    report::eq("Lcha::hue()/rotate_hue", "", 120.0, Lcha::lch(0.5, 0.5, 30.0).rotate_hue(90.0).hue);
    report::eq("Lcha::gray(0.0) == BLACK", "", <Lcha as Gray>::BLACK, Lcha::gray(0.0));
    let refs = [0.0_f32, 222.49225, 84.984474];
    for (i, r) in refs.into_iter().enumerate() {
        let got = Lcha::sequential_dispersed(i as u32).hue;
        chk(&format!("Lcha::sequential_dispersed({i}).hue"), r, got, (got - r).abs() <= 0.02);
    }
    // Lcha <-> Laba round-trip
    let lcha = Lcha::new(0.6, 0.4, 120.0, 1.0);
    let back: Lcha = Laba::from(lcha).into();
    chk("Lcha -> Laba -> Lcha round-trip", lcha, back, near(lcha, back, CONV_EPS));
}

#[test]
fn oklaba_surface() {
    report::eq("Oklaba::lab sets alpha 1", "", 1.0, Oklaba::lab(0.5, 0.1, -0.1).alpha);
    report::eq("Oklaba::new", "", Oklaba::lab(0.5, 0.1, -0.1), Oklaba::new(0.5, 0.1, -0.1, 1.0));
    report::eq("Oklaba::with_lightness", "", 0.2, Oklaba::lab(0.5, 0.0, 0.0).with_lightness(0.2).lightness);
    report::eq("Oklaba::with_a", "", 0.3, Oklaba::lab(0.5, 0.0, 0.0).with_a(0.3).a);
    report::eq("Oklaba::with_b", "", 0.3, Oklaba::lab(0.5, 0.0, 0.0).with_b(0.3).b);
    report::eq("Oklaba::gray(0.0) == BLACK", "", <Oklaba as Gray>::BLACK, Oklaba::gray(0.0));
    report::f("Oklaba::distance (BLACK<->WHITE)", "= L difference", 1.0, Oklaba::BLACK.distance(&Oklaba::WHITE));
    chk("Oklaba::mix @ 0.5", Oklaba::gray(0.5), Oklaba::BLACK.mix(&Oklaba::WHITE, 0.5), near(Oklaba::BLACK.mix(&Oklaba::WHITE, 0.5), Oklaba::gray(0.5), EPS));
    // red: Oklaba(0.6279554, 0.22486295, 0.1258463) per bevy_color's table
    let ok: Oklaba = LinearRgba::RED.into();
    chk("LinearRgba::RED -> Oklaba", Oklaba::new(0.6279554, 0.22486295, 0.1258463, 1.0), ok, near(ok, Oklaba::new(0.6279554, 0.22486295, 0.1258463, 1.0), 2.0e-3));
    let back: LinearRgba = ok.into();
    chk("LinearRgba -> Oklaba -> LinearRgba (red)", LinearRgba::RED, back, near(back, LinearRgba::RED, CONV_EPS));
}

#[test]
fn oklcha_surface() {
    report::eq("Oklcha::lch sets alpha 1", "", 1.0, Oklcha::lch(0.5, 0.1, 90.0).alpha);
    report::eq("Oklcha::new", "", Oklcha::lch(0.5, 0.1, 90.0), Oklcha::new(0.5, 0.1, 90.0, 1.0));
    report::eq("Oklcha::with_lightness", "", 0.2, Oklcha::lch(0.5, 0.1, 90.0).with_lightness(0.2).lightness);
    report::eq("Oklcha::with_chroma", "", 0.3, Oklcha::lch(0.5, 0.1, 90.0).with_chroma(0.3).chroma);
    report::eq("Oklcha::hue()/rotate_hue", "", 120.0, Oklcha::lch(0.5, 0.2, 30.0).rotate_hue(90.0).hue);
    report::eq("Oklcha::gray(0.0) == BLACK", "", <Oklcha as Gray>::BLACK, Oklcha::gray(0.0));
    report::is_true("Oklcha::distance (RED vs BLUE) > 0", "", Oklcha::from(Srgba::RED).distance(&Oklcha::from(Srgba::BLUE)) > 0.1);
    // Oklcha <-> Oklaba round-trip
    let oklcha = Oklcha::new(0.7, 0.2, 200.0, 1.0);
    let back: Oklcha = Oklaba::from(oklcha).into();
    chk("Oklcha -> Oklaba -> Oklcha round-trip", oklcha, back, near(oklcha, back, CONV_EPS));
}

#[test]
fn xyza_surface() {
    report::eq("Xyza::xyz sets alpha 1", "", 1.0, Xyza::xyz(0.3, 0.4, 0.5).alpha);
    report::eq("Xyza::new", "", Xyza::xyz(0.3, 0.4, 0.5), Xyza::new(0.3, 0.4, 0.5, 1.0));
    report::eq("Xyza::with_x", "", 0.9, Xyza::xyz(0.0, 0.0, 0.0).with_x(0.9).x);
    report::eq("Xyza::with_y", "", 0.9, Xyza::xyz(0.0, 0.0, 0.0).with_y(0.9).y);
    report::eq("Xyza::with_z", "", 0.9, Xyza::xyz(0.0, 0.0, 0.0).with_z(0.9).z);
    report::eq("Xyza::D65_WHITE", "", Xyza::xyz(0.95047, 1.0, 1.08883), Xyza::D65_WHITE);
    report::f("Xyza::luminance == y", "", 0.42, Xyza::new(0.3, 0.42, 0.5, 1.0).luminance());
    report::eq("Xyza::gray(0.0) == BLACK", "", <Xyza as Gray>::BLACK, Xyza::gray(0.0));
    chk("Xyza::mix @ 0.5", Xyza::gray(0.5), Xyza::BLACK.mix(&Xyza::WHITE, 0.5), near(Xyza::BLACK.mix(&Xyza::WHITE, 0.5), Xyza::gray(0.5), EPS));
    // LinearRgba::WHITE maps to the D65 white point.
    let xyz: Xyza = LinearRgba::WHITE.into();
    chk("LinearRgba::WHITE -> Xyza ~ D65_WHITE", Xyza::D65_WHITE, xyz, near(xyz, Xyza::D65_WHITE, 2.0e-3));
    let back: LinearRgba = xyz.into();
    chk("LinearRgba -> Xyza -> LinearRgba (white)", LinearRgba::WHITE, back, near(back, LinearRgba::WHITE, CONV_EPS));
}
