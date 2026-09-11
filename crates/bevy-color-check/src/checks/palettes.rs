//! `palettes::{basic, css, tailwind}` — spot-check a few named constants in
//! each table plus their size, since the tables themselves are just data.

use super::report;
use bevy_color::palettes::{basic, css, tailwind};
use bevy_color::Srgba;

#[test]
fn basic_palette() {
    report::eq("basic::RED", "", Srgba::rgb(1.0, 0.0, 0.0), basic::RED);
    report::eq("basic::BLACK", "", Srgba::rgb(0.0, 0.0, 0.0), basic::BLACK);
    report::eq("basic::WHITE", "", Srgba::rgb(1.0, 1.0, 1.0), basic::WHITE);
    report::eq("basic::GRAY (VGA 50.19%)", "", Srgba::rgb(0.5019608, 0.5019608, 0.5019608), basic::GRAY);
    report::eq("basic::LIME", "", Srgba::rgb(0.0, 1.0, 0.0), basic::LIME);
}

#[test]
fn css_palette() {
    report::eq("css::REBECCA_PURPLE", "", Srgba::new(0.4, 0.2, 0.6, 1.0), css::REBECCA_PURPLE);
    report::eq("css::ALICE_BLUE", "", Srgba::new(0.941, 0.973, 1.0, 1.0), css::ALICE_BLUE);
    report::eq("css::WHITE_SMOKE", "", Srgba::new(0.961, 0.961, 0.961, 1.0), css::WHITE_SMOKE);
}

#[test]
fn tailwind_palette() {
    report::eq("tailwind::BLUE_500", "", Srgba::rgb(0.23137255, 0.50980395, 0.9647059), tailwind::BLUE_500);
    report::eq("tailwind::RED_500", "", Srgba::rgb(0.9372549, 0.26666668, 0.26666668), tailwind::RED_500);
    report::eq("tailwind::SLATE_50", "", Srgba::rgb(0.972549, 0.98039216, 0.9882353), tailwind::SLATE_50);
    report::eq("tailwind::GRAY_950", "", Srgba::rgb(0.011764706, 0.02745098, 0.07058824), tailwind::GRAY_950);
}
