//! `Stopwatch` — the full public surface (mirrors its own doc examples).

use super::report;
use bevy_time::Stopwatch;
use core::time::Duration;

#[test]
fn new_and_tick() {
    let mut sw = Stopwatch::new();
    report::f("Stopwatch::new -> elapsed_secs", "", 0.0, sw.elapsed_secs());
    report::is_true("Stopwatch::new -> is_paused", "", !sw.is_paused());
    sw.tick(Duration::from_secs_f32(1.0));
    report::f("Stopwatch::tick(1.0)", "", 1.0, sw.elapsed_secs());
    sw.tick(Duration::from_secs_f32(1.5));
    report::f("Stopwatch::tick(1.5) accumulates", "1.0 + 1.5", 2.5, sw.elapsed_secs());
    report::eq("Stopwatch::elapsed (Duration)", "", Duration::from_secs_f32(2.5), sw.elapsed());
    report::f("Stopwatch::elapsed_secs_f64", "", 2.5, sw.elapsed_secs_f64() as f32);
}

#[test]
fn pause_unpause_and_reset() {
    let mut sw = Stopwatch::new();
    sw.pause();
    sw.tick(Duration::from_secs_f32(1.5));
    report::is_true("Stopwatch::pause -> tick has no effect", "", sw.is_paused());
    report::f("… elapsed_secs stays 0", "", 0.0, sw.elapsed_secs());

    sw.unpause();
    sw.tick(Duration::from_secs_f32(1.0));
    report::is_true("Stopwatch::unpause -> ticking resumes", "", !sw.is_paused());
    report::f("… elapsed_secs", "", 1.0, sw.elapsed_secs());

    sw.reset();
    report::f("Stopwatch::reset -> elapsed_secs", "", 0.0, sw.elapsed_secs());
    report::is_true("Stopwatch::reset doesn't unpause", "was already unpaused", !sw.is_paused());
}

#[test]
fn reset_keeps_paused_state() {
    let mut sw = Stopwatch::new();
    sw.pause();
    sw.tick(Duration::from_secs_f32(1.5));
    sw.reset();
    report::is_true("Stopwatch::reset() keeps `paused` set", "paused before reset", sw.is_paused());
    report::f("… elapsed_secs is 0 after reset", "", 0.0, sw.elapsed_secs());
}

#[test]
fn set_elapsed() {
    let mut sw = Stopwatch::new();
    sw.set_elapsed(Duration::from_secs_f32(1.0));
    report::f("Stopwatch::set_elapsed(1.0)", "", 1.0, sw.elapsed_secs());
}
