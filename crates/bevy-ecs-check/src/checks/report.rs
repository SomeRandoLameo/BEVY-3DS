//! Test-time helper: every check prints one Markdown table row
//! (`| function | input | expected | got | status |`) and then asserts, so the
//! emulator run output can be pasted straight into `README.md`.
#![allow(dead_code)] // not every check crate uses every helper

use core::fmt::Debug;
use std::io::Write;

pub const EPS: f32 = 1.0e-4;

fn line(func: &str, input: &str, expected: &str, got: &str, pass: bool) {
    // Write straight to the stdout handle so libtest's per-test output capture
    // doesn't swallow the row (there's no `--nocapture` with this test runner).
    let _ = writeln!(
        std::io::stdout(),
        "ROW| `{func}` | {input} | `{expected}` | `{got}` | {} |",
        if pass { "✅" } else { "❌" }
    );
}

/// f32 comparison with `EPS` tolerance.
#[track_caller]
pub fn f(func: &str, input: &str, expected: f32, got: f32) {
    let pass = (got - expected).abs() <= EPS;
    line(func, input, &format!("{expected}"), &format!("{got}"), pass);
    assert!(pass, "{func} [{input}]: expected {expected}, got {got}");
}

/// Exact equality for `Debug + PartialEq` values.
#[track_caller]
pub fn eq<T: Debug + PartialEq>(func: &str, input: &str, expected: T, got: T) {
    let pass = expected == got;
    line(func, input, &format!("{expected:?}"), &format!("{got:?}"), pass);
    assert!(pass, "{func} [{input}]: expected {expected:?}, got {got:?}");
}

/// Approximate check where the caller computes `pass` (e.g. `abs_diff_eq`).
#[track_caller]
pub fn approx<T: Debug>(func: &str, input: &str, expected: T, got: T, pass: bool) {
    line(func, input, &format!("{expected:?}"), &format!("{got:?}"), pass);
    assert!(pass, "{func} [{input}]: expected ~{expected:?}, got {got:?}");
}

/// Free-form check with string expected/got and a caller-computed `pass`.
#[track_caller]
pub fn ok(func: &str, input: &str, expected: &str, got: &str, pass: bool) {
    line(func, input, expected, got, pass);
    assert!(pass, "{func} [{input}]: expected {expected}, got {got}");
}

/// Boolean check: `expected = "true"`, got = the value.
#[track_caller]
pub fn is_true(func: &str, input: &str, got: bool) {
    line(func, input, "true", if got { "true" } else { "false" }, got);
    assert!(got, "{func} [{input}]: expected true");
}
