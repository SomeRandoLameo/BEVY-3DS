//! One submodule per `bevy_color` area. Every `#[test]` fn emits `ROW|` lines
//! (see `report`), so the emulator output transcribes straight into `README.md`.
//!
//! Kept self-contained (helpers + a private `mod report;`, no `crate::` refs) so
//! `crates/all-checks` can `#[path]`-include this tree unchanged.

mod report;

use bevy_color::{ColorToComponents, Hsla, LinearRgba, Oklaba, Srgba, Xyza};

/// Tolerance for same-space / exact-ish checks.
const EPS: f32 = report::EPS; // 1e-4

/// Tolerance for values that cross a color-space conversion. `bevy_color`'s own
/// tests use 1e-3 on well-behaved primaries; the 3DS pulls `cbrt`/`powf`/`ckbrt`
/// from devkitPro newlib so we allow a touch more slack on the compound paths.
const CONV_EPS: f32 = 4.0e-3;

/// Componentwise near-equality on the four `f32` channels of any color type.
#[track_caller]
fn near<C: ColorToComponents + Copy>(a: C, b: C, eps: f32) -> bool {
    let [a0, a1, a2, a3] = a.to_f32_array();
    let [b0, b1, b2, b3] = b.to_f32_array();
    (a0 - b0).abs() <= eps
        && (a1 - b1).abs() <= eps
        && (a2 - b2).abs() <= eps
        && (a3 - b3).abs() <= eps
}

/// Compare two colors after projecting both into `LinearRgba` — sidesteps hue
/// ambiguity for grays/blacks in the cylindrical spaces.
#[track_caller]
fn near_lin(a: impl Into<LinearRgba>, b: impl Into<LinearRgba>, eps: f32) -> bool {
    near(a.into(), b.into(), eps)
}

/// `report::approx` with the (always-empty, for this crate) input column
/// dropped, since almost every check here is a bare `expected`/`got` pair.
#[track_caller]
fn chk<T: core::fmt::Debug>(func: &str, expected: T, got: T, pass: bool) {
    report::approx(func, "", expected, got, pass);
}

/// A handful of colors with their coordinates in every space, lifted verbatim
/// from `bevy_color`'s generated `test_colors::TEST_COLORS` table. Drives the
/// conversion checks.
struct Tc {
    name: &'static str,
    rgb: Srgba,
    lin: LinearRgba,
    hsl: Hsla,
    oklab: Oklaba,
    xyz: Xyza,
}

const TABLE: &[Tc] = &[
    Tc {
        name: "black",
        rgb: Srgba::new(0.0, 0.0, 0.0, 1.0),
        lin: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
        hsl: Hsla::new(0.0, 0.0, 0.0, 1.0),
        oklab: Oklaba::new(0.0, 0.0, 0.0, 1.0),
        xyz: Xyza::new(0.0, 0.0, 0.0, 1.0),
    },
    Tc {
        name: "white",
        rgb: Srgba::new(1.0, 1.0, 1.0, 1.0),
        lin: LinearRgba::new(1.0, 1.0, 1.0, 1.0),
        hsl: Hsla::new(0.0, 0.0, 1.0, 1.0),
        oklab: Oklaba::new(1.0, 0.0, 0.000_000_059_604_645, 1.0),
        xyz: Xyza::new(0.95047, 1.0, 1.08883, 1.0),
    },
    Tc {
        name: "red",
        rgb: Srgba::new(1.0, 0.0, 0.0, 1.0),
        lin: LinearRgba::new(1.0, 0.0, 0.0, 1.0),
        hsl: Hsla::new(0.0, 1.0, 0.5, 1.0),
        oklab: Oklaba::new(0.6279554, 0.22486295, 0.1258463, 1.0),
        xyz: Xyza::new(0.4124564, 0.2126729, 0.0193339, 1.0),
    },
    Tc {
        name: "green",
        rgb: Srgba::new(0.0, 1.0, 0.0, 1.0),
        lin: LinearRgba::new(0.0, 1.0, 0.0, 1.0),
        hsl: Hsla::new(120.0, 1.0, 0.5, 1.0),
        oklab: Oklaba::new(0.8664396, -0.2338874, 0.1794985, 1.0),
        xyz: Xyza::new(0.3575761, 0.7151522, 0.119192, 1.0),
    },
    Tc {
        name: "blue",
        rgb: Srgba::new(0.0, 0.0, 1.0, 1.0),
        lin: LinearRgba::new(0.0, 0.0, 1.0, 1.0),
        hsl: Hsla::new(240.0, 1.0, 0.5, 1.0),
        oklab: Oklaba::new(0.4520137, -0.032456964, -0.31152815, 1.0),
        xyz: Xyza::new(0.1804375, 0.072175, 0.9503041, 1.0),
    },
    Tc {
        name: "gray",
        rgb: Srgba::new(0.5, 0.5, 0.5, 1.0),
        lin: LinearRgba::new(0.21404114, 0.21404114, 0.21404114, 1.0),
        hsl: Hsla::new(0.0, 0.0, 0.5, 1.0),
        oklab: Oklaba::new(0.5981807, 0.000_000_119_209_29, 0.0, 1.0),
        xyz: Xyza::new(0.2034397, 0.21404117, 0.23305441, 1.0),
    },
];

mod color_enum;
mod conversions;
mod cylindrical;
mod gradient;
mod layout;
mod linear_rgba;
mod ops;
mod palettes;
mod perceptual;
mod srgba;
