//! `Timer` + `TimerMode` — the full public surface (values lifted from
//! `bevy_time`'s own doc examples / `mod tests`, so they're known-good).

use super::report;
use bevy_time::{Timer, TimerMode};
use core::time::Duration;

#[test]
fn once_vs_repeating_finish() {
    let mut once = Timer::from_seconds(1.0, TimerMode::Once);
    once.tick(Duration::from_secs_f32(1.5));
    report::is_true("Timer::Once finishes past duration", "tick(1.5) on 1.0s timer", once.is_finished());
    once.tick(Duration::from_secs_f32(0.5));
    report::is_true("Timer::Once stays finished", "ticked again", once.is_finished());

    let mut repeating = Timer::from_seconds(1.0, TimerMode::Repeating);
    repeating.tick(Duration::from_secs_f32(1.1));
    report::is_true("Timer::Repeating finishes at duration", "tick(1.1) on 1.0s", repeating.is_finished());
    repeating.tick(Duration::from_secs_f32(0.8));
    report::is_true("Timer::Repeating un-finishes mid-cycle", "1.1+0.8=1.9 -> wraps to 0.9", !repeating.is_finished());
    repeating.tick(Duration::from_secs_f32(0.6));
    report::is_true("Timer::Repeating finishes again", "0.9+0.6=1.5 -> wraps past 1.0", repeating.is_finished());
}

#[test]
fn just_finished() {
    let mut t = Timer::from_seconds(1.0, TimerMode::Once);
    t.tick(Duration::from_secs_f32(1.5));
    report::is_true("Timer::just_finished on the finishing tick", "", t.just_finished());
    t.tick(Duration::from_secs_f32(0.5));
    report::is_true("Timer::just_finished false on later ticks", "", !t.just_finished());
}

#[test]
fn elapsed_and_set_elapsed() {
    let mut t = Timer::from_seconds(1.0, TimerMode::Once);
    t.tick(Duration::from_secs_f32(0.5));
    report::eq("Timer::elapsed", "", Duration::from_secs_f32(0.5), t.elapsed());

    let mut t2 = Timer::from_seconds(1.0, TimerMode::Once);
    t2.set_elapsed(Duration::from_secs(2));
    report::eq("Timer::set_elapsed doesn't check duration", "", Duration::from_secs(2), t2.elapsed());
    report::is_true("… and doesn't finish the timer", "elapsed > duration but never ticked", !t2.is_finished());
}

#[test]
fn duration_and_finish() {
    let t = Timer::new(Duration::from_secs(1), TimerMode::Once);
    report::eq("Timer::duration", "", Duration::from_secs(1), t.duration());
    let mut t2 = Timer::from_seconds(1.5, TimerMode::Once);
    t2.set_duration(Duration::from_secs(1));
    report::eq("Timer::set_duration", "", Duration::from_secs(1), t2.duration());

    let mut t3 = Timer::from_seconds(1.5, TimerMode::Once);
    t3.finish();
    report::is_true("Timer::finish", "", t3.is_finished());

    let mut t4 = Timer::from_seconds(1.5, TimerMode::Once);
    t4.almost_finish();
    report::is_true("Timer::almost_finish doesn't finish", "", !t4.is_finished());
    report::eq("Timer::almost_finish leaves 1ns remaining", "", Duration::from_nanos(1), t4.remaining());
}

#[test]
fn mode_accessors() {
    let mut t = Timer::from_seconds(1.0, TimerMode::Repeating);
    report::eq("Timer::mode", "", TimerMode::Repeating, t.mode());
    t.set_mode(TimerMode::Once);
    report::eq("Timer::set_mode", "", TimerMode::Once, t.mode());
}

#[test]
fn tick_clamps_once_wraps_repeating() {
    let mut once = Timer::from_seconds(1.0, TimerMode::Once);
    let mut repeating = Timer::from_seconds(1.0, TimerMode::Repeating);
    once.tick(Duration::from_secs_f32(1.5));
    repeating.tick(Duration::from_secs_f32(1.5));
    report::f("Timer::Once clamps elapsed at duration", "tick(1.5) on 1.0s", 1.0, once.elapsed_secs());
    report::f("Timer::Repeating wraps elapsed", "tick(1.5) on 1.0s", 0.5, repeating.elapsed_secs());
}

#[test]
fn pause_unpause_reset() {
    let mut t = Timer::from_seconds(1.0, TimerMode::Once);
    t.pause();
    t.tick(Duration::from_secs_f32(0.5));
    report::is_true("Timer::pause stops ticking", "", t.is_paused());
    report::f("… elapsed_secs stays 0", "", 0.0, t.elapsed_secs());
    t.unpause();
    t.tick(Duration::from_secs_f32(0.5));
    report::is_true("Timer::unpause resumes", "", !t.is_paused());
    report::f("… elapsed_secs", "", 0.5, t.elapsed_secs());

    let mut t2 = Timer::from_seconds(1.0, TimerMode::Once);
    t2.tick(Duration::from_secs_f32(1.5));
    t2.reset();
    report::is_true("Timer::reset un-finishes", "", !t2.is_finished());
    report::is_true("Timer::reset clears just_finished", "", !t2.just_finished());
    report::f("Timer::reset zeroes elapsed_secs", "", 0.0, t2.elapsed_secs());
}

#[test]
fn fraction_and_remaining() {
    let mut t = Timer::from_seconds(2.0, TimerMode::Once);
    t.tick(Duration::from_secs_f32(0.5));
    report::f("Timer::fraction", "0.5 / 2.0s", 0.25, t.fraction());
    report::f("Timer::fraction_remaining", "", 0.75, t.fraction_remaining());
    report::f("Timer::remaining_secs", "", 1.5, t.remaining_secs());
    report::eq("Timer::remaining", "", Duration::from_secs_f32(1.5), t.remaining());
}

#[test]
fn times_finished_this_tick_multi_fire() {
    let mut t = Timer::from_seconds(1.0, TimerMode::Repeating);
    t.tick(Duration::from_secs_f32(6.0));
    report::eq("Timer::times_finished_this_tick (6s / 1s)", "", 6, t.times_finished_this_tick());
    t.tick(Duration::from_secs_f32(2.0));
    report::eq("… next tick (2s / 1s)", "", 2, t.times_finished_this_tick());
    t.tick(Duration::from_secs_f32(0.5));
    report::eq("… sub-duration tick fires zero times", "", 0, t.times_finished_this_tick());
}

#[test]
fn default_mode_is_once() {
    report::eq("TimerMode::default()", "", TimerMode::Once, TimerMode::default());
}
