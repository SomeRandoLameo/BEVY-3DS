//! `Time<Real>` — wall-clock time. `update_with_duration` (used here) is
//! deterministic and mirrors `bevy_time`'s own `test_update_with_duration`.
//! The one place this touches the *actual* clock is `instant_now_sanity`,
//! which only checks two `Instant::now()` calls don't panic and don't go
//! backwards — it says nothing about `Instant::now()`'s accuracy/resolution
//! on real 3DS hardware (port.md §B6, still open).

use super::report;
use bevy_platform::time::Instant;
use bevy_time::{Real, Time};
use core::time::Duration;

#[test]
fn update_with_duration_advances_from_last_update() {
    let mut t = Time::<Real>::default();
    report::eq("Time::<Real>::default() first_update", "", None, t.first_update());
    report::eq("… last_update", "", None, t.last_update());

    t.update_with_duration(Duration::from_secs(1));
    report::eq("Time::<Real>::update_with_duration(1s) — first update: delta", "", Duration::ZERO, t.delta());
    report::eq("… elapsed (no previous update to diff against)", "", Duration::ZERO, t.elapsed());
    report::is_true("… first_update is now Some", "", t.first_update().is_some());

    t.update_with_duration(Duration::from_secs(1));
    report::eq("… second update: delta", "1s since last_update", Duration::from_secs(1), t.delta());
    report::eq("… elapsed", "", Duration::from_secs(1), t.elapsed());

    t.update_with_duration(Duration::from_secs(1));
    report::eq("… third update: delta", "", Duration::from_secs(1), t.delta());
    report::eq("… elapsed accumulates", "1+1s", Duration::from_secs(2), t.elapsed());
}

#[test]
fn startup_is_preserved_across_updates() {
    let startup = Instant::now();
    let mut t = Time::<Real>::new(startup);
    report::eq("Time::<Real>::new(startup) -> startup()", "", startup, t.startup());
    t.update_with_duration(Duration::from_secs(1));
    t.update_with_duration(Duration::from_secs(1));
    report::eq("Time::<Real>::startup() unchanged by updates", "", startup, t.startup());
}

/// Does **not** exercise real timekeeping accuracy on hardware — see module docs.
#[test]
fn instant_now_sanity() {
    let a = Instant::now();
    // Some trivial CPU work so `b` is taken at a later point in wall-clock terms
    // than `a`, without relying on a busy-wait loop that could hang if the
    // clock source has coarser resolution than expected.
    let mut acc: u64 = 0;
    for i in 0..10_000u64 {
        acc = acc.wrapping_add(i);
    }
    core::hint::black_box(acc);
    let b = Instant::now();
    report::is_true("Instant::now() called twice doesn't panic and is monotonic", "b after a", b >= a);
}
