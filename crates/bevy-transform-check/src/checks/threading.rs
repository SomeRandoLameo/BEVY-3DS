//! Concerns specific to `bevy_transform`'s **parallel** propagation path
//! (`mod parallel` in `bevy_transform::systems`, active because
//! `multi_threaded` is this crate's default — see `Cargo.toml`): a work-queue
//! of subtrees shared across `ComputeTaskPool` worker threads via `mpsc`
//! channels + a spin-locked `try_lock()`, seeded by `roots.par_iter_mut()`.
//! `checks::propagation` already checks that this produces the same
//! `GlobalTransform`s as the serial fallback for small trees; this file
//! targets what's actually unique to the parallel path — multiple roots
//! processed concurrently, and enough tree depth/width/repetition to
//! meaningfully exercise the shared queue instead of trivially no-op'ing.
//! Port.md §B2/§B10.

use super::{gt_translation, near3, report, world_with_propagation};
use bevy_math::Vec3;
use bevy_tasks::{ComputeTaskPool, TaskPoolBuilder};
use bevy_transform::prelude::*;

/// Mirrors `bevy-ecs-check`'s `ensure_compute_task_pool()`: `ComputeTaskPool`
/// is a process-global `OnceLock`, so whichever test in this binary calls
/// this first wins the thread count — hence no test here asserts an exact
/// count, only that propagation is *correct* once it's initialized.
fn ensure_compute_task_pool() -> &'static ComputeTaskPool {
    ComputeTaskPool::get_or_init(|| {
        TaskPoolBuilder::new()
            .num_threads(2)
            .thread_name("dove-transform-compute".into())
            .build()
    })
}

#[test]
fn compute_task_pool_initializes_with_a_real_thread_pool() {
    let pool = ensure_compute_task_pool();
    report::is_true("ComputeTaskPool::get_or_init() yields a pool with >= 1 real thread", "num_threads(2) requested somewhere in this binary", pool.thread_num() >= 1);
}

/// `propagate_parent_transforms`'s parallel path seeds its work queue by
/// running `roots.par_iter_mut()` over every root **concurrently** — so this
/// specifically needs more than one root to test anything the serial path's
/// single-threaded root loop wouldn't already cover.
#[test]
fn parallel_propagation_handles_many_independent_roots_concurrently() {
    ensure_compute_task_pool();
    let (mut world, mut schedule) = world_with_propagation();

    let roots: Vec<_> = (0..40)
        .map(|i| world.spawn(Transform::from_xyz(i as f32, 0.0, 0.0)).id())
        .collect();
    let children: Vec<_> = roots
        .iter()
        .map(|&root| world.spawn((Transform::from_xyz(0.0, 1.0, 0.0), ChildOf(root))).id())
        .collect();

    schedule.run(&mut world);

    let roots_correct = roots.iter().enumerate().all(|(i, &e)| near3(gt_translation(&world, e), Vec3::new(i as f32, 0.0, 0.0)));
    let children_correct = children.iter().enumerate().all(|(i, &e)| near3(gt_translation(&world, e), Vec3::new(i as f32, 1.0, 0.0)));
    report::is_true("40 independent root+child trees, propagated via roots.par_iter_mut()", "root i at (i,0,0), child at (i,1,0)", roots_correct && children_correct);
}

/// Deep enough that the work-queue's single worker (run locally, per
/// `propagate_parent_transforms`'s own comment) has to hand off to the
/// `ComputeTaskPool` workers via the `mpsc` channel at least once, and wide
/// enough (many roots, each moderately deep) that there's real concurrent
/// contention on the shared queue's `try_lock()`.
#[test]
fn parallel_propagation_deep_and_wide_hierarchy() {
    ensure_compute_task_pool();
    let (mut world, mut schedule) = world_with_propagation();

    let mut leaves = Vec::new();
    for root_i in 0..8 {
        let mut parent = world.spawn(Transform::from_xyz(root_i as f32, 0.0, 0.0)).id();
        for _ in 0..15 {
            parent = world.spawn((Transform::from_xyz(1.0, 0.0, 0.0), ChildOf(parent))).id();
        }
        leaves.push((root_i, parent));
    }

    schedule.run(&mut world);

    let all_correct = leaves.iter().all(|&(root_i, leaf)| near3(gt_translation(&world, leaf), Vec3::new(root_i as f32 + 15.0, 0.0, 0.0)));
    report::is_true("8 roots x 15-deep chains of T(1,0,0), one schedule.run()", "leaf_i = (root_i + 15, 0, 0)", all_correct);
}

/// The parallel-path equivalent of `bevy-ecs-check`'s
/// `repeated_multi_threaded_runs_do_not_deadlock_or_corrupt_state`: repeated
/// `schedule.run()`s across a real multi-root hierarchy, each one moving
/// every root a bit further, checking the shared work-queue/channel
/// machinery doesn't deadlock, drop work, or double-apply it over many
/// rounds.
#[test]
fn repeated_parallel_propagation_runs_do_not_deadlock_or_corrupt_state() {
    ensure_compute_task_pool();
    let (mut world, mut schedule) = world_with_propagation();

    let roots: Vec<_> = (0..12).map(|_| world.spawn(Transform::from_xyz(0.0, 0.0, 0.0)).id()).collect();
    let leaves: Vec<_> = roots
        .iter()
        .map(|&root| {
            let mid = world.spawn((Transform::from_xyz(1.0, 0.0, 0.0), ChildOf(root))).id();
            world.spawn((Transform::from_xyz(1.0, 0.0, 0.0), ChildOf(mid))).id()
        })
        .collect();

    const ROUNDS: i32 = 25;
    for step in 1..=ROUNDS {
        for &root in &roots {
            world.entity_mut(root).get_mut::<Transform>().unwrap().translation.x = step as f32;
        }
        schedule.run(&mut world);
    }

    // Every leaf is 2 links (T(1,0,0) each) below a root that ended at x=ROUNDS.
    let all_correct = leaves.iter().all(|&leaf| near3(gt_translation(&world, leaf), Vec3::new(ROUNDS as f32 + 2.0, 0.0, 0.0)));
    report::is_true("25 rounds moving 12 roots + re-propagating 2-deep chains", "no deadlock, no stale/lost GlobalTransform", all_correct);
}
