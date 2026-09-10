//! `bevy_transform::commands::BuildChildrenTransformExt`:
//! `set_parent_in_place` / `remove_parent_in_place` (via `EntityWorldMut`),
//! which re-parent while keeping the world position via `Transform` fix-up.

use super::{gt_translation, near3, report, world_with_propagation};
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_transform::commands::BuildChildrenTransformExt;
use bevy_transform::prelude::*;

#[test]
fn set_parent_in_place_preserves_world_position() {
    let (mut world, mut schedule) = world_with_propagation();

    let parent = world.spawn(Transform::from_xyz(10.0, 0.0, 0.0)).id();
    // free entity at world (3, 0, 0)
    let child = world.spawn(Transform::from_xyz(3.0, 0.0, 0.0)).id();
    schedule.run(&mut world);

    // re-parent under `parent` (global 10,0,0) keeping the (3,0,0) world position.
    world.entity_mut(child).set_parent_in_place(parent);
    schedule.run(&mut world);

    report::approx(
        "EntityWorldMut::set_parent_in_place — world position kept",
        "child was at world (3,0,0), new parent at (10,0,0)",
        Vec3::new(3.0, 0.0, 0.0),
        gt_translation(&world, child),
        near3(gt_translation(&world, child), Vec3::new(3.0, 0.0, 0.0)),
    );
    report::approx(
        "… and its local Transform is now (-7,0,0)",
        "",
        Vec3::new(-7.0, 0.0, 0.0),
        world.entity(child).get::<Transform>().unwrap().translation,
        near3(world.entity(child).get::<Transform>().unwrap().translation, Vec3::new(-7.0, 0.0, 0.0)),
    );
    report::is_true("… ChildOf(parent) inserted", "", world.entity(child).get::<ChildOf>().map(|c| c.parent()) == Some(parent));
}

#[test]
fn remove_parent_in_place_preserves_world_position() {
    let (mut world, mut schedule) = world_with_propagation();
    let parent = world.spawn(Transform::from_xyz(10.0, 0.0, 0.0)).id();
    let child = world.spawn((Transform::from_xyz(2.0, 0.0, 0.0), ChildOf(parent))).id();
    schedule.run(&mut world);
    // world (12, 0, 0)

    world.entity_mut(child).remove_parent_in_place();
    schedule.run(&mut world);

    report::approx(
        "EntityWorldMut::remove_parent_in_place — world position kept",
        "child was at world (12,0,0)",
        Vec3::new(12.0, 0.0, 0.0),
        gt_translation(&world, child),
        near3(gt_translation(&world, child), Vec3::new(12.0, 0.0, 0.0)),
    );
    report::approx(
        "… and local Transform is now the world transform (12,0,0)",
        "",
        Vec3::new(12.0, 0.0, 0.0),
        world.entity(child).get::<Transform>().unwrap().translation,
        near3(world.entity(child).get::<Transform>().unwrap().translation, Vec3::new(12.0, 0.0, 0.0)),
    );
    report::eq("… ChildOf removed", "", false, world.entity(child).contains::<ChildOf>());
}
