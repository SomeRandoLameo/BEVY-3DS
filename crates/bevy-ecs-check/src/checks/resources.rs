//! `Resource` — singleton data on the `World`: init/insert/remove, `FromWorld`,
//! and the `Local<T>` system param (per-system, not global, state).

use super::report;
use bevy_ecs::prelude::*;

#[derive(Resource, Debug, Default, PartialEq)]
struct Score(u32);

#[test]
fn insert_get_and_contains() {
    let mut world = World::new();
    report::is_true("World::contains_resource before insert", "", !world.contains_resource::<Score>());
    world.insert_resource(Score(5));
    report::is_true("… after insert", "", world.contains_resource::<Score>());
    report::eq("World::resource::<T>()", "", 5u32, world.resource::<Score>().0);
    report::eq("World::get_resource::<T>() is Some", "", Some(5u32), world.get_resource::<Score>().map(|s| s.0));
}

#[test]
fn init_resource_uses_default() {
    let mut world = World::new();
    world.init_resource::<Score>();
    report::eq("World::init_resource::<T>() uses Default", "", 0u32, world.resource::<Score>().0);
}

#[test]
fn from_world_runs_custom_initialization() {
    #[derive(Resource, Debug, PartialEq)]
    struct Doubled(u32);
    impl FromWorld for Doubled {
        fn from_world(world: &mut World) -> Self {
            // Can read other already-inserted resources while initializing.
            Doubled(world.resource::<Score>().0 * 2)
        }
    }
    let mut world = World::new();
    world.insert_resource(Score(21));
    world.init_resource::<Doubled>();
    report::eq("FromWorld::from_world() can read prior resources", "Score(21) -> Doubled", 42u32, world.resource::<Doubled>().0);
}

#[test]
fn remove_resource() {
    let mut world = World::new();
    world.insert_resource(Score(1));
    let removed = world.remove_resource::<Score>();
    report::eq("World::remove_resource::<T>() returns it", "", Some(1u32), removed.map(|s| s.0));
    report::is_true("… and it's gone", "", !world.contains_resource::<Score>());
}

#[test]
fn res_mut_mutates_in_place() {
    let mut world = World::new();
    world.insert_resource(Score(0));
    fn bump(mut score: ResMut<Score>) {
        score.0 += 1;
    }
    let mut schedule = Schedule::default();
    schedule.add_systems(bump);
    for _ in 0..5 {
        schedule.run(&mut world);
    }
    report::eq("ResMut<T> mutation persists across 5 runs", "", 5u32, world.resource::<Score>().0);
}

#[test]
fn local_state_is_per_system_not_global() {
    #[derive(Resource, Default)]
    struct Calls(Vec<u32>);

    fn counter_a(mut count: Local<u32>, mut calls: ResMut<Calls>) {
        *count += 1;
        calls.0.push(*count);
    }
    fn counter_b(mut count: Local<u32>, mut calls: ResMut<Calls>) {
        *count += 10;
        calls.0.push(*count);
    }

    let mut world = World::new();
    world.init_resource::<Calls>();
    let mut schedule = Schedule::default();
    schedule.add_systems((counter_a, counter_b));
    schedule.run(&mut world);
    schedule.run(&mut world);
    schedule.run(&mut world);

    let mut calls = world.resource::<Calls>().0.clone();
    calls.sort();
    // counter_a: 1, 2, 3 — counter_b: 10, 20, 30 — each Local<u32> starts at 0
    // and accumulates independently of the other system's Local.
    report::eq("Local<u32> is independent per system across 3 runs", "counter_a +1 each run, counter_b +10 each run", vec![1, 2, 3, 10, 20, 30], calls);
}
