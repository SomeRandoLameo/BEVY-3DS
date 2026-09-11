//! `Messages<M>`/`MessageWriter`/`MessageReader` — Bevy's `Events` rename in
//! 0.19. Adapted from `bevy_ecs`'s own standalone `examples/events.rs`
//! (`MessageRegistry::register_message` sets up both the registry and the
//! `Messages<M>` resource without needing `bevy_app`).

use super::report;
use bevy_ecs::message::MessageRegistry;
use bevy_ecs::prelude::*;

#[derive(Message, Debug, Clone, PartialEq)]
struct Ping(u32);

/// Messages must be flushed (`message_update_system`) exactly once per frame,
/// ordered before whatever reads/writes them that frame — mirrors
/// `examples/events.rs`'s `EventFlusherSystems` set.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct Flush;

/// Registers `Ping` and schedules the flush system in the `Flush` set.
/// Callers add their reader/writer systems `.after(Flush)`.
fn setup() -> (World, Schedule) {
    let mut world = World::new();
    MessageRegistry::register_message::<Ping>(&mut world);

    let mut schedule = Schedule::default();
    schedule.add_systems(bevy_ecs::message::message_update_system.in_set(Flush));
    (world, schedule)
}

#[derive(Resource, Default)]
struct Seen(Vec<u32>);

#[test]
fn reader_sees_a_message_written_earlier_the_same_frame() {
    let (mut world, mut schedule) = setup();
    world.init_resource::<Seen>();

    fn write(mut w: MessageWriter<Ping>) {
        w.write(Ping(1));
    }
    fn read(mut r: MessageReader<Ping>, mut seen: ResMut<Seen>) {
        seen.0.extend(r.read().map(|p| p.0));
    }
    schedule.add_systems((write.after(Flush), read.after(write)));
    schedule.run(&mut world);
    report::eq("MessageReader sees a message written earlier the same frame", "", vec![1u32], world.resource::<Seen>().0.clone());
}

#[test]
fn reader_ordered_before_the_writer_sees_it_next_frame_instead() {
    let (mut world, mut schedule) = setup();
    world.init_resource::<Seen>();

    fn read(mut r: MessageReader<Ping>, mut seen: ResMut<Seen>) {
        seen.0.extend(r.read().map(|p| p.0));
    }
    fn write(mut w: MessageWriter<Ping>) {
        w.write(Ping(99));
    }
    schedule.add_systems((read.after(Flush), write.after(read)));

    schedule.run(&mut world);
    report::eq("MessageReader ordered before the writer sees nothing yet", "", Vec::<u32>::new(), world.resource::<Seen>().0.clone());

    // Next frame: `Flush` (message_update_system) advances the double buffer
    // before `read` runs, so the previously-written message is now visible.
    schedule.run(&mut world);
    report::eq("… but it's there on the next frame", "message survives one buffer swap", vec![99u32], world.resource::<Seen>().0.clone());
}

#[test]
fn messages_older_than_two_updates_are_dropped() {
    let (mut world, mut schedule) = setup();
    fn write_once(mut w: MessageWriter<Ping>, mut done: Local<bool>) {
        if !*done {
            w.write(Ping(7));
            *done = true;
        }
    }
    schedule.add_systems(write_once.after(Flush));

    // Frame 1: written. Frames 2 and 3's `Flush` each swap the double buffer
    // once more; after the second swap following the write, it's dropped.
    schedule.run(&mut world);
    schedule.run(&mut world);
    schedule.run(&mut world);

    let messages = world.resource::<Messages<Ping>>();
    let mut cursor = messages.get_cursor();
    report::eq("A message not read within 2 buffer-swaps is dropped", "3 frames after the write", 0usize, cursor.read(messages).count());
}

#[test]
fn message_writer_write_batch() {
    let (mut world, mut schedule) = setup();
    world.init_resource::<Seen>();

    fn write_batch(mut w: MessageWriter<Ping>) {
        w.write_batch([Ping(1), Ping(2), Ping(3)]);
    }
    fn read(mut r: MessageReader<Ping>, mut seen: ResMut<Seen>) {
        seen.0.extend(r.read().map(|p| p.0));
    }
    schedule.add_systems((write_batch.after(Flush), read.after(write_batch)));
    schedule.run(&mut world);
    report::eq("MessageWriter::write_batch() delivers all of them in order", "", vec![1u32, 2, 3], world.resource::<Seen>().0.clone());
}
