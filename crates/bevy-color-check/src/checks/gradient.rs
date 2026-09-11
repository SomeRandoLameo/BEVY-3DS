//! `ColorCurve` — a `bevy_math::curve::Curve` whose samples are a sequence of
//! colors, `EvenCore`-backed. Needs the `alloc` feature (on via `std`).

use super::report;
use bevy_color::palettes::basic::{BLUE, LIME, RED, WHITE};
use bevy_color::{ColorCurve, Mix, Srgba};
use bevy_math::curve::{Curve, CurveExt, Interval};

#[test]
fn construction_needs_at_least_two_colors() {
    report::is_true("ColorCurve::new([RED]) is Err (needs >= 2 colors)", "", ColorCurve::new([RED]).is_err());
    report::is_true("ColorCurve::new([RED, BLUE]) is Ok", "", ColorCurve::new([RED, BLUE]).is_ok());
}

#[test]
fn domain_and_sampling() {
    let gradient = [RED, LIME, BLUE];
    let curve = ColorCurve::new(gradient).unwrap();
    report::eq("ColorCurve::domain == [0, len-1]", "3 colors", Interval::new(0.0, 2.0).unwrap(), curve.domain());
    report::eq("ColorCurve::sample_clamped(0.0)", "", RED, curve.sample_clamped(0.0));
    report::eq("ColorCurve::sample_clamped(1.0)", "", LIME, curve.sample_clamped(1.0));
    report::eq("ColorCurve::sample_clamped(2.0)", "", BLUE, curve.sample_clamped(2.0));
    report::eq("ColorCurve::sample_clamped(0.5) == RED.mix(LIME, 0.5)", "", RED.mix(&LIME, 0.5), curve.sample_clamped(0.5));
    // out-of-domain `Curve::sample` (not `_clamped`) returns `None`.
    report::eq("ColorCurve::sample(-0.1) is None (outside domain)", "", None, curve.sample(-0.1));
    report::eq("ColorCurve::sample(2.1) is None (outside domain)", "", None, curve.sample(2.1));
    report::eq("ColorCurve::sample(1.0) is Some(LIME)", "", Some(LIME), curve.sample(1.0));
}

#[test]
fn curve_adaptors_compose() {
    // `.map` (CurveExt) — brighten every sampled color by mixing with white.
    let curve = ColorCurve::new([RED, BLUE]).unwrap();
    let brighter = curve.map(|c: Srgba| c.mix(&WHITE, 0.5));
    report::eq("ColorCurve.map(brighten) @ 0.0", "", RED.mix(&WHITE, 0.5), brighter.sample(0.0).unwrap());
    report::eq("ColorCurve.map(brighten) @ 1.0", "", BLUE.mix(&WHITE, 0.5), brighter.sample(1.0).unwrap());
}
