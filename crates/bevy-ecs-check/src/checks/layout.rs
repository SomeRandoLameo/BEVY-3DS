//! Memory layout of a few `bevy_ecs` public types on `armv6k-nintendo-3ds`
//! (records the actual `size_of`/`align_of`).

use super::report;
use bevy_ecs::prelude::*;
use core::mem::{align_of, size_of};

fn rec<T>(name: &str) {
    report::ok(name, "", "size / align", &format!("{} / {}", size_of::<T>(), align_of::<T>()), true);
}

#[test]
fn type_layout() {
    rec::<Entity>("size/align Entity");
    rec::<Option<Entity>>("size/align Option<Entity> (niche-optimised, same size as Entity)");
    rec::<ChildOf>("size/align ChildOf");
    report::eq("Option<Entity> has no size overhead over Entity (niche optimisation)", "", size_of::<Entity>(), size_of::<Option<Entity>>());
}
