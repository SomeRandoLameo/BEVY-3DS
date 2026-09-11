# bevy-time-check

On-device check that **`bevy_time`** — clocks, `Timer`/`Stopwatch`, fixed timestep, run conditions, delayed commands, and `TimePlugin` under a real `bevy_app::App` — works on the Nintendo 3DS. Aims to touch every public item.

| | |
|---|---|
| Bevy-Einheit | `bevy_time` (zieht `bevy_app`, `bevy_ecs`, `bevy_platform`, `crossbeam-channel`) |
| Version | `0.19.1` |
| Features | `default-features = false`, `features = ["std"]` (kein `bevy_reflect`, kein `serialize`) |
| Target | `armv6k-nintendo-3ds` |
| Toleranz | ε = 1e-4 (f32) / exakte `Duration`-Vergleiche sonst |
| Stand | 2026-09-11 · Azahar-Emulator · **47/47 Testfunktionen · 161/161 Checks bestanden** (+ 1 `#[should_panic]`-Test ohne Tabellenzeile) |

Ausführen: `./scripts/test-emulator.sh -p bevy-time-check`

## Determinismus-Hinweis

Jeder Test hier treibt die Uhr **deterministisch** an — über `Time::advance_by`/`advance_to`, `Time::<Real>::update_with_duration`, oder `TimeUpdateStrategy::ManualDuration`/`ManualInstant`/`FixedTimesteps` unter einer echten `TimePlugin`-`App`. Kein Test hängt von der tatsächlichen Wanduhr ab — bis auf einen einzigen Sanity-Check (`real_time::instant_now_sanity`), der nur prüft, dass zwei `Instant::now()`-Aufrufe nicht panicken und nicht rückwärts laufen. Ob `Instant::now()` auf **echter 3DS-Hardware** eine sinnvolle/monotone Auflösung liefert, bleibt offen (port.md §B6) — dieser Emulator-Lauf sagt darüber nichts aus.

## Abgedeckt

**`Stopwatch`** — `new`/`tick`/`pause`/`unpause`/`is_paused`/`reset` (inkl. „reset behält den Pause-Status"), `set_elapsed`, `elapsed`/`elapsed_secs`/`elapsed_secs_f64`.

**`Timer`/`TimerMode`** — `from_seconds`/`new`, `tick` (Once klemmt bei `duration`, Repeating wickelt um), `is_finished`/`just_finished`, `times_finished_this_tick` (Mehrfach-Trigger pro Tick), `pause`/`unpause`/`reset`, `fraction`/`fraction_remaining`/`remaining`/`remaining_secs`, `finish`/`almost_finish`, `mode`/`set_mode`, `set_elapsed`/`set_duration`/`duration`.

**`Time<T>`** (generische Uhr) — `advance_by` (ersetzt `delta`, akkumuliert `elapsed`; `ZERO` erlaubt), `advance_to` (+ Panic bei Rückwärtslauf), Sekunden-Konvertierungen, `wrap_period`/`set_wrap_period`/`elapsed_wrapped`, `context`/`context_mut`/`as_generic`.

**`Time<Real>`** — `update_with_duration` (deterministisch, erster Aufruf liefert `delta=ZERO`), `startup`/`first_update`/`last_update`, plus ein einzelner `Instant::now()`-Sanity-Check (s. o.).

**`Time<Virtual>`** — über die öffentliche `update_virtual_time(current, virt, real)` (denselben Pfad, den `TimePlugin` jeden Frame läuft): folgt `Time<Real>` bei 1x Geschwindigkeit, `pause`/`unpause`/`toggle` (→ `delta=ZERO`, `effective_speed=0`), `set_relative_speed` (skaliert `delta`), `from_max_delta`/`set_max_delta` (klemmt große reale Sprünge).

**`Time<Fixed>`** — `from_duration`/`from_seconds`/`from_hz`, `set_timestep`/`set_timestep_seconds`/`set_timestep_hz`, `accumulate_overstep`/`discard_overstep`/`overstep_fraction`, und `run_fixed_main_schedule` End-to-End über eine echte `App` (erwartete Lauf-Anzahl über 5 Frames nachgerechnet, wie `bevy_time`s eigener Test), sowie dass die generische `Time`-Resource während `FixedUpdate` den Fixed-Kontext zeigt.

**`common_conditions`** — `on_timer` (Repeating-Timer, feuert pro Intervall), `once_after_delay` (feuert genau einmal), `repeating_after_delay` (trotz des Namens ein `Once`-Timer via `is_finished()` — bleibt **dauerhaft wahr** nach der Verzögerung, kein Intervall-Repeater), `on_real_timer`, `paused`.

**`DelayedCommandsExt`/`DelayedCommandQueue`** — `.duration()`/`.secs()`, verzögerte Spawns über mehrere Frames (End-to-End über `App`+`TimePlugin`, nachgerechnet wie `bevy_time`s eigener Test), sowie dass eine per `.spawn()` auf einer verzögerten `Commands` erzeugte `Entity`-Id sofort für eine zweite, spätere verzögerte Aktion (hier: `despawn`) nutzbar ist.

**`TimePlugin`** — fügt alle vier Uhr-Ressourcen + `TimeUpdateStrategy` ein; alle vier `TimeUpdateStrategy`-Varianten (`Automatic`-Default, `ManualDuration`, `ManualInstant`, `FixedTimesteps`) unter einer echten `App`.

**Layout** — `size_of`/`align_of` von `Stopwatch`, `Timer`, `TimerMode`, `Time`, `Time<Real>`, `Time<Virtual>`, `Time<Fixed>`.

## Ergebnis

Funktion · Eingabe · Erwartet · **tatsächliche Ausgabe** · OK. Reihenfolge = Testausführung (alphabetisch nach Modul/Funktion). Ein Test (`time_generic::advance_to_panics_if_moving_backwards`) hat keine Tabellenzeile — er prüft nur, dass `Time::advance_to` mit einem früheren Zeitpunkt panickt (`#[should_panic(expected = "backwards")]`).

### `conditions::on_real_timer_uses_time_real`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`on_real_timer(500ms) over 3x250ms` | first tick is delta=0, see real_time.rs | `1` | `1` | ✅ |

### `conditions::on_timer_fires_once_per_interval`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`on_timer(1s) fired twice over 2.4s in 400ms steps` |  | `2` | `2` | ✅ |

### `conditions::once_after_delay_fires_exactly_once`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`once_after_delay(500ms) fires exactly once` |  | `1` | `1` | ✅ |

### `conditions::paused_condition_reads_virtual_pause_state`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`paused condition true while Time::<Virtual> is paused` |  | `1` | `1` | ✅ |
|`… false once unpaused` |  | `1` | `1` | ✅ |

### `conditions::repeating_after_delay_stays_true_once_past_the_delay`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`repeating_after_delay(300ms): true on steps 3-6 of 6x100ms` |  | `4` | `4` | ✅ |

### `delayed_commands::delayed_commands_land_on_schedule`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`delayed spawns landed by frame 0 (0.2s steps)` |  | `1` | `1` | ✅ |
|`delayed spawns landed by frame 1 (0.2s steps)` |  | `2` | `2` | ✅ |
|`delayed spawns landed by frame 2 (0.2s steps)` |  | `2` | `2` | ✅ |
|`delayed spawns landed by frame 3 (0.2s steps)` |  | `3` | `3` | ✅ |
|`delayed spawns landed by frame 4 (0.2s steps)` |  | `3` | `3` | ✅ |
|`delayed spawns landed by frame 5 (0.2s steps)` |  | `6` | `6` | ✅ |
|`delayed spawns landed by frame 6 (0.2s steps)` |  | `6` | `6` | ✅ |
|`delayed spawns landed by frame 7 (0.2s steps)` |  | `6` | `6` | ✅ |
|`delayed spawns landed by frame 8 (0.2s steps)` |  | `6` | `6` | ✅ |
|`delayed spawns landed by frame 9 (0.2s steps)` |  | `6` | `6` | ✅ |

### `delayed_commands::delayed_spawns_id_is_usable_for_a_later_delayed_command`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`delayed spawn-then-despawn of a shared Entity id, frame 0` |  | `0` | `0` | ✅ |
|`delayed spawn-then-despawn of a shared Entity id, frame 1` |  | `0` | `0` | ✅ |
|`delayed spawn-then-despawn of a shared Entity id, frame 2` |  | `1` | `1` | ✅ |
|`delayed spawn-then-despawn of a shared Entity id, frame 3` |  | `1` | `1` | ✅ |
|`delayed spawn-then-despawn of a shared Entity id, frame 4` |  | `0` | `0` | ✅ |
|`delayed spawn-then-despawn of a shared Entity id, frame 5` |  | `0` | `0` | ✅ |

### `fixed_time::constructors_and_setters`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Time::<Fixed>::from_duration(500ms)` |  | `500ms` | `500ms` | ✅ |
|`Time::<Fixed>::from_seconds(0.25)` |  | `250ms` | `250ms` | ✅ |
|`Time::<Fixed>::from_hz(8.0)` |  | `125ms` | `125ms` | ✅ |
|`Time::<Fixed>::set_timestep` |  | `500ms` | `500ms` | ✅ |
|`Time::<Fixed>::set_timestep_seconds` |  | `250ms` | `250ms` | ✅ |
|`Time::<Fixed>::set_timestep_hz` |  | `125ms` | `125ms` | ✅ |

### `fixed_time::default_timestep_is_64hz`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Time::<Fixed>::default().timestep() == 1/64s` |  | `15.625ms` | `15.625ms` | ✅ |
|`… overstep starts at ZERO` |  | `0ns` | `0ns` | ✅ |

### `fixed_time::fixed_update_runs_expected_number_of_times`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`FixedUpdate run count after app.update() #0` |  | `0` | `0` | ✅ |
|`FixedUpdate run count after app.update() #1` |  | `0` | `0` | ✅ |
|`FixedUpdate run count after app.update() #2` |  | `1` | `1` | ✅ |
|`FixedUpdate run count after app.update() #3` |  | `1` | `1` | ✅ |
|`FixedUpdate run count after app.update() #4` |  | `2` | `2` | ✅ |

### `fixed_time::generic_time_switches_context_between_schedules`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Time::delta() inside FixedUpdate == Time::<Fixed>::timestep()` |  | `Some(15.625ms)` | `Some(15.625ms)` | ✅ |
|`Time::delta() inside Update is the virtual-time delta (not fixed)` |  | `true` | `true` | ✅ |

### `fixed_time::overstep_accumulate_and_discard`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Time::<Fixed>::accumulate_overstep(1s)` |  | `1s` | `1s` | ✅ |
|`… overstep_fraction (1s / 2s)` |  | `0.5` | `0.5` | ✅ |
|`… overstep_fraction_f64` |  | `0.5` | `0.5` | ✅ |
|`Time::<Fixed>::discard_overstep(400ms)` | 1s - 400ms | `600ms` | `600ms` | ✅ |

### `layout::type_layout`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`size/align Stopwatch` |  | `size / align` | `24 / 8` | ✅ |
|`size/align Timer` |  | `size / align` | `48 / 8` | ✅ |
|`size/align TimerMode` |  | `size / align` | `1 / 1` | ✅ |
|`size/align Time (= Time<()>)` |  | `size / align` | `104 / 8` | ✅ |
|`size/align Time<Real>` |  | `size / align` | `152 / 8` | ✅ |
|`size/align Time<Virtual>` |  | `size / align` | `144 / 8` | ✅ |
|`size/align Time<Fixed>` |  | `size / align` | `136 / 8` | ✅ |

### `plugin::default_strategy_is_automatic`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`TimeUpdateStrategy::default() is Automatic` |  | `true` | `true` | ✅ |

### `plugin::fixed_timesteps_strategy_advances_by_n_times_the_fixed_timestep`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`TimeUpdateStrategy::FixedTimesteps(3) -> Time::<Real>::delta` | 3 x default timestep | `46.875ms` | `46.875ms` | ✅ |

### `plugin::manual_duration_strategy_advances_real_time`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`TimeUpdateStrategy::ManualDuration(16ms) -> Time::<Real>::delta` | 2nd update | `16ms` | `16ms` | ✅ |

### `plugin::manual_instant_strategy_uses_the_given_instant`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`TimeUpdateStrategy::ManualInstant -> Time::<Real>::last_update` |  | `Some(Instant { tv_sec: 9, tv_nsec: 326466100 })` | `Some(Instant { tv_sec: 9, tv_nsec: 326466100 })` | ✅ |
|`… next update with a later Instant -> delta` |  | `50ms` | `50ms` | ✅ |

### `plugin::plugin_inserts_all_four_clocks`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`TimePlugin inserts Res<Time>` |  | `true` | `true` | ✅ |
|`TimePlugin inserts Res<Time<Real>>` |  | `true` | `true` | ✅ |
|`TimePlugin inserts Res<Time<Virtual>>` |  | `true` | `true` | ✅ |
|`TimePlugin inserts Res<Time<Fixed>>` |  | `true` | `true` | ✅ |
|`TimePlugin inserts Res<TimeUpdateStrategy>` |  | `true` | `true` | ✅ |

### `real_time::instant_now_sanity`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Instant::now() called twice doesn't panic and is monotonic` | b after a | `true` | `true` | ✅ |

### `real_time::startup_is_preserved_across_updates`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Time::<Real>::new(startup) -> startup()` |  | `Instant { tv_sec: 9, tv_nsec: 341049349 }` | `Instant { tv_sec: 9, tv_nsec: 341049349 }` | ✅ |
|`Time::<Real>::startup() unchanged by updates` |  | `Instant { tv_sec: 9, tv_nsec: 341049349 }` | `Instant { tv_sec: 9, tv_nsec: 341049349 }` | ✅ |

### `real_time::update_with_duration_advances_from_last_update`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Time::<Real>::default() first_update` |  | `None` | `None` | ✅ |
|`… last_update` |  | `None` | `None` | ✅ |
|`Time::<Real>::update_with_duration(1s) — first update: delta` |  | `0ns` | `0ns` | ✅ |
|`… elapsed (no previous update to diff against)` |  | `0ns` | `0ns` | ✅ |
|`… first_update is now Some` |  | `true` | `true` | ✅ |
|`… second update: delta` | 1s since last_update | `1s` | `1s` | ✅ |
|`… elapsed` |  | `1s` | `1s` | ✅ |
|`… third update: delta` |  | `1s` | `1s` | ✅ |
|`… elapsed accumulates` | 1+1s | `2s` | `2s` | ✅ |

### `stopwatch::new_and_tick`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Stopwatch::new -> elapsed_secs` |  | `0` | `0` | ✅ |
|`Stopwatch::new -> is_paused` |  | `true` | `true` | ✅ |
|`Stopwatch::tick(1.0)` |  | `1` | `1` | ✅ |
|`Stopwatch::tick(1.5) accumulates` | 1.0 + 1.5 | `2.5` | `2.5` | ✅ |
|`Stopwatch::elapsed (Duration)` |  | `2.5s` | `2.5s` | ✅ |
|`Stopwatch::elapsed_secs_f64` |  | `2.5` | `2.5` | ✅ |

### `stopwatch::pause_unpause_and_reset`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Stopwatch::pause -> tick has no effect` |  | `true` | `true` | ✅ |
|`… elapsed_secs stays 0` |  | `0` | `0` | ✅ |
|`Stopwatch::unpause -> ticking resumes` |  | `true` | `true` | ✅ |
|`… elapsed_secs` |  | `1` | `1` | ✅ |
|`Stopwatch::reset -> elapsed_secs` |  | `0` | `0` | ✅ |
|`Stopwatch::reset doesn't unpause` | was already unpaused | `true` | `true` | ✅ |

### `stopwatch::reset_keeps_paused_state`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Stopwatch::reset() keeps `paused` set` | paused before reset | `true` | `true` | ✅ |
|`… elapsed_secs is 0 after reset` |  | `0` | `0` | ✅ |

### `stopwatch::set_elapsed`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Stopwatch::set_elapsed(1.0)` |  | `1` | `1` | ✅ |

### `time_generic::advance_by_sets_delta_and_accumulates_elapsed`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Time::default() delta` |  | `0ns` | `0ns` | ✅ |
|`Time::default() elapsed` |  | `0ns` | `0ns` | ✅ |
|`Time::advance_by(500ms) -> delta` |  | `500ms` | `500ms` | ✅ |
|`… -> elapsed` |  | `500ms` | `500ms` | ✅ |
|`Time::advance_by replaces delta (not additive)` |  | `250ms` | `250ms` | ✅ |
|`… but elapsed keeps accumulating` | 500+250ms | `750ms` | `750ms` | ✅ |
|`Time::advance_by(ZERO) zeroes delta` |  | `0ns` | `0ns` | ✅ |
|`… elapsed unchanged` |  | `750ms` | `750ms` | ✅ |

### `time_generic::advance_to_computes_delta_from_target`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Time::advance_to(2s) from 0 -> delta` |  | `2s` | `2s` | ✅ |
|`… -> elapsed` |  | `2s` | `2s` | ✅ |
|`Time::advance_to(5s) from 2s -> delta` |  | `3s` | `3s` | ✅ |
|`… -> elapsed` |  | `5s` | `5s` | ✅ |

### `time_generic::context_accessors`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Time::context()` |  | `42` | `42` | ✅ |
|`Time::context_mut()` |  | `7` | `7` | ✅ |
|`Time::as_generic() carries delta/elapsed, drops context` |  | `1s` | `1s` | ✅ |

### `time_generic::seconds_conversions`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Time::delta_secs` |  | `1.5` | `1.5` | ✅ |
|`Time::delta_secs_f64` |  | `1.5` | `1.5` | ✅ |
|`Time::elapsed_secs` |  | `1.5` | `1.5` | ✅ |
|`Time::elapsed_secs_f64` |  | `1.5` | `1.5` | ✅ |

### `time_generic::wrap_period`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Time::wrap_period() default is 1 hour` |  | `3600s` | `3600s` | ✅ |
|`Time::set_wrap_period(10s) -> elapsed_wrapped` | elapsed=25s | `5s` | `5s` | ✅ |
|`Time::elapsed_secs_wrapped` |  | `5` | `5` | ✅ |
|`Time::elapsed_secs_wrapped_f64` |  | `5` | `5` | ✅ |

### `timer::default_mode_is_once`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`TimerMode::default()` |  | `Once` | `Once` | ✅ |

### `timer::duration_and_finish`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Timer::duration` |  | `1s` | `1s` | ✅ |
|`Timer::set_duration` |  | `1s` | `1s` | ✅ |
|`Timer::finish` |  | `true` | `true` | ✅ |
|`Timer::almost_finish doesn't finish` |  | `true` | `true` | ✅ |
|`Timer::almost_finish leaves 1ns remaining` |  | `1ns` | `1ns` | ✅ |

### `timer::elapsed_and_set_elapsed`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Timer::elapsed` |  | `500ms` | `500ms` | ✅ |
|`Timer::set_elapsed doesn't check duration` |  | `2s` | `2s` | ✅ |
|`… and doesn't finish the timer` | elapsed > duration but never ticked | `true` | `true` | ✅ |

### `timer::fraction_and_remaining`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Timer::fraction` | 0.5 / 2.0s | `0.25` | `0.25` | ✅ |
|`Timer::fraction_remaining` |  | `0.75` | `0.75` | ✅ |
|`Timer::remaining_secs` |  | `1.5` | `1.5` | ✅ |
|`Timer::remaining` |  | `1.5s` | `1.5s` | ✅ |

### `timer::just_finished`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Timer::just_finished on the finishing tick` |  | `true` | `true` | ✅ |
|`Timer::just_finished false on later ticks` |  | `true` | `true` | ✅ |

### `timer::mode_accessors`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Timer::mode` |  | `Repeating` | `Repeating` | ✅ |
|`Timer::set_mode` |  | `Once` | `Once` | ✅ |

### `timer::once_vs_repeating_finish`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Timer::Once finishes past duration` | tick(1.5) on 1.0s timer | `true` | `true` | ✅ |
|`Timer::Once stays finished` | ticked again | `true` | `true` | ✅ |
|`Timer::Repeating finishes at duration` | tick(1.1) on 1.0s | `true` | `true` | ✅ |
|`Timer::Repeating un-finishes mid-cycle` | 1.1+0.8=1.9 -> wraps to 0.9 | `true` | `true` | ✅ |
|`Timer::Repeating finishes again` | 0.9+0.6=1.5 -> wraps past 1.0 | `true` | `true` | ✅ |

### `timer::pause_unpause_reset`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Timer::pause stops ticking` |  | `true` | `true` | ✅ |
|`… elapsed_secs stays 0` |  | `0` | `0` | ✅ |
|`Timer::unpause resumes` |  | `true` | `true` | ✅ |
|`… elapsed_secs` |  | `0.5` | `0.5` | ✅ |
|`Timer::reset un-finishes` |  | `true` | `true` | ✅ |
|`Timer::reset clears just_finished` |  | `true` | `true` | ✅ |
|`Timer::reset zeroes elapsed_secs` |  | `0` | `0` | ✅ |

### `timer::tick_clamps_once_wraps_repeating`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Timer::Once clamps elapsed at duration` | tick(1.5) on 1.0s | `1` | `1` | ✅ |
|`Timer::Repeating wraps elapsed` | tick(1.5) on 1.0s | `0.5` | `0.5` | ✅ |

### `timer::times_finished_this_tick_multi_fire`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Timer::times_finished_this_tick (6s / 1s)` |  | `6` | `6` | ✅ |
|`… next tick (2s / 1s)` |  | `2` | `2` | ✅ |
|`… sub-duration tick fires zero times` |  | `0` | `0` | ✅ |

### `virtual_time::defaults`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Time::<Virtual>::default() is not paused` |  | `true` | `true` | ✅ |
|`… relative_speed` |  | `1` | `1` | ✅ |
|`… max_delta` |  | `250ms` | `250ms` | ✅ |
|`… delta` |  | `0ns` | `0ns` | ✅ |

### `virtual_time::max_delta_clamps_large_real_jumps`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Time::<Virtual>::from_max_delta(50ms) clamps a 1s real jump` |  | `50ms` | `50ms` | ✅ |

### `virtual_time::paused_clock_has_zero_delta_and_zero_effective_speed`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Time::<Virtual>::pause()` |  | `true` | `true` | ✅ |
|`… paused -> delta is ZERO` |  | `0ns` | `0ns` | ✅ |
|`… effective_speed is 0.0` |  | `0` | `0` | ✅ |
|`… elapsed does not grow while paused` |  | `0ns` | `0ns` | ✅ |
|`Time::<Virtual>::unpause()` |  | `true` | `true` | ✅ |
|`… ticking resumes` |  | `100ms` | `100ms` | ✅ |

### `virtual_time::relative_speed_scales_delta`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Time::<Virtual>::set_relative_speed(2.0)` |  | `2` | `2` | ✅ |
|`… effective_speed matches (not paused)` |  | `2` | `2` | ✅ |
|`… delta is doubled` | 100ms real -> 200ms virtual | `200ms` | `200ms` | ✅ |

### `virtual_time::toggle_flips_pause_state`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Time::<Virtual>::toggle() (unpaused -> paused)` |  | `true` | `true` | ✅ |
|`… toggle again (paused -> unpaused)` |  | `true` | `true` | ✅ |

### `virtual_time::tracks_real_time_at_normal_speed`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Time<Virtual>::delta tracks Time<Real>::delta at 1x speed` |  | `100ms` | `100ms` | ✅ |
|`Time<Virtual>::elapsed tracks Time<Real>::elapsed at 1x speed` |  | `100ms` | `100ms` | ✅ |
|`update_virtual_time also refreshes the generic Time` |  | `100ms` | `100ms` | ✅ |

## Nicht abgedeckt

`bevy_reflect`, `serialize` (serde), `TimeReceiver`/`TimeSender`/`create_time_channels` (Render-Welt-Kanal, nicht relevant ohne `bevy_render`), `TimeUpdateStrategy::Automatic`s tatsächliches Zeitverhalten unter Last, `Instant::now()`-Genauigkeit/Auflösung auf **echter Hardware** (nur ein Monotonie-Sanity-Check im Emulator, s. o.).

**Caveat:** nur Emulator. Siehe `../../port.md` §B6/§B12.
