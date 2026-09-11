//! The conversion graph. Driven by `super::TABLE` (a slice of colors with their
//! coordinates in every space, taken from `bevy_color`'s own generated
//! `test_colors` table). Every `From` impl on the 3DS newlib math path.

use super::{chk, near, near_lin, CONV_EPS, TABLE};
use bevy_color::{Hsla, Hsva, Hwba, Laba, Lcha, LinearRgba, Oklaba, Oklcha, Srgba, Xyza};

#[test]
fn srgba_to_linear_matches_table() {
    for t in TABLE {
        let got: LinearRgba = t.rgb.into();
        chk(&format!("Srgba -> LinearRgba ({})", t.name), t.lin, got, near(t.lin, got, CONV_EPS));
    }
}

#[test]
fn linear_to_srgba_matches_table() {
    for t in TABLE {
        let got: Srgba = t.lin.into();
        chk(&format!("LinearRgba -> Srgba ({})", t.name), t.rgb, got, near(t.rgb, got, CONV_EPS));
    }
}

#[test]
fn srgba_to_hsla_matches_table() {
    for t in TABLE {
        let got: Hsla = t.rgb.into();
        // hue is meaningless for black/white/gray (saturation 0) — check via round-trip there.
        let ok = if t.hsl.saturation == 0.0 {
            near_lin(got, t.rgb, CONV_EPS)
        } else {
            (got.hue - t.hsl.hue).abs() <= 0.05
                && (got.saturation - t.hsl.saturation).abs() <= CONV_EPS
                && (got.lightness - t.hsl.lightness).abs() <= CONV_EPS
        };
        chk(&format!("Srgba -> Hsla ({})", t.name), t.hsl, got, ok);
    }
}

#[test]
fn srgba_to_oklaba_matches_table() {
    for t in TABLE {
        let got: Oklaba = t.rgb.into();
        chk(&format!("Srgba -> Oklaba ({})", t.name), t.oklab, got, near(t.oklab, got, 2.0e-3));
    }
}

#[test]
fn srgba_to_xyza_matches_table() {
    for t in TABLE {
        let got: Xyza = t.rgb.into();
        chk(&format!("Srgba -> Xyza ({})", t.name), t.xyz, got, near(t.xyz, got, 2.0e-3));
    }
}

/// Every space: `Srgba -> space -> LinearRgba` must land back on the table's
/// linear value. Exercises both directions of all 9 `From` impls at once.
#[test]
fn every_space_round_trips_through_linear() {
    for t in TABLE {
        macro_rules! rt {
            ($S:ty, $name:literal) => {{
                let s: $S = t.rgb.into();
                let back: LinearRgba = s.into();
                chk(
                    concat!("Srgba -> ", $name, " -> LinearRgba"),
                    t.lin,
                    back,
                    near(t.lin, back, CONV_EPS),
                );
            }};
        }
        rt!(Hsla, "Hsla");
        rt!(Hsva, "Hsva");
        rt!(Hwba, "Hwba");
        rt!(Laba, "Laba");
        rt!(Lcha, "Lcha");
        rt!(Oklaba, "Oklaba");
        rt!(Oklcha, "Oklcha");
        rt!(Xyza, "Xyza");
    }
}

/// Spot-check a couple of the "derived" conversions that route through an
/// intermediate space (e.g. `Laba <-> Oklaba`, `Lcha <-> Xyza`).
#[test]
fn derived_conversions() {
    let mid = Srgba::new(0.3, 0.6, 0.2, 1.0);

    let laba: Laba = mid.into();
    let laba_rt: Laba = Oklaba::from(laba).into();
    chk("Laba -> Oklaba -> Laba", laba, laba_rt, near(laba, laba_rt, CONV_EPS));

    let lcha: Lcha = mid.into();
    let lcha_rt: Lcha = Xyza::from(lcha).into();
    chk("Lcha -> Xyza -> Lcha", lcha, lcha_rt, near(lcha, lcha_rt, CONV_EPS));

    let hwba: Hwba = mid.into();
    let hwba_rt: Hwba = Lcha::from(hwba).into();
    chk("Hwba -> Lcha -> Hwba", hwba, hwba_rt, near(hwba, hwba_rt, CONV_EPS));
}
