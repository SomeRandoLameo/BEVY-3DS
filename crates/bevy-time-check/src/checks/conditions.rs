//! `common_conditions` — the `.run_if(...)` closures. Driven by a plain
//! `World` + `Schedule` with a manually-advanced `Time`/`Time<Real>` resource
//! (no `App`/`TimePlugin` needed — these are just `FnMut(Res<Time<_>>) -> bool`).

use super::report;
use bevy_ecs::prelude::*;
use bevy_time::common_conditions::{on_timer, once_after_delay, paused, repeating_after_delay};
use bevy_time::{Real, Time, Virtual};
use core::time::Duration;

#[derive(Resource, Default)]
struct Ran(u32);

fn mark(mut ran: ResMut<Ran>) {
    ran.0 += 1;
}

/// Run `schedule` once per `dt` in `steps`, advancing `Time::<()>` by each `dt`
/// first (mirrors how `time_system` advances the generic `Time` every frame).
fn drive(world: &mut World, schedule: &mut Schedule, steps: &[u64]) {
    for &ms in steps {
        world.resource_mut::<Time>().advance_by(Duration::from_millis(ms));
        schedule.run(world);
    }
}

#[test]
fn on_timer_fires_once_per_interval() {
    let mut world = World::new();
    world.insert_resource(Time::<()>::default());
    world.insert_resource(Ran::default());
    let mut schedule = Schedule::default();
    schedule.add_systems(mark.run_if(on_timer(Duration::from_secs(1))));

    // 400ms steps: fires on step 3 (1.2s) and step 6 (2.4s, i.e. the 2nd second).
    drive(&mut world, &mut schedule, &[400, 400, 400, 400, 400, 400]);
    report::eq("on_timer(1s) fired twice over 2.4s in 400ms steps", "", 2, world.resource::<Ran>().0);
}

#[test]
fn once_after_delay_fires_exactly_once() {
    let mut world = World::new();
    world.insert_resource(Time::<()>::default());
    world.insert_resource(Ran::default());
    let mut schedule = Schedule::default();
    schedule.add_systems(mark.run_if(once_after_delay(Duration::from_millis(500))));

    drive(&mut world, &mut schedule, &[200, 200, 200, 200, 200]);
    report::eq("once_after_delay(500ms) fires exactly once", "", 1, world.resource::<Ran>().0);
}

#[test]
fn repeating_after_delay_stays_true_once_past_the_delay() {
    // Despite the name, this wraps a `TimerMode::Once` timer and returns
    // `is_finished()` (not `just_finished()`) — so it's a one-way gate that
    // flips true once the delay has passed and then stays true on every
    // later tick, not a "fire every N" repeater like `on_timer`.
    let mut world = World::new();
    world.insert_resource(Time::<()>::default());
    world.insert_resource(Ran::default());
    let mut schedule = Schedule::default();
    schedule.add_systems(mark.run_if(repeating_after_delay(Duration::from_millis(300))));

    drive(&mut world, &mut schedule, &[100, 100, 100, 100, 100, 100]);
    report::eq("repeating_after_delay(300ms): true on steps 3-6 of 6x100ms", "", 4, world.resource::<Ran>().0);
}

#[test]
fn on_real_timer_uses_time_real() {
    let mut world = World::new();
    world.insert_resource(Time::<Real>::default());
    world.insert_resource(Ran::default());
    let mut schedule = Schedule::default();
    schedule.add_systems(mark.run_if(bevy_time::common_conditions::on_real_timer(Duration::from_millis(500))));

    for _ in 0..3 {
        world.resource_mut::<Time<Real>>().update_with_duration(Duration::from_millis(250));
        schedule.run(&mut world);
    }
    report::eq("on_real_timer(500ms) over 3x250ms", "first tick is delta=0, see real_time.rs", 1, world.resource::<Ran>().0);
}

#[test]
fn paused_condition_reads_virtual_pause_state() {
    let mut world = World::new();
    let mut t = Time::<Virtual>::default();
    t.pause();
    world.insert_resource(t);
    world.insert_resource(Ran::default());
    let mut schedule = Schedule::default();
    schedule.add_systems(mark.run_if(paused));
    schedule.run(&mut world);
    report::eq("paused condition true while Time::<Virtual> is paused", "", 1, world.resource::<Ran>().0);

    world.resource_mut::<Time<Virtual>>().unpause();
    schedule.run(&mut world);
    report::eq("… false once unpaused", "", 1, world.resource::<Ran>().0);
}
