//! `multi_threaded` — real OS threads via `pthread-3ds`, `bevy_tasks::TaskPool`/
//! `ComputeTaskPool`, and `bevy_ecs`'s `MultiThreadedExecutor`. Only compiled
//! with `--features multi_threaded` (off by default, not in `all-checks` —
//! see the crate's `Cargo.toml`). Port.md §B1/§B2/§B14.
//!
//! `ComputeTaskPool` is a process-global `OnceLock` (`bevy_tasks::usages`):
//! whichever call initializes it first wins for the rest of the process, so
//! every test here goes through `ensure_compute_task_pool()` instead of
//! asserting an exact thread count.

use super::report;
use bevy_ecs::prelude::*;
use bevy_tasks::{ComputeTaskPool, TaskPoolBuilder};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

/// Old 3DS has 2 usable cores for a Homebrew (Core1 is mostly OS-owned);
/// New 3DS frees up Cores 2+3. `std::thread::available_parallelism()` isn't
/// meaningfully implemented on this target, so `TaskPool`'s auto-sizing
/// falls back to 1 — pin it explicitly instead (port.md's own "Threadpools
/// explizit begrenzen" guidance).
fn ensure_compute_task_pool() -> &'static ComputeTaskPool {
    ComputeTaskPool::get_or_init(|| {
        TaskPoolBuilder::new()
            .num_threads(2)
            .thread_name("dove-compute".into())
            .build()
    })
}

#[test]
fn std_thread_spawn_and_join_actually_runs() {
    // The lowest-level smoke test: does `pthread_create`/`pthread_join` (the
    // symbols `pthread-3ds` provides, linked in transitively via `ctru-rs`)
    // actually run the closure on a real second thread, not just compile?
    let ran = Arc::new(AtomicBool::new(false));
    let ran_in_thread = Arc::clone(&ran);
    let handle = std::thread::Builder::new()
        .name("dove-smoke".into())
        .spawn(move || {
            ran_in_thread.store(true, Ordering::SeqCst);
            1 + 1
        })
        .expect("pthread_create failed");
    let result = handle.join().expect("pthread_join failed / thread panicked");
    report::eq("std::thread::spawn() + join() returns the closure's value", "1 + 1", 2, result);
    report::is_true("… and the closure actually ran (not skipped/no-op'd)", "", ran.load(Ordering::SeqCst));
}

#[test]
fn compute_task_pool_initializes_with_a_real_thread_pool() {
    let pool = ensure_compute_task_pool();
    // Can't assert an exact count: whichever test in this binary calls
    // `ensure_compute_task_pool()` first wins the `OnceLock`, and libtest
    // doesn't guarantee run order. `thread_num() >= 1` is what's actually
    // guaranteed regardless of who won the race.
    report::is_true("ComputeTaskPool::get_or_init() yields a pool with >= 1 real thread", "num_threads(2) requested somewhere in this binary", pool.thread_num() >= 1);
}

#[test]
fn multi_threaded_schedule_runs_two_non_conflicting_systems() {
    ensure_compute_task_pool();

    #[derive(Resource, Default)]
    struct CounterA(u32);
    #[derive(Resource, Default)]
    struct CounterB(u32);

    fn bump_a(mut a: ResMut<CounterA>) {
        a.0 += 1;
    }
    fn bump_b(mut b: ResMut<CounterB>) {
        b.0 += 1;
    }

    let mut world = World::new();
    world.init_resource::<CounterA>();
    world.init_resource::<CounterB>();

    // `Schedule::default()`'s `default_executor()` picks `MultiThreadedExecutor`
    // automatically once the `multi_threaded` feature is on for a non-wasm
    // `std` target (see bevy_ecs's `schedule/executor/mod.rs`) — no explicit
    // "use the parallel executor" opt-in needed, unlike older Bevy versions.
    let mut schedule = Schedule::default();
    schedule.add_systems((bump_a, bump_b));
    for _ in 0..5 {
        schedule.run(&mut world);
    }

    report::eq("5 schedule.run()s through the MultiThreadedExecutor: CounterA", "2 non-conflicting systems/run", 5u32, world.resource::<CounterA>().0);
    report::eq("… CounterB", "", 5u32, world.resource::<CounterB>().0);
}

#[test]
fn query_par_iter_for_each_visits_every_entity_exactly_once() {
    ensure_compute_task_pool();

    #[derive(Component)]
    struct Value(u32);
    #[derive(Resource)]
    struct Total(AtomicU32);

    let mut world = World::new();
    world.insert_resource(Total(AtomicU32::new(0)));
    for i in 1..=200u32 {
        world.spawn(Value(i));
    }

    // Real usage of the parallel query path: `bevy_tasks::ComputeTaskPool`'s
    // scoped threads process batches of the 200 entities. `ComputeTaskPool::
    // get()` (not `get_or_init`) is called internally here, which panics if
    // nothing initialized the pool first — that's exactly what
    // `ensure_compute_task_pool()` above is for.
    fn sum_values(query: Query<&Value>, total: Res<Total>) {
        query.par_iter().for_each(|v| {
            total.0.fetch_add(v.0, Ordering::Relaxed);
        });
    }

    let mut schedule = Schedule::default();
    schedule.add_systems(sum_values);
    schedule.run(&mut world);

    let expected: u32 = (1..=200).sum();
    report::eq("Query::par_iter() across 200 entities on a real TaskPool", "sum(1..=200)", expected, world.resource::<Total>().0.load(Ordering::Relaxed));
}

#[test]
fn repeated_multi_threaded_runs_do_not_deadlock_or_corrupt_state() {
    ensure_compute_task_pool();

    #[derive(Component)]
    struct Counter(u32);

    fn increment_all(mut query: Query<&mut Counter>) {
        query.par_iter_mut().for_each(|mut c| {
            c.0 += 1;
        });
    }

    let mut world = World::new();
    let entities: Vec<Entity> = (0..50).map(|_| world.spawn(Counter(0)).id()).collect();

    let mut schedule = Schedule::default();
    schedule.add_systems(increment_all);
    // A real deadlock (e.g. two systems contending for the same lock in the
    // wrong order) or a lost update (a racy `+= 1` under the parallel query
    // batcher) would show up as a wrong count here. 30 runs is enough to
    // shake out an occasional race without turning this into a long-running
    // stress test that dominates the whole crate's runtime.
    for _ in 0..30 {
        schedule.run(&mut world);
    }

    let all_correct = entities
        .iter()
        .all(|&e| world.entity(e).get::<Counter>().unwrap().0 == 30);
    report::is_true("30 rounds of Query::par_iter_mut() across 50 entities: every counter == 30", "no deadlock, no lost update", all_correct);
}
