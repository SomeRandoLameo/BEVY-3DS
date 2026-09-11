//! `EntityRef`/`EntityMut`/`EntityWorldMut` — the entity-scoped views, and
//! `Entity` itself (index/equality).

use super::report;
use bevy_ecs::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Position(f32);

#[test]
fn entity_ref_get_and_contains() {
    let mut world = World::new();
    let e = world.spawn(Position(1.0)).id();
    let r = world.entity(e);
    report::is_true("EntityRef::contains::<T>()", "", r.contains::<Position>());
    report::eq("EntityRef::get::<T>()", "", Position(1.0), *r.get::<Position>().unwrap());
    report::eq("EntityRef::id()", "", e, r.id());
}

#[test]
fn entity_world_mut_get_mut() {
    let mut world = World::new();
    let e = world.spawn(Position(1.0)).id();
    world.entity_mut(e).get_mut::<Position>().unwrap().0 = 9.0;
    report::f("EntityWorldMut::get_mut::<T>() mutates in place", "", 9.0, world.entity(e).get::<Position>().unwrap().0);
}

#[test]
fn entity_world_mut_take_removes_and_returns_owned() {
    let mut world = World::new();
    let e = world.spawn(Position(5.0)).id();
    let taken = world.entity_mut(e).take::<Position>();
    report::eq("EntityWorldMut::take::<T>() returns the owned value", "", Some(Position(5.0)), taken);
    report::is_true("… and removes it from the entity", "", !world.entity(e).contains::<Position>());
}

#[test]
fn entity_world_mut_insert_if_new_does_not_overwrite() {
    let mut world = World::new();
    let e = world.spawn(Position(1.0)).id();
    world.entity_mut(e).insert_if_new(Position(2.0));
    report::eq("EntityWorldMut::insert_if_new() keeps the existing value", "already had Position(1.0)", Position(1.0), *world.entity(e).get::<Position>().unwrap());

    let empty = world.spawn_empty().id();
    world.entity_mut(empty).insert_if_new(Position(3.0));
    report::eq("… but does insert when the component is absent", "", Position(3.0), *world.entity(empty).get::<Position>().unwrap());
}

#[test]
fn entity_index_and_equality() {
    let mut world = World::new();
    let a = world.spawn_empty().id();
    let b = world.spawn_empty().id();
    report::is_true("Entity::index() differs between two live entities", "", a.index() != b.index());
    report::is_true("Entity == Entity by value", "", a == a);
    report::is_true("… different entities compare unequal", "", a != b);
}

#[test]
fn get_entity_mut_allows_mutation_via_world() {
    let mut world = World::new();
    let e = world.spawn(Position(0.0)).id();
    world.get_entity_mut(e).unwrap().get_mut::<Position>().unwrap().0 = 42.0;
    report::f("World::get_entity_mut(e) -> mutation round-trips", "", 42.0, world.entity(e).get::<Position>().unwrap().0);
    let despawned = world.spawn_empty().id();
    world.despawn(despawned);
    report::is_true("World::get_entity_mut() on a despawned entity is Err", "", world.get_entity_mut(despawned).is_err());
}
