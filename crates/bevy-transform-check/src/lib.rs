//! Standalone check for `bevy_transform` on the 3DS target
//! (`armv6k-nintendo-3ds`).
//!
//! Two halves:
//! 1. `Transform` / `GlobalTransform` math (compose, `transform_point`,
//!    `looking_at`, matrix round-trips).
//! 2. The **ECS propagation pipeline** — `mark_dirty_trees` →
//!    `propagate_parent_transforms` → `sync_simple_transforms` run over a real
//!    parent/child/grandchild hierarchy. port.md §B2 flags that propagation
//!    normally uses `par_iter` / `ComputeTaskPool`; without the `multi_threaded`
//!    feature bevy_transform has serial fallbacks, and this proves they work and
//!    give correct `GlobalTransform`s on-device.
//!
//! Each check prints a Markdown table row (function | input | expected | got |
//! status); see `README.md`. Run:
//! `./scripts/test-emulator.sh -p bevy-transform-check`.

#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(test_runner::run_gdb))]

use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_transform::prelude::*;
use bevy_transform::systems::{
    mark_dirty_trees, propagate_parent_transforms, sync_simple_transforms,
};

/// Build a `World` + `Schedule` wired up like `TransformPlugin`'s propagation
/// set, minus the `App`.
fn world_with_propagation() -> (World, Schedule) {
    let mut world = World::new();
    world.init_resource::<StaticTransformOptimizations>();

    let mut schedule = Schedule::default();
    schedule.add_systems(
        (
            mark_dirty_trees,
            propagate_parent_transforms,
            sync_simple_transforms,
        )
            .chain(),
    );
    (world, schedule)
}

/// Representative use: a 3-deep rig, propagated, grandchild's world position.
pub fn demo() -> Vec3 {
    let (mut world, mut schedule) = world_with_propagation();
    let root = world.spawn(Transform::from_xyz(10.0, 0.0, 0.0)).id();
    let mid = world.spawn((Transform::from_xyz(0.0, 5.0, 0.0), ChildOf(root))).id();
    let tip = world.spawn((Transform::from_xyz(0.0, 0.0, 2.0), ChildOf(mid))).id();
    schedule.run(&mut world);
    world.entity(tip).get::<GlobalTransform>().unwrap().translation()
}

#[cfg(test)]
mod report;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report;
    use bevy_math::Quat;
    use core::f32::consts::FRAC_PI_2;

    const EPS: f32 = report::EPS;
    fn near3(a: Vec3, b: Vec3) -> bool {
        a.abs_diff_eq(b, EPS)
    }
    fn gt(world: &World, e: Entity) -> Vec3 {
        world.entity(e).get::<GlobalTransform>().unwrap().translation()
    }

    // --- Transform / GlobalTransform math (no ECS) ----------------------

    #[test]
    fn transform_compose_and_point() {
        let parent = Transform::from_xyz(1.0, 2.0, 3.0);
        let child = Transform::from_xyz(0.0, 0.0, 1.0).with_scale(Vec3::splat(2.0));
        let combined = parent.mul_transform(child);
        report::approx(
            "Transform::mul_transform — translation",
            "T(1,2,3) * (T(0,0,1), S=2)",
            Vec3::new(1.0, 2.0, 4.0),
            combined.translation,
            near3(combined.translation, Vec3::new(1.0, 2.0, 4.0)),
        );
        report::eq("Transform::mul_transform — scale", "same", Vec3::splat(2.0), combined.scale);

        let rot = Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2));
        report::approx(
            "Transform::from_rotation(Rz PI/2).transform_point(X)",
            "",
            Vec3::Y,
            rot.transform_point(Vec3::X),
            near3(rot.transform_point(Vec3::X), Vec3::Y),
        );
    }

    #[test]
    fn transform_looking_at_and_basis() {
        let t = Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(0.0, 0.0, -1.0), Vec3::Y);
        report::approx("Transform::looking_at(-Z).forward()", "", Vec3::NEG_Z, t.forward().as_vec3(), near3(t.forward().as_vec3(), Vec3::NEG_Z));
        report::approx("Transform::looking_at(-Z).up()", "", Vec3::Y, t.up().as_vec3(), near3(t.up().as_vec3(), Vec3::Y));
        report::approx("Transform::looking_at(-Z).right()", "", Vec3::X, t.right().as_vec3(), near3(t.right().as_vec3(), Vec3::X));
    }

    #[test]
    fn transform_matrix_roundtrip() {
        let t = Transform {
            translation: Vec3::new(3.0, -1.0, 2.0),
            rotation: Quat::from_rotation_y(0.6),
            scale: Vec3::splat(1.5),
        };
        let via_matrix = t.to_matrix().transform_point3(Vec3::X);
        let direct = t.transform_point(Vec3::X);
        report::approx(
            "Transform::to_matrix().transform_point3 == Transform::transform_point",
            "T=(3,-1,2), Ry=0.6, S=1.5, point X",
            direct,
            via_matrix,
            near3(via_matrix, direct),
        );
    }

    #[test]
    fn global_transform_ops() {
        let g = GlobalTransform::from(Transform::from_xyz(1.0, 0.0, 0.0));
        let child = g * Transform::from_xyz(0.0, 2.0, 0.0);
        report::approx(
            "GlobalTransform * Transform (propagation compose op)",
            "G(T(1,0,0)) * T(0,2,0)",
            Vec3::new(1.0, 2.0, 0.0),
            child.translation(),
            near3(child.translation(), Vec3::new(1.0, 2.0, 0.0)),
        );
        report::approx("GlobalTransform::transform_point", "G(T(1,0,0)) * point Z", Vec3::new(1.0, 0.0, 1.0), g.transform_point(Vec3::Z), near3(g.transform_point(Vec3::Z), Vec3::new(1.0, 0.0, 1.0)));

        let (scale, rot, trans) = GlobalTransform::from(
            Transform::from_xyz(5.0, 6.0, 7.0).with_scale(Vec3::splat(3.0)),
        )
        .to_scale_rotation_translation();
        report::approx("GlobalTransform::to_scale_rotation_translation — translation", "T(5,6,7), S=3", Vec3::new(5.0, 6.0, 7.0), trans, near3(trans, Vec3::new(5.0, 6.0, 7.0)));
        report::approx("GlobalTransform::to_scale_rotation_translation — scale", "same", Vec3::splat(3.0), scale, near3(scale, Vec3::splat(3.0)));
        report::ok("GlobalTransform::to_scale_rotation_translation — rotation ~ identity", "same", "near-identity", if rot.is_near_identity() { "near-identity" } else { "rotated" }, rot.is_near_identity());

        let src = Transform::from_xyz(-2.0, 4.0, 1.0);
        let back = GlobalTransform::from(src).compute_transform().translation;
        report::approx("GlobalTransform::from(t).compute_transform() round-trip", "T(-2,4,1)", src.translation, back, near3(back, src.translation));
    }

    #[test]
    fn transform_point_trait() {
        let t = Transform::from_xyz(10.0, 0.0, 0.0);
        report::eq("TransformPoint::transform_point on Transform", "T(10,0,0) * (0,0,0)", Vec3::new(10.0, 0.0, 0.0), TransformPoint::transform_point(&t, Vec3::ZERO));
        let g = GlobalTransform::from(t);
        report::eq("TransformPoint::transform_point on GlobalTransform", "G(T(10,0,0)) * (0,0,0)", Vec3::new(10.0, 0.0, 0.0), TransformPoint::transform_point(&g, Vec3::ZERO));
    }

    // --- ECS propagation pipeline (the port.md §B2 concern) ------------

    #[test]
    fn sync_simple_transform_no_hierarchy() {
        let (mut world, mut schedule) = world_with_propagation();
        let e = world.spawn(Transform::from_xyz(4.0, -5.0, 6.0)).id();
        schedule.run(&mut world);
        report::approx(
            "sync_simple_transforms (serial iter_mut branch)",
            "entity with only Transform(4,-5,6), 1 schedule run",
            Vec3::new(4.0, -5.0, 6.0),
            gt(&world, e),
            near3(gt(&world, e), Vec3::new(4.0, -5.0, 6.0)),
        );
    }

    #[test]
    fn propagate_three_level_translation() {
        let (mut world, mut schedule) = world_with_propagation();
        let root = world.spawn(Transform::from_xyz(10.0, 0.0, 0.0)).id();
        let mid = world.spawn((Transform::from_xyz(0.0, 5.0, 0.0), ChildOf(root))).id();
        let tip = world.spawn((Transform::from_xyz(0.0, 0.0, 2.0), ChildOf(mid))).id();
        schedule.run(&mut world);

        report::approx("propagate: root GlobalTransform", "root T(10,0,0)", Vec3::new(10.0, 0.0, 0.0), gt(&world, root), near3(gt(&world, root), Vec3::new(10.0, 0.0, 0.0)));
        report::approx("propagate: mid (child of root) GlobalTransform", "mid T(0,5,0)", Vec3::new(10.0, 5.0, 0.0), gt(&world, mid), near3(gt(&world, mid), Vec3::new(10.0, 5.0, 0.0)));
        report::approx("propagate: tip (grandchild) GlobalTransform", "tip T(0,0,2)", Vec3::new(10.0, 5.0, 2.0), gt(&world, tip), near3(gt(&world, tip), Vec3::new(10.0, 5.0, 2.0)));
    }

    #[test]
    fn propagate_with_parent_rotation() {
        let (mut world, mut schedule) = world_with_propagation();
        let root = world.spawn(Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2))).id();
        let child = world.spawn((Transform::from_xyz(1.0, 0.0, 0.0), ChildOf(root))).id();
        schedule.run(&mut world);
        report::approx(
            "propagate: rotation composes through hierarchy",
            "root Rz(PI/2), child local T(1,0,0)",
            Vec3::new(0.0, 1.0, 0.0),
            gt(&world, child),
            near3(gt(&world, child), Vec3::new(0.0, 1.0, 0.0)),
        );
    }

    #[test]
    fn propagation_reacts_to_parent_change() {
        let (mut world, mut schedule) = world_with_propagation();
        let root = world.spawn(Transform::from_xyz(0.0, 0.0, 0.0)).id();
        let child = world.spawn((Transform::from_xyz(1.0, 1.0, 0.0), ChildOf(root))).id();
        schedule.run(&mut world);
        report::approx("propagate: child after 1st run", "root at origin", Vec3::new(1.0, 1.0, 0.0), gt(&world, child), near3(gt(&world, child), Vec3::new(1.0, 1.0, 0.0)));

        world.entity_mut(root).get_mut::<Transform>().unwrap().translation = Vec3::new(100.0, 0.0, 0.0);
        schedule.run(&mut world);
        report::approx(
            "propagate: child updates after parent Transform mutated (change detection)",
            "root moved to (100,0,0), 2nd run",
            Vec3::new(101.0, 1.0, 0.0),
            gt(&world, child),
            near3(gt(&world, child), Vec3::new(101.0, 1.0, 0.0)),
        );
    }

    #[test]
    fn childof_maintains_children_relationship() {
        let mut world = World::new();
        let parent = world.spawn(Transform::IDENTITY).id();
        let a = world.spawn((Transform::IDENTITY, ChildOf(parent))).id();
        let b = world.spawn((Transform::IDENTITY, ChildOf(parent))).id();
        let children = world.entity(parent).get::<Children>().unwrap();
        report::ok(
            "ChildOf(parent) -> parent gets Children (bevy_ecs relationship hooks)",
            "spawn 2 children with ChildOf(parent)",
            "len 2, contains both",
            &format!("len {}, contains both = {}", children.len(), children.contains(&a) && children.contains(&b)),
            children.len() == 2 && children.contains(&a) && children.contains(&b),
        );
    }

    #[test]
    fn transform_component_roundtrip() {
        let mut world = World::new();
        let e = world.spawn(Transform::from_xyz(1.0, 2.0, 3.0)).id();
        report::ok(
            "Transform::require(GlobalTransform) auto-inserts on spawn",
            "spawn(Transform)",
            "GlobalTransform present",
            if world.entity(e).contains::<GlobalTransform>() { "present" } else { "missing" },
            world.entity(e).contains::<GlobalTransform>(),
        );
        world.entity_mut(e).get_mut::<Transform>().unwrap().translation.x += 9.0;
        let mut q = world.query::<&Transform>();
        let got = q.get(&world, e).unwrap().translation;
        report::approx("Query<&mut Transform> mutate + read back", "T(1,2,3), x += 9", Vec3::new(10.0, 2.0, 3.0), got, near3(got, Vec3::new(10.0, 2.0, 3.0)));
    }

    #[test]
    fn demo_runs() {
        let p = demo();
        report::approx("demo() — 3-level rig, propagated, tip translation", "root(10,0,0)/mid(0,5,0)/tip(0,0,2)", Vec3::new(10.0, 5.0, 2.0), p, near3(p, Vec3::new(10.0, 5.0, 2.0)));
    }
}
