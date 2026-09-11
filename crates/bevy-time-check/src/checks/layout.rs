//! Memory layout of `bevy_time`'s public types on `armv6k-nintendo-3ds`
//! (records the actual `size_of`/`align_of`).

use super::report;
use bevy_time::{Fixed, Real, Stopwatch, Time, Timer, TimerMode, Virtual};
use core::mem::{align_of, size_of};

fn rec<T>(name: &str) {
    report::ok(name, "", "size / align", &format!("{} / {}", size_of::<T>(), align_of::<T>()), true);
}

#[test]
fn type_layout() {
    rec::<Stopwatch>("size/align Stopwatch");
    rec::<Timer>("size/align Timer");
    rec::<TimerMode>("size/align TimerMode");
    rec::<Time>("size/align Time (= Time<()>)");
    rec::<Time<Real>>("size/align Time<Real>");
    rec::<Time<Virtual>>("size/align Time<Virtual>");
    rec::<Time<Fixed>>("size/align Time<Fixed>");
}
