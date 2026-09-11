//! `Color` — the type-erased enum every Bevy render API actually takes.
//! Constructors for all 10 variants, `Alpha`/`Luminance`/`Hue`/`Saturation`/
//! `Mix` delegating to (and, when needed, through `Oklcha`), and
//! `TryStableInterpolate`'s mismatched-unit error.

use super::{chk, report};
use bevy_color::{Alpha, Color, Hsla, Hue, Luminance, Mix, Saturation, Srgba};
use bevy_math::TryStableInterpolate;

#[test]
fn constructors_agree_with_their_concrete_type() {
    chk("Color::srgb == Srgba::rgb", Color::Srgba(Srgba::rgb(0.1, 0.2, 0.3)), Color::srgb(0.1, 0.2, 0.3), Color::srgb(0.1, 0.2, 0.3) == Color::Srgba(Srgba::rgb(0.1, 0.2, 0.3)));
    report::eq("Color::srgba", "", Color::Srgba(Srgba::new(0.1, 0.2, 0.3, 0.4)), Color::srgba(0.1, 0.2, 0.3, 0.4));
    report::eq("Color::srgb_from_array", "", Color::srgb(0.1, 0.2, 0.3), Color::srgb_from_array([0.1, 0.2, 0.3]));
    report::eq("Color::srgb_u8", "", Color::srgb(1.0, 0.0, 0.0), Color::srgb_u8(255, 0, 0));
    report::eq("Color::srgba_u8", "", Color::srgba(1.0, 0.0, 0.0, 0.0), Color::srgba_u8(255, 0, 0, 0));
    report::eq("Color::srgb_u32(0xff0000) -> red", "", Color::srgb(1.0, 0.0, 0.0), Color::srgb_u32(0xff0000));
    report::eq("Color::srgba_u32(0xff000080)", "", Color::srgba_u8(255, 0, 0, 0x80), Color::srgba_u32(0xff00_0080));
    report::eq("Color::linear_rgb", "", Color::linear_rgba(0.1, 0.2, 0.3, 1.0), Color::linear_rgb(0.1, 0.2, 0.3));
    report::eq("Color::hsl", "", Color::hsla(120.0, 1.0, 0.5, 1.0), Color::hsl(120.0, 1.0, 0.5));
    report::eq("Color::hsv", "", Color::hsva(120.0, 1.0, 0.5, 1.0), Color::hsv(120.0, 1.0, 0.5));
    report::eq("Color::hwb", "", Color::hwba(120.0, 0.1, 0.2, 1.0), Color::hwb(120.0, 0.1, 0.2));
    report::eq("Color::lab", "", Color::laba(0.5, 0.1, -0.1, 1.0), Color::lab(0.5, 0.1, -0.1));
    report::eq("Color::lch", "", Color::lcha(0.5, 0.1, 90.0, 1.0), Color::lch(0.5, 0.1, 90.0));
    report::eq("Color::oklab", "", Color::oklaba(0.5, 0.1, -0.1, 1.0), Color::oklab(0.5, 0.1, -0.1));
    report::eq("Color::oklch", "", Color::oklcha(0.5, 0.1, 90.0, 1.0), Color::oklch(0.5, 0.1, 90.0));
    report::eq("Color::xyz", "", Color::xyza(0.3, 0.4, 0.5, 1.0), Color::xyz(0.3, 0.4, 0.5));
}

#[test]
fn constants_and_conversions() {
    report::eq("Color::WHITE is LinearRgba", "", Color::linear_rgb(1.0, 1.0, 1.0), Color::WHITE);
    report::eq("Color::BLACK is LinearRgba", "", Color::linear_rgb(0.0, 0.0, 0.0), Color::BLACK);
    report::eq("Color::default() == WHITE", "", Color::WHITE, Color::default());
    report::eq("Color::to_srgba", "srgb(1,0,0)", Srgba::RED, Color::srgb(1.0, 0.0, 0.0).to_srgba());
    report::eq("Color::to_linear (WHITE)", "", bevy_color::LinearRgba::WHITE, Color::WHITE.to_linear());
    report::eq("From<Srgba> for Color (derive_more)", "", Color::Srgba(Srgba::RED), Color::from(Srgba::RED));
    report::eq("From<Hsla> for Color", "", Color::Hsla(Hsla::hsl(120.0, 1.0, 0.5)), Color::from(Hsla::hsl(120.0, 1.0, 0.5)));
    report::eq("Hsla::from(Color) round-trips the variant", "", Hsla::hsl(120.0, 1.0, 0.5), Hsla::from(Color::hsl(120.0, 1.0, 0.5)));
}

#[test]
fn op_traits_delegate() {
    report::f("Color::alpha()", "srgba(_,_,_,0.4)", 0.4, Color::srgba(0.1, 0.2, 0.3, 0.4).alpha());
    report::f("Color::with_alpha().alpha()", "", 0.7, Color::srgb(1.0, 0.0, 0.0).with_alpha(0.7).alpha());
    let mut c = Color::WHITE;
    c.set_alpha(0.5);
    report::f("Color::set_alpha", "", 0.5, c.alpha());
    report::f("Color::luminance (WHITE)", "", 1.0, Color::WHITE.luminance());
    report::eq("Color::hue (native, Hsla)", "hsl(120,..)", 120.0, Color::hsl(120.0, 1.0, 0.5).hue());
    report::is_true("Color::hue (no native def, Srgba) is finite", "converts via Oklch", Color::srgb(1.0, 0.0, 0.0).hue().is_finite());
    report::f("Color::saturation (native, Hsla)", "", 0.5, Color::hsl(0.0, 0.5, 0.5).saturation());
    chk("Color::mix(srgb RED, srgb BLUE, 0.5)", Color::srgb(0.5, 0.0, 0.5), Color::srgb(1.0, 0.0, 0.0).mix(&Color::srgb(0.0, 0.0, 1.0), 0.5), matches!(Color::srgb(1.0, 0.0, 0.0).mix(&Color::srgb(0.0, 0.0, 1.0), 0.5), Color::Srgba(s) if (s.red - 0.5).abs() < 1e-4 && (s.blue - 0.5).abs() < 1e-4));
}

#[test]
fn try_stable_interpolate() {
    let a = Color::srgb(0.0, 0.0, 0.0);
    let b = Color::srgb(1.0, 1.0, 1.0);
    report::is_true("Color::try_interpolate_stable (same variant) is Ok", "", a.try_interpolate_stable(&b, 0.5).is_ok());
    let c = Color::hsl(0.0, 0.0, 1.0);
    report::is_true("Color::try_interpolate_stable (mismatched variants) is Err", "Srgba vs Hsla", a.try_interpolate_stable(&c, 0.5).is_err());
}
