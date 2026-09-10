//! Standalone check for `bevy_ecs` on the 3DS target (`armv6k-nintendo-3ds`).
//!
//! No `bevy_render`, no `App` — just core ECS: spawn, `Query` iteration with a
//! `&mut` component, a `Resource`, and a `Schedule` with several systems, run for
//! a few steps. Enough to (a) force monomorphisation so "it compiled" means
//! something and (b) confirm the ECS actually *behaves* when run on-device.
//!
//! Runs on the emulator / hardware via `cargo 3ds test -p bevy-ecs-check`
//! (see `../../scripts/test-emulator.sh`).

#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(test_runner::run_gdb))]

use bevy_ecs::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Resource, Default, Debug)]
struct StepCount(u32);

fn integrate(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in &mut query {
        pos.x += vel.x;
        pos.y += vel.y;
    }
}

fn tick(mut steps: ResMut<StepCount>) {
    steps.0 += 1;
}

/// What `run()` observed, so tests can assert on real ECS behaviour.
#[derive(Debug, PartialEq)]
pub struct SmokeReport {
    /// Value of the `StepCount` resource after running.
    pub steps: u32,
    /// Number of entities matched by `Query<&Position>`.
    pub positions: usize,
    /// Final position of the one entity that also had a `Velocity`.
    pub moving_entity: Position,
}

/// Build a tiny world and step the schedule `steps` times.
pub fn run(steps: u32) -> SmokeReport {
    let mut world = World::new();
    world.init_resource::<StepCount>();

    let mover = world
        .spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: -2.0 }))
        .id();
    world.spawn(Position { x: 5.0, y: 5.0 });
    world.spawn(Position { x: -1.0, y: 3.0 });

    let mut schedule = Schedule::default();
    schedule.add_systems((integrate, tick));

    for _ in 0..steps {
        schedule.run(&mut world);
    }

    SmokeReport {
        steps: world.resource::<StepCount>().0,
        positions: world.query::<&Position>().iter(&world).count(),
        moving_entity: *world.entity(mover).get::<Position>().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schedule_runs_the_expected_number_of_steps() {
        assert_eq!(run(10).steps, 10);
    }

    #[test]
    fn query_sees_every_position_entity() {
        assert_eq!(run(1).positions, 3);
    }

    #[test]
    fn mutable_query_integrates_velocity_each_step() {
        let report = run(5);
        assert_eq!(report.moving_entity, Position { x: 5.0, y: -10.0 });
    }
}
