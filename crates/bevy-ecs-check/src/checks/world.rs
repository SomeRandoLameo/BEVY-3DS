//! `World` — the raw storage API: spawn/despawn, entity lookups, batch ops.

use super::report;
use bevy_ecs::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Marker;

#[test]
fn spawn_and_get_entity() {
    let mut world = World::new();
    let e = world.spawn(Position { x: 1.0, y: 2.0 }).id();
    report::eq("World::entity(e).get::<T>()", "spawn(Position{1,2})", Position { x: 1.0, y: 2.0 }, *world.entity(e).get::<Position>().unwrap());
    report::is_true("World::get_entity(e) is Ok for a live entity", "", world.get_entity(e).is_ok());
}

#[test]
fn despawn_removes_the_entity() {
    let mut world = World::new();
    let a = world.spawn(Position { x: 0.0, y: 0.0 }).id();
    let b = world.spawn(Position { x: 0.0, y: 0.0 }).id();
    let removed = world.despawn(a);
    report::is_true("World::despawn(entity) returns true for a live entity", "", removed);
    report::is_true("… the entity is now gone (get_entity errs)", "", world.get_entity(a).is_err());
    report::is_true("… a different entity is untouched", "", world.get_entity(b).is_ok());
}

#[test]
fn despawn_a_missing_entity_returns_false() {
    let mut world = World::new();
    let e = world.spawn_empty().id();
    world.despawn(e);
    report::is_true("World::despawn() on an already-despawned entity returns false", "", !world.despawn(e));
}

#[test]
fn spawn_batch_creates_every_entity() {
    let mut world = World::new();
    let entities: Vec<Entity> = world
        .spawn_batch((0..5).map(|i| Position { x: i as f32, y: 0.0 }))
        .collect();
    report::eq("World::spawn_batch(5 positions).count()", "", 5, entities.len());
    report::eq("… Query count matches", "", 5usize, world.query::<&Position>().iter(&world).count());
    report::eq("… last entity's Position", "", Position { x: 4.0, y: 0.0 }, *world.entity(entities[4]).get::<Position>().unwrap());
}

/// In 0.19, resources are stored as components on hidden per-resource
/// entities (`World::resource_entities()`) — so `clear_entities()`, which
/// despawns *every* entity, wipes resources too. This is explicitly called
/// out in `World::clear_entities`'s own doc ("This includes all resources, as
/// they are stored as components"), which contradicts the older-Bevy
/// intuition that `clear_entities` only touches spawned game entities.
#[test]
fn clear_entities_also_wipes_resources_now() {
    #[derive(Resource)]
    #[allow(dead_code, reason = "only whether the resource itself survives is checked")]
    struct Kept(u32);

    let mut world = World::new();
    world.insert_resource(Kept(7));
    world.spawn(Position { x: 0.0, y: 0.0 });
    world.spawn(Position { x: 0.0, y: 0.0 });
    world.clear_entities();
    report::eq("World::clear_entities() -> no Position entities left", "2 spawned", 0usize, world.query::<&Position>().iter(&world).count());
    report::is_true("… and resources are gone too (they're entity-backed)", "not the pre-0.19 behaviour", !world.contains_resource::<Kept>());
}

#[test]
fn entity_mut_inserts_and_removes_components() {
    let mut world = World::new();
    let e = world.spawn(Position { x: 0.0, y: 0.0 }).id();
    world.entity_mut(e).insert(Marker);
    report::is_true("EntityWorldMut::insert() adds a component", "", world.entity(e).contains::<Marker>());
    world.entity_mut(e).remove::<Marker>();
    report::is_true("EntityWorldMut::remove() removes it again", "", !world.entity(e).contains::<Marker>());
}

/// `Entities::len()` is **not** a live-entity count — it's the total number
/// of entity slots ever allocated (it doesn't shrink on despawn, since freed
/// slots are recycled by generation-bump, not removed). Internally `len()` is
/// `self.meta.len()`, and `ensure_index_index_is_valid`'s `expand()` grows
/// that `Vec` by resizing it to `meta.capacity()` (not just to the index that
/// was actually needed) — so `len()` jumps ahead in Vec-growth-strategy-sized
/// chunks, not one slot per spawned entity. That makes the exact chunk size
/// an implementation detail that isn't safe to assert on; what's guaranteed
/// (and checked below) is that `len()` never shrinks, even across a despawn,
/// and is always `>= count_spawned()`. A fresh `World::new()` already has
/// `len() > count_spawned()` before any user code runs, because its own
/// `bootstrap()` (which `init_resource`s `DefaultQueryFilters`, since
/// resources are entity-backed in 0.19) allocates a few scratch entities and
/// frees them again. `count_spawned()` (documented as "intended only ... for
/// tests") is the actual live count, and — unlike `len()` — grows/shrinks
/// precisely one-for-one with spawns/despawns.
#[test]
fn entities_len_is_a_high_water_mark_not_a_live_count() {
    let mut world = World::new();
    let baseline_len = world.entities().len();
    let baseline_spawned = world.entities().count_spawned();
    report::is_true("World::new() already has >= 1 live entity (resource-backing)", "bootstrap's DefaultQueryFilters resource", baseline_spawned >= 1);
    report::is_true("… and len() (slots ever allocated) is >= count_spawned() (live count)", "bootstrap frees some scratch slots along the way", baseline_len >= baseline_spawned);

    world.spawn_empty();
    world.spawn_empty();
    let e = world.spawn_empty().id();
    report::eq("count_spawned() grows by exactly 3 after 3 spawns", "unlike len(), it's a precise live count", baseline_spawned + 3, world.entities().count_spawned());
    report::is_true("… and len() has kept pace (still >= count_spawned())", "", world.entities().len() >= baseline_len);

    let len_after_spawns = world.entities().len();
    world.despawn(e);
    report::eq("… count_spawned() shrinks by 1 on despawn", "", baseline_spawned + 2, world.entities().count_spawned());
    report::eq("… but len() (slots ever allocated) does not shrink", "despawning frees the slot for reuse, it doesn't remove it", len_after_spawns, world.entities().len());
}

#[test]
fn get_entity_batch_partial_failure() {
    let mut world = World::new();
    let a = world.spawn_empty().id();
    let b = world.spawn_empty().id();
    world.despawn(b);
    report::is_true("World::get_entity(live) is Ok", "", world.get_entity(a).is_ok());
    report::is_true("World::get_entity(despawned) is Err", "", world.get_entity(b).is_err());
}
