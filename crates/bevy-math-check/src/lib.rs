//! Standalone correctness check for **`bevy_math`** (and its `glam` 0.32) on the
//! 3DS target (`armv6k-nintendo-3ds`).
//!
//! Goal: exercise the whole public API — every module, every type, every
//! distinct operation — and confirm it **computes the right numbers** on-device
//! (trig / `sqrt` / `exp` / `f64` math come from devkitPro's newlib, not the
//! host libm).
//!
//! Each check prints a Markdown table row (`| function | input | expected | got
//! | status |`) straight to stdout; the rows are transcribed into `README.md`.
//! Run: `./scripts/test-emulator.sh -p bevy-math-check`.

#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(test_runner::run_gdb))]

use bevy_math::{Mat4, Quat, Vec3};

/// A representative transform pipeline (build an MVP matrix, project a point).
pub fn demo() -> Vec3 {
    let model = Mat4::from_scale_rotation_translation(
        Vec3::splat(2.0),
        Quat::from_rotation_y(core::f32::consts::FRAC_PI_2),
        Vec3::new(1.0, 0.0, -5.0),
    );
    let view = Mat4::look_at_rh(Vec3::new(0.0, 2.0, 8.0), Vec3::ZERO, Vec3::Y);
    let proj = Mat4::perspective_rh(60_f32.to_radians(), 400.0 / 240.0, 0.1, 100.0);
    (proj * view * model).project_point3(Vec3::new(0.5, 0.5, 0.0))
}

#[cfg(test)]
mod report;
#[cfg(test)]
mod checks;
