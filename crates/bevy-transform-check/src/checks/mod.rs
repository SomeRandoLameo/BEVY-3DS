//! One submodule per `bevy_transform` area. Every `#[test]` fn emits `ROW|` lines.

mod report;

use bevy_ecs::prelude::*;
use bevy_math::{Quat, Vec3};
use bevy_transform::prelude::*;
use bevy_transform::systems::{
    mark_dirty_trees, propagate_parent_transforms, sync_simple_transforms,
    StaticTransformOptimizations,
};

const EPS: f32 = report::EPS;

#[track_caller]
fn near3(a: Vec3, b: Vec3) -> bool {
    a.abs_diff_eq(b, EPS)
}
#[track_caller]
fn nearq(a: Quat, b: Quat) -> bool {
    a.abs_diff_eq(b, EPS)
}

/// A `World` + `Schedule` wired up exactly like `TransformPlugin`'s propagation
/// set, minus the `App`.
fn world_with_propagation() -> (World, Schedule) {
    let mut world = World::new();
    world.init_resource::<StaticTransformOptimizations>();
    let mut schedule = Schedule::default();
    schedule.add_systems(
        (mark_dirty_trees, propagate_parent_transforms, sync_simple_transforms).chain(),
    );
    (world, schedule)
}

fn gt_translation(world: &World, e: Entity) -> Vec3 {
    world.entity(e).get::<GlobalTransform>().unwrap().translation()
}

mod commands_ext;
mod ecs_components;
mod global_transform;
mod helper;
mod layout;
mod plugin;
mod propagation;
mod transform_basis;
mod transform_compose;
mod transform_convert;
mod transform_ctors;
mod transform_look;
mod transform_point_trait;
mod transform_rotate;
