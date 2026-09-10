//! Interactive `#![test_runner]` for the Nintendo 3DS.
//!
//! * **Bottom screen** — the test list, with a cursor. The first row is a
//!   "run all tests" entry; the rest are the individual `#[test]`s. `Up`/`Down`
//!   move, `L`/`R` page, `A` runs the highlighted entry, `X` runs all,
//!   `START` exits.
//! * **Top screen** — a log of what ran and the pass/fail result (with the panic
//!   message for failures).
//!
//! Wire a crate up by giving it a `console` feature that selects this runner:
//! ```ignore
//! #![cfg_attr(all(test, not(feature = "console")), test_runner(test_runner::run_gdb))]
//! #![cfg_attr(all(test, feature = "console"), test_runner(test_console::run))]
//! ```
//! The `#[test]` functions are not modified.

#![feature(test)]

extern crate test;

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Mutex;

use ctru::prelude::*;
use test::{ShouldPanic, TestDescAndFn, TestFn};

static LAST_PANIC: Mutex<Option<String>> = Mutex::new(None);

struct Case {
    name: String,
    run: fn() -> Result<(), String>,
    should_panic: ShouldPanic,
}

enum Outcome {
    Passed,
    Failed(String),
}

/// The `#[test_runner]` entry point.
pub fn run(tests: &[&TestDescAndFn]) {
    unsafe { std::env::set_var("RUST_BACKTRACE", "0") };

    // Route every panic message into `LAST_PANIC` instead of stderr.
    std::panic::set_hook(Box::new(|info| {
        *LAST_PANIC.lock().unwrap() = Some(info.to_string());
    }));

    let mut cases: Vec<Case> = tests
        .iter()
        .filter(|t| !t.desc.ignore)
        .filter_map(|t| match t.testfn {
            TestFn::StaticTestFn(f) => Some(Case {
                name: strip_prefix(t.desc.name.as_slice()),
                run: f,
                should_panic: t.desc.should_panic,
            }),
            _ => None,
        })
        .collect();
    cases.sort_by(|a, b| a.name.cmp(&b.name));

    let gfx = Gfx::new().expect("Gfx");
    let mut hid = Hid::new().expect("Hid");
    let apt = Apt::new().expect("Apt");
    let top = Console::new(gfx.top_screen.borrow_mut());
    let bottom = Console::new(gfx.bottom_screen.borrow_mut());

    top.select();
    println!("\x1b[2J\x1b[Htest-console - {} tests", cases.len());
    println!("pick a test on the bottom screen.");

    // Row 0 is a synthetic "run every test, one after another" entry; row `i`
    // (i >= 1) is `cases[i - 1]`.
    let n_items = cases.len() + 1;
    let mut cursor = 0usize;
    let mut view_top = 0usize;
    let visible = 24usize;
    let mut redraw = true;
    let mut ran = 0u32;
    let mut failed = 0u32;

    while apt.main_loop() {
        hid.scan_input();
        let down = hid.keys_down();

        if down.contains(KeyPad::START) {
            break;
        }
        if down.contains(KeyPad::DPAD_DOWN) {
            cursor = (cursor + 1).min(n_items - 1);
            redraw = true;
        }
        if down.contains(KeyPad::DPAD_UP) {
            cursor = cursor.saturating_sub(1);
            redraw = true;
        }
        if down.contains(KeyPad::R) {
            cursor = (cursor + visible).min(n_items - 1);
            redraw = true;
        }
        if down.contains(KeyPad::L) {
            cursor = cursor.saturating_sub(visible);
            redraw = true;
        }
        // `A` (or `X`) on any row runs it; `A`/`X` on row 0 runs all.
        if (down.contains(KeyPad::A) || down.contains(KeyPad::X)) && !cases.is_empty() {
            let (r, f) = if cursor == 0 || down.contains(KeyPad::X) {
                run_all(&cases, &top)
            } else {
                run_one(&cases[cursor - 1], &top)
            };
            ran += r;
            failed += f;
            redraw = true;
        }

        if cursor < view_top {
            view_top = cursor;
        } else if cursor >= view_top + visible {
            view_top = cursor + 1 - visible;
        }

        if redraw {
            draw_menu(&bottom, &cases, n_items, cursor, view_top, visible, ran, failed);
            redraw = false;
        }

        gfx.wait_for_vblank();
    }

    // final summary on the top screen, then wait for START again
    top.select();
    println!(
        "\x1b[2J\x1b[Hsession: {} run, {} passed, {} failed",
        ran,
        ran - failed,
        failed
    );
    println!("press START to exit.");
    while apt.main_loop() {
        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
        gfx.wait_for_vblank();
    }
}

fn strip_prefix(name: &str) -> String {
    name.strip_prefix("checks::")
        .or_else(|| name.strip_prefix("tests::"))
        .unwrap_or(name)
        .to_string()
}

fn draw_menu(
    console: &Console,
    cases: &[Case],
    n_items: usize,
    cursor: usize,
    view_top: usize,
    visible: usize,
    ran: u32,
    failed: u32,
) {
    console.select();
    print!("\x1b[2J\x1b[H");
    println!("== tests ({}/{})  ran {} fail {}", cursor + 1, n_items, ran, failed);
    println!();
    let end = (view_top + visible).min(n_items);
    // `item` 0 is the synthetic "run all" row; `item` i (>=1) is `cases[i - 1]`.
    for item in view_top..end {
        let mark = if item == cursor { ">" } else { " " };
        if item == 0 {
            println!("{mark} \x1b[33m[ run all {} tests ]\x1b[0m", cases.len());
        } else {
            let mut n = cases[item - 1].name.clone();
            n.truncate(36);
            println!("{mark} {n}");
        }
    }
    print!("\x1b[29;0H");
    print!("Up/Dn move  L/R page  A run  X all  START");
}

fn run_one(case: &Case, top: &Console) -> (u32, u32) {
    top.select();
    print!("\x1b[2J\x1b[H");
    println!("running: {}", case.name);
    match exec(case) {
        Outcome::Passed => {
            println!("\n  \x1b[32mok\x1b[0m");
            (1, 0)
        }
        Outcome::Failed(msg) => {
            println!("\n  \x1b[31mFAILED\x1b[0m");
            for line in msg.lines().take(20) {
                println!("  {line}");
            }
            (1, 1)
        }
    }
}

fn run_all(cases: &[Case], top: &Console) -> (u32, u32) {
    top.select();
    print!("\x1b[2J\x1b[Hrunning all {} tests...\n", cases.len());
    let mut fails: Vec<(String, String)> = Vec::new();
    for case in cases {
        // Tests print their own `ROW|` lines to stdout (= this console).
        if let Outcome::Failed(msg) = exec(case) {
            fails.push((case.name.clone(), msg.lines().next().unwrap_or("").to_string()));
        }
    }
    let passed = cases.len() - fails.len();
    // Fresh screen for the summary so it isn't buried in the row output.
    print!("\x1b[2J\x1b[H");
    if fails.is_empty() {
        println!("\x1b[32mall {} passed\x1b[0m", cases.len());
    } else {
        println!("\x1b[31m{} FAILED\x1b[0m / {passed} passed", fails.len());
        for (name, why) in fails.iter().take(24) {
            let mut line = format!("{name}: {why}");
            line.truncate(48);
            println!("  {line}");
        }
    }
    (cases.len() as u32, fails.len() as u32)
}

fn exec(case: &Case) -> Outcome {
    *LAST_PANIC.lock().unwrap() = None;
    let result = catch_unwind(AssertUnwindSafe(case.run));

    match (result, &case.should_panic) {
        (Ok(Ok(())), ShouldPanic::No) => Outcome::Passed,
        (Ok(Ok(())), _) => Outcome::Failed("expected a panic, but the test returned Ok".into()),
        (Ok(Err(msg)), ShouldPanic::No) => Outcome::Failed(msg),
        (Ok(Err(msg)), _) => Outcome::Failed(format!("expected a panic, got Err: {msg}")),
        (Err(_payload), ShouldPanic::No) => {
            Outcome::Failed(panic_text().unwrap_or_else(|| "panicked".into()))
        }
        (Err(_payload), ShouldPanic::Yes) => Outcome::Passed,
        (Err(_payload), ShouldPanic::YesWithMessage(expected)) => {
            let got = panic_text().unwrap_or_default();
            if got.contains(expected) {
                Outcome::Passed
            } else {
                Outcome::Failed(format!("panic message did not contain {expected:?}: {got}"))
            }
        }
    }
}

fn panic_text() -> Option<String> {
    LAST_PANIC.lock().unwrap().clone()
}
