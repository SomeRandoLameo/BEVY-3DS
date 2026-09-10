// Under `cargo 3ds test`, swap the std test harness (which needs a hosted OS)
// for `test-runner`'s GDB-backed one. No effect on normal builds.
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(test_runner::run_gdb))]

use ctru::prelude::*;

fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let _console = Console::new(gfx.top_screen.borrow_mut());

    println!("Hello, World!");
    println!("\x1b[29;16HPress Start to exit");

    while apt.main_loop() {
        gfx.wait_for_vblank();

        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    /// Sanity check that the emulator test harness is wired up.
    /// Run with `./scripts/test-emulator.sh` (or `cargo 3ds test` on hardware).
    #[test]
    fn it_runs_on_the_3ds() {
        assert_eq!(2 + 2, 4);
    }
}
