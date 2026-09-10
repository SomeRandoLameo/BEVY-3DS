//! `bevy_transform::plugins::{TransformPlugin, TransformSystems}` under a real
//! `bevy_app::App` — also the first on-device exercise of `App::update()`.

use super::{near3, report};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::{Quat, Vec3};
use bevy_transform::prelude::*;
use core::f32::consts::FRAC_PI_2;

fn app() -> App {
    let mut app = App::new();
    app.add_plugins(TransformPlugin);
    app
}

#[test]
fn transform_plugin_propagates_on_update() {
    let mut app = app();
    let root = app.world_mut().spawn(Transform::from_xyz(10.0, 0.0, 0.0)).id();
    let child = app
        .world_mut()
        .spawn((Transform::from_xyz(0.0, 5.0, 0.0), ChildOf(root)))
        .id();

    app.update(); // runs PostStartup + first PostUpdate -> Propagate set

    let g = app.world().entity(child).get::<GlobalTransform>().unwrap().translation();
    report::approx(
        "TransformPlugin: GlobalTransform after App::update()",
        "root(10,0,0)/child(0,5,0)",
        Vec3::new(10.0, 5.0, 0.0),
        g,
        near3(g, Vec3::new(10.0, 5.0, 0.0)),
    );
}

#[test]
fn transform_plugin_reacts_across_updates() {
    let mut app = app();
    let root = app.world_mut().spawn(Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2))).id();
    let child = app
        .world_mut()
        .spawn((Transform::from_xyz(1.0, 0.0, 0.0), ChildOf(root)))
        .id();
    app.update();

    let g1 = app.world().entity(child).get::<GlobalTransform>().unwrap().translation();
    report::approx("TransformPlugin: rotation compose after update", "root Rz(PI/2)", Vec3::new(0.0, 1.0, 0.0), g1, near3(g1, Vec3::new(0.0, 1.0, 0.0)));

    // move the root, update again
    app.world_mut().entity_mut(root).get_mut::<Transform>().unwrap().translation = Vec3::new(0.0, 0.0, 7.0);
    app.update();
    let g2 = app.world().entity(child).get::<GlobalTransform>().unwrap().translation();
    report::approx("TransformPlugin: child follows root across a 2nd update", "root translated to (0,0,7)", Vec3::new(0.0, 1.0, 7.0), g2, near3(g2, Vec3::new(0.0, 1.0, 7.0)));
}

#[test]
fn transform_systems_set_exists() {
    // `TransformSystems::Propagate` is a usable SystemSet value.
    let set = TransformSystems::Propagate;
    report::eq("TransformSystems::Propagate is Eq/Hash-able", "", set.clone(), set);
}
