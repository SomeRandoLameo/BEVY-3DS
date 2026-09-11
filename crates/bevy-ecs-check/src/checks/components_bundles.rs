//! `Component`/`Bundle` derives — marker/tuple-struct components, `Bundle`,
//! and required components (`#[require(...)]`).

use super::report;
use bevy_ecs::prelude::*;

#[derive(Component)]
struct Marker;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Position(f32, f32);

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Velocity(f32, f32);

#[derive(Bundle)]
struct Moving {
    pos: Position,
    vel: Velocity,
}

#[test]
fn bundle_derive_spawns_every_field_as_a_component() {
    let mut world = World::new();
    let e = world
        .spawn(Moving { pos: Position(1.0, 2.0), vel: Velocity(0.5, -0.5) })
        .id();
    report::eq("Bundle field 1 (Position) landed", "", Position(1.0, 2.0), *world.entity(e).get::<Position>().unwrap());
    report::eq("Bundle field 2 (Velocity) landed", "", Velocity(0.5, -0.5), *world.entity(e).get::<Velocity>().unwrap());
}

#[test]
fn marker_unit_component() {
    let mut world = World::new();
    let e = world.spawn(Marker).id();
    report::is_true("Unit-struct Component derive", "", world.entity(e).contains::<Marker>());
}

#[test]
fn required_component_with_default_is_auto_inserted() {
    #[derive(Component, Default, Debug, Clone, Copy, PartialEq)]
    struct Team(u32);

    #[derive(Component)]
    #[require(Team)]
    struct Soldier;

    let mut world = World::new();
    let e = world.spawn(Soldier).id();
    report::is_true("#[require(Team)] auto-inserts Team on spawn(Soldier)", "", world.entity(e).contains::<Team>());
    report::eq("… using Team::default()", "", Team(0), *world.entity(e).get::<Team>().unwrap());
}

#[test]
fn required_component_with_custom_initializer() {
    #[derive(Component, Debug, Clone, Copy, PartialEq)]
    struct Health(u32);

    #[derive(Component)]
    #[require(Health = Health(100))]
    struct Player;

    let mut world = World::new();
    let e = world.spawn(Player).id();
    report::eq("#[require(Health = Health(100))] uses the custom initializer", "", Health(100), *world.entity(e).get::<Health>().unwrap());
}

#[test]
fn explicitly_provided_component_overrides_the_required_default() {
    #[derive(Component, Default, Debug, Clone, Copy, PartialEq)]
    struct Team(u32);
    #[derive(Component)]
    #[require(Team)]
    struct Soldier;

    let mut world = World::new();
    // Bundle order doesn't matter — an explicitly-provided Team wins over the
    // auto-inserted default either way.
    let e = world.spawn((Soldier, Team(7))).id();
    report::eq("Explicit Team(7) wins over #[require(Team)]'s default", "", Team(7), *world.entity(e).get::<Team>().unwrap());
}

#[test]
fn transitive_required_components() {
    #[derive(Component, Default, Debug, Clone, Copy, PartialEq)]
    struct C(u32);
    #[derive(Component, Default)]
    #[require(C)]
    struct B;
    #[derive(Component)]
    #[require(B)]
    struct A;

    let mut world = World::new();
    let e = world.spawn(A).id();
    report::is_true("A requires B", "", world.entity(e).contains::<B>());
    report::is_true("… B (transitively) requires C", "", world.entity(e).contains::<C>());
    report::eq("… C got its default", "", C(0), *world.entity(e).get::<C>().unwrap());
}
