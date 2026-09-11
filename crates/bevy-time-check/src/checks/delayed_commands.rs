//! `DelayedCommandsExt` / `DelayedCommandQueue` / `check_delayed_command_queues`
//! under a real `App` + `TimePlugin`. Adapted from `bevy_time`'s own
//! `delayed_queues_should_run_with_time_plugin_enabled` test.

use super::report;
use bevy_app::{App, Startup};
use bevy_ecs::prelude::*;
use bevy_time::{DelayedCommandsExt, TimePlugin, TimeUpdateStrategy};
use core::time::Duration;

#[derive(Component)]
struct Dummy;

fn count_dummies(app: &mut App) -> usize {
    app.world_mut().query::<&Dummy>().iter(app.world()).count()
}

#[test]
fn delayed_commands_land_on_schedule() {
    fn queue_commands(mut commands: Commands) {
        // Immediate (not delayed).
        commands.spawn(Dummy);
        // 0.1s delay.
        commands.delayed().secs(0.1).spawn(Dummy);
        // 0.5s delay.
        let mut delayed = commands.delayed();
        delayed.secs(0.5).spawn(Dummy);
        // 1.0s delay, three entities sharing one queue.
        let mut in_1s = delayed.duration(Duration::from_secs_f32(1.0));
        in_1s.spawn(Dummy);
        in_1s.spawn(Dummy);
        in_1s.spawn(Dummy);
    }

    let mut app = App::new();
    app.add_plugins(TimePlugin)
        .add_systems(Startup, queue_commands)
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f32(0.2)));

    // frame -> total Dummy entities that should exist by then.
    let expected = [1u32, 2, 2, 3, 3, 6, 6, 6, 6, 6];
    for (frame, &want) in expected.iter().enumerate() {
        app.update();
        let got = count_dummies(&mut app) as u32;
        report::eq(&format!("delayed spawns landed by frame {frame} (0.2s steps)"), "", want, got);
    }
}

/// The `Entity` id from a delayed `spawn()` is usable *immediately* to queue a
/// second delayed command against it (per `DelayedCommandsExt`'s own doc
/// example) — entity allocation isn't itself delayed, only the component
/// insertion / the second command is.
#[test]
fn delayed_spawns_id_is_usable_for_a_later_delayed_command() {
    fn queue(mut commands: Commands) {
        let mut delayed = commands.delayed();
        // `.duration(...)` (exact `Duration`s) rather than `.secs(0.2)` (which
        // goes through `Duration::from_secs_f32` and can round a hair past an
        // exact-millisecond boundary) — keeps this test's tie-break with the
        // exact-millisecond `ManualDuration` steps below unambiguous.
        let entity = delayed.duration(Duration::from_millis(200)).spawn(Dummy).id();
        delayed.duration(Duration::from_millis(400)).entity(entity).despawn();
    }

    let mut app = App::new();
    app.add_plugins(TimePlugin)
        .add_systems(Startup, queue)
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(100)));

    // elapsed after update #N (first update's delta is ZERO, see real_time.rs):
    // 0, 0.1, 0.2, 0.3, 0.4, 0.5s — spawn lands at 0.2s, despawn at 0.4s.
    let expected = [0u32, 0, 1, 1, 0, 0];
    for (frame, &want) in expected.iter().enumerate() {
        app.update();
        let got = count_dummies(&mut app) as u32;
        report::eq(&format!("delayed spawn-then-despawn of a shared Entity id, frame {frame}"), "", want, got);
    }
}
