//! Standalone check for **`bevy_transform`** on `armv6k-nintendo-3ds` — aims to
//! touch every public function.
//!
//! Two halves:
//! 1. `Transform` / `GlobalTransform` / `TransformPoint` — the pure math.
//! 2. The ECS side — the propagation systems (`mark_dirty_trees` →
//!    `propagate_parent_transforms` → `sync_simple_transforms`), `TransformHelper`,
//!    `BuildChildrenTransformExt`, and `TransformPlugin` under a real `App`.
//!    port.md §B2: propagation normally uses `par_iter` / `ComputeTaskPool`;
//!    without `multi_threaded` the serial fallbacks run — verified here.
//!
//! Each check prints a Markdown row (function | input | expected | got | status);
//! see `README.md`. Run: `./scripts/test-emulator.sh -p bevy-transform-check`.

#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(test_runner::run_gdb))]

use bevy_math::Vec3;
use bevy_transform::prelude::*;

/// A representative rig: root → mid → tip, propagated; the tip's world position.
pub fn demo() -> Vec3 {
    use bevy_ecs::prelude::*;
    use bevy_transform::systems::{
        mark_dirty_trees, propagate_parent_transforms, sync_simple_transforms,
        StaticTransformOptimizations,
    };
    let mut world = World::new();
    world.init_resource::<StaticTransformOptimizations>();
    let mut schedule = Schedule::default();
    schedule.add_systems(
        (mark_dirty_trees, propagate_parent_transforms, sync_simple_transforms).chain(),
    );
    let root = world.spawn(Transform::from_xyz(10.0, 0.0, 0.0)).id();
    let mid = world.spawn((Transform::from_xyz(0.0, 5.0, 0.0), ChildOf(root))).id();
    let tip = world.spawn((Transform::from_xyz(0.0, 0.0, 2.0), ChildOf(mid))).id();
    schedule.run(&mut world);
    world.entity(tip).get::<GlobalTransform>().unwrap().translation()
}

#[cfg(test)]
mod report;
#[cfg(test)]
mod checks;
