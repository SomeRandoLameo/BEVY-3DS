//! `Time<Fixed>` — the fixed-timestep clock, and `run_fixed_main_schedule`
//! exercised end-to-end through a real `bevy_app::App` + `TimePlugin` (mirrors
//! `bevy_time`'s own `fixed_main_schedule_should_run_with_time_plugin_enabled`).

use super::report;
use bevy_app::{App, FixedUpdate, Update};
use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};
use core::time::Duration;

#[test]
fn default_timestep_is_64hz() {
    let t = Time::<Fixed>::default();
    report::eq("Time::<Fixed>::default().timestep() == 1/64s", "", Duration::from_micros(15625), t.timestep());
    report::eq("… overstep starts at ZERO", "", Duration::ZERO, t.overstep());
}

#[test]
fn constructors_and_setters() {
    let t = Time::<Fixed>::from_duration(Duration::from_millis(500));
    report::eq("Time::<Fixed>::from_duration(500ms)", "", Duration::from_millis(500), t.timestep());

    let t = Time::<Fixed>::from_seconds(0.25);
    report::eq("Time::<Fixed>::from_seconds(0.25)", "", Duration::from_millis(250), t.timestep());

    let t = Time::<Fixed>::from_hz(8.0);
    report::eq("Time::<Fixed>::from_hz(8.0)", "", Duration::from_millis(125), t.timestep());

    let mut t = Time::<Fixed>::default();
    t.set_timestep(Duration::from_millis(500));
    report::eq("Time::<Fixed>::set_timestep", "", Duration::from_millis(500), t.timestep());
    t.set_timestep_seconds(0.25);
    report::eq("Time::<Fixed>::set_timestep_seconds", "", Duration::from_millis(250), t.timestep());
    t.set_timestep_hz(8.0);
    report::eq("Time::<Fixed>::set_timestep_hz", "", Duration::from_millis(125), t.timestep());
}

#[test]
fn overstep_accumulate_and_discard() {
    let mut t = Time::<Fixed>::from_seconds(2.0);
    t.accumulate_overstep(Duration::from_secs(1));
    report::eq("Time::<Fixed>::accumulate_overstep(1s)", "", Duration::from_secs(1), t.overstep());
    report::f("… overstep_fraction (1s / 2s)", "", 0.5, t.overstep_fraction());
    report::f("… overstep_fraction_f64", "", 0.5, t.overstep_fraction_f64() as f32);

    t.discard_overstep(Duration::from_millis(400));
    report::eq("Time::<Fixed>::discard_overstep(400ms)", "1s - 400ms", Duration::from_millis(600), t.overstep());
}

/// `app.update()` with a known `ManualDuration` step should run `FixedUpdate`
/// exactly the number of times bevy_time's own upstream test expects.
#[test]
fn fixed_update_runs_expected_number_of_times() {
    #[derive(Resource, Default)]
    struct Counter(u8);
    fn count(mut c: ResMut<Counter>) {
        c.0 += 1;
    }

    let fixed_timestep = Time::<Fixed>::default().timestep();
    let time_step = fixed_timestep / 2 + Duration::from_millis(1);

    let mut app = App::new();
    app.add_plugins(TimePlugin)
        .add_systems(FixedUpdate, count)
        .init_resource::<Counter>()
        .insert_resource(TimeUpdateStrategy::ManualDuration(time_step));

    let expected = [0u8, 0, 1, 1, 2];
    for (frame, &want) in expected.iter().enumerate() {
        app.update();
        let got = app.world().resource::<Counter>().0;
        report::eq(&format!("FixedUpdate run count after app.update() #{frame}"), "", want, got);
    }
}

/// The generic `Time` resource reflects `Time<Fixed>` while `FixedUpdate` runs,
/// and `Time<Virtual>` again for ordinary `Update` systems (per `Time`'s docs).
#[test]
fn generic_time_switches_context_between_schedules() {
    #[derive(Resource, Default)]
    struct Seen {
        fixed_delta: Option<Duration>,
        update_delta: Option<Duration>,
    }
    fn see_fixed(time: Res<Time>, mut seen: ResMut<Seen>) {
        seen.fixed_delta = Some(time.delta());
    }
    fn see_update(time: Res<Time>, mut seen: ResMut<Seen>) {
        seen.update_delta = Some(time.delta());
    }

    let fixed_timestep = Time::<Fixed>::default().timestep();
    let mut app = App::new();
    app.add_plugins(TimePlugin)
        .add_systems(FixedUpdate, see_fixed)
        .add_systems(Update, see_update)
        .init_resource::<Seen>()
        .insert_resource(TimeUpdateStrategy::ManualDuration(fixed_timestep));

    // The very first `app.update()` only establishes `Time<Real>::last_update`
    // (its delta is ZERO, see `real_time::update_with_duration_advances_from_last_update`),
    // so FixedUpdate doesn't accumulate enough overstep to run until the 2nd.
    app.update();
    app.update();
    let seen = app.world().resource::<Seen>();
    report::eq("Time::delta() inside FixedUpdate == Time::<Fixed>::timestep()", "", Some(fixed_timestep), seen.fixed_delta);
    report::is_true("Time::delta() inside Update is the virtual-time delta (not fixed)", "", seen.update_delta.is_some());
}
