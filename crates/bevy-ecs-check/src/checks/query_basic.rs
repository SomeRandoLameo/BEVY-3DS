//! `Query<&T>` / `Query<&mut T>` — basic iteration, `get`/`get_mut`, `single`.

use super::report;
use bevy_ecs::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Velocity {
    x: f32,
}

#[test]
fn immutable_iteration_sees_every_matching_entity() {
    let mut world = World::new();
    world.spawn(Position { x: 1.0, y: 0.0 });
    world.spawn(Position { x: 2.0, y: 0.0 });
    world.spawn(Position { x: 3.0, y: 0.0 });
    let sum: f32 = world.query::<&Position>().iter(&world).map(|p| p.x).sum();
    report::f("Query<&Position>::iter().map(..).sum()", "3 entities, x=1,2,3", 6.0, sum);
}

#[test]
fn mutable_iteration_persists_across_two_schedule_runs() {
    fn integrate(mut q: Query<(&mut Position, &Velocity)>) {
        for (mut pos, vel) in &mut q {
            pos.x += vel.x;
        }
    }
    let mut world = World::new();
    let e = world.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.5 })).id();
    let mut schedule = Schedule::default();
    schedule.add_systems(integrate);
    schedule.run(&mut world);
    schedule.run(&mut world);
    report::f("Query<&mut Position> mutation persists across runs", "vel=1.5, 2 runs", 3.0, world.entity(e).get::<Position>().unwrap().x);
}

#[test]
fn get_and_get_mut_by_entity_id() {
    let mut world = World::new();
    let a = world.spawn(Position { x: 1.0, y: 1.0 }).id();
    let b = world.spawn(Position { x: 2.0, y: 2.0 }).id();
    let mut query = world.query::<&mut Position>();
    report::eq("QueryState::get(a)", "", Position { x: 1.0, y: 1.0 }, *query.get(&world, a).unwrap());
    query.get_mut(&mut world, b).unwrap().x = 20.0;
    report::f("QueryState::get_mut(b).x = 20", "", 20.0, world.entity(b).get::<Position>().unwrap().x);
}

#[test]
fn get_on_a_non_matching_entity_errs() {
    let mut world = World::new();
    let no_position = world.spawn_empty().id();
    let mut query = world.query::<&Position>();
    report::is_true("QueryState::get() on an entity missing the component is Err", "", query.get(&world, no_position).is_err());
}

#[test]
fn single_succeeds_with_exactly_one_match_and_errs_otherwise() {
    let mut world = World::new();
    let mut query = world.query::<&Position>();
    report::is_true("QueryState::single() on zero matches is Err", "", query.single(&world).is_err());

    let e = world.spawn(Position { x: 9.0, y: 9.0 }).id();
    let mut query = world.query::<&Position>();
    report::eq("QueryState::single() with exactly one match", "", Position { x: 9.0, y: 9.0 }, *query.single(&world).unwrap());

    world.spawn(Position { x: 0.0, y: 0.0 });
    let mut query = world.query::<&Position>();
    report::is_true("QueryState::single() with two matches is Err", "", query.single(&world).is_err());
    let _ = e; // silence unused warning if the entity id itself isn't otherwise read
}

#[test]
fn query_contains_checks_without_borrowing_data() {
    // `Query::contains` (the live system-param wrapper) is the ergonomic path;
    // exercised here via a system so the ticks it needs come for free.
    #[derive(Resource)]
    struct Seen {
        with: bool,
        without: bool,
    }
    fn check(with_vel: Entity, without_vel: Entity, q: Query<&Velocity>, mut seen: ResMut<Seen>) {
        seen.with = q.contains(with_vel);
        seen.without = q.contains(without_vel);
    }

    let mut world = World::new();
    let with_vel = world.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 0.0 })).id();
    let without_vel = world.spawn(Position { x: 0.0, y: 0.0 }).id();
    world.insert_resource(Seen { with: false, without: true });

    let mut schedule = Schedule::default();
    schedule.add_systems(move |q: Query<&Velocity>, seen: ResMut<Seen>| {
        check(with_vel, without_vel, q, seen);
    });
    schedule.run(&mut world);

    let seen = world.resource::<Seen>();
    report::is_true("Query::contains() true when the component is present", "", seen.with);
    report::is_true("… false when it's not", "", !seen.without);
}
