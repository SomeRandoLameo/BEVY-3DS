//! `Transform` / `GlobalTransform` / `TransformTreeChanged` as ECS components.

use super::{near3, report};
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_transform::components::TransformTreeChanged;
use bevy_transform::prelude::*;

#[test]
fn required_components_and_spawn() {
    let mut world = World::new();
    let e = world.spawn(Transform::from_xyz(1.0, 2.0, 3.0)).id();
    report::is_true("Transform requires GlobalTransform (auto-inserted on spawn)", "spawn(Transform)", world.entity(e).contains::<GlobalTransform>());
    report::is_true("Transform requires TransformTreeChanged", "spawn(Transform)", world.entity(e).contains::<TransformTreeChanged>());
    report::eq("GlobalTransform starts at default", "before any propagation", GlobalTransform::default(), *world.entity(e).get::<GlobalTransform>().unwrap());
}

#[test]
fn query_and_mutate() {
    let mut world = World::new();
    let e = world.spawn(Transform::from_xyz(1.0, 2.0, 3.0)).id();

    world.entity_mut(e).get_mut::<Transform>().unwrap().translation.x += 9.0;
    let mut q = world.query::<&Transform>();
    report::approx("Query<&mut Transform> mutate + read back", "T(1,2,3), x += 9", Vec3::new(10.0, 2.0, 3.0), q.get(&world, e).unwrap().translation, near3(q.get(&world, e).unwrap().translation, Vec3::new(10.0, 2.0, 3.0)));

    // change detection: Transform is Changed after a get_mut
    let mut changed = world.query_filtered::<Entity, Changed<Transform>>();
    report::eq("Changed<Transform> sees the mutation", "after get_mut", 1usize, changed.iter(&world).count());
}

#[test]
fn transform_tree_changed_component() {
    let mut world = World::new();
    let e = world.spawn(Transform::IDENTITY).id();
    let mut em = world.entity_mut(e);
    em.get_mut::<TransformTreeChanged>().unwrap().set_changed();
    report::eq("TransformTreeChanged is a unit component", "", TransformTreeChanged, TransformTreeChanged);
    report::is_true("TransformTreeChanged present + mutable", "spawn(Transform)", world.entity(e).contains::<TransformTreeChanged>());
}
