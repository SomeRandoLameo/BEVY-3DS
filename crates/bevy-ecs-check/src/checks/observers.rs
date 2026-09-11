//! Observers — `World::add_observer`/`World::trigger` for custom `Event`s,
//! and the built-in component-lifecycle events `On<Add, T>`/`On<Insert, T>`/
//! `On<Remove, T>` (adapted from `Observer`'s own doc examples).

use super::report;
use bevy_ecs::prelude::*;

#[derive(Component)]
struct A;

#[derive(Resource, Default)]
struct Log(Vec<&'static str>);

#[test]
fn custom_event_reaches_its_observer() {
    #[derive(Event)]
    struct Speak(&'static str);

    let mut world = World::new();
    world.init_resource::<Log>();
    world.add_observer(|event: On<Speak>, mut log: ResMut<Log>| {
        log.0.push(event.0);
    });
    world.trigger(Speak("hello"));
    report::eq("World::trigger() runs the matching observer synchronously", "", vec!["hello"], world.resource::<Log>().0.clone());
}

#[test]
fn observer_can_query_and_use_commands() {
    #[derive(Event)]
    struct SpawnThing;
    #[derive(Component)]
    struct Thing;

    let mut world = World::new();
    world.add_observer(|_: On<SpawnThing>, mut commands: Commands| {
        commands.spawn(Thing);
    });
    world.trigger(SpawnThing);
    // Unlike a system's `Commands` in a `Schedule` (auto-flushed by the
    // executor), an observer's deferred `Commands` need an explicit
    // `World::flush()` — `trigger()` itself doesn't apply them.
    world.flush();
    report::eq("An observer's Commands::spawn() lands in the World after World::flush()", "", 1usize, world.query::<&Thing>().iter(&world).count());
}

#[test]
fn on_add_fires_exactly_once_per_insertion_not_per_later_mutation() {
    let mut world = World::new();
    world.init_resource::<Log>();
    world.add_observer(|_: On<Add, A>, mut log: ResMut<Log>| log.0.push("add"));

    let e = world.spawn(A).id();
    report::eq("On<Add, A> fires when A is first inserted (spawn)", "", vec!["add"], world.resource::<Log>().0.clone());

    world.entity_mut(e).remove::<A>();
    world.entity_mut(e).insert(A);
    report::eq("… and again on a fresh insert after removal", "", vec!["add", "add"], world.resource::<Log>().0.clone());
}

#[test]
fn on_insert_fires_on_every_insert_including_overwrites() {
    #[derive(Component)]
    #[allow(dead_code, reason = "only its presence, not its value, matters for this check")]
    struct Val(u32);

    let mut world = World::new();
    world.init_resource::<Log>();
    world.add_observer(|_: On<Insert, Val>, mut log: ResMut<Log>| log.0.push("insert"));

    let e = world.spawn(Val(1)).id();
    world.entity_mut(e).insert(Val(2)); // overwrite, still "insert"
    report::eq("On<Insert, T> fires on the initial spawn and on overwriting inserts", "", vec!["insert", "insert"], world.resource::<Log>().0.clone());
}

#[test]
fn on_remove_fires_before_the_component_is_gone() {
    let mut world = World::new();
    world.init_resource::<Log>();
    world.add_observer(|_: On<Remove, A>, mut log: ResMut<Log>| log.0.push("remove"));

    let e = world.spawn(A).id();
    world.entity_mut(e).remove::<A>();
    report::eq("On<Remove, A> fires when A is removed", "", vec!["remove"], world.resource::<Log>().0.clone());

    // Despawning an entity also removes its components, so it fires too.
    let e2 = world.spawn(A).id();
    world.entity_mut(e2).despawn();
    report::eq("… and despawn() counts as a removal", "", vec!["remove", "remove"], world.resource::<Log>().0.clone());
}

#[test]
fn add_insert_remove_fire_in_the_documented_order() {
    let mut world = World::new();
    world.init_resource::<Log>();
    world.add_observer(|_: On<Add, A>, mut log: ResMut<Log>| log.0.push("add"));
    world.add_observer(|_: On<Insert, A>, mut log: ResMut<Log>| log.0.push("insert"));
    world.add_observer(|_: On<Remove, A>, mut log: ResMut<Log>| log.0.push("remove"));

    let e = world.spawn(A).id();
    world.entity_mut(e).despawn();
    report::eq("A single spawn+despawn cycle: Add, then Insert, then Remove", "", vec!["add", "insert", "remove"], world.resource::<Log>().0.clone());
}
