//! `bevy_math::ops` — the std-or-libm float shim. On the 3DS this resolves to
//! `f32`'s newlib implementations, so all 32 functions are checked here.

use super::report;
use bevy_math::ops;
use core::f32::consts::{E, FRAC_PI_4, FRAC_PI_6, LN_2, PI, SQRT_2};

#[test]
fn ops_trigonometric() {
    report::f("ops::sin", "PI/6", 0.5, ops::sin(FRAC_PI_6));
    report::f("ops::cos", "PI/3", 0.5, ops::cos(PI / 3.0));
    report::f("ops::tan", "PI/4", 1.0, ops::tan(FRAC_PI_4));
    let (s, c) = ops::sin_cos(FRAC_PI_6);
    report::f("ops::sin_cos.0", "PI/6", 0.5, s);
    report::f("ops::sin_cos.1", "PI/6", 3.0_f32.sqrt() / 2.0, c);
    report::f("ops::asin", "0.5", FRAC_PI_6, ops::asin(0.5));
    report::f("ops::acos", "0.5", PI / 3.0, ops::acos(0.5));
    report::f("ops::atan", "1", FRAC_PI_4, ops::atan(1.0));
    report::f("ops::atan2", "(1, 1)", FRAC_PI_4, ops::atan2(1.0, 1.0));
    report::f("ops::atan2", "(1, -1)", 3.0 * FRAC_PI_4, ops::atan2(1.0, -1.0));
}

#[test]
fn ops_hyperbolic() {
    report::f("ops::sinh", "0", 0.0, ops::sinh(0.0));
    report::f("ops::cosh", "0", 1.0, ops::cosh(0.0));
    report::f("ops::tanh", "0", 0.0, ops::tanh(0.0));
    report::f("ops::sinh", "1", 1.1752012, ops::sinh(1.0));
    report::f("ops::cosh", "1", 1.5430806, ops::cosh(1.0));
    report::f("ops::asinh", "sinh(1)", 1.0, ops::asinh(ops::sinh(1.0)));
    report::f("ops::acosh", "cosh(1)", 1.0, ops::acosh(ops::cosh(1.0)));
    report::f("ops::atanh", "tanh(0.5)", 0.5, ops::atanh(ops::tanh(0.5)));
}

#[test]
fn ops_exp_log() {
    report::f("ops::exp", "0", 1.0, ops::exp(0.0));
    report::f("ops::exp", "1", E, ops::exp(1.0));
    report::f("ops::exp2", "10", 1024.0, ops::exp2(10.0));
    report::f("ops::exp_m1", "0", 0.0, ops::exp_m1(0.0));
    report::f("ops::ln", "E", 1.0, ops::ln(E));
    report::f("ops::ln_1p", "0", 0.0, ops::ln_1p(0.0));
    report::f("ops::log2", "8", 3.0, ops::log2(8.0));
    report::f("ops::log10", "1000", 3.0, ops::log10(1000.0));
    report::f("ops::ln", "2", LN_2, ops::ln(2.0));
}

#[test]
fn ops_powers_roots() {
    report::f("ops::powf", "2^10", 1024.0, ops::powf(2.0, 10.0));
    report::f("ops::powf", "9^0.5", 3.0, ops::powf(9.0, 0.5));
    report::f("ops::sqrt", "2", SQRT_2, ops::sqrt(2.0));
    report::f("ops::sqrt", "144", 12.0, ops::sqrt(144.0));
    report::f("ops::cbrt", "27", 3.0, ops::cbrt(27.0));
    report::f("ops::cbrt", "-8", -2.0, ops::cbrt(-8.0));
    report::f("ops::hypot", "(3, 4)", 5.0, ops::hypot(3.0, 4.0));
}

#[test]
fn ops_rounding_and_misc() {
    report::f("ops::abs", "-3.5", 3.5, ops::abs(-3.5));
    report::f("ops::floor", "2.7", 2.0, ops::floor(2.7));
    report::f("ops::floor", "-2.1", -3.0, ops::floor(-2.1));
    report::f("ops::ceil", "2.1", 3.0, ops::ceil(2.1));
    report::f("ops::round", "2.5", 3.0, ops::round(2.5));
    report::f("ops::round", "-2.5", -3.0, ops::round(-2.5));
    report::f("ops::fract", "2.75", 0.75, ops::fract(2.75));
    report::f("ops::copysign", "(3, -1)", -3.0, ops::copysign(3.0, -1.0));
    report::f("ops::rem_euclid", "(-1, 3)", 2.0, ops::rem_euclid(-1.0, 3.0));
}
