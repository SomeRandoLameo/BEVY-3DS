//! `Commands` — the deferred mutation API systems use instead of `&mut World`.

use super::report;
use bevy_ecs::prelude::*;
use bevy_ecs::world::CommandQueue;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Marker;

#[test]
fn commands_are_deferred_until_the_queue_is_applied() {
    let mut world = World::new();
    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, &world);
    let e = commands.spawn(Position { x: 1.0, y: 1.0 }).id();

    report::is_true("Commands::spawn() doesn't touch the World yet", "before CommandQueue::apply", world.get_entity(e).is_err());
    queue.apply(&mut world);
    report::is_true("… CommandQueue::apply() makes it real", "", world.get_entity(e).is_ok());
    report::eq("… with the spawned data", "", Position { x: 1.0, y: 1.0 }, *world.entity(e).get::<Position>().unwrap());
}

#[test]
fn commands_applied_after_a_schedule_run() {
    fn spawn_one(mut commands: Commands) {
        commands.spawn(Position { x: 3.0, y: 4.0 });
    }
    let mut world = World::new();
    let mut schedule = Schedule::default();
    schedule.add_systems(spawn_one);
    schedule.run(&mut world);
    report::eq("A Commands::spawn() inside a system is visible once schedule.run() returns", "", 1usize, world.query::<&Position>().iter(&world).count());
}

#[test]
fn entity_commands_insert_and_remove() {
    let mut world = World::new();
    let mut queue = CommandQueue::default();
    let e = {
        let mut commands = Commands::new(&mut queue, &world);
        commands.spawn_empty().id()
    };
    queue.apply(&mut world);

    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, &world);
    commands.entity(e).insert(Marker);
    queue.apply(&mut world);
    report::is_true("Commands::entity(e).insert()", "", world.entity(e).contains::<Marker>());

    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, &world);
    commands.entity(e).remove::<Marker>();
    queue.apply(&mut world);
    report::is_true("Commands::entity(e).remove()", "", !world.entity(e).contains::<Marker>());
}

#[test]
fn entity_commands_despawn() {
    let mut world = World::new();
    let e = world.spawn_empty().id();
    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, &world);
    commands.entity(e).despawn();
    queue.apply(&mut world);
    report::is_true("Commands::entity(e).despawn()", "", world.get_entity(e).is_err());
}

#[test]
fn commands_spawn_batch() {
    let mut world = World::new();
    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, &world);
    commands.spawn_batch((0..4).map(|i| Position { x: i as f32, y: 0.0 }));
    queue.apply(&mut world);
    report::eq("Commands::spawn_batch(4).len()", "", 4usize, world.query::<&Position>().iter(&world).count());
}

#[test]
fn commands_insert_and_remove_resource() {
    #[derive(Resource, Debug, PartialEq)]
    struct Score(u32);

    let mut world = World::new();
    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, &world);
    commands.insert_resource(Score(10));
    queue.apply(&mut world);
    report::eq("Commands::insert_resource()", "", 10u32, world.resource::<Score>().0);

    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, &world);
    commands.remove_resource::<Score>();
    queue.apply(&mut world);
    report::is_true("Commands::remove_resource()", "", !world.contains_resource::<Score>());
}
