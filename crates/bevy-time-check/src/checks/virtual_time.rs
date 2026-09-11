//! `Time<Virtual>` — pause / relative-speed / `max_delta` clamping, all
//! driven through the public `update_virtual_time(current, virt, real)` (the
//! function `TimePlugin`'s `time_system` calls every frame), so it's the same
//! code path the real app uses.

use super::report;
use bevy_time::{update_virtual_time, Real, Time, Virtual};
use core::time::Duration;

/// Advance `real` by `dt`, then run the same update `TimePlugin` runs.
fn tick(current: &mut Time, virt: &mut Time<Virtual>, real: &mut Time<Real>, dt: Duration) {
    real.update_with_duration(dt);
    update_virtual_time(current, virt, real);
}

#[test]
fn defaults() {
    let t = Time::<Virtual>::default();
    report::is_true("Time::<Virtual>::default() is not paused", "", !t.is_paused());
    report::f("… relative_speed", "", 1.0, t.relative_speed());
    report::eq("… max_delta", "", Duration::from_millis(250), t.max_delta());
    report::eq("… delta", "", Duration::ZERO, t.delta());
}

#[test]
fn tracks_real_time_at_normal_speed() {
    let mut current = Time::default();
    let mut real = Time::<Real>::default();
    let mut virt = Time::<Virtual>::default();

    // First `update_with_duration` only establishes `last_update` (real delta
    // is ZERO), matching `Time<Real>`'s own "first tick is zero" behaviour.
    tick(&mut current, &mut virt, &mut real, Duration::from_millis(100));
    tick(&mut current, &mut virt, &mut real, Duration::from_millis(100));

    report::eq("Time<Virtual>::delta tracks Time<Real>::delta at 1x speed", "", real.delta(), virt.delta());
    report::eq("Time<Virtual>::elapsed tracks Time<Real>::elapsed at 1x speed", "", real.elapsed(), virt.elapsed());
    report::eq("update_virtual_time also refreshes the generic Time", "", virt.elapsed(), current.elapsed());
}

#[test]
fn paused_clock_has_zero_delta_and_zero_effective_speed() {
    let mut current = Time::default();
    let mut real = Time::<Real>::default();
    let mut virt = Time::<Virtual>::default();
    tick(&mut current, &mut virt, &mut real, Duration::from_millis(100)); // establish last_update

    virt.pause();
    tick(&mut current, &mut virt, &mut real, Duration::from_millis(100));
    report::is_true("Time::<Virtual>::pause()", "", virt.is_paused());
    report::eq("… paused -> delta is ZERO", "", Duration::ZERO, virt.delta());
    report::f("… effective_speed is 0.0", "", 0.0, virt.effective_speed());
    let elapsed_before = virt.elapsed();

    tick(&mut current, &mut virt, &mut real, Duration::from_millis(100));
    report::eq("… elapsed does not grow while paused", "", elapsed_before, virt.elapsed());

    virt.unpause();
    tick(&mut current, &mut virt, &mut real, Duration::from_millis(100));
    report::is_true("Time::<Virtual>::unpause()", "", !virt.is_paused());
    report::eq("… ticking resumes", "", Duration::from_millis(100), virt.delta());
}

#[test]
fn toggle_flips_pause_state() {
    let mut t = Time::<Virtual>::default();
    t.toggle();
    report::is_true("Time::<Virtual>::toggle() (unpaused -> paused)", "", t.is_paused());
    t.toggle();
    report::is_true("… toggle again (paused -> unpaused)", "", !t.is_paused());
}

#[test]
fn relative_speed_scales_delta() {
    let mut current = Time::default();
    let mut real = Time::<Real>::default();
    let mut virt = Time::<Virtual>::default();
    tick(&mut current, &mut virt, &mut real, Duration::from_millis(100)); // establish last_update

    virt.set_relative_speed(2.0);
    tick(&mut current, &mut virt, &mut real, Duration::from_millis(100));
    report::f("Time::<Virtual>::set_relative_speed(2.0)", "", 2.0, virt.relative_speed());
    report::f("… effective_speed matches (not paused)", "", 2.0, virt.effective_speed());
    report::eq("… delta is doubled", "100ms real -> 200ms virtual", Duration::from_millis(200), virt.delta());
}

#[test]
fn max_delta_clamps_large_real_jumps() {
    let mut current = Time::default();
    let mut real = Time::<Real>::default();
    let mut virt = Time::<Virtual>::from_max_delta(Duration::from_millis(50));
    tick(&mut current, &mut virt, &mut real, Duration::ZERO); // establish last_update

    tick(&mut current, &mut virt, &mut real, Duration::from_secs(1));
    report::eq("Time::<Virtual>::from_max_delta(50ms) clamps a 1s real jump", "", Duration::from_millis(50), virt.delta());
}
