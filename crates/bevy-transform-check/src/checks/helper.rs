//! `bevy_transform::helper::TransformHelper` — a `SystemParam` that computes an
//! entity's `GlobalTransform` on demand from its `Transform` + ancestors.

use super::{near3, report};
use bevy_ecs::prelude::*;
use bevy_ecs::system::RunSystemOnce;
use bevy_math::{Quat, Vec3};
use bevy_transform::helper::TransformHelper;
use bevy_transform::prelude::*;
use core::f32::consts::FRAC_PI_2;

#[test]
fn compute_global_transform_via_helper() {
    let mut world = World::new();
    let root = world.spawn(Transform::from_xyz(10.0, 0.0, 0.0)).id();
    let mid = world.spawn((Transform::from_xyz(0.0, 5.0, 0.0), ChildOf(root))).id();
    let tip = world.spawn((Transform::from_xyz(0.0, 0.0, 2.0), ChildOf(mid))).id();

    // No propagation systems have run — TransformHelper walks the ancestry itself.
    let got = world
        .run_system_once(move |helper: TransformHelper| {
            helper.compute_global_transform(tip).unwrap().translation()
        })
        .unwrap();
    report::approx(
        "TransformHelper::compute_global_transform (no propagation run)",
        "root(10,0,0)/mid(0,5,0)/tip(0,0,2)",
        Vec3::new(10.0, 5.0, 2.0),
        got,
        near3(got, Vec3::new(10.0, 5.0, 2.0)),
    );

    let rot_got = {
        let mut w = World::new();
        let r = w.spawn(Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2))).id();
        let c = w.spawn((Transform::from_xyz(1.0, 0.0, 0.0), ChildOf(r))).id();
        w.run_system_once(move |helper: TransformHelper| {
            helper.compute_global_transform(c).unwrap().translation()
        })
        .unwrap()
    };
    report::approx(
        "TransformHelper — rotation composes through ancestors",
        "root Rz(PI/2), child (1,0,0)",
        Vec3::new(0.0, 1.0, 0.0),
        rot_got,
        near3(rot_got, Vec3::new(0.0, 1.0, 0.0)),
    );

    // error path: missing Transform
    let missing = world.spawn_empty().id();
    let is_err = world
        .run_system_once(move |helper: TransformHelper| helper.compute_global_transform(missing).is_err())
        .unwrap();
    report::is_true("TransformHelper — Err for entity without Transform", "spawn_empty()", is_err);
}
