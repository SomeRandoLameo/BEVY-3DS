//! The hue-based spaces: `Hsla`, `Hsva`, `Hwba`. Hue/Saturation traits, the
//! shortest-path hue interpolation in `Mix`, `Gray`, and `sequential_dispersed`.

use super::{chk, near, near_lin, report, CONV_EPS};
use bevy_color::{Gray, Hsla, Hsva, Hue, Hwba, Luminance, Mix, Saturation, Srgba};

#[test]
fn hsla_constructors_and_accessors() {
    report::eq("Hsla::hsl sets alpha 1", "", 1.0, Hsla::hsl(120.0, 0.5, 0.5).alpha);
    report::eq("Hsla::new", "", Hsla::hsl(120.0, 0.5, 0.5), Hsla::new(120.0, 0.5, 0.5, 1.0));
    report::eq("Hsla::with_saturation", "", 0.25, Hsla::hsl(0.0, 1.0, 0.5).with_saturation(0.25).saturation);
    report::eq("Hsla::with_lightness", "", 0.25, Hsla::hsl(0.0, 1.0, 0.5).with_lightness(0.25).lightness);
    report::eq("Hsla::default() (white: lightness 1)", "", Hsla::new(0.0, 0.0, 1.0, 1.0), Hsla::default());
}

#[test]
fn hsla_hue_and_saturation_traits() {
    report::eq("Hsla::hue()", "hsl(180,1,.5)", 180.0, Hsla::hsl(180.0, 1.0, 0.5).hue());
    report::eq("Hsla::with_hue", "", 270.0, Hsla::hsl(180.0, 1.0, 0.5).with_hue(270.0).hue);
    report::eq("Hsla::rotate_hue(+90)", "", Hsla::hsl(270.0, 1.0, 0.5), Hsla::hsl(180.0, 1.0, 0.5).rotate_hue(90.0));
    report::eq("Hsla::rotate_hue(-90) wraps via rem_euclid", "", Hsla::hsl(90.0, 1.0, 0.5), Hsla::hsl(180.0, 1.0, 0.5).rotate_hue(-90.0));
    report::eq("Hsla::rotate_hue(180)", "", Hsla::hsl(0.0, 1.0, 0.5), Hsla::hsl(180.0, 1.0, 0.5).rotate_hue(180.0));
    report::eq("Hsla::rotate_hue(360) is identity", "", Hsla::hsl(180.0, 1.0, 0.5), Hsla::hsl(180.0, 1.0, 0.5).rotate_hue(360.0));
    let mut h = Hsla::hsl(10.0, 1.0, 0.5);
    h.set_hue(200.0);
    report::eq("Hsla::set_hue", "", 200.0, h.hue);
    report::eq("Hsla::saturation()", "", 0.3, Hsla::hsl(0.0, 0.3, 0.5).saturation());
    let mut s = Hsla::hsl(0.0, 0.3, 0.5);
    s.set_saturation(0.9);
    report::eq("Hsla::set_saturation", "", 0.9, s.saturation);
}

#[test]
fn hsla_mix_takes_shortest_hue_path() {
    // From bevy_color's own `test_mix_wrap`: 10° and 350° interpolate through 0°.
    let a = Hsla::new(10.0, 0.5, 0.5, 1.0);
    let b = Hsla::new(350.0, 0.5, 0.5, 1.0);
    report::f("Hsla::mix hue 10->350 @ 0.25", "", 5.0, a.mix(&b, 0.25).hue);
    report::f("Hsla::mix hue 10->350 @ 0.5 (through 0)", "", 0.0, a.mix(&b, 0.5).hue);
    report::f("Hsla::mix hue 10->350 @ 0.75", "", 355.0, a.mix(&b, 0.75).hue);
    report::f("Hsla::mix hue 350->10 @ 0.5", "", 0.0, b.mix(&a, 0.5).hue);
    let mut m = Hsla::new(10.0, 0.5, 0.5, 1.0);
    m.mix_assign(Hsla::new(20.0, 0.5, 0.5, 1.0), 0.5);
    report::f("Hsla::mix_assign hue 10->20 @ 0.5", "", 15.0, m.hue);
}

#[test]
fn hsla_luminance_gray_and_dispersed() {
    report::f("Hsla::luminance == lightness", "", 0.5, Hsla::hsl(0.0, 0.0, 0.5).luminance());
    report::eq("Hsla::darker clamps at 0", "", 0.0, Hsla::hsl(0.0, 0.0, 0.2).darker(0.5).lightness);
    report::eq("Hsla::lighter clamps at 1", "", 1.0, Hsla::hsl(0.0, 0.0, 0.8).lighter(0.5).lightness);
    report::eq("Hsla::gray(0.0) == BLACK", "", <Hsla as Gray>::BLACK, Hsla::gray(0.0));
    report::eq("Hsla::gray(1.0) == WHITE", "", <Hsla as Gray>::WHITE, Hsla::gray(1.0));
    // `sequential_dispersed` golden-angle hue sequence (bevy_color `test_from_index`).
    let refs = [0.0_f32, 222.49225, 84.984474, 307.4767, 169.96895];
    for (i, r) in refs.into_iter().enumerate() {
        let got = Hsla::sequential_dispersed(i as u32).hue;
        chk(&format!("Hsla::sequential_dispersed({i}).hue"), r, got, (got - r).abs() <= 0.02);
    }
}

#[test]
fn hsva_surface() {
    report::eq("Hsva::hsv sets alpha 1", "", 1.0, Hsva::hsv(120.0, 0.5, 0.5).alpha);
    report::eq("Hsva::new", "", Hsva::hsv(120.0, 0.5, 0.5), Hsva::new(120.0, 0.5, 0.5, 1.0));
    report::eq("Hsva::with_saturation", "", 0.2, Hsva::hsv(0.0, 1.0, 1.0).with_saturation(0.2).saturation);
    report::eq("Hsva::with_value", "", 0.2, Hsva::hsv(0.0, 1.0, 1.0).with_value(0.2).value);
    report::eq("Hsva::hue()/rotate_hue", "", Hsva::hsv(60.0, 1.0, 1.0), Hsva::hsv(300.0, 1.0, 1.0).rotate_hue(120.0));
    report::f("Hsva::saturation() trait", "", 0.4, Hsva::hsv(0.0, 0.4, 1.0).saturation());
    report::eq("Hsva::gray(0.0) == BLACK", "", <Hsva as Gray>::BLACK, Hsva::gray(0.0));
    // Hsla <-> Hsva round-trip (a well-defined mid color).
    let hsva = Hsva::new(180.0, 0.5, 0.5, 1.0);
    let back: Hsva = Hsla::from(hsva).into();
    chk("Hsva -> Hsla -> Hsva round-trip", hsva, back, near(hsva, back, CONV_EPS));
}

#[test]
fn hwba_surface() {
    report::eq("Hwba::hwb sets alpha 1", "", 1.0, Hwba::hwb(120.0, 0.2, 0.3).alpha);
    report::eq("Hwba::new", "", Hwba::hwb(120.0, 0.2, 0.3), Hwba::new(120.0, 0.2, 0.3, 1.0));
    report::eq("Hwba::with_whiteness", "", 0.7, Hwba::hwb(0.0, 0.0, 0.0).with_whiteness(0.7).whiteness);
    report::eq("Hwba::with_blackness", "", 0.7, Hwba::hwb(0.0, 0.0, 0.0).with_blackness(0.7).blackness);
    report::eq("Hwba::hue()/rotate_hue", "", Hwba::hwb(60.0, 0.0, 0.0), Hwba::hwb(300.0, 0.0, 0.0).rotate_hue(120.0));
    report::eq("Hwba::gray(0.0) == BLACK", "", <Hwba as Gray>::BLACK, Hwba::gray(0.0));
    // pure red is hue 0, whiteness 0, blackness 0
    let hwb: Hwba = Srgba::RED.into();
    report::f("Srgba::RED -> Hwba whiteness", "", 0.0, hwb.whiteness);
    report::f("Srgba::RED -> Hwba blackness", "", 0.0, hwb.blackness);
    // Hsva <-> Hwba round-trip
    let hwba = Hwba::new(200.0, 0.2, 0.3, 1.0);
    let back: Hwba = Hsva::from(hwba).into();
    chk("Hwba -> Hsva -> Hwba round-trip", hwba, back, near(hwba, back, CONV_EPS));
}

#[test]
fn cylindrical_spaces_roundtrip_srgba() {
    // A mid gray-blue that is unambiguous in every space.
    let base = Srgba::new(0.25, 0.4, 0.7, 1.0);
    let via_hsla: Srgba = Hsla::from(base).into();
    let via_hsva: Srgba = Hsva::from(base).into();
    let via_hwba: Srgba = Hwba::from(base).into();
    chk("Srgba -> Hsla -> Srgba", base, via_hsla, near_lin(base, via_hsla, CONV_EPS));
    chk("Srgba -> Hsva -> Srgba", base, via_hsva, near_lin(base, via_hsva, CONV_EPS));
    chk("Srgba -> Hwba -> Srgba", base, via_hwba, near_lin(base, via_hwba, CONV_EPS));
}
