# bevy-ecs-check

On-device check that **`bevy_ecs`** works on the Nintendo 3DS.

| | |
|---|---|
| Bevy-Einheit | `bevy_ecs` |
| Version | `0.19.1` |
| Features | `default-features = false`, `features = ["std"]` (kein `bevy_reflect`, kein `multi_threaded`) |
| Target | `armv6k-nintendo-3ds` (kein 64-bit-Atomic → `bevy_platform` nutzt `portable-atomic`-Fallback) |
| Stand | 2026-09-10 · Azahar-Emulator · **6/6 Testfunktionen · 9/9 Checks bestanden** |

Ausführen: `./scripts/test-emulator.sh -p bevy-ecs-check`

## Ergebnis

Jede Zeile: geprüfte Funktion · Eingabe · Erwartung · tatsächliche Ausgabe · Status.

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `World::spawn` + `Query::<&Position>::iter().count()` | 2 entities spawned | `2` | `2` | ✅ |
| `EntityRef::get::<Position>()` | `spawn(Position { x: 1, y: 2 })` | `Position { x: 1.0, y: 2.0 }` | `Position { x: 1.0, y: 2.0 }` | ✅ |
| `World::despawn(entity)` | 2 spawned, despawn 1 | `true, 1 left` | `true, 1 left` | ✅ |
| `Query<(&mut Position, &Velocity)>` — System mutiert über 2 Runs | pos=(0,0), vel=(1.5,-0.5), 2 steps | `Position { x: 3.0, y: -1.0 }` | `Position { x: 3.0, y: -1.0 }` | ✅ |
| `ResMut<StepCount>` — `steps.0 += 1` pro Run | 7 schedule runs | `7` | `7` | ✅ |
| `Schedule::add_systems((integrate, tick))` + `run()` ×5 — StepCount | 5 runs | `5` | `5` | ✅ |
| `Query<&Position>::iter().count()` nach Run | 3 Position-Entities | `3` | `3` | ✅ |
| `integrate()` auf der bewegten Entity nach 5 Runs | pos=(0,0), vel=(1,-2), 5 steps | `Position { x: 5.0, y: -10.0 }` | `Position { x: 5.0, y: -10.0 }` | ✅ |
| `Query<Entity, With<Velocity>>::iter().count()` | 1 von 2 Entities hat Velocity | `1` | `1` | ✅ |

## Nicht abgedeckt

`Commands` / `world.flush()`, Events (`EventReader`/`EventWriter`), Change Detection über Frames
(`Added`/`Changed`), Observers/Hooks, `SystemParam`-derive / `Local` / `NonSend`, exklusive Systeme,
**`multi_threaded`** (paralleler Executor + Threadpool — der harte Block, siehe `../../port.md` §B1/§B2).
