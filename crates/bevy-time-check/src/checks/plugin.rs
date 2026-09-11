//! `TimePlugin` itself: resource initialization and `TimeUpdateStrategy`'s
//! non-`Automatic` variants (the `Automatic` / real-`Instant::now()` path is
//! covered qualitatively in `real_time.rs`, not timed here).

use super::report;
use bevy_app::App;
use bevy_time::{Fixed, Real, Time, TimePlugin, TimeUpdateStrategy, Virtual};
use core::time::Duration;

#[test]
fn plugin_inserts_all_four_clocks() {
    let mut app = App::new();
    app.add_plugins(TimePlugin);
    let world = app.world();
    report::is_true("TimePlugin inserts Res<Time>", "", world.get_resource::<Time>().is_some());
    report::is_true("TimePlugin inserts Res<Time<Real>>", "", world.get_resource::<Time<Real>>().is_some());
    report::is_true("TimePlugin inserts Res<Time<Virtual>>", "", world.get_resource::<Time<Virtual>>().is_some());
    report::is_true("TimePlugin inserts Res<Time<Fixed>>", "", world.get_resource::<Time<Fixed>>().is_some());
    report::is_true("TimePlugin inserts Res<TimeUpdateStrategy>", "", world.get_resource::<TimeUpdateStrategy>().is_some());
}

#[test]
fn default_strategy_is_automatic() {
    report::is_true("TimeUpdateStrategy::default() is Automatic", "", matches!(TimeUpdateStrategy::default(), TimeUpdateStrategy::Automatic));
}

#[test]
fn manual_duration_strategy_advances_real_time() {
    let mut app = App::new();
    app.add_plugins(TimePlugin)
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(16)));

    // First update only establishes `last_update` (delta ZERO, see real_time.rs).
    app.update();
    app.update();
    report::eq("TimeUpdateStrategy::ManualDuration(16ms) -> Time::<Real>::delta", "2nd update", Duration::from_millis(16), app.world().resource::<Time<Real>>().delta());
}

#[test]
fn manual_instant_strategy_uses_the_given_instant() {
    use bevy_platform::time::Instant;

    let mut app = App::new();
    app.add_plugins(TimePlugin);
    let first = Instant::now();
    app.insert_resource(TimeUpdateStrategy::ManualInstant(first));
    app.update();
    report::eq("TimeUpdateStrategy::ManualInstant -> Time::<Real>::last_update", "", Some(first), app.world().resource::<Time<Real>>().last_update());

    let second: Instant = first + Duration::from_millis(50);
    app.insert_resource(TimeUpdateStrategy::ManualInstant(second));
    app.update();
    report::eq("… next update with a later Instant -> delta", "", Duration::from_millis(50), app.world().resource::<Time<Real>>().delta());
}

#[test]
fn fixed_timesteps_strategy_advances_by_n_times_the_fixed_timestep() {
    let mut app = App::new();
    app.add_plugins(TimePlugin)
        .insert_resource(TimeUpdateStrategy::FixedTimesteps(3));

    app.update(); // establishes last_update, delta ZERO
    app.update();
    let fixed_timestep = app.world().resource::<Time<Fixed>>().timestep();
    report::eq("TimeUpdateStrategy::FixedTimesteps(3) -> Time::<Real>::delta", "3 x default timestep", fixed_timestep * 3, app.world().resource::<Time<Real>>().delta());
}
