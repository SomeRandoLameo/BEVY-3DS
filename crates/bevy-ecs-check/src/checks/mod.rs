//! Core `bevy_ecs` checks: spawn/despawn, `Query` iteration (incl. `&mut`),
//! `Resource` accumulation, and a `Schedule` of several systems.
//!
//! Kept self-contained (helpers + `report` live here, not in `lib.rs`) so
//! `crates/all-checks` can `#[path]`-include this tree unchanged.

mod report;

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

/// Build a tiny world, step the schedule `steps` times, report `(StepCount,
/// #Position entities, moving entity's Position)`.
pub fn run(steps: u32) -> (u32, usize, Position) {
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

    (
        world.resource::<StepCount>().0,
        world.query::<&Position>().iter(&world).count(),
        *world.entity(mover).get::<Position>().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_spawn_and_query() {
        let mut world = World::new();
        let e = world.spawn(Position { x: 1.0, y: 2.0 }).id();
        world.spawn(Position { x: 3.0, y: 4.0 });

        report::eq(
            "World::spawn + Query::<&Position>::iter().count()",
            "2 entities spawned",
            2usize,
            world.query::<&Position>().iter(&world).count(),
        );
        report::eq(
            "EntityRef::get::<Position>()",
            "spawn(Position { x: 1, y: 2 })",
            Position { x: 1.0, y: 2.0 },
            *world.entity(e).get::<Position>().unwrap(),
        );
    }

    #[test]
    fn world_despawn() {
        let mut world = World::new();
        let a = world.spawn(Position { x: 0.0, y: 0.0 }).id();
        world.spawn(Position { x: 0.0, y: 0.0 });
        let removed = world.despawn(a);
        let left = world.query::<&Position>().iter(&world).count();
        report::ok(
            "World::despawn(entity)",
            "2 spawned, despawn 1",
            "true, 1 left",
            &format!("{removed}, {left} left"),
            removed && left == 1,
        );
    }

    #[test]
    fn mutable_query_iteration() {
        let mut world = World::new();
        let e = world
            .spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.5, y: -0.5 }))
            .id();
        let mut schedule = Schedule::default();
        schedule.add_systems(integrate);
        schedule.run(&mut world);
        schedule.run(&mut world);
        report::eq(
            "Query<(&mut Position, &Velocity)> — system mutates over 2 runs",
            "pos=(0,0), vel=(1.5,-0.5), 2 steps",
            Position { x: 3.0, y: -1.0 },
            *world.entity(e).get::<Position>().unwrap(),
        );
    }

    #[test]
    fn resource_mut_accumulates() {
        let mut world = World::new();
        world.init_resource::<StepCount>();
        let mut schedule = Schedule::default();
        schedule.add_systems(tick);
        for _ in 0..7 {
            schedule.run(&mut world);
        }
        report::eq(
            "ResMut<StepCount> — steps.0 += 1 per run",
            "7 schedule runs",
            7u32,
            world.resource::<StepCount>().0,
        );
    }

    #[test]
    fn schedule_with_multiple_systems() {
        let (steps, positions, mover) = run(5);
        report::eq(
            "Schedule::add_systems((integrate, tick)) + run() x5 — StepCount",
            "5 runs",
            5u32,
            steps,
        );
        report::eq(
            "Query<&Position>::iter().count() after run",
            "3 Position entities",
            3usize,
            positions,
        );
        report::eq(
            "integrate() on the moving entity after 5 runs",
            "pos=(0,0), vel=(1,-2), 5 steps",
            Position { x: 5.0, y: -10.0 },
            mover,
        );
    }

    #[test]
    fn query_filtered_with() {
        let mut world = World::new();
        world.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 0.0, y: 0.0 }));
        world.spawn(Position { x: 0.0, y: 0.0 });
        report::eq(
            "Query<Entity, With<Velocity>>::iter().count()",
            "1 of 2 entities has Velocity",
            1usize,
            world
                .query_filtered::<Entity, With<Velocity>>()
                .iter(&world)
                .count(),
        );
    }
}
