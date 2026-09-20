//! The propagation pipeline: `mark_dirty_trees` → `propagate_parent_transforms`
//! → `sync_simple_transforms`. Runs the **serial fallbacks** (no `multi_threaded`
//! — the parallel path currently doesn't even compile on this target, see
//! `Cargo.toml`'s `multi_threaded` feature doc and port.md §B2/§B10-Threading).

use super::{gt_translation, near3, report, world_with_propagation};
use bevy_ecs::prelude::*;
use bevy_math::{Quat, Vec3};
use bevy_transform::prelude::*;
use bevy_transform::systems::StaticTransformOptimizations;
use core::f32::consts::FRAC_PI_2;

#[test]
fn sync_simple_no_hierarchy() {
    let (mut world, mut schedule) = world_with_propagation();
    let e = world.spawn(Transform::from_xyz(4.0, -5.0, 6.0)).id();
    schedule.run(&mut world);
    report::approx("sync_simple_transforms (serial iter_mut)", "entity with only Transform(4,-5,6)", Vec3::new(4.0, -5.0, 6.0), gt_translation(&world, e), near3(gt_translation(&world, e), Vec3::new(4.0, -5.0, 6.0)));
}

#[test]
fn three_level_translation() {
    let (mut world, mut schedule) = world_with_propagation();
    let root = world.spawn(Transform::from_xyz(10.0, 0.0, 0.0)).id();
    let mid = world.spawn((Transform::from_xyz(0.0, 5.0, 0.0), ChildOf(root))).id();
    let tip = world.spawn((Transform::from_xyz(0.0, 0.0, 2.0), ChildOf(mid))).id();
    schedule.run(&mut world);
    report::approx("propagate — root", "T(10,0,0)", Vec3::new(10.0, 0.0, 0.0), gt_translation(&world, root), near3(gt_translation(&world, root), Vec3::new(10.0, 0.0, 0.0)));
    report::approx("propagate — mid (child of root)", "T(0,5,0)", Vec3::new(10.0, 5.0, 0.0), gt_translation(&world, mid), near3(gt_translation(&world, mid), Vec3::new(10.0, 5.0, 0.0)));
    report::approx("propagate — tip (grandchild)", "T(0,0,2)", Vec3::new(10.0, 5.0, 2.0), gt_translation(&world, tip), near3(gt_translation(&world, tip), Vec3::new(10.0, 5.0, 2.0)));
}

#[test]
fn rotation_composes_through_hierarchy() {
    let (mut world, mut schedule) = world_with_propagation();
    let root = world.spawn(Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2))).id();
    let child = world.spawn((Transform::from_xyz(1.0, 0.0, 0.0), ChildOf(root))).id();
    schedule.run(&mut world);
    report::approx("propagate — rotation compose", "root Rz(PI/2), child local (1,0,0)", Vec3::new(0.0, 1.0, 0.0), gt_translation(&world, child), near3(gt_translation(&world, child), Vec3::new(0.0, 1.0, 0.0)));
}

#[test]
fn reacts_to_parent_change() {
    let (mut world, mut schedule) = world_with_propagation();
    let root = world.spawn(Transform::from_xyz(0.0, 0.0, 0.0)).id();
    let child = world.spawn((Transform::from_xyz(1.0, 1.0, 0.0), ChildOf(root))).id();
    schedule.run(&mut world);
    report::approx("propagate — child after 1st run", "root at origin", Vec3::new(1.0, 1.0, 0.0), gt_translation(&world, child), near3(gt_translation(&world, child), Vec3::new(1.0, 1.0, 0.0)));

    world.entity_mut(root).get_mut::<Transform>().unwrap().translation = Vec3::new(100.0, 0.0, 0.0);
    schedule.run(&mut world);
    report::approx("propagate — child updates after parent moved (change detection)", "root -> (100,0,0), 2nd run", Vec3::new(101.0, 1.0, 0.0), gt_translation(&world, child), near3(gt_translation(&world, child), Vec3::new(101.0, 1.0, 0.0)));
}

#[test]
fn deep_hierarchy() {
    let (mut world, mut schedule) = world_with_propagation();
    let mut parent = world.spawn(Transform::from_xyz(1.0, 0.0, 0.0)).id();
    let mut leaf = parent;
    for _ in 0..20 {
        leaf = world.spawn((Transform::from_xyz(1.0, 0.0, 0.0), ChildOf(parent))).id();
        parent = leaf;
    }
    schedule.run(&mut world);
    report::approx("propagate — 21-deep chain of T(1,0,0)", "each level +1 on x", Vec3::new(21.0, 0.0, 0.0), gt_translation(&world, leaf), near3(gt_translation(&world, leaf), Vec3::new(21.0, 0.0, 0.0)));
}

#[test]
fn childof_maintains_children() {
    let mut world = World::new();
    let parent = world.spawn(Transform::IDENTITY).id();
    let a = world.spawn((Transform::IDENTITY, ChildOf(parent))).id();
    let b = world.spawn((Transform::IDENTITY, ChildOf(parent))).id();
    let children = world.entity(parent).get::<Children>().unwrap();
    report::ok(
        "ChildOf(parent) -> parent gets Children (bevy_ecs relationship hooks)",
        "2 children spawned",
        "len 2, contains both",
        &format!("len {}, contains both = {}", children.len(), children.contains(&a) && children.contains(&b)),
        children.len() == 2 && children.contains(&a) && children.contains(&b),
    );
}

#[test]
fn static_optimizations_toggle() {
    // Disabled: mark_dirty_trees early-returns, propagation relies purely on change detection.
    let (mut world, mut schedule) = world_with_propagation();
    world.insert_resource(StaticTransformOptimizations::Disabled);
    let root = world.spawn(Transform::from_xyz(2.0, 0.0, 0.0)).id();
    let child = world.spawn((Transform::from_xyz(3.0, 0.0, 0.0), ChildOf(root))).id();
    schedule.run(&mut world);
    report::approx("propagate with StaticTransformOptimizations::Disabled", "root(2,0,0)/child(3,0,0)", Vec3::new(5.0, 0.0, 0.0), gt_translation(&world, child), near3(gt_translation(&world, child), Vec3::new(5.0, 0.0, 0.0)));
    report::eq("StaticTransformOptimizations::Enabled.is_enabled()", "", true, StaticTransformOptimizations::Enabled.is_enabled());
    report::eq("StaticTransformOptimizations::default() == Enabled", "", StaticTransformOptimizations::Enabled, StaticTransformOptimizations::default());
}
