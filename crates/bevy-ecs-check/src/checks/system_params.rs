//! `ParamSet` (conflicting query/resource access in one system),
//! `RemovedComponents`, and exclusive systems (`&mut World` as a parameter).

use super::report;
use bevy_ecs::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Position(f32);
#[derive(Component)]
struct Marker;

#[test]
fn param_set_lets_one_system_hold_two_conflicting_queries() {
    fn touch(mut set: ParamSet<(Query<&mut Position>, Query<&Position, With<Marker>>)>) {
        // p0: mutable access to every Position.
        for mut p in set.p0().iter_mut() {
            p.0 += 1.0;
        }
        // p1: read-only access, but only to `Position`s with `Marker` — would
        // conflict with p0 if both were live query borrows at once; `ParamSet`
        // makes that legal by only ever handing out one side at a time.
        let marked_count = set.p1().iter().count();
        assert_eq!(marked_count, 1);
    }

    let mut world = World::new();
    world.spawn(Position(0.0));
    world.spawn((Position(0.0), Marker));
    let mut schedule = Schedule::default();
    schedule.add_systems(touch);
    schedule.run(&mut world);

    let total: f32 = world.query::<&Position>().iter(&world).map(|p| p.0).sum();
    report::f("ParamSet lets p0 (mutable, all) and p1 (read-only, filtered) coexist", "2 entities bumped by p0", 2.0, total);
}

#[test]
fn removed_components_reports_entities_that_lost_a_component() {
    fn track(mut removed: RemovedComponents<Marker>, mut log: ResMut<Log>) {
        log.0.extend(removed.read());
    }
    #[derive(Resource, Default)]
    struct Log(Vec<Entity>);

    let mut world = World::new();
    world.init_resource::<Log>();
    let e = world.spawn(Marker).id();
    let mut schedule = Schedule::default();
    schedule.add_systems(track);

    schedule.run(&mut world);
    report::eq("RemovedComponents<T> sees nothing before any removal", "", 0usize, world.resource::<Log>().0.len());

    world.entity_mut(e).remove::<Marker>();
    schedule.run(&mut world);
    report::eq("… reports the entity the frame after Marker was removed", "", vec![e], world.resource::<Log>().0.clone());
}

#[test]
fn removed_components_also_fires_on_despawn() {
    fn track(mut removed: RemovedComponents<Marker>, mut log: ResMut<Log>) {
        log.0.extend(removed.read());
    }
    #[derive(Resource, Default)]
    struct Log(Vec<Entity>);

    let mut world = World::new();
    world.init_resource::<Log>();
    let e = world.spawn(Marker).id();
    world.despawn(e);

    let mut schedule = Schedule::default();
    schedule.add_systems(track);
    schedule.run(&mut world);
    report::eq("Despawning an entity with Marker also shows up in RemovedComponents<Marker>", "", vec![e], world.resource::<Log>().0.clone());
}

#[test]
fn exclusive_system_gets_unrestricted_world_access() {
    fn exclusive(world: &mut World) {
        world.spawn(Position(7.0));
        let count = world.query::<&Position>().iter(world).count();
        world.insert_resource(Seen(count));
    }
    #[derive(Resource)]
    struct Seen(usize);

    let mut world = World::new();
    let mut schedule = Schedule::default();
    schedule.add_systems(exclusive);
    schedule.run(&mut world);
    report::eq("An exclusive system (&mut World) can spawn and immediately query in the same call", "", 1usize, world.resource::<Seen>().0);
}
