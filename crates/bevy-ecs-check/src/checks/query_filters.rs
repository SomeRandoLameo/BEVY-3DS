//! Query filters — `With`/`Without`, `Or`, `AnyOf`, `Has`.

use super::report;
use bevy_ecs::prelude::*;

#[derive(Component)]
struct A;
#[derive(Component)]
struct B;
#[derive(Component)]
struct C;

#[test]
fn with_and_without() {
    let mut world = World::new();
    // `Without<A>` (unlike `With<A>`) also matches the hidden resource-backing
    // entities every `World` carries in 0.19 (see `world.rs`'s layout test) —
    // so baseline it before spawning rather than assuming a fresh World has
    // exactly zero entities lacking A.
    let without_a_baseline = world.query_filtered::<Entity, Without<A>>().iter(&world).count();

    world.spawn((A, B));
    world.spawn(A);
    world.spawn(B);
    report::eq("Query<Entity, With<A>>::iter().count()", "2 of 3 have A", 2usize, world.query_filtered::<Entity, With<A>>().iter(&world).count());
    report::eq("Query<Entity, Without<A>>::iter().count() (relative to baseline)", "1 of 3 lacks A", without_a_baseline + 1, world.query_filtered::<Entity, Without<A>>().iter(&world).count());
    report::eq("Query<Entity, (With<A>, With<B>)>::iter().count()", "1 has both", 1usize, world.query_filtered::<Entity, (With<A>, With<B>)>().iter(&world).count());
}

#[test]
fn or_filter_matches_either_side() {
    let mut world = World::new();
    world.spawn(A);
    world.spawn(B);
    world.spawn(C);
    let count = world.query_filtered::<Entity, Or<(With<A>, With<B>)>>().iter(&world).count();
    report::eq("Query<Entity, Or<(With<A>, With<B>)>>::iter().count()", "A-only, B-only, C-only spawned", 2usize, count);
}

#[test]
fn any_of_returns_options_for_each_side() {
    let mut world = World::new();
    #[derive(Component, Debug, PartialEq)]
    struct Va(u32);
    #[derive(Component, Debug, PartialEq)]
    struct Vb(u32);

    world.spawn(Va(1));
    world.spawn(Vb(2));
    world.spawn((Va(3), Vb(4)));

    let mut query = world.query::<AnyOf<(&Va, &Vb)>>();
    let mut results: Vec<(Option<u32>, Option<u32>)> =
        query.iter(&world).map(|(a, b)| (a.map(|v| v.0), b.map(|v| v.0))).collect();
    results.sort();
    report::eq("Query<AnyOf<(&Va, &Vb)>> sees all 3, with the missing side as None", "", vec![(None, Some(2)), (Some(1), None), (Some(3), Some(4))], results);
}

#[test]
fn has_reports_presence_as_a_bool_without_borrowing_the_component() {
    let mut world = World::new();
    // An unfiltered `Query<Entity, ..>` also visits 0.19's hidden
    // resource-backing entities (see `world.rs`) — baseline first.
    let baseline = world.query::<(Entity, Has<A>)>().iter(&world).count();

    world.spawn(A);
    world.spawn_empty();

    let mut query = world.query::<(Entity, Has<A>)>();
    let has_count = query.iter(&world).filter(|(_, has)| *has).count();
    report::eq("Query<(Entity, Has<A>)> — entities reporting has=true", "1 of 2 has A", 1usize, has_count);
    report::eq("… every entity is still visited (Has isn't a filter), relative to baseline", "", baseline + 2, query.iter(&world).count());
}

#[test]
fn query_filtered_reused_after_spawning_more() {
    let mut world = World::new();
    world.spawn(A);
    let mut query = world.query_filtered::<Entity, With<A>>();
    report::eq("query_filtered count before more spawns", "", 1usize, query.iter(&world).count());
    world.spawn(A);
    report::eq("… re-querying the same World sees the new entity too", "", 2usize, query.iter(&world).count());
}
