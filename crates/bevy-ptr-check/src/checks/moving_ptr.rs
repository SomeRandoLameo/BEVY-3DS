//! `MovingPtr<'a, T, A>` + the `move_as_ptr!`/`deconstruct_moving_ptr!` macros
//! — `Box`-like moves without passing `T` by value. Struct/tuple/`MaybeUninit`
//! deconstruction examples are adapted from `bevy_ptr`'s own doc examples.

use super::{report, DropTracker};
use bevy_ptr::{move_as_ptr, MovingPtr, OwningPtr};
use core::cell::Cell;
use core::mem::MaybeUninit;

#[test]
fn move_as_ptr_and_read() {
    let x = 5_i32;
    move_as_ptr!(x);
    report::eq("move_as_ptr!(x) + MovingPtr::read()", "", 5_i32, x.read());
}

#[test]
fn drop_runs_exactly_once_if_a_moving_ptr_is_never_consumed() {
    let counter = Cell::new(0_u32);
    {
        let tracker = DropTracker(&counter);
        move_as_ptr!(tracker);
        // `tracker` (now a `MovingPtr`) just goes out of scope here, unread.
    }
    report::eq("MovingPtr::drop() runs the pointee's Drop when never consumed", "", 1_u32, counter.get());
}

#[test]
fn write_to_does_not_drop_the_destination() {
    // `write_to` is a raw overwrite — the caller owns dropping whatever was
    // at `dst` beforehand. Here `dst` starts uninitialized, so there's
    // nothing to (incorrectly) drop.
    let mut dst = MaybeUninit::<i32>::uninit();
    let value = 17_i32;
    move_as_ptr!(value);
    // SAFETY: `dst.as_mut_ptr()` is valid for writes and properly aligned for `i32`.
    unsafe { value.write_to(dst.as_mut_ptr()) };
    // SAFETY: `dst` was just initialized by `write_to`.
    report::eq("MovingPtr::write_to(dst)", "", 17_i32, unsafe { dst.assume_init() });
}

#[test]
fn assign_to_drops_the_old_value_and_moves_in_the_new_one() {
    let counter_old = Cell::new(0_u32);
    let counter_new = Cell::new(0_u32);
    let mut dst = DropTracker(&counter_old);
    let src = DropTracker(&counter_new);
    move_as_ptr!(src);
    src.assign_to(&mut dst);
    report::eq("MovingPtr::assign_to() drops the previous value at dst", "", 1_u32, counter_old.get());
    report::eq("… and does not drop the newly-moved-in value", "", 0_u32, counter_new.get());
    drop(dst);
    report::eq("… dropping dst now runs the new value's Drop", "", 1_u32, counter_new.get());
}

#[test]
fn deconstruct_moving_ptr_struct_fields() {
    struct FieldA(usize);
    struct FieldB(usize);
    struct FieldC(usize);
    struct Parent {
        field_a: FieldA,
        field_b: FieldB,
        field_c: FieldC,
    }

    let parent = Parent {
        field_a: FieldA(11),
        field_b: FieldB(22),
        field_c: FieldC(33),
    };
    let mut target_a = FieldA(0);
    let mut target_b = FieldB(0);
    let mut target_c = FieldC(0);

    move_as_ptr!(parent);
    bevy_ptr::deconstruct_moving_ptr!({
        let Parent { field_a, field_b, field_c } = parent;
    });
    field_a.assign_to(&mut target_a);
    field_b.assign_to(&mut target_b);
    field_c.assign_to(&mut target_c);

    report::eq("deconstruct_moving_ptr!(struct) — field_a", "", 11, target_a.0);
    report::eq("deconstruct_moving_ptr!(struct) — field_b", "", 22, target_b.0);
    report::eq("deconstruct_moving_ptr!(struct) — field_c", "", 33, target_c.0);
}

#[test]
fn deconstruct_moving_ptr_tuple_fields() {
    let parent = (11_usize, 22_usize, 33_usize);
    let mut target_0 = 0_usize;
    let mut target_1 = 0_usize;
    let mut target_2 = 0_usize;

    move_as_ptr!(parent);
    bevy_ptr::deconstruct_moving_ptr!({
        let tuple { 0: field_0, 1: field_1, 2: field_2 } = parent;
    });
    field_0.assign_to(&mut target_0);
    field_1.assign_to(&mut target_1);
    field_2.assign_to(&mut target_2);

    report::eq("deconstruct_moving_ptr!(tuple) — .0", "", 11_usize, target_0);
    report::eq("deconstruct_moving_ptr!(tuple) — .1", "", 22_usize, target_1);
    report::eq("deconstruct_moving_ptr!(tuple) — .2", "", 33_usize, target_2);
}

#[test]
fn deconstruct_moving_ptr_maybe_uninit_fields() {
    struct Parent {
        field_a: usize,
        field_b: usize,
    }
    let parent = MaybeUninit::new(Parent { field_a: 11, field_b: 22 });
    let mut target_a = MaybeUninit::new(0_usize);
    let mut target_b = MaybeUninit::new(0_usize);

    move_as_ptr!(parent);
    bevy_ptr::deconstruct_moving_ptr!({
        let MaybeUninit::<Parent> { field_a, field_b } = parent;
    });
    field_a.assign_to(&mut target_a);
    field_b.assign_to(&mut target_b);

    // SAFETY: both targets were just assigned a fully-initialized `usize`.
    report::eq("deconstruct_moving_ptr!(MaybeUninit) — field_a", "", 11_usize, unsafe { target_a.assume_init() });
    // SAFETY: same as above.
    report::eq("deconstruct_moving_ptr!(MaybeUninit) — field_b", "", 22_usize, unsafe { target_b.assume_init() });
}

#[test]
fn partial_move_splits_a_value_and_returns_the_rest() {
    struct FieldA(usize);
    struct FieldB(usize);
    struct Parent {
        field_a: FieldA,
        field_b: FieldB,
    }
    let parent = Parent { field_a: FieldA(1), field_b: FieldB(2) };
    move_as_ptr!(parent);

    let (rest, taken_a) = MovingPtr::partial_move(parent, |parent_ptr| {
        // SAFETY: `field_a` is unique and not aliased by anything else here.
        let field_a = unsafe { parent_ptr.move_field(|p| &raw mut (*p).field_a) };
        field_a.read().0
    });
    report::eq("MovingPtr::partial_move() returns the closure's result", "", 1_usize, taken_a);

    // The rest (a `MovingPtr<MaybeUninit<Parent>>`) still owns `field_b`.
    bevy_ptr::deconstruct_moving_ptr!({
        let MaybeUninit::<Parent> { field_a: _unused, field_b } = rest;
    });
    // SAFETY: `field_b` was never touched by `partial_move` above, so it's
    // still the fully-initialized `FieldB` from the original `parent`.
    let field_b = unsafe { field_b.assume_init() };
    report::eq("… and the untouched field is still reachable", "", 2_usize, field_b.read().0);
}

#[test]
fn owning_ptr_round_trip() {
    let value = 202_i32;
    move_as_ptr!(value);
    let owning: OwningPtr = value.into();
    // SAFETY: `i32` is the type this `MovingPtr` (and thus `OwningPtr`) erased.
    report::eq("MovingPtr -> OwningPtr::into() round-trips", "", 202_i32, unsafe { owning.read() });
}
