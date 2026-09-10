//! `bevy_math::curve` — the `Curve` trait + adaptors, every `EaseFunction`
//! variant, and every cubic-spline generator.

use super::report;
use bevy_math::curve::{Curve, CurveExt, EaseFunction, EasingCurve, FunctionCurve, Interval};
use bevy_math::cubic_splines::{
    CubicBSpline, CubicBezier, CubicCardinalSpline, CubicCurve, CubicGenerator, CubicHermite,
    CubicNurbs, CubicSegment, RationalGenerator,
};
use bevy_math::Vec2;

#[test]
fn curve_trait_and_adaptors() {
    let base = FunctionCurve::new(Interval::UNIT, |t| t * 10.0);
    report::f("Curve::sample(0.5) on FunctionCurve", "|t| t*10 over [0,1]", 5.0, base.sample(0.5).unwrap());
    report::ok("Curve::sample(2.0) outside domain", "", "None", if base.sample(2.0).is_none() { "None" } else { "Some" }, base.sample(2.0).is_none());
    report::f("Curve::sample_clamped(2.0)", "clamps to 1.0", 10.0, base.sample_clamped(2.0));

    let doubled = FunctionCurve::new(Interval::UNIT, |t| t * 10.0).map(|v| v * 2.0);
    report::f("Curve::map", "(|t| t*10).map(|v| v*2) @ 0.5", 10.0, doubled.sample(0.5).unwrap());

    let rev = FunctionCurve::new(Interval::UNIT, |t| t).reverse().unwrap();
    report::f("Curve::reverse", "(|t| t).reverse() @ 0.25", 0.75, rev.sample(0.25).unwrap());

    let rep = FunctionCurve::new(Interval::UNIT, |t| t).repeat(1).unwrap();
    report::f("Curve::repeat(1) domain length", "", 2.0, rep.domain().length());

    let pp = FunctionCurve::new(Interval::UNIT, |t| t).ping_pong().unwrap();
    report::f("Curve::ping_pong @ 1.5 (going back)", "", 0.5, pp.sample(1.5).unwrap());

    let repar = FunctionCurve::new(Interval::UNIT, |t| t).reparametrize_linear(Interval::new(0.0, 10.0).unwrap()).unwrap();
    report::f("Curve::reparametrize_linear to [0,10] @ 5", "", 0.5, repar.sample(5.0).unwrap());

    let samples: Vec<f32> = FunctionCurve::new(Interval::UNIT, |t| t * 4.0).samples(5).unwrap().collect();
    report::eq("Curve::samples(5)", "|t| t*4 over [0,1]", vec![0.0_f32, 1.0, 2.0, 3.0, 4.0], samples);

    report::eq("Interval::UNIT bounds", "", (0.0_f32, 1.0_f32), (Interval::UNIT.start(), Interval::UNIT.end()));
    report::f("Interval::length", "[2,7]", 5.0, Interval::new(2.0, 7.0).unwrap().length());
}

#[test]
fn ease_functions_all_variants() {
    use EaseFunction::*;
    let all = [
        Linear, QuadraticIn, QuadraticOut, QuadraticInOut, CubicIn, CubicOut, CubicInOut, QuarticIn,
        QuarticOut, QuarticInOut, QuinticIn, QuinticOut, QuinticInOut, SmoothStepIn, SmoothStepOut,
        SmoothStep, SmootherStepIn, SmootherStepOut, SmootherStep, SineIn, SineOut, SineInOut,
        CircularIn, CircularOut, CircularInOut, ExponentialIn, ExponentialOut, ExponentialInOut,
        ElasticIn, ElasticOut, ElasticInOut, BackIn, BackOut, BackInOut, BounceIn, BounceOut,
        BounceInOut,
    ];
    for ef in all {
        let c = EasingCurve::new(0.0_f32, 1.0, ef);
        let (a, m, b) = (c.sample(0.0).unwrap(), c.sample(0.5).unwrap(), c.sample(1.0).unwrap());
        let ok = (a - 0.0).abs() < 1e-3 && (b - 1.0).abs() < 1e-3 && m.is_finite();
        report::ok(
            &format!("EaseFunction::{ef:?}"),
            "sample @ 0.0 / 0.5 / 1.0",
            "0.0 / finite / 1.0",
            &format!("{a:.4} / {m:.4} / {b:.4}"),
            ok,
        );
    }
    report::f("EaseFunction::Linear @ 0.5", "", 0.5, EasingCurve::new(0.0_f32, 1.0, Linear).sample(0.5).unwrap());
    report::f("EaseFunction::QuadraticIn @ 0.5", "", 0.25, EasingCurve::new(0.0_f32, 1.0, QuadraticIn).sample(0.5).unwrap());
    report::f("EaseFunction::SmoothStep @ 0.5", "", 0.5, EasingCurve::new(0.0_f32, 1.0, SmoothStep).sample(0.5).unwrap());
}

#[test]
fn cubic_spline_generators() {
    let pts = [
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 2.0),
        Vec2::new(3.0, 2.0),
        Vec2::new(4.0, 0.0),
    ];
    let seg = CubicSegment::new_bezier([Vec2::ZERO, Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0), Vec2::new(1.0, 0.0)]);
    report::approx("CubicSegment::position(0.5)", "bezier (0,0)(0,1)(1,1)(1,0)", Vec2::new(0.5, 0.75), seg.position(0.5), super::near2(seg.position(0.5), Vec2::new(0.5, 0.75)));
    report::approx("CubicSegment::velocity(0)", "same", Vec2::new(0.0, 3.0), seg.velocity(0.0), super::near2(seg.velocity(0.0), Vec2::new(0.0, 3.0)));
    report::approx("CubicSegment::acceleration(0) = 6·(P0-2P1+P2)", "same", Vec2::new(6.0, -6.0), seg.acceleration(0.0), super::near2(seg.acceleration(0.0), Vec2::new(6.0, -6.0)));

    let bez: CubicCurve<Vec2> = CubicBezier::new([pts]).to_curve().unwrap();
    report::approx("CubicBezier::to_curve.position(0)", "4 control points", pts[0], bez.position(0.0), super::near2(bez.position(0.0), pts[0]));
    report::approx("CubicBezier::to_curve.position(1)", "same", pts[3], bez.position(1.0), super::near2(bez.position(1.0), pts[3]));

    let herm: CubicCurve<Vec2> = CubicHermite::new([Vec2::ZERO, Vec2::new(2.0, 0.0)], [Vec2::X, Vec2::X]).to_curve().unwrap();
    report::approx("CubicHermite::to_curve endpoints", "p0=0 p1=(2,0)", Vec2::ZERO, herm.position(0.0), super::near2(herm.position(0.0), Vec2::ZERO));

    let card: CubicCurve<Vec2> = CubicCardinalSpline::new(0.5, pts).to_curve().unwrap();
    report::is_true("CubicCardinalSpline::to_curve finite", "", card.position(0.5).is_finite());

    let bspline: CubicCurve<Vec2> = CubicBSpline::new(pts).to_curve().unwrap();
    report::is_true("CubicBSpline::to_curve finite", "", bspline.position(0.5).is_finite());

    let nurbs = CubicNurbs::new(pts, Some([1.0_f32, 1.0, 1.0, 1.0]), None::<[f32; 0]>).unwrap().to_curve().unwrap();
    report::is_true("CubicNurbs::to_curve finite (uniform weights)", "", nurbs.position(0.5).is_finite());
}
