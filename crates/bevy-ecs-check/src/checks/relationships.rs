//! `ChildOf`/`Children` — the built-in parent/child relationship (adapted
//! from `ChildOf`'s own doc examples).

use super::report;
use bevy_ecs::prelude::*;

#[test]
fn child_of_auto_populates_children_on_the_parent() {
    let mut world = World::new();
    let root = world.spawn_empty().id();
    let child1 = world.spawn(ChildOf(root)).id();
    let child2 = world.spawn(ChildOf(root)).id();
    let grandchild = world.spawn(ChildOf(child1)).id();

    report::eq("Inserting ChildOf(root) auto-adds root's Children", "2 children", &[child1, child2][..], &**world.entity(root).get::<Children>().unwrap());
    report::eq("… nested: child1's Children", "", &[grandchild][..], &**world.entity(child1).get::<Children>().unwrap());
    report::eq("ChildOf::parent()", "", root, world.entity(child1).get::<ChildOf>().unwrap().parent());
}

#[test]
fn removing_child_of_updates_the_parents_children() {
    let mut world = World::new();
    let root = world.spawn_empty().id();
    let child1 = world.spawn(ChildOf(root)).id();
    let child2 = world.spawn(ChildOf(root)).id();

    world.entity_mut(child2).remove::<ChildOf>();
    report::eq("Removing ChildOf updates the old parent's Children", "child2 removed", &[child1][..], &**world.entity(root).get::<Children>().unwrap());
    report::is_true("… and the removed child has no Children component of its own to worry about (it's a leaf)", "", !world.entity(child2).contains::<ChildOf>());
}

#[test]
fn despawning_a_parent_despawns_the_whole_subtree() {
    let mut world = World::new();
    let root = world.spawn_empty().id();
    let child1 = world.spawn(ChildOf(root)).id();
    let grandchild = world.spawn(ChildOf(child1)).id();

    world.entity_mut(root).despawn();
    report::is_true("Despawning root also despawns child1", "", world.get_entity(child1).is_err());
    report::is_true("… and grandchild, transitively", "", world.get_entity(grandchild).is_err());
    report::is_true("… root itself is gone too", "", world.get_entity(root).is_err());
}

#[test]
fn with_children_builder_spawns_a_whole_subtree_at_once() {
    let mut world = World::new();
    let mut child1 = Entity::PLACEHOLDER;
    let mut child2 = Entity::PLACEHOLDER;
    let mut grandchild = Entity::PLACEHOLDER;
    let root = world
        .spawn_empty()
        .with_children(|p| {
            child1 = p
                .spawn_empty()
                .with_children(|p| {
                    grandchild = p.spawn_empty().id();
                })
                .id();
            child2 = p.spawn_empty().id();
        })
        .id();

    report::eq("EntityWorldMut::with_children builds the whole tree", "", &[child1, child2][..], &**world.entity(root).get::<Children>().unwrap());
    report::eq("… nested with_children too", "", &[grandchild][..], &**world.entity(child1).get::<Children>().unwrap());
}

#[test]
fn children_component_is_iterable() {
    let mut world = World::new();
    let root = world.spawn_empty().id();
    let a = world.spawn(ChildOf(root)).id();
    let b = world.spawn(ChildOf(root)).id();

    // Force the coercion to `&[Entity]` before calling `.to_vec()` — `Children`
    // also implements `RelationshipSourceCollection`, which has its own
    // `iter()`/etc. that otherwise makes method resolution ambiguous here.
    let slice: &[Entity] = world.entity(root).get::<Children>().unwrap();
    let children: Vec<Entity> = slice.to_vec();
    report::eq("Children derefs to a plain &[Entity] slice", "", vec![a, b], children);
}
