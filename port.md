| Crate | Einstufung | Begründung | getestet | % |
|---|---|---|---|---|
| bevy_ecs | Sicher (core + Multi-Threaded-Executor) | Core-ECS = reine Logik/Daten. Aber: nicht "platformfrei" — Entity-Allocator nutzt 64-bit-Atomics (Fallback, §B1), `thread_local!`, Parallel-Executor jetzt Crate-Default (§B2). | ✅ **145/145 Checks** (80 Testfn, Emu, `crates/bevy-ecs-check`) — gesamte öffentliche API: `World`, `Commands`, Queries + alle Filter, Change Detection, Resources, Components/Bundles/`#[require(...)]`, `ChildOf`/`Children`, Messages (ex-`Events`), Observers, Schedules/`run_if`/`SystemSet`, **und Threading** (§B14-Threading). Liste → `crates/bevy-ecs-check/README.md`, Überblick §B14 | 100% |
| bevy_math | Sicher | glam-Reexport + Bevy-Primitives. Zieht **glam 0.32** neben citro3ds glam 0.30 (koexistieren, §B8), `scalar`-Backend (kein NEON). `rand`-Feature baut auf 3DS (kein `getrandom`, §B9). | ✅ **488/488 Checks** (57 Testfn, Emu, `crates/bevy-math-check`) — quasi die ganze öffentliche API. Vollständige Liste → `crates/bevy-math-check/README.md`, Überblick §B9 | 90% |
| bevy_transform | Sicher (1 Thread); `multi_threaded` ✗ | Mathe + Hierarchie. Propagation nutzt `par_iter`/`ComputeTaskPool` — hat aber ohne `multi_threaded` **explizite serielle Fallbacks** (`serial`-Modul, `iter_mut`-Branches). Die laufen korrekt (§B10). Der parallele Pfad selbst **kompiliert nicht** auf diesem Target — Upstream-Bug, §B10-Threading. | ✅ **129/129 Checks** (38 Testfn, Emu) — ganze öffentliche API inkl. Propagation, `TransformHelper`, `BuildChildrenTransformExt`, `TransformPlugin`/`App::update()`. Liste → `crates/bevy-transform-check/README.md`, Überblick §B10. ❌ `multi_threaded`: `error[E0425]` (`AtomicU64` fehlt), §B10-Threading | 70% |
| bevy_ptr | Sicher | keine Platform-Annahmen; fast ausschließlich `unsafe`-Zeigercode (Typ-Erasure, `Aligned`/`Unaligned`-Zugriffe) — genau die Art Code, bei der ARM-Alignment relevant werden könnte | ✅ **69/69 Checks** (39 Testfn, Emu, `crates/bevy-ptr-check`) — gesamte öffentliche API inkl. eines echten unausgerichteten `u32`-Reads auf `armv6k`. Liste → `crates/bevy-ptr-check/README.md`, Überblick §B13 | 100% |
| bevy_utils | Sicher | keine Platform-Annahmen | ✅ baut (transitiv) | 20% |
| bevy_platform | Sicher (mit Fallbacks) | **ist** die Platform-Abstraktion: Atomic-Shim (64-bit → `portable-atomic` Fallback), `Instant`, sync-Primitive. Baut für 3DS. | ✅ baut (transitiv) | 20% |
| bevy_derive | Sicher | Makros, keine Runtime-Abhängigkeit || 20% |
| bevy_macro_utils | Sicher | Makros, keine Runtime-Abhängigkeit || 20% |
| bevy_color | Sicher | reine Farbraum-Mathe | ✅ **381/381 Checks** (44 Testfn, Emu, `crates/bevy-color-check`) — jeder Farbraum + Konversionsgraph. Liste → `crates/bevy-color-check/README.md`, Überblick §B11. Auch **im Demo verbaut** (`Tint`/`Hue::rotate_hue`, Emu-verifiziert, s. u.) | 100% |
| bevy_time | Sicher (deterministischer Kern) | Baut, `App::update()` mit `TimePlugin` läuft. `bevy_platform::Instant` → `std::time::Instant`. Ob `Instant::now()` auf **3DS-Hardware** akkurat/monoton liefert, ist weiterhin UNGEPRÜFT — im Demo läuft `TimePlugin` jetzt mit, aber fest auf `TimeUpdateStrategy::ManualDuration` gepinnt, sodass es nie `Instant::now()` aufruft (die FPS-Anzeige nutzt weiterhin direkt `svcGetSystemTick`, §B6). | ✅ **161/161 Checks** (47 Testfn, Emu, `crates/bevy-time-check`) — gesamte öffentliche API, komplett deterministisch (`TimeUpdateStrategy`/`advance_by`, keine echte Wanduhr außer einem einzelnen `Instant::now()`-Sanity-Check). Liste → `crates/bevy-time-check/README.md`, Überblick §B12. **Im Demo verbaut:** `TimePlugin` unter `App::new()`, `ManualDuration(DT)` als Sim-Uhr (ersetzt das handgerollte `SimTime`), Emu-verifiziert inkl. echtem `app.update()`-Loop. `Instant` auf HW weiterhin ✗ | 90% |
| bevy_app | Sicher (Kern) | Runner passt nicht, `App::update()` manuell nötig. `App::new().add_plugins(...)` + **`app.update()` läuft auf dem 3DS** — jetzt auch als echter interaktiver Loop im Demo, nicht nur in Tests (via `bevy-transform-check` §B10). Eigener `bevy-app-check` noch offen (SubApps, Plugin-Ordering, `AppExit`). | ✅ `update()` (Emu, Tests + Demo-Loop, 60×/s) | 50% |
| bevy_state | Wahrscheinlich okay | reine State-Machine-Logik || 0% |
| bevy_diagnostic | Wahrscheinlich okay | evtl. OS-Metriken disablen || 0% |
| bevy_asset | Riskant | Async-Loader, Thread-Pool-Abhängigkeit || 0% |
| bevy_tasks | Sicher (Emu, HW offen) | **Nicht-optionale Dep von bevy_ecs.** `multi_threaded` (zieht `async-executor` + `concurrent-queue` + echte Threads) ist seit 2026-09-20 **Crate-Default** in `dove` selbst und allen `bevy-*-check`-Crates, matcht normales Bevys eigenen Default → §B2. | ✅ **10/10 Checks** (5 Testfn, Emu, `crates/bevy-ecs-check`, Default-Build) — `std::thread`/`ComputeTaskPool`/`MultiThreadedExecutor`/`Query::par_iter` laufen korrekt über `pthread-3ds`, kein Patch nötig. Überblick §B2/§B14-Threading. Echte HW-Contention ungeprüft | 50% |
| bevy_reflect | Riskant | schwergewichtig, Compile-Zeit/Binary-Size. `uuid`-Feature-Pfade können `getrandom` ziehen → §"Plattform-Fakten". Gate für app/scene/state/asset. || 0% |
| bevy_scene | Riskant | hängt an reflect || 0% |
| bevy_gltf | Riskant | hängt an Asset/Mesh/Image-Pipeline || 0% |
| bevy_mesh | Riskant | Datenstruktur ok, aber eng an render gekoppelt || 0% |
| bevy_animation | Riskant | hängt an curve, evtl. isolierbar || 0% |
| bevy_render | Ausgeschlossen | wgpu-gebunden || 0% |
| bevy_core_pipeline | Ausgeschlossen | wgpu-gebunden || 0% |
| bevy_pbr | Ausgeschlossen | wgpu-gebunden || 0% |
| bevy_sprite | Ausgeschlossen | wgpu-gebunden || 0% |
| bevy_sprite_render | Ausgeschlossen | wgpu-gebunden || 0% |
| bevy_ui | Ausgeschlossen | wgpu-gebunden || 0% |
| bevy_ui_render | Ausgeschlossen | wgpu-gebunden || 0% |
| bevy_ui_widgets | Ausgeschlossen | wgpu-gebunden || 0% |
| bevy_window | Ausgeschlossen | Desktop-Fenstersystem || 0% |
| bevy_winit | Ausgeschlossen | Desktop-Fenstersystem || 0% |
| bevy_camera | Ausgeschlossen | render-gekoppelt || 0% |
| bevy_light | Ausgeschlossen | render-gekoppelt || 0% |
| bevy_material | Ausgeschlossen | render-gekoppelt || 0% |
| bevy_image | Ausgeschlossen | GPU-Upload-Pfad || 0% |
| bevy_gizmos | Ausgeschlossen | render-gekoppelt || 0% |
| bevy_picking | Ausgeschlossen | Desktop-Input || 0% |
| bevy_input_focus | Ausgeschlossen | Desktop-Input || 0% |
| bevy_text | Ausgeschlossen | render-gekoppelt || 0% |
| bevy_audio | Ausgeschlossen | cpal statt 3DS-Audio-API || 0% |
| bevy_android | Ausgeschlossen | irrelevante Plattform || 0% |
| bevy_gilrs | Ausgeschlossen | irrelevante Plattform || 0% |

`%` = geschätzter Anteil der Crate, der auf diesem Target tatsächlich on-device
getestet UND bestätigt funktionsfähig ist (nicht: Anteil bestandener Checks —
die sind per Definition immer 100%, weil kaputte Tests hier repariert statt
stehen gelassen werden). 100% = eigene Check-Crate deckt die gesamte
öffentliche API ab, alles grün, keine bekannten Lücken. Abzüge für: bekannte
Teil-Lücken trotz "ganzer API" (90%), einen bestätigt kaputten/ungetesteten
Subsystem-Zweig wie `multi_threaded` bei `bevy_transform` (70%), nur den Kern
statt einer dedizierten Check-Crate getestet (50%), nur transitiv gebaut ohne
jeden Funktionstest (20%), gar nicht angefasst bzw. bewusst ausgeschlossen (0%).

---

## Befunde (Stand 2026-09-11, Bevy 0.19.1, `armv6k-nintendo-3ds`)

### Was tatsächlich verifiziert ist

Kompiliert **+ gelinkt + auf echter 3DS-Hardware gelaufen** (Swarm-Demo `src/main.rs`,
Stand vor der `bevy_color`/`Tint`-, `bevy_time`/`Time`- und `bevy_app`/`App`-
Erweiterung — alle drei sind bisher nur Emulator-verifiziert, s. u.) sowie mit
Assertions im Emulator (`./scripts/test-emulator.sh`):

- `bevy_ecs` 0.19.1, `default-features = false, features = ["std"]`:
  **135 Assertion-Checks** (75 Testfunktionen) im Emulator — die gesamte
  öffentliche API: `World`, `Commands`, Queries + alle Filter, Change
  Detection, Resources, Components/Bundles/`#[require(...)]`, `ChildOf`/
  `Children`, Messages (ex-`Events`), Observers, Schedules/`run_if`/
  `SystemSet`, exotische System-Params. **Alles single-threaded.**
  Siehe §B14 / `crates/bevy-ecs-check/README.md`.
- `bevy_math` 0.19.1, `default-features = false, features = ["std", "curve", "rand"]`:
  **488 Assertion-Checks** (57 Testfunktionen) im Emulator — nahezu die komplette
  öffentliche API. Siehe §B9 / `crates/bevy-math-check/README.md`. In `dove` selbst nur `["std"]` nötig.
- `bevy_transform` 0.19.1, `default-features = false, features = ["std", "bevy-support"]`:
  **129 Assertion-Checks** (38 Testfunktionen) im Emulator — die gesamte öffentliche API
  inkl. Propagations-Pipeline, `TransformHelper`, `BuildChildrenTransformExt`,
  `TransformPlugin`. Siehe §B10 / `crates/bevy-transform-check/README.md`. Der §B2-Verdacht
  ist damit für `bevy_transform` entschärft.
- `bevy_app` 0.19.1, `default-features = false, features = ["std"]`:
  `App::new()` + `add_plugins` + **`app.update()` propagiert korrekt** — via
  `bevy-transform-check` §B10. `bevy_ecs::system::RunSystemOnce` funktioniert
  ebenfalls. **Im Demo verbaut** (2026-09-11): `src/main.rs` läuft jetzt über
  `App::new()` + `TimePlugin` statt einem nackten `World`/`Schedule` —
  `main()`'s Loop ruft `app.update()` einmal pro Frame auf (60×/s in Azahar),
  was `First → PreUpdate → RunFixedMainLoop → Update → PostUpdate → Last`
  inkl. Message-Registry tatsächlich als **echten interaktiven App-Loop**
  durchläuft, nicht nur in isolierten Tests. Bewusst `App::new()`, nicht
  `App::default()`s `DefaultPlugins`-Pendant (braucht `bevy_render`/
  `bevy_winit`, die es hier nicht gibt) — nur `TimePlugin` und die eigenen
  Systeme. `TimeUpdateStrategy::ManualDuration` gepinnt, damit `TimePlugin`
  nie `Instant::now()` aufruft (§B6 bleibt dadurch unberührt offen). Neuer
  Test `app_update_drives_the_swarm_end_to_end` in `src/main.rs` prüft die
  ganze Pipeline End-to-End, nicht nur einzelne Systeme über einen nackten
  `Schedule`.
- `bevy_color` 0.19.1, `default-features = false, features = ["std"]`:
  **381 Assertion-Checks** (44 Testfunktionen) im Emulator — jeder Farbraum
  (`Srgba`/`LinearRgba`/`Hsla`/`Hsva`/`Hwba`/`Laba`/`Lcha`/`Oklaba`/`Oklcha`/`Xyza`),
  der `Color`-Enum, `ColorCurve`, die Paletten und der komplette Konversionsgraph.
  Siehe §B11 / `crates/bevy-color-check/README.md`. **Im Demo verbaut:** jedes
  Dreieck ist jetzt `(Body, Spin, Pulse, Tint)` — `Tint` dreht eine `Hsla`-Hue
  (`Hue::rotate_hue`) und bäckt sie über `Srgba` zu den drei Eck-`Vec3`-Farben,
  120° auf dem Farbrad auseinander (Emu-verifiziert inkl. `cargo 3ds test`;
  noch nicht auf echter Hardware nachgezogen).
- `bevy_time` 0.19.1, `default-features = false, features = ["std"]`:
  **161 Assertion-Checks** (47 Testfunktionen) im Emulator — die gesamte
  öffentliche API (`Stopwatch`, `Timer`, `Time<Real/Virtual/Fixed>`,
  `common_conditions`, `DelayedCommandsExt`, `TimePlugin`), komplett
  deterministisch über `TimeUpdateStrategy`/`advance_by` statt der echten
  Wanduhr. Siehe §B12 / `crates/bevy-time-check/README.md`. **Im Demo
  verbaut:** `src/main.rs` läuft jetzt über `App::new()` + `TimePlugin`
  (`Res<Time>` in `drift`/`spin`/`tint`), mit `TimeUpdateStrategy::
  ManualDuration(DT)` gepinnt — ersetzt das handgerollte `SimTime`, ohne dass
  `TimePlugin` je `Instant::now()` aufruft (Emu-verifiziert inkl. echtem
  `app.update()`-Loop; die FPS-Anzeige bleibt bei `svcGetSystemTick`).
  **Nicht beantwortet:** ob `Instant::now()` auf echter 3DS-Hardware eine
  brauchbare/monotone Auflösung liefert (§B6 bleibt offen;
  nur ein einzelner Monotonie-Sanity-Check lief im Emulator).
- `bevy_ptr` 0.19.1, keine Features (die Crate hat keine): **69 Assertion-
  Checks** (39 Testfunktionen) im Emulator — die gesamte öffentliche API
  (`Ptr`/`PtrMut`/`OwningPtr`/`MovingPtr`/`ConstNonNull`/`ThinSlicePtr`,
  `move_as_ptr!`/`deconstruct_moving_ptr!`), inkl. eines echten unausgerichteten
  `u32`-Reads über einen absichtlich fehlausgerichteten Byte-Puffer — bestätigt,
  dass `armv6k` unausgerichtete Mehrbyte-Loads korrekt handhabt. Siehe §B13 /
  `crates/bevy-ptr-check/README.md`. Nicht ins Demo eingebaut (reines
  ECS-internes Hilfswerkzeug, `dove` selbst benutzt es nie direkt).
- Transitiv mitgebaut **+ gelinkt** (nicht separat funktionsgetestet):
  `bevy_platform`, `bevy_utils`, `bevy_tasks` (single-threaded), `bevy_ecs_macros`,
  `bevy_derive`, `glam` 0.32, `crossbeam-channel` (via `bevy_time`'s `std`-Feature).

Vehikel für weitere isolierte Tests: **`crates/bevy-ecs-check`**,
**`crates/bevy-math-check`**, **`crates/bevy-transform-check`**,
**`crates/bevy-color-check`**, **`crates/bevy-time-check`**,
**`crates/bevy-ptr-check`** (jeweils ohne citro3d im Baum, on-device via
`./scripts/test-emulator.sh -p <crate>`; `crates/all-checks` bündelt alle
sechs in eine `.3dsx`, **305/305 Testfunktionen** bestanden im Emulator —
Default-Build, `multi_threaded` inklusive, s. §B2/§B14-Threading).

### Plattform-Fakten `armv6k-nintendo-3ds`

- **Tier-3-Target.** Nightly + `rust-src`; `cargo-3ds` baut mit `-Z build-std=std,test`. Kein prebuilt `std`.
- **Atomics:** 8 / 16 / 32 / ptr **nativ** (ARMv6K hat LDREX/STREX). **Kein 64-bit-Atomic.**
  `pointer_width = 32`.
- **Panic:** `unwind` wird unterstützt (`std`-Build zieht `panic_unwind`/`unwind`). Bevy-Panics
  (fehlende Resource, Query-Konflikt) beenden die Homebrew sauber statt `abort`.
- **`getrandom`:** kein Backend für dieses Target. **`rand` selbst ist okay** — mit
  `default-features = false` (wie `bevy_math`/`bevy_ecs` es pinnen) zieht es kein `getrandom`
  und baut sauber; man muss nur einen expliziten `RngCore`/`TryRng` liefern (kein `OsRng`).
  Erst Features die `getrandom` *direkt* wollen (`rand/os_rng`, `uuid/v4`, evtl. `rand/thread_rng`)
  brechen den Build — dann `getrandom` „custom"-Backend nötig (`ps:GenerateRandomBytes`).
- **Threads:** `std::thread` läuft über `pthread-3ds`. Old 3DS: 2 Cores, Core 1 (der
  "System-Core") standardmäßig **0% Zeitscheibe für Homebrew** — braucht explizit
  `Apt::set_app_cpu_time_limit(percent)` (5–89%, empfohlen ~30–45%) **vor** dem ersten
  Thread-Spawn, sonst laufen alle Worker-Threads auf Core 0 mit (kein Speedup ohne
  diesen Call — auf Old-3DS-Hardware bestätigt, s. §B2; der Fix selbst noch nicht
  nachgemessen). New 3DS: Cores 2+3 frei ohne diese Erlaubnis-Prozedur (804 MHz via
  `osSetSpeedupEnable`). Kein automatisches `num_cpus`-Sizing — Threadpools
  **explizit** begrenzen.
- **Zeit:** `std::time::Instant` **nicht auf HW verifiziert** (Demo nutzt `svcGetSystemTick`).
- **Speicher:** Heap grob Old 3DS ~64–96 MB, New 3DS ~124 MB+. GPU-Buffer via `ctru::linear`
  (LinearAllocator), getrennter Pool.
- **Editions:** Bevy-Crates sind `edition = "2024"`, `rust-version` ~1.94–1.95.

### Wo Macken liegen könnten — bevy_ecs

**§B1 — 64-bit-Atomic-Fallback (betrifft *jeden* spawn/despawn).**
`bevy_ecs::entity::remote_allocator::Allocator` ist der Kern-Entity-Allocator. Er nutzt
`bevy_platform`-Atomics; auf 3DS wird `AtomicU64` → `portable_atomic::AtomicU64` im
**lock-basierten `fallback`** (globaler Spinlock, `spin`-Crate). Einzelne `Slot`s sind als
`low_bits`/`high_bits: AtomicU32` gesplittet, `FreeCount` ist ein `portable_atomic::AtomicU64`.
- Single-threaded: unkritisch (unkontendiert, `Relaxed`).
- Multi-threaded: *korrekt* (echter Lock), aber (a) **prozess-globaler** Lock für ALLE
  64-bit-Atomic-Ops, (b) Spinlock-Holder-Preemption auf 2 Cores → Cycle-Waste / Prioritäts-
  inversion, (c) getornte Reads im Split-Pfad + `compare_exchange`-Retry auf schwach geordnetem
  ARM11 — **nur auf Hardware unter echter Contention validierbar.**
- **Teilentschärft (§B14-Threading, Emulator):** 30 Runden `Query::par_iter_mut()` über
  50 Entities (also durchgängige Spawn/Despawn-Allocator-Aktivität unter dem parallelen
  Executor) liefen ohne falsches Ergebnis oder Deadlock. Sagt nichts über echte
  Zwei-Kern-Contention/Preemption auf ARM11-Hardware aus (Azahar emuliert das nicht
  zyklengenau) — (c) bleibt bis zu einem Hardware-Lauf offen.

**§B2 — `multi_threaded` / paralleler Executor / Threadpools.**
**Erledigt (Emulator, §B14-Threading):** `bevy_tasks`' `multi_threaded`-Feature
(`async-executor`/`concurrent-queue`/`async-channel`) baut und linkt;
`ComputeTaskPool::get_or_init()` mit explizitem `TaskPoolBuilder::num_threads(2)`
(bzw. `bevy_app::TaskPoolPlugin { task_pool_options: TaskPoolOptions::
with_num_threads(2) }` im Demo) erzeugt einen echten Thread-Pool über
`pthread-3ds`; `Schedule::default()` wählt automatisch den `MultiThreadedExecutor`
(kein `ExecutorKind`-Opt-in mehr in 0.19) und läuft über mehrere `run()`s korrekt;
`Query::par_iter()`/`par_iter_mut()` (die gebatchte Parallel-Query-Iteration über
`TaskPool::scope`) lief über 200 bzw. 50 Entities korrekt, inkl. 30
Wiederholungsrunden ohne Deadlock oder verlorenes Update. **Kein Patch an
`pthread-3ds` nötig** — `pthread_create`/`pthread_join`/Mutex/Condvar/TLS
funktionieren bereits wie erwartet. **Seit 2026-09-20 Crate-Default** — sowohl
in `dove` selbst (`Cargo.toml`s `bevy_ecs`-Feature, `App::new()` + `TaskPoolPlugin`
in `src/main.rs`) als auch in `bevy-ecs-check`/`all-checks` (matcht normales
Bevys eigenen Default; vorher war es ein `--features multi_threaded`-Opt-in).
`dove`s eigener Demo-Loop (Triangle-Swarm über `app.update()`) lief nach der
Umstellung fehlerfrei im Emulator (Boot + Laufzeit ohne Crash/Panic geprüft,
alle 5 `#[test]`s in `src/main.rs` weiterhin grün). Details, Testtabelle und
offene Punkte: §B14-Threading / `crates/bevy-ecs-check/README.md`
„Threading"-Abschnitt. **Weiterhin offen:** alles davon auf echter
**Hardware** (der Emulator validiert Korrektheit, nicht reale
Zwei-Kern-Nebenläufigkeit/Timing/Contention — s. §B1), `AsyncComputeTaskPool`/
`IoTaskPool`, echte Systemkonflikt-Serialisierung unter Last, Langlauf (§B7).

**Erster echter Hardware-Befund (2026-09-20, User-Test):** `dove`s eigenes
Demo (parallelisiert: `drift`/`bounce`/`spin`/`tint` über `Query::par_iter_mut()`,
der Vertex-Bake-Loop der Render-Schleife über `std::thread::scope` in 2
Chunks — s. `src/main.rs`) zeigte auf **Old 3DS bei 8k Dreiecken keinen
Unterschied** (weiterhin 9.7 fps), auf **New 3DS dagegen einen echten
Sprung** (19 fps Peak bei 8k Dreiecken). Ursache gefunden: Old 3DS' zweiter
Kern (Core #1, der "System-Core") ist standardmäßig für OS-Dienste reserviert
— Homebrew bekommt davon 0%, bis `APT_SetAppCpuTimeLimit` explizit eine
Zeitscheibe anfordert (ctru-rs' eigene Doku zu `Apt::set_app_cpu_time_limit`:
"It is necessary to set a time limit before spawning threads on the syscore
(core #1)"). `dove` rief das nie auf — beide Worker-Threads liefen also
vermutlich beide auf Core 0, kein echter Speedup. New 3DS ist davon nicht
betroffen (die Extra-Kerne #2/#3 sind frei nutzbar, keine Erlaubnis nötig —
erklärt, warum dort schon vorher ein Unterschied da war). **Fix:** `main()`
ruft jetzt `apt.set_app_cpu_time_limit(30)` (ctru-rs' empfohlener Bereich:
5–89%, "around 30–45%") gleich nach `Apt::new()`, vor `TaskPoolPlugin`/jedem
Thread-Spawn. Noch nicht auf Old-3DS-Hardware nachgemessen, ob das den fps-
Unterschied tatsächlich bringt — **Azahar kann das nicht validieren**, der
Emulator loggt `APT_SetAppCpuTimeLimit` als `(STUBBED)` (kein Effekt in der
Emulation, nur ein No-op-Erfolg). Das ist ein weiterer konkreter Beleg für die
Emulator-Grenze aus §B1: für alles, was echte Kern-/Scheduling-Details auf
Old 3DS angeht, ist nur ein Hardware-Lauf aussagekräftig.

**§B3 — `thread_local!`.** `bevy_ecs` nutzt TLS im Error-Handling (`error/bevy_error.rs`).
`pthread-3ds` liefert TLS (`has-thread-local = true`) — durch §B14-Threading indirekt
mitgetestet (`pthread-3ds`s eigene Thread-ID/Executor-Bücher laufen selbst über
`#[thread_local]`, und alle Threading-Tests liefen fehlerfrei), aber `bevy_ecs`s
*eigener* TLS-Pfad im Error-Handling selbst nicht gezielt geprüft.

**§B4 — Kern-APIs jenseits von spawn/despawn/Query/Res (alle single-thread).**
**Erledigt (§B14, `bevy-ecs-check`):** `Commands` + deferred apply (`world.flush`/
`CommandQueue::apply`), `EntityCommands`; Messages (0.19s Umbenennung von
Events: `MessageReader`/`MessageWriter`, Doppelpuffer + Update-System);
Change Detection über Frames (`Added`/`Changed`/`Ref`, inkl. der lazy
`Mut<T>`-Markierung); Hooks/Observers (`On<Add/Insert/Remove, T>`, Custom-
`Event`s über `On<E>`); Relationships (`ChildOf`/`Children`); `Local`,
`ParamSet`, exklusive Systeme (`&mut World`). **Weiterhin ungetestet:**
`NonSend` (auf einer Single-Thread-Homebrew ohnehin fragwürdig relevant),
Tick-Wraparound bei ~2^31 Ticks (praktisch nicht erreichbar in einem Testlauf).

**§B5 — Default-Features.** Wir fahren `default-features = false`. Bevy-Default =
`bevy_reflect` + `async_executor` + `backtrace`. `backtrace` deaktiviert lassen
(`std::backtrace` auf 3DS fragwürdig). `bevy_reflect` separat prüfen (§table).

**§B6 — Zeit.** `bevy_ecs` core nutzt **keine** Wall-Clock. `bevy_time`/`bevy_app` schon:
`bevy_platform::Instant` → `std::time::Instant`. Vor `bevy_time`/`bevy_app` unbedingt
`Instant::now()` + `elapsed()` auf **Hardware** testen; sonst `bevy_platform` mit eigener
Instant-Impl (svcGetSystemTick) patchen.

**§B7 — Dauerlauf/Speicher.** Emulator fängt Leaks, Archetype-Wachstum, Allocator-
Fragmentierung und Lock-Verhalten unter Last **nicht**. Langlauf-Test auf HW nötig.

**§B8 — glam-Doppelversion.** `citro3d` → glam 0.30, `bevy_math` → glam 0.32. Koexistieren im
Binary (semver-inkompatibel, Cargo hält beide). `bevy_math`-`Mat4` kann **nicht** direkt an
citro3ds Uniform-API übergeben werden — über `[f32; 16]` / `Matrix4::from_*` marshallen.
Bei weiteren Bevy-Crates die glam brauchen: alle auf 0.32 halten.

**§B9 — bevy_math: Testergebnis (`crates/bevy-math-check`).**
`bevy_math` 0.19.1, Features `["std", "curve", "rand"]`. Lauf:
`./scripts/test-emulator.sh -p bevy-math-check` (Azahar, `test-runner`/GDB).
**57 / 57 Testfunktionen · 488 / 488 Checks bestanden**, ε = 1e-4 (f32) / 1e-9 (f64).

→ **Vollständige Liste** (jede Funktion · Eingabe · Erwartet · **tatsächliche Ausgabe** · OK,
57 Sektionen): **`crates/bevy-math-check/README.md`**.

Abgedeckt (nahezu die gesamte öffentliche API):

| Bereich | Umfang |
|---|---|
| `bevy_math::ops` | **alle 32 Funktionen** (das newlib-Risiko) — trig, hyperbolisch, exp/log, Wurzeln, Rundung |
| glam Vektoren | `Vec2/3/3A/4` (jede distinkte Methode), `DVec2/3/4` (f64 → newlib double), `IVec2/3/4`, `UVec3`, `I64Vec3`, `U64Vec3`, `BVec3` |
| glam Matrizen | `Mat2`, `Mat3`, `Mat3A`, `Mat4`, `DMat4` — Konstruktoren, `transform_point/vector`, `inverse`, `determinant`, `transpose`, `look_at_rh`, `perspective_rh`, `orthographic_rh` |
| glam Rotation | `Quat` (jede Methode), `DQuat`, `EulerRot` (XYZ + ZYX Roundtrip), `Affine2`, `Affine3A` |
| `FloatExt` | `lerp` / `inverse_lerp` / `remap`; bevy `VectorSpace`/`ScalarField`/`NormedVectorSpace` |
| Richtungen | `Dir2`, `Dir3` (`new`/`new_unchecked`/`new_and_length`/`slerp`/`fast_renormalize`), `Rot2` (jede Methode) |
| `Isometry2d/3d`, `Ray2d/3d` | Konstruktion, `transform_point`, `inverse`, Compose, `get_point` |
| `Rect` / `IRect` / `URect` | width/height/area/center/size/contains/union/intersect/inflate/`as_*` |
| `bounding` | `Aabb2d/3d`, `BoundingCircle/Sphere`, `BoundingVolume` (center/half_size/visible_area/contains/merge/grow/shrink/closest_point), **jedes `IntersectsVolume`-Paar 2D+3D**, `RayCast2d/3d`, `AabbCast2d`, `BoundingCircleCast`, `BoundingSphereCast` |
| Primitives 2D | `Measured2d` (perimeter/area) + `Bounded2d` (`aabb_2d`, `bounding_circle`) für Circle/Ellipse/Rectangle/Rhombus/Triangle2d/Annulus/RegularPolygon/CircularSector/Capsule2d |
| Primitives 3D | `Measured3d` (area/volume) + `Bounded3d` für Sphere/Cuboid/Cylinder/Cone/Capsule3d/Torus/Tetrahedron |
| `curve` | `Curve` + Adaptoren (`map`/`reverse`/`repeat`/`ping_pong`/`reparametrize_linear`/`samples`), `Interval`, **alle 39 `EaseFunction`-Varianten** (je @ 0.0/0.5/1.0) |
| `cubic_splines` | `CubicSegment` (position/velocity/acceleration), `CubicBezier`, `CubicHermite`, `CubicCardinalSpline`, `CubicBSpline`, `CubicNurbs` → `to_curve` |
| `sampling` (`rand`) | `ShapeSample::sample_interior/sample_boundary` für Rectangle/Circle/Cuboid/Sphere (je 500 Punkte in Grenzen), + Determinismus (gleicher Seed → gleiches Sample) |
| Sonstiges | `compass` (`CompassQuadrant`/`CompassOctant` ↔ `Dir2`, `opposite`, `is_in_direction`), `FloatOrd` (`total_cmp`, NaN sortiert **zuerst**, `Neg`), `AspectRatio`, `StableInterpolate` (Vec3/Quat/Dir3/Rot2, `smooth_nudge`) |
| Layout | `size_of` / `align_of` **jedes** öffentlichen Typs (records only) |

- **`rand` funktioniert auf dem 3DS-Target.** `rand` 0.10 mit `default-features = false`
  (wie `bevy_math` es pinnt) zieht **kein `getrandom`** — baut sauber. Nur ein expliziter
  `RngCore`/`TryRng` nötig (`getrandom`'s `OsRng` gibt's nicht). Damit ist die frühere
  `getrandom`-Sorge (Plattform-Fakten) für `bevy_math`'s `rand`/`sampling` vom Tisch.
- glam nutzt auf `armv6k` den **`scalar`-Backend** (kein SSE/NEON) — `Vec3A`/`Mat3A` echt
  skalar, aber 16-Byte-aligned (bestätigt).
- **Rest-Unsicherheit:** nur Emulator. VFP-Rundungsmodi / Denormals / exakte NaN-Bitmuster auf
  echter Hardware nicht 1:1 garantiert (für die Toleranzen unkritisch).
  Nicht abgedeckt: die ~200 Swizzles pro Vektor (Makro-generiert), kleine Integer-Vektoren
  (`I8/I16/U8/U16Vec`), `mint`-Interop, `mesh_sampling`, `bevy_reflect`-Integration.

**§B10 — bevy_transform: Testergebnis (`crates/bevy-transform-check`).**
`bevy_transform` 0.19.1, Features `["std", "bevy-support"]` (zieht `bevy_ecs` + `bevy_app`,
**kein** `bevy_reflect`, **kein** `multi_threaded`). Lauf:
`./scripts/test-emulator.sh -p bevy-transform-check`.
**38 / 38 Testfunktionen · 129 / 129 Checks bestanden**, ε = 1e-4.

→ **Vollständige Liste** (jede Funktion · Eingabe · Erwartet · **tatsächliche Ausgabe** · OK,
38 Sektionen): **`crates/bevy-transform-check/README.md`**.

Abgedeckt (die gesamte öffentliche API):

| Bereich | Umfang |
|---|---|
| `Transform` — Konstruktoren | `from_xyz` / `from_translation` / `from_rotation` / `from_scale` / `from_matrix` / `from_isometry`, `with_translation/rotation/scale`, `IDENTITY`, `Default`, `From<GlobalTransform>` |
| `Transform` — Achsen | `local_x/y/z`, `forward` / `back` / `up` / `down` / `left` / `right` (alle → `Dir3`), identity + rotiert |
| `Transform` — Komposition | `mul_transform`, `*` für `Transform` / `GlobalTransform` / `Vec3`, `transform_point` |
| `Transform` — Rotation | `rotate`, `rotate_x/y/z`, `rotate_axis`, `rotate_local` + `rotate_local_x/y/z` + `rotate_local_axis`, `rotate_around`, `translate_around` |
| `Transform` — Ausrichtung | `look_at`, `looking_at`, `look_to`, `looking_to`, `align`, `aligned_by` |
| `Transform` — Konversion | `to_matrix`, `compute_affine`, `to_isometry` / `from_isometry` / `from_matrix` Round-Trips, `is_finite` |
| `GlobalTransform` | alle 18 Methoden + `From<Mat4>` / `From<Transform>` + `*`-Operatoren: `translation`/`_vec3a`, `rotation`, `scale`, `to_scale_rotation_translation`, `compute_transform`, `to_matrix`/`affine`/`to_isometry`, `transform_point`, `reparented_to`, `radius_vec3a`, `mul_transform` |
| `TransformPoint` | auf `Transform`, `GlobalTransform`, `Mat4`, `Affine3A` (+ generisch) |
| ECS-Komponenten | `Transform` `require(GlobalTransform, TransformTreeChanged)`, `Query<&mut Transform>` + `Changed<Transform>`, `TransformTreeChanged` |
| Propagation | `mark_dirty_trees` → `propagate_parent_transforms` → `sync_simple_transforms` (serielle Fallbacks): 3-Ebenen + **21-Ebenen-Kette**, Rotations-Komposition, Reaktion auf Parent-Änderung, `ChildOf`→`Children`, `StaticTransformOptimizations` (Enabled **und** Disabled), Non-Hierarchy-Pfad |
| `TransformHelper` | `SystemParam` via `world.run_system_once` — `compute_global_transform` **ohne** Propagations-Lauf + Rotation durch Vorfahren + Err-Pfad |
| `BuildChildrenTransformExt` | `EntityWorldMut::set_parent_in_place` / `remove_parent_in_place` — Weltposition bleibt, lokaler `Transform` wird nachgezogen, `ChildOf` ein/aus |
| `TransformPlugin` | unter echter `bevy_app::App`: `App::update()` propagiert; Child folgt Parent über **mehrere Updates**; `TransformSystems::Propagate` als SystemSet |
| Layout | `size_of` / `align_of` von `Transform` / `GlobalTransform` / `TransformTreeChanged` |

- **Serielle Fallbacks bestätigt:** ohne `multi_threaded` nimmt bevy_transform `mod serial` /
  die `#[cfg(not(feature = "multi_threaded"))] iter_mut`-Branches — **kein `ComputeTaskPool`**
  wird angefasst, kein `par_iter`-Panic. `bevy_ecs::par_iter` degradiert selbst zu seriell.
- **`bevy_app::App::update()` läuft auf dem 3DS.** `App::new().add_plugins(TransformPlugin)`
  + `app.update()` propagiert korrekt (§B6-Frage für den App-Loop damit teil-beantwortet;
  `bevy_time`/`Instant` weiterhin separat).
- **Nicht abgedeckt:** sehr breite Hierarchien, `TransformPlugin` mit anderen Plugins kombiniert.

**§B10-Threading — bevy_transform `multi_threaded`: BLOCKIERT (kompiliert nicht).**
Im Gegensatz zu `bevy_ecs` (§B2/§B14-Threading, erfolgreich getestet) **kompiliert
`bevy_transform`s parallele Propagation auf `armv6k-nintendo-3ds` nicht.**
`bevy_transform::systems`' `mod parallel` (der `ComputeTaskPool`-basierte
Work-Queue-Tree-Walker mit einem geteilten "dirty subtree"-Bitset über
`mpsc`-Channels) nutzt für dieses Bitset **direkt** `core::sync::atomic::AtomicU64`
(`systems.rs:119,177`) — **nicht** `bevy_platform::sync::atomic`s
Portable-Atomic-Shim, den `bevy_ecs`/`bevy_tasks` korrekt verwenden (genau der
Mechanismus, der §B1s 64-Bit-Atomic-Problem für `bevy_ecs` bereits entschärft
hat). Da dieses Target **keine nativen 64-Bit-Atomics** hat, existiert
`core::sync::atomic::AtomicU64` hier schlicht nicht → `error[E0425]: cannot
find type 'AtomicU64' in module 'core::sync::atomic'` bei `cargo 3ds build
--features multi_threaded`. Das ist ein **Upstream-Bug/-Lücke in
`bevy_transform` 0.19.1** selbst, kein Problem in `dove`s Setup — reproduzierbar
mit `cargo 3ds test --no-run -p bevy-transform-check --features multi_threaded`.
Vorbereitete Tests liegen bereit (`crates/bevy-transform-check/src/checks/
threading.rs`: 40-Wurzeln-Parallelitätstest, 8×15-tiefe Hierarchie,
25-Runden-Deadlock-Stresstest — spiegeln §B14-Threadings Muster), können aber
erst laufen, sobald das behoben ist (entweder upstream oder per lokalem
Patch, analog zum ursprünglich für `pthread-3ds` erwogenen Vorgehen — bei
`pthread-3ds` hat sich das als unnötig erwiesen, hier wäre es tatsächlich
nötig). `multi_threaded`-Feature bleibt deshalb bewusst **kein** Crate-Default
für `bevy-transform-check` (im Gegensatz zu `bevy-ecs-check`/`all-checks`).

**§B11 — bevy_color: Testergebnis (`crates/bevy-color-check`).**
`bevy_color` 0.19.1, Features `["std"]` (= `alloc` + `bevy_math/std`, **kein**
`bevy_reflect`, **kein** `serialize`, **kein** `wgpu-types`). Lauf:
`./scripts/test-emulator.sh -p bevy-color-check`.
**44 / 44 Testfunktionen · 381 / 381 Checks bestanden**, ε = 1e-4 (gleicher Raum) /
4e-3 (über eine Farbraum-Konversion — mehrere `cbrt`/`powf`/Trig-Aufrufe aus
devkitPro-newlib verkettet, s. Crate-README).

→ **Vollständige Liste** (jede Funktion · Eingabe · Erwartet · **tatsächliche Ausgabe** · OK,
44 Sektionen): **`crates/bevy-color-check/README.md`**.

Abgedeckt (die gesamte öffentliche API):

| Bereich | Umfang |
|---|---|
| `Srgba` | Konstanten, Konstruktoren, `hex`/`to_hex` (alle Längen + Fehlerpfade), `gamma_function`/`_inverse`, `Mix`/`Alpha`/`Luminance`/`Gray`/`EuclideanDistance`, `ColorToComponents`/`ColorToPacked`, componentwise Vektor-Ops, `StableInterpolate` |
| `LinearRgba` | Konstanten (inkl. `NAN`), CIE-Luminanz-Gewichte, `with_luminance`/`darker`/`lighter`, `as_u32`/`to_u8_array` (+ Clamping), Vektor-Ops |
| `Hsla`/`Hsva`/`Hwba` | `Hue`/`Saturation`-Traits, `Mix` mit kürzestem Hue-Pfad, `Luminance`, `Gray`, `sequential_dispersed`, paarweise Konversionen |
| `Laba`/`Lcha`/`Oklaba`/`Oklcha`/`Xyza` | Konstruktoren, `CIE_EPSILON`/`CIE_KAPPA`, `D65_WHITE`, `Luminance`, `EuclideanDistance`, `Hue`, `Gray`, `sequential_dispersed` |
| Konversionsgraph | 6 Testfarben (schwarz/weiß/rot/grün/blau/grau, aus `bevy_color`s eigener `test_colors`-Tabelle) durch **alle 8 Räume** → `LinearRgba` und zurück |
| `Color` (Enum) | alle 10 Konstruktoren, `WHITE`/`BLACK`/`NONE`/`Default`, `to_srgba`/`to_linear`, `From<konkreter Typ>` (`derive_more`), `Alpha`/`Luminance`/`Hue`/`Saturation`/`Mix` (Delegation über `Oklcha`), `TryStableInterpolate` inkl. Mismatch-Fehler |
| `color_ops`/`color_range` | `Alpha for f32`, `Gray::gray` generisch über alle 10 Typen, `ColorRange::at` |
| `ColorCurve` | `new` (Fehlerpfad), `domain`, `sample_clamped`, `Curve::sample`, `CurveExt::map` |
| `palettes` | Stichproben aus `basic`/`css`/`tailwind` |
| Layout | `size_of`/`align_of` jedes Farbtyps + `Color` |

- **Konversionsgraph vollständig durchlaufen:** alle `From`-Impls zwischen den 8
  Nicht-Enum-Räumen (direkt oder über eine Zwischenstufe wie `Hsva`/`Oklaba`)
  wurden mit realen Testfarben gegengeprüft, nicht nur einzeln kompiliert.
- **`ColorCurve`** (das `bevy_math::curve::Curve`-Interface für Farbverläufe,
  `EvenCore`/`Vec`-basiert) braucht `alloc` — läuft mit `features = ["std"]`.
- **Nicht abgedeckt:** `bevy_reflect`, `serialize` (serde), `wgpu-types`-Interop,
  `encase`/`ShaderType` (nur `LinearRgba`, GPU-Uniform-Pfad), die vollständigen
  `css`/`tailwind`-Palettentabellen (nur Stichproben — reine Daten).

**§B12 — bevy_time: Testergebnis (`crates/bevy-time-check`).**
`bevy_time` 0.19.1, Features `["std"]` (**kein** `bevy_reflect`, **kein**
`serialize`; zieht `bevy_app`/`bevy_ecs`/`bevy_platform` mit `std` + optional
`crossbeam-channel` für den Render-Welt-Zeitkanal). Lauf:
`./scripts/test-emulator.sh -p bevy-time-check`.
**47 / 47 Testfunktionen · 161 / 161 Checks bestanden** (+ 1 `#[should_panic]`-
Test ohne Tabellenzeile), exakte `Duration`-Vergleiche / ε=1e-4 für f32.

→ **Vollständige Liste** (jede Funktion · Eingabe · Erwartet · **tatsächliche Ausgabe** · OK,
46 Sektionen): **`crates/bevy-time-check/README.md`**.

Abgedeckt (die gesamte öffentliche API):

| Bereich | Umfang |
|---|---|
| `Stopwatch` | `tick`/`pause`/`unpause`/`reset` (inkl. „reset behält Pause-Status"), `set_elapsed`, alle `elapsed*`-Varianten |
| `Timer`/`TimerMode` | `tick` (Once klemmt, Repeating wickelt um), `is_finished`/`just_finished`, `times_finished_this_tick` (Mehrfach-Trigger pro Tick), `fraction`/`fraction_remaining`/`remaining*`, `finish`/`almost_finish`, `mode`/`set_mode` |
| `Time<T>` (generisch) | `advance_by` (ersetzt `delta`, akkumuliert `elapsed`), `advance_to` (+ Panic-Pfad bei Rückwärtslauf), `wrap_period`/`elapsed_wrapped`, `context`/`context_mut`/`as_generic` |
| `Time<Real>` | `update_with_duration` (deterministisch), `startup`/`first_update`/`last_update`, ein `Instant::now()`-Monotonie-Sanity-Check |
| `Time<Virtual>` | über die öffentliche `update_virtual_time()` — Folgeverhalten bei 1x, `pause`/`unpause`/`toggle`, `set_relative_speed`, `from_max_delta`/`set_max_delta`-Klemmung |
| `Time<Fixed>` | Konstruktoren/Setter, `overstep`-Buchhaltung, `run_fixed_main_schedule` End-to-End über eine echte `App` (Lauf-Anzahl über 5 Frames nachgerechnet) |
| `common_conditions` | `on_timer`, `once_after_delay`, `repeating_after_delay` (Semantik-Falle dokumentiert, s. u.), `on_real_timer`, `paused` |
| `DelayedCommandsExt` | `.duration()`/`.secs()`, Mehrfach-Frame-Spawns End-to-End über `App`+`TimePlugin`, Entity-Id einer verzögerten Aktion sofort für eine zweite nutzbar |
| `TimePlugin` | alle vier Uhr-Ressourcen inserted; alle 4 `TimeUpdateStrategy`-Varianten unter echter `App` |
| Layout | `size_of`/`align_of` jedes Uhr-/Timer-Typs |

- **Vollständig deterministisch.** Jeder Test treibt die Uhr über
  `Time::advance_by`/`advance_to`, `Time::<Real>::update_with_duration` oder
  `TimeUpdateStrategy::ManualDuration`/`ManualInstant`/`FixedTimesteps` — nie
  über die echte Wanduhr. Das umgeht die eigentliche offene Frage komplett,
  statt sie zu beantworten.
- **`common_conditions::repeating_after_delay`-Falle gefunden:** trotz des
  Namens wickelt es keinen `Repeating`-Timer, sondern einen `Once`-Timer und
  liest `is_finished()` (nicht `just_finished()`) — die Bedingung wird nach
  der Verzögerung **dauerhaft wahr**, nicht periodisch. Für "alle N Sekunden"
  ist `on_timer` das richtige Werkzeug.
- **`Time::<Real>::update_with_duration`/`update_with_instant`:** der
  *erste* Aufruf liefert immer `delta = ZERO` (er etabliert nur
  `first_update`/`last_update`) — relevant für jeden Test, der `TimePlugin` +
  `TimeUpdateStrategy::ManualDuration` nutzt (erster `app.update()` "verpufft").
- **Nicht beantwortet:** ob `Instant::now()` auf echter 3DS-**Hardware**
  akkurat/monoton ist — nur ein einzelner Monotonie-Check lief im Emulator
  (siehe `crates/bevy-time-check/README.md` „Determinismus-Hinweis"). §B6
  bleibt offen. **Im Demo verbaut** (2026-09-11, überarbeitet auf `bevy_app`
  am selben Tag): `src/main.rs` läuft jetzt über `App::new()` + `TimePlugin`
  statt der Sim-Uhr direkt zu treiben — aber mit `TimeUpdateStrategy::
  ManualDuration(DT)` gepinnt, damit `TimePlugin`s `time_system` nie
  `Instant::now()` aufruft. Die FPS-Anzeige nutzt nach wie vor direkt
  `svcGetSystemTick`, gerade um die offene Frage nicht anzufassen.
- **Nicht abgedeckt:** `bevy_reflect`, `serialize`, `TimeReceiver`/`TimeSender`
  (Render-Welt-Kanal, ohne `bevy_render` irrelevant), Zeitverhalten von
  `TimeUpdateStrategy::Automatic` unter echter Last.

**§B13 — bevy_ptr: Testergebnis (`crates/bevy-ptr-check`).**
`bevy_ptr` 0.19.1, keine Features (die Crate hat keine — `#![no_std]`,
null Dependencies). Lauf: `./scripts/test-emulator.sh -p bevy-ptr-check`.
**39 / 39 Testfunktionen · 69 / 69 Checks bestanden** (+ 1 `#[should_panic]`-
Test ohne Tabellenzeile), exakte Werte-/Adressvergleiche.

→ **Vollständige Liste** (jede Funktion · Eingabe · Erwartet · **tatsächliche Ausgabe** · OK,
38 Sektionen): **`crates/bevy-ptr-check/README.md`**.

Abgedeckt (die gesamte öffentliche API):

| Bereich | Umfang |
|---|---|
| `ConstNonNull<T>` | `new`/`new_unchecked`/`as_ref`, alle `From`-Impls, `Copy`/`Clone` |
| `Ptr<'a, A>` | `from(&T)`, `deref::<T>()`, `as_ptr`, `byte_offset`/`byte_add`, `to_unaligned`, `assert_unique()` → `PtrMut` |
| `PtrMut<'a, A>` | `from(&mut T)`, `deref_mut`, `reborrow`, `as_ref` → `Ptr`, `promote()` → `OwningPtr` |
| `OwningPtr<'a, A>` | `make()` (sichere Konstruktion), `read`, `drop_as` (Destruktor-Lauf per Zähler verifiziert), `cast()` → `MovingPtr`, `as_ref`/`as_mut`, `read_unaligned` |
| `MovingPtr<'a, T, A>` + Makros | `read`/`write_to`/`assign_to` (alte Werte korrekt gedroppt, per Zähler verifiziert), `Drop`-Verhalten bei nie-konsumiertem Pointer, `move_as_ptr!`/`deconstruct_moving_ptr!` (Struct/Tuple/`MaybeUninit`, aus `bevy_ptr`s eigenen Doku-Beispielen), `partial_move`, `From<MovingPtr> for OwningPtr` |
| `ThinSlicePtr<'a, T>` | `From<&[T]>`, `get_unchecked`, `as_slice_unchecked`, `UnsafeCell<T>`-Spezialisierung (`as_mut_slice_unchecked`, `cast`) |
| `UnsafeCellDeref` | `deref`/`deref_mut`/`read` |
| Alignment | echter unausgerichteter `u32`-Read über einen absichtlich fehlausgerichteten Puffer; `#[should_panic]`, dass der `Aligned`-Pfad Fehlausrichtung im Debug-Build tatsächlich ablehnt |
| Layout | `size_of`/`align_of` — bestätigt, dass `Ptr`/`PtrMut`/`OwningPtr`/`ConstNonNull` exakt zeigergroß sind |

- **Unausgerichtete Reads funktionieren auf `armv6k`.** Ein `u32` an einem
  absichtlich nicht-4-Byte-ausgerichteten Offset (Offset dynamisch aus der
  tatsächlichen Pufferadresse berechnet, nicht fest verdrahtet — sonst war der
  Test nur zufällig "misaligned", je nachdem wo der Compiler den Stack-Puffer
  platziert hat) lässt sich über `OwningPtr<Unaligned>::read_unaligned::<u32>()`
  korrekt zurücklesen. Bestätigt die Grundannahme hinter `bevy_ptr`s
  `Aligned`/`Unaligned`-Unterscheidung für diese Plattform.
- **Debug-Alignment-Check funktioniert:** `Ptr::deref` auf einem fehlausgerichteten
  Zeiger panickt im Debug-Build wie dokumentiert (`debug_ensure_aligned`) —
  `debug_assertions` ist im `cargo 3ds test`-Profil aktiv.
- **Nicht abgedeckt:** `TryFrom<MovingPtr<Unaligned>> for MovingPtr<Aligned>`
  (weder Erfolgs- noch Fehlerpfad explizit getestet), `partial_move`/
  `move_field` nur an einem einfachen Zwei-Felder-Beispiel, `IsAligned`-Trait
  direkt (nur indirekt über `Ptr`/`OwningPtr`/`MovingPtr`).

**§B14 — bevy_ecs: Testergebnis (`crates/bevy-ecs-check`).**
`bevy_ecs` 0.19.1, Features `["std"]` (**kein** `bevy_reflect`). Lauf:
`./scripts/test-emulator.sh -p bevy-ecs-check`.
**80 / 80 Testfunktionen · 145 / 145 Checks bestanden** (Default-Build,
`multi_threaded` inklusive — s. §B14-Threading unten für die 5 Threading-
Testfunktionen davon), exakte Werte-/Strukturvergleiche.

→ **Vollständige Liste** (jede Funktion · Eingabe · Erwartet · **tatsächliche Ausgabe** · OK):
**`crates/bevy-ecs-check/README.md`**.

Abgedeckt (die gesamte öffentliche API):

| Bereich | Umfang |
|---|---|
| `World` (Rohzugriff) | `spawn`/`spawn_empty`/`spawn_batch`, `despawn`, `get_entity`/`entity`/`entity_mut`, `clear_entities()`, `Entities::len()` vs. `count_spawned()` |
| `Commands` | standalone via `CommandQueue`/`Commands::new()`, Commands in einem `Schedule`-Lauf, `EntityCommands`, `spawn_batch`, Resource-Commands |
| Queries (Basis) | `Query<&T>`/`Query<&mut T>`, `get`/`get_mut`, `contains`, `single()`/`single_mut()` (0/1/2-Treffer) |
| Query-Kombinationen | `iter_combinations::<K>()`, `iter_many()` |
| Query-Filter | `With`/`Without`, `Or`, `AnyOf`, `Has` |
| Change Detection | `Added<T>`/`Changed<T>` über echte `Schedule`-Läufe, `Ref<T>`, `Res::is_changed()`, die lazy `Mut<T>`-Markierung |
| Resources | `insert_resource`/`init_resource`, `remove_resource`, `FromWorld`, `Local<T>` |
| Components/Bundles | `#[derive(Component)]`/`#[derive(Bundle)]`, `#[require(T)]`/`#[require(T = expr())]`, Transitivität |
| `EntityRef`/`EntityMut`/`EntityWorldMut` | `contains`/`get`/`get_mut`/`insert_if_new`/`take`, `Entity`-Gleichheit/`index()` |
| Relationships | `ChildOf`/`Children` (Auto-Populate, Despawn-Kaskade, `with_children`-Builder) |
| Messages (ex-`Events`) | `MessageRegistry`, `MessageWriter`/`MessageReader`, Buffer-Swap-Semantik, `write_batch` |
| Observers | `add_observer`/`trigger`, Component-Lifecycle (`On<Add/Insert/Remove, T>`), Commands-Flush |
| Schedules & Conditions | `.chain()`, `.run_if()`, benannte `Schedules`, `run_system_once`, `SystemSet` |
| Exotische System-Params | `ParamSet`, `RemovedComponents<T>`, exklusive Systeme |
| Layout | `size_of`/`align_of` von `Entity`, `Option<Entity>`, `ChildOf` |

- **Resources sind jetzt Components auf versteckten Entities.** 0.19s größte
  interne Umstellung: `World::resource_entities()` mappt `ComponentId ->
  Entity`. Direkte Folge: `World::clear_entities()` — das *jede* Entity
  despawnt — **löscht jetzt auch alle Resources mit** (dokumentiert:
  "This includes all resources, as they are stored as components"). Wer das
  aus älteren Bevy-Versionen anders kennt, tappt hier in eine Falle.
- **`World::new()` ist nie wirklich leer.** `bootstrap()` registriert
  Lifecycle-Event-Keys und `init_resource`t `DefaultQueryFilters` — ein
  frischer `World` hat also schon vor jedem Nutzercode mindestens eine
  lebende (versteckte) Entity.
- **`Entities::len()` ist kein Live-Count.** Es ist `self.meta.len()`, die
  Anzahl je vergebener Index-Slots — schrumpft nie durch `despawn()` (Slots
  werden per Generation-Bump recycelt, nicht entfernt). Intern wächst `meta`
  zudem in Vec-Capacity-Sprüngen (`ensure_index_index_is_valid`'s `expand()`
  resized auf `meta.capacity()`, nicht nur bis zum gebrauchten Index) — die
  genaue Sprunggröße ist Implementierungsdetail und nicht verlässlich
  vorhersagbar. `Entities::count_spawned()` (dokumentiert "intended only ...
  for tests", O(n)) ist der tatsächliche Live-Count und wächst/schrumpft
  exakt 1:1 mit spawn/despawn.
- **`Mut<T>`s Änderungsmarkierung ist lazy.** Nur ein echter `DerefMut`-Zugriff
  (z. B. `*m = x` oder `m.feld = x`) markiert eine Component als geändert —
  bloßes `get_mut()` aufrufen und den Handle wieder fallen lassen zählt
  **nicht**. Selbst der aktuelle Wert zurückschreiben (`m.0 = m.0`) zählt
  aber sehr wohl, weil das trotzdem durch `DerefMut` geht — `bevy_ecs` trackt
  "wurde geschrieben", nicht "hat sich der Wert wirklich geändert".
- **`Added`/`Changed`-Filter "clearen" nur über einen echten `Schedule`-Lauf.**
  Wiederholtes manuelles `QueryState::iter(&world)` ohne dazwischenliegenden
  `Schedule::run()` advanced den "last observed tick" nicht richtig — für
  Change-Detection-Tests zwingend über ein `Schedule` laufen lassen.
- **Observer-`Commands` werden nicht automatisch geflusht.** Im Unterschied
  zu einem System in einem `Schedule` (dessen deferred Params der Executor
  automatisch anwendet) braucht ein Observer nach `World::trigger()` ein
  explizites `World::flush()`, damit seine `Commands` tatsächlich wirken.
- **Nicht abgedeckt:** `bevy_reflect`-Integration, `System`-Piping,
  `EntityHashMap`/`EntityHashSet` direkt, Observer-Propagation/Bubbling über
  `EntityEvent`. (`multi_threaded` war hier der letzte offene Punkt — jetzt
  separat getestet, s. u.)

**§B14-Threading — bevy_ecs `multi_threaded`: Testergebnis
(`crates/bevy-ecs-check`).**
`bevy_ecs`/`bevy_tasks` 0.19.1, `features = ["multi_threaded"]` — seit
2026-09-20 **Crate-Default** (vorher ein `--features multi_threaded`-Opt-in;
matcht normales Bevys eigenen Default, s. §B2). Lauf:
`./scripts/test-emulator.sh -p bevy-ecs-check` (kein extra Flag mehr nötig).
**5 / 5 Testfunktionen · 10 / 10 Checks bestanden** — beantwortet §B2 (bisher
"der harte Block") und liefert eine erste Emulator-Datenlage für §B1. Auch in
`all-checks`s Default-Build verdrahtet (305/305 Testfunktionen dort, s. u.)
und in `dove` selbst (`App::new()` + `TaskPoolPlugin` in `src/main.rs`,
Demo-Loop im Emulator gebootet und ohne Crash gelaufen).

→ **Vollständige Liste**: **`crates/bevy-ecs-check/README.md`** Abschnitt „Threading".

Abgedeckt:

| Bereich | Umfang |
|---|---|
| `std::thread` | `spawn`/`join`, Rückgabewert + tatsächliche Ausführung verifiziert (der rohe `pthread-3ds`-Unterbau) |
| `bevy_tasks::ComputeTaskPool` | `get_or_init()` mit explizitem `TaskPoolBuilder::num_threads(2)` |
| `Schedule` | automatische `MultiThreadedExecutor`-Wahl (0.19 hat kein `ExecutorKind`-Opt-in mehr), 2 nicht-konfligierende Systeme über mehrere `run()`s |
| `Query::par_iter()`/`par_iter_mut()` | 200 bzw. 50 Entities, inkl. 30 Wiederholungsrunden als Deadlock-/Race-Stichprobe |

- **Kein Patch an `pthread-3ds` nötig.** Die Hypothese im ursprünglichen §B2
  ("Gar nicht getestet") war, dass hier eventuell nachgebessert werden müsste
  — tatsächlich funktionieren `pthread_create`/`pthread_join`/Mutex/Condvar/TLS
  bereits wie von `bevy_tasks`/`bevy_ecs` erwartet, ungepatcht.
- **`std::thread::available_parallelism()` ist auf diesem Target faktisch
  ungestützt.** `bevy_tasks::available_parallelism()` fängt den Fehler ab und
  liefert `1` (kein Panic) — für tatsächliche Parallelität muss
  `TaskPoolBuilder::num_threads(...)` explizit gesetzt werden, sonst läuft der
  "Multi-Threaded"-Executor real einthreadig.
- **`Query::par_iter()`/`par_iter_mut()` ruft `ComputeTaskPool::get()`
  (nicht `get_or_init()`) auf und panickt, wenn der Pool nicht vorher
  initialisiert wurde** — ein leicht zu übersehender Ordering-Fallstrick.
- **`ComputeTaskPool` ist ein prozessweiter `OnceLock`** — nur der erste
  `get_or_init()`-Aufruf im ganzen Prozess bestimmt die Thread-Zahl; das macht
  eine exakte Thread-Count-Assertion testreihenfolgeabhängig (deshalb prüft
  der Test nur `>= 1`).
- **Nicht abgedeckt / weiterhin offen:** alles davon auf echter **Hardware**
  (Azahar validiert Korrektheit, nicht reale Zwei-Kern-Nebenläufigkeit/Timing
  — §B1s Kontention-Fragen bleiben offen), `AsyncComputeTaskPool`/
  `IoTaskPool`, echte Systemkonflikt-Serialisierung unter Last, Langlauf (§B7).

### Erledigt

- ✅ `bevy_math` inkl. `rand`/`sampling` — §B9 (`bevy-math-check`, 488/488 Checks, quasi ganze API)
- ✅ `bevy_transform` — ganze öffentliche API inkl. `TransformPlugin`/`App::update()` — §B10 (`bevy-transform-check`, 129/129 Checks)
- ✅ `bevy_ecs` — gesamte öffentliche API — §B14 (`bevy-ecs-check`, 145/145 Checks)
- ✅ `multi_threaded` (Emulator, jetzt Crate-Default) — echte `std::thread`s über
  `pthread-3ds`, `ComputeTaskPool`, `MultiThreadedExecutor`, `Query::par_iter` —
  §B2/§B14-Threading (`bevy-ecs-check`, 10/10 Checks; auch Default in `all-checks`
  und in `dove` selbst — `App::new()` + `TaskPoolPlugin`, Demo im Emulator gebootet
  ohne Crash). Kein `pthread-3ds`-Patch nötig. Auf Hardware bleibt es offen (s. Schritt 5)
- ✅ `bevy_app` — kompiliert, linkt **und `App::update()` läuft** (via bevy-transform-check §B10)
  — und treibt jetzt das Demo selbst: `App::new()` + `TimePlugin`, `app.update()` 60×/s im
  echten interaktiven Loop (2026-09-11)
- ✅ `bevy_color` — gesamte öffentliche API + Konversionsgraph — §B11 (`bevy-color-check`,
  381/381 Checks) — und im Demo verbaut (`Tint`)
- ✅ `bevy_time` — gesamte öffentliche API, deterministisch — §B12 (`bevy-time-check`,
  161/161 Checks). `Instant::now()` auf Hardware bleibt offen (s. Schritt 7)
- ✅ `bevy_ptr` — gesamte öffentliche API inkl. echtem unausgerichtetem Read auf
  `armv6k` — §B13 (`bevy-ptr-check`, 69/69 Checks)

### Empfohlene nächste Schritte (on-device via ein `bevy-*-check`)

1. ~~`Commands` + `world.flush()` + `EntityCommands`~~ ✅ erledigt — §B14 (`bevy-ecs-check`)
2. ~~Events (`Events<T>`, Reader/Writer, Update-System über mehrere Frames)~~ ✅ erledigt
   (0.19s Umbenennung zu Messages) — §B14 (`bevy-ecs-check`)
3. ~~Change Detection als eigener Test (`Added`/`Changed`/`Ref`, Tick-Verhalten)~~ ✅ erledigt
   — §B14 (`bevy-ecs-check`)
4. ~~`SystemParam`-derive, `Local`, exklusives System, `ParamSet`~~ ✅ erledigt — §B14
   (`bevy-ecs-check`)
5. ~~`multi_threaded`: `Schedule` mit parallelem Executor + 2 nicht-konfligierenden Systemen,
   `TaskPoolBuilder` auf 1–2 Threads~~ ✅ erledigt **im Emulator** — §B2/§B14-Threading
   (`bevy-ecs-check`, 10/10 Checks). **Noch offen: auf echter Hardware** (reale Zwei-Kern-
   Contention/Timing, §B1) + Langlauf unter Last (§B7) — das ist jetzt der verbleibende
   harte Block, nicht mehr "ob es überhaupt läuft"
6. `bevy_reflect` isoliert: `#[derive(Reflect)]`, `TypeRegistry`, `Box<dyn Reflect>`, Downcast
7. `bevy_platform::Instant` auf Hardware (§B6)
8. ~~`bevy_app` mit manuellem `app.update()`-Loop~~ ✅ erledigt — treibt jetzt das Demo (s. o.)
9. ~~`bevy_transform` mit `multi_threaded` (parallele Propagation)~~ ❌ **blockiert** —
   kompiliert nicht auf diesem Target, Upstream-Bug in `bevy_transform` 0.19.1
   (`core::sync::atomic::AtomicU64` statt `bevy_platform`s Portable-Shim), §B10-Threading.
   Übrig: entweder upstream fixen lassen oder selbst lokal patchen, falls gewünscht.
10. Langlauf-/Speichertest (§B7)