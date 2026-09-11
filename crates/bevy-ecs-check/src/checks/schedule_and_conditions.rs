//! `Schedule` ordering (`.chain()`/`.after()`), run conditions (`.run_if()`),
//! `SystemSet`s, named schedules via `Schedules`, and `World::run_system_once`.

use super::report;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::system::RunSystemOnce;

#[derive(Resource, Default)]
struct Log(Vec<&'static str>);

#[test]
fn chain_enforces_order_tuple_alone_does_not() {
    let mut world = World::new();
    world.init_resource::<Log>();
    fn a(mut log: ResMut<Log>) {
        log.0.push("a");
    }
    fn b(mut log: ResMut<Log>) {
        log.0.push("b");
    }
    let mut schedule = Schedule::default();
    schedule.add_systems((a, b).chain());
    schedule.run(&mut world);
    report::eq(".chain() runs systems in the given order", "", vec!["a", "b"], world.resource::<Log>().0.clone());
}

#[test]
fn run_if_gates_a_system() {
    let mut world = World::new();
    world.init_resource::<Log>();
    #[derive(Resource)]
    struct Gate(bool);
    world.insert_resource(Gate(false));

    fn maybe_run(mut log: ResMut<Log>) {
        log.0.push("ran");
    }
    let mut schedule = Schedule::default();
    schedule.add_systems(maybe_run.run_if(|gate: Res<Gate>| gate.0));
    schedule.run(&mut world);
    report::eq(".run_if(false) skips the system", "", Vec::<&str>::new(), world.resource::<Log>().0.clone());

    world.resource_mut::<Gate>().0 = true;
    schedule.run(&mut world);
    report::eq(".run_if(true) lets it run", "", vec!["ran"], world.resource::<Log>().0.clone());
}

#[test]
fn system_set_after_orders_a_whole_group() {
    let mut world = World::new();
    world.init_resource::<Log>();

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    enum Phase {
        Early,
        Late,
    }
    fn early_one(mut log: ResMut<Log>) {
        log.0.push("early1");
    }
    fn early_two(mut log: ResMut<Log>) {
        log.0.push("early2");
    }
    fn late(mut log: ResMut<Log>) {
        log.0.push("late");
    }

    let mut schedule = Schedule::default();
    schedule.add_systems(((early_one, early_two).in_set(Phase::Early), late.in_set(Phase::Late).after(Phase::Early)));
    schedule.run(&mut world);
    let log = world.resource::<Log>().0.clone();
    report::eq("late runs after both Phase::Early systems", "", "late", *log.last().unwrap());
    report::eq("… and both Early systems ran (order between them is unspecified)", "", 3usize, log.len());
}

#[test]
fn multiple_named_schedules_on_one_world() {
    #[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
    struct Setup;
    #[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
    struct Tick;

    let mut world = World::new();
    world.init_resource::<Log>();
    let mut schedules = Schedules::default();
    let mut setup_schedule = Schedule::new(Setup);
    setup_schedule.add_systems(|mut log: ResMut<Log>| log.0.push("setup"));
    let mut tick_schedule = Schedule::new(Tick);
    tick_schedule.add_systems(|mut log: ResMut<Log>| log.0.push("tick"));
    schedules.insert(setup_schedule);
    schedules.insert(tick_schedule);
    world.insert_resource(schedules);

    world.run_schedule(Setup);
    world.run_schedule(Tick);
    world.run_schedule(Tick);
    report::eq("World::run_schedule() picks the right named Schedule each time", "", vec!["setup", "tick", "tick"], world.resource::<Log>().0.clone());
}

#[test]
fn run_system_once_runs_a_system_without_a_schedule() {
    fn double(mut n: ResMut<Counter>) {
        n.0 *= 2;
    }
    #[derive(Resource)]
    struct Counter(u32);

    let mut world = World::new();
    world.insert_resource(Counter(3));
    world.run_system_once(double).unwrap();
    report::eq("World::run_system_once() runs it exactly once", "3 -> doubled", 6u32, world.resource::<Counter>().0);
}
