//! Change detection — `Added<T>`/`Changed<T>` filters, `Ref<T>`, and
//! `DetectChanges` on components/resources.

use super::report;
use bevy_ecs::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Position(f32);

// NOTE: `Added`/`Changed` "clearing" is tracked per *observer* (a system's own
// last-run tick, advanced by the schedule executor each run) — calling
// `QueryState::iter(&world)` directly, repeatedly, by hand does **not**
// advance that tracking the same way (there's no schedule run to advance the
// world's change tick between two back-to-back manual calls). So both checks
// below go through a real `Schedule`, one run per "frame", matching how
// `Added`/`Changed` are actually meant to be used.

#[test]
fn added_filter_is_true_once_then_clears() {
    #[derive(Resource, Default)]
    struct Seen(Vec<bool>);
    fn observe(q: Query<Entity, Added<Position>>, mut seen: ResMut<Seen>) {
        seen.0.push(!q.is_empty());
    }

    let mut world = World::new();
    world.init_resource::<Seen>();
    let mut schedule = Schedule::default();
    schedule.add_systems(observe);

    world.spawn(Position(1.0));
    schedule.run(&mut world); // frame right after the spawn
    schedule.run(&mut world); // a later, untouched frame
    schedule.run(&mut world);

    report::eq("Added<Position>: true the frame right after spawn, then false", "", vec![true, false, false], world.resource::<Seen>().0.clone());
}

#[test]
fn changed_filter_fires_on_mutation_and_then_clears() {
    #[derive(Resource, Default)]
    struct Seen(Vec<bool>);
    fn observe(q: Query<Entity, Changed<Position>>, mut seen: ResMut<Seen>) {
        seen.0.push(!q.is_empty());
    }

    let mut world = World::new();
    world.init_resource::<Seen>();
    let e = world.spawn(Position(0.0)).id();
    let mut schedule = Schedule::default();
    schedule.add_systems(observe);

    schedule.run(&mut world); // spawning counts as a change too
    schedule.run(&mut world); // nothing changed since — clears

    world.entity_mut(e).get_mut::<Position>().unwrap().0 = 5.0;
    schedule.run(&mut world); // real mutation
    schedule.run(&mut world); // settled again

    // Obtaining a `Mut<T>` handle via `get_mut()` alone, without ever actually
    // touching it, does *not* mark the component changed — the tracking is
    // lazy, triggered by `Mut`'s `DerefMut` impl, not by `get_mut()` itself.
    let _ = world.entity_mut(e).get_mut::<Position>();
    schedule.run(&mut world);

    // But assigning the component's *current* value back to itself — a
    // genuine no-op data-wise — still marks it changed, because that
    // assignment *does* go through `DerefMut`. bevy_ecs tracks "was this
    // written to", not "did the value actually differ".
    let current = world.entity_mut(e).get_mut::<Position>().unwrap().0;
    world.entity_mut(e).get_mut::<Position>().unwrap().0 = current;
    schedule.run(&mut world);

    report::eq(
        "Changed<Position>: spawn, mutation, and a same-value reassignment each show up once — but an untouched Mut<T> handle doesn't",
        "DerefMut is the trigger, not data equality",
        vec![true, false, true, false, false, true],
        world.resource::<Seen>().0.clone(),
    );
}

#[test]
fn ref_gives_read_access_plus_is_added_is_changed() {
    fn observe(q: Query<Ref<Position>>, mut out: ResMut<Seen>) {
        let r = q.single().unwrap();
        out.added = r.is_added();
        out.changed = r.is_changed();
        out.value = r.0;
    }
    #[derive(Resource, Default)]
    struct Seen {
        added: bool,
        changed: bool,
        value: f32,
    }

    let mut world = World::new();
    world.insert_resource(Seen::default());
    world.spawn(Position(7.0));

    let mut schedule = Schedule::default();
    schedule.add_systems(observe);
    schedule.run(&mut world);
    let seen = world.resource::<Seen>();
    report::is_true("Ref<T>::is_added() true on the run right after spawn", "", seen.added);
    report::is_true("Ref<T>::is_changed() true too (spawn counts)", "", seen.changed);
    report::f("Ref<T> derefs to the component", "", 7.0, seen.value);

    schedule.run(&mut world);
    let seen = world.resource::<Seen>();
    report::is_true("… is_added() false on a later, untouched run", "", !seen.added);
    report::is_true("… is_changed() false too", "", !seen.changed);
}

#[test]
fn resource_is_changed_after_res_mut_access() {
    #[derive(Resource)]
    struct Score(u32);

    let mut world = World::new();
    world.insert_resource(Score(0));

    fn bump(mut score: ResMut<Score>) {
        score.0 += 1;
    }
    fn observe(score: Res<Score>, mut out: ResMut<Seen>) {
        out.0 = score.is_changed();
    }
    #[derive(Resource, Default)]
    struct Seen(bool);
    world.insert_resource(Seen::default());

    let mut schedule = Schedule::default();
    schedule.add_systems((bump, observe).chain());
    schedule.run(&mut world);
    report::is_true("Res<T>::is_changed() true right after a ResMut write", "", world.resource::<Seen>().0);
}
