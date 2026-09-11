//! `QueryState::iter_combinations`/`iter_many` — the non-linear iteration
//! helpers (all-pairs, and iterating a specific entity subset).

use super::report;
use bevy_ecs::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Position(f32);

#[test]
fn iter_combinations_visits_every_unordered_pair() {
    let mut world = World::new();
    world.spawn(Position(1.0));
    world.spawn(Position(2.0));
    world.spawn(Position(3.0));

    let mut query = world.query::<&Position>();
    let pairs: usize = query.iter_combinations::<2>(&world).count();
    // C(3, 2) = 3
    report::eq("QueryState::iter_combinations::<2>() count on 3 entities", "C(3,2)", 3usize, pairs);

    let mut sums: Vec<f32> = query.iter_combinations::<2>(&world).map(|[a, b]: [&Position; 2]| a.0 + b.0).collect();
    sums.sort_by(|a, b| a.total_cmp(b));
    report::eq("… pairwise sums", "(1,2)=3, (1,3)=4, (2,3)=5", vec![3.0_f32, 4.0, 5.0], sums);
}

#[test]
fn iter_combinations_k_larger_than_n_is_empty() {
    let mut world = World::new();
    world.spawn(Position(1.0));
    let mut query = world.query::<&Position>();
    report::eq("QueryState::iter_combinations::<3>() on 1 entity is empty", "K > N", 0usize, query.iter_combinations::<3>(&world).count());
}

#[test]
fn iter_many_visits_only_the_given_entities_in_order() {
    let mut world = World::new();
    let a = world.spawn(Position(10.0)).id();
    let b = world.spawn(Position(20.0)).id();
    let _unrelated = world.spawn(Position(99.0)).id();

    let mut query = world.query::<&Position>();
    let values: Vec<f32> = query.iter_many(&world, [b, a]).map(|p| p.0).collect();
    report::eq("QueryState::iter_many([b, a]) — order follows the given list", "", vec![20.0_f32, 10.0], values);
}
