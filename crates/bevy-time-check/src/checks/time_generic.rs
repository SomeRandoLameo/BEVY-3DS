 //! `Time<T>` — the generic clock plumbing shared by `Real`/`Virtual`/`Fixed`
//! (and the context-free `Time` = `Time<()>` seen by ordinary systems).

use super::report;
use bevy_time::Time;
use core::time::Duration;

#[test]
fn advance_by_sets_delta_and_accumulates_elapsed() {
    let mut t = Time::<()>::default();
    report::eq("Time::default() delta", "", Duration::ZERO, t.delta());
    report::eq("Time::default() elapsed", "", Duration::ZERO, t.elapsed());

    t.advance_by(Duration::from_millis(500));
    report::eq("Time::advance_by(500ms) -> delta", "", Duration::from_millis(500), t.delta());
    report::eq("… -> elapsed", "", Duration::from_millis(500), t.elapsed());

    t.advance_by(Duration::from_millis(250));
    report::eq("Time::advance_by replaces delta (not additive)", "", Duration::from_millis(250), t.delta());
    report::eq("… but elapsed keeps accumulating", "500+250ms", Duration::from_millis(750), t.elapsed());

    // Advancing by ZERO is explicitly allowed and zeroes delta.
    t.advance_by(Duration::ZERO);
    report::eq("Time::advance_by(ZERO) zeroes delta", "", Duration::ZERO, t.delta());
    report::eq("… elapsed unchanged", "", Duration::from_millis(750), t.elapsed());
}

#[test]
fn advance_to_computes_delta_from_target() {
    let mut t = Time::<()>::default();
    t.advance_to(Duration::from_secs(2));
    report::eq("Time::advance_to(2s) from 0 -> delta", "", Duration::from_secs(2), t.delta());
    report::eq("… -> elapsed", "", Duration::from_secs(2), t.elapsed());

    t.advance_to(Duration::from_secs(5));
    report::eq("Time::advance_to(5s) from 2s -> delta", "", Duration::from_secs(3), t.delta());
    report::eq("… -> elapsed", "", Duration::from_secs(5), t.elapsed());
}

#[test]
#[should_panic(expected = "backwards")]
fn advance_to_panics_if_moving_backwards() {
    let mut t = Time::<()>::default();
    t.advance_to(Duration::from_secs(2));
    t.advance_to(Duration::from_secs(1));
}

#[test]
fn seconds_conversions() {
    let mut t = Time::<()>::default();
    t.advance_by(Duration::from_millis(1500));
    report::f("Time::delta_secs", "", 1.5, t.delta_secs());
    report::f("Time::delta_secs_f64", "", 1.5, t.delta_secs_f64() as f32);
    report::f("Time::elapsed_secs", "", 1.5, t.elapsed_secs());
    report::f("Time::elapsed_secs_f64", "", 1.5, t.elapsed_secs_f64() as f32);
}

#[test]
fn wrap_period() {
    let mut t = Time::<()>::default();
    report::eq("Time::wrap_period() default is 1 hour", "", Duration::from_secs(3600), t.wrap_period());

    t.set_wrap_period(Duration::from_secs(10));
    t.advance_by(Duration::from_secs(25));
    report::eq("Time::set_wrap_period(10s) -> elapsed_wrapped", "elapsed=25s", Duration::from_secs(5), t.elapsed_wrapped());
    report::f("Time::elapsed_secs_wrapped", "", 5.0, t.elapsed_secs_wrapped());
    report::f("Time::elapsed_secs_wrapped_f64", "", 5.0, t.elapsed_secs_wrapped_f64() as f32);
}

#[test]
fn context_accessors() {
    let mut t = Time::new_with(42_u32);
    report::eq("Time::context()", "", &42_u32, t.context());
    *t.context_mut() = 7;
    report::eq("Time::context_mut()", "", &7_u32, t.context());

    t.advance_by(Duration::from_secs(1));
    let generic = t.as_generic();
    report::eq("Time::as_generic() carries delta/elapsed, drops context", "", Duration::from_secs(1), generic.elapsed());
}
