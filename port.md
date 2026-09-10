| Crate | Einstufung | Begründung | getestet |
|---|---|---|---|
| bevy_ecs | Sicher (core, 1 Thread) | Core-ECS = reine Logik/Daten. Aber: nicht "platformfrei" — Entity-Allocator nutzt 64-bit-Atomics (Fallback, §B1), `thread_local!`, optionaler Parallel-Executor. | ✅ HW: spawn/despawn, `Query<&T>`/`Query<&mut T>`, `Res`/`ResMut`, `Schedule`+`.chain()`, `entity().get()` |
| bevy_math | Sicher | glam-Reexport + Bevy-Primitives. Zieht **glam 0.32** neben citro3ds glam 0.30 (koexistieren, §B8). glam nutzt auf armv6k den `scalar`-Backend (kein NEON). | ✅ **14/14** (Emu, `crates/bevy-math-check`) — Test-für-Test-Tabelle in **§B9** |
| bevy_transform | Sicher (1 Thread) | Mathe + Hierarchie. `propagate_transforms` iteriert in neueren Bevy-Versionen **parallel** (`ComputeTaskPool`) → erbt §B2. Single-thread trivial. ||
| bevy_ptr | Sicher | keine Platform-Annahmen | ✅ baut (transitiv via bevy_ecs) |
| bevy_utils | Sicher | keine Platform-Annahmen | ✅ baut (transitiv) |
| bevy_platform | Sicher (mit Fallbacks) | **ist** die Platform-Abstraktion: Atomic-Shim (64-bit → `portable-atomic` Fallback), `Instant`, sync-Primitive. Baut für 3DS. | ✅ baut (transitiv) |
| bevy_derive | Sicher | Makros, keine Runtime-Abhängigkeit ||
| bevy_macro_utils | Sicher | Makros, keine Runtime-Abhängigkeit ||
| bevy_color | Sicher | reine Farbraum-Mathe ||
| bevy_time | Wahrscheinlich okay | Baut (`bevy_platform::Instant` → `std::time::Instant`). Ob `Instant::now()` auf **3DS-Hardware** liefert, ist UNGEPRÜFT — im Demo bewusst mit `svcGetSystemTick` umgangen (§B6). | Instant auf HW ✗ |
| bevy_app | Wahrscheinlich okay | Runner passt nicht, App::update() manuell nötig ||
| bevy_state | Wahrscheinlich okay | reine State-Machine-Logik ||
| bevy_diagnostic | Wahrscheinlich okay | evtl. OS-Metriken disablen ||
| bevy_asset | Riskant | Async-Loader, Thread-Pool-Abhängigkeit ||
| bevy_tasks | Riskant | **Nicht-optionale Dep von bevy_ecs** — baut + linkt schon jetzt (single-threaded Form). `multi_threaded` zieht `async-executor` + `concurrent-queue` und echte Threads → §B2. | ⚠️ baut single-thread; `multi_threaded` ✗ |
| bevy_reflect | Riskant | schwergewichtig, Compile-Zeit/Binary-Size. Manche Feature-Pfade (`rand`, `uuid`) können `getrandom` in den Runtime-Graph ziehen → §B5. Gate für app/scene/state/asset. ||
| bevy_scene | Riskant | hängt an reflect ||
| bevy_gltf | Riskant | hängt an Asset/Mesh/Image-Pipeline ||
| bevy_mesh | Riskant | Datenstruktur ok, aber eng an render gekoppelt ||
| bevy_animation | Riskant | hängt an curve, evtl. isolierbar ||
| bevy_render | Ausgeschlossen | wgpu-gebunden ||
| bevy_core_pipeline | Ausgeschlossen | wgpu-gebunden ||
| bevy_pbr | Ausgeschlossen | wgpu-gebunden ||
| bevy_sprite | Ausgeschlossen | wgpu-gebunden ||
| bevy_sprite_render | Ausgeschlossen | wgpu-gebunden ||
| bevy_ui | Ausgeschlossen | wgpu-gebunden ||
| bevy_ui_render | Ausgeschlossen | wgpu-gebunden ||
| bevy_ui_widgets | Ausgeschlossen | wgpu-gebunden ||
| bevy_window | Ausgeschlossen | Desktop-Fenstersystem ||
| bevy_winit | Ausgeschlossen | Desktop-Fenstersystem ||
| bevy_camera | Ausgeschlossen | render-gekoppelt ||
| bevy_light | Ausgeschlossen | render-gekoppelt ||
| bevy_material | Ausgeschlossen | render-gekoppelt ||
| bevy_image | Ausgeschlossen | GPU-Upload-Pfad ||
| bevy_gizmos | Ausgeschlossen | render-gekoppelt ||
| bevy_picking | Ausgeschlossen | Desktop-Input ||
| bevy_input_focus | Ausgeschlossen | Desktop-Input ||
| bevy_text | Ausgeschlossen | render-gekoppelt ||
| bevy_audio | Ausgeschlossen | cpal statt 3DS-Audio-API ||
| bevy_android | Ausgeschlossen | irrelevante Plattform ||
| bevy_gilrs | Ausgeschlossen | irrelevante Plattform ||

---

## Befunde (Stand 2026-09-10, Bevy 0.19.1, `armv6k-nintendo-3ds`)

### Was tatsächlich verifiziert ist

Kompiliert **+ gelinkt + auf echter 3DS-Hardware gelaufen** (Swarm-Demo `src/main.rs`)
sowie mit Assertions im Emulator (`./scripts/test-emulator.sh`):

- `bevy_ecs` 0.19.1, `default-features = false, features = ["std"]`:
  `World`, `spawn`/`despawn`, `Query<&T>` **und** `Query<&mut T>` (Iteration + Mutation),
  `Res`/`ResMut`, `Schedule` mit mehreren Systemen + `.chain()`, `world.entity(e).get::<T>()`,
  Bulk-Spawn/Despawn. **Alles single-threaded.**
- `bevy_math` 0.19.1, `default-features = false, features = ["std", "curve"]`:
  14 Assertion-Tests im Emulator (ε = 1e-4), siehe §B9. In `dove` selbst nur
  `["std"]` nötig.
- Transitiv mitgebaut + gelinkt (nicht separat funktionsgetestet):
  `bevy_platform`, `bevy_ptr`, `bevy_utils`, `bevy_tasks` (single-threaded), `bevy_ecs_macros`,
  `glam` 0.32.

Vehikel für weitere isolierte Tests: **`crates/bevy-ecs-check`** und
**`crates/bevy-math-check`** (jeweils ohne citro3d im Baum, on-device via
`./scripts/test-emulator.sh -p <crate>`).

### Plattform-Fakten `armv6k-nintendo-3ds`

- **Tier-3-Target.** Nightly + `rust-src`; `cargo-3ds` baut mit `-Z build-std=std,test`. Kein prebuilt `std`.
- **Atomics:** 8 / 16 / 32 / ptr **nativ** (ARMv6K hat LDREX/STREX). **Kein 64-bit-Atomic.**
  `pointer_width = 32`.
- **Panic:** `unwind` wird unterstützt (`std`-Build zieht `panic_unwind`/`unwind`). Bevy-Panics
  (fehlende Resource, Query-Konflikt) beenden die Homebrew sauber statt `abort`.
- **`getrandom`:** kein Backend für dieses Target. Aktuell **nicht** im Runtime-Graph (nur Host-
  Build-Deps). Sobald ein Feature es reinzieht (`rand`, teils `bevy_reflect`, `uuid`), bricht der
  Build — dann `getrandom` „custom"-Backend + Register nötig (`ps:GenerateRandomBytes` / `ctru`).
- **Threads:** `std::thread` läuft über `pthread-3ds`. Old 3DS: 2 Cores, Core 1 großteils OS
  (Homebrew bekommt Zeitscheibe); New 3DS: Cores 2+3 frei (804 MHz via `osSetSpeedupEnable`).
  Kein automatisches `num_cpus`-Sizing — Threadpools **explizit** begrenzen.
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

**§B2 — `multi_threaded` / paralleler Executor / Threadpools.** Gar nicht getestet.
`bevy_tasks` ist Pflicht-Dep und baut single-threaded; `multi_threaded` aktiviert
`async-executor` + `concurrent-queue` + echte Threads. `ComputeTaskPool`/`TaskPool` müssen
mit `TaskPoolBuilder`-Threadzahl an die 3DS-Cores angepasst werden (Default-Sizing greift nicht).
Kombiniert mit §B1 der Haupt-Risikoblock des ganzen Ports.

**§B3 — `thread_local!`.** `bevy_ecs` nutzt TLS im Error-Handling (`error/bevy_error.rs`).
`pthread-3ds` liefert TLS (`has-thread-local = true`), aber nicht funktionsgetestet.

**§B4 — noch nicht ausgeübte Kern-APIs (alle single-thread, aber ungetestet):**
`Commands` + deferred apply (`world.flush`), `EntityCommands`; Events/Messages
(`EventReader`/`EventWriter`, `Events<T>`-Doppelpuffer + Update-System); Change Detection über
Frames (`Added`/`Changed`/`Ref`, Tick-Wraparound bei ~2^31 Ticks); Hooks/Observers
(`on_add`/`Trigger`/`Observer`); Relationships (`ChildOf`/`Children`); `SystemParam`-derive,
`Local`, `NonSend`, `ParamSet`, exklusive Systeme (`&mut World`).

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
`bevy_math` 0.19.1, Features `["std", "curve"]`. Lauf: `./scripts/test-emulator.sh -p bevy-math-check`
(Azahar, `test-runner`/GDB). **14 / 14 bestanden**, `test result: ok`, ε = 1e-4.

Funktion für Funktion:

| Test-Funktion | geprüfte APIs / Aussagen | Status |
|---|---|---|
| `float_ops_match_known_values` | `bevy_math::ops::{sin,cos,tan,atan2,sqrt,powf,exp,ln,cbrt,hypot}` (= newlib) gegen bekannte Werte; zusätzlich `f32::sin`/`f32::sqrt` | ✅ |
| `vector_algebra` | `Vec3` dot / cross / length / `normalize().length()==1` / distance / lerp; `Vec4::length_squared`; `Vec2 * f32` | ✅ |
| `vec2_rotation_helpers` | `Vec2::from_angle` + `.rotate`; `from_angle`↔`to_angle`-Roundtrip; `Vec2::perp`; `Vec2::angle_to` | ✅ |
| `quaternion_rotation` | `Quat::from_axis_angle` / `from_rotation_x` / `from_rotation_z` (`mul_vec3`); Quat-Komposition; `from_xyzw().normalize()`; `slerp` Mittelpunkt; `from_euler`↔`to_euler` (`EulerRot::XYZ`) | ✅ |
| `mat4_transforms` | `Mat4::from_translation` / `from_rotation_z` / `from_scale` / `from_scale_rotation_translation`; `transform_point3` / `transform_vector3`; `inverse()` (M·M⁻¹ == I); `determinant`; `Mat4 * Vec4` | ✅ |
| `mat3_and_mat4_camera` | `Mat3::from_scale_angle_translation` + `transform_point2`; `Mat4::look_at_rh` (Auge → Ursprung); `Mat4::perspective_rh` + `project_point3` (Near-Plane → z≈0) | ✅ |
| `vec3a_layout_and_parity` | `size_of::<Vec3A>()==16`, `align_of==16`; `Vec3A::cross` == `Vec3::cross` (scalar-Backend) | ✅ |
| `directions_normalise_and_reject` | `Dir3::new` normalisiert; `Dir3::new(ZERO)` → `Err`; `Dir2::new((3,4))` → `(0.6,0.8)` | ✅ |
| `rot2_isometry_ray` | `Rot2::degrees(90) * Vec2::X == Y`; `Isometry3d::new(t,r).transform_point`; `Ray3d::new + get_point` | ✅ |
| `bounding_volume_raycast` | `Aabb3d::new`; `RayCast3d::from_ray` + `IntersectsVolume::intersects` (Hit **und** Miss); `aabb_intersection_at` == 4.0 | ✅ |
| `primitive_measurements` | `Sphere::new(2).{volume,area}`; `Cuboid::new(2,3,4).{volume,area}` (`Measured3d`) | ✅ |
| `cubic_bezier_segment` | `CubicSegment::new_bezier([Vec2;4])` + `.position(0/0.5/1)` | ✅ |
| `curve_feature` | `EasingCurve` (`EaseFunction::Linear` / `QuadraticIn`) + `Curve::sample`; `FunctionCurve` über `Interval::UNIT` (inkl. `sample()==None` außerhalb) | ✅ |
| `demo_pipeline_runs` | vollständige MVP-Pipeline `proj·view·model` + `project_point3` liefert `is_finite()` | ✅ |

- glam nutzt auf `armv6k` den **`scalar`-Backend** (kein SSE/NEON) — `Vec3A` ist echt skalar,
  aber weiterhin 16-Byte-aligned.
- **Rest-Unsicherheit:** nur Emulator. VFP-Rundungsmodi / Denormals / exakte NaN-Bitmuster auf
  echter Hardware nicht 1:1 garantiert (für ε=1e-4-Mathe unkritisch). `rand`-Feature bewusst
  aus (§B5). `bevy_reflect`-Integration von `bevy_math` (Feature `bevy_reflect`) ungetestet.
  Nicht abgedeckt: Swizzles, `DVec`/`IVec`/`UVec`, `Affine2/3`, alle `EaseFunction`-Varianten,
  NURBS/`RationalCurve`, `sampling`, `compass`, `FloatOrd`.

### Empfohlene Test-Reihenfolge (jeweils on-device via `crates/bevy-ecs-check`)

1. `Commands` + `world.flush()` + `EntityCommands`
2. Events (`Events<T>`, Reader/Writer, Update-System über mehrere Frames)
3. Change Detection über Frames (`Added`/`Changed`)
4. `SystemParam`-derive, `Local`, exklusives System
5. **`multi_threaded`**: `Schedule` mit parallelem Executor + 2 nicht-konfligierenden Systemen,
   `TaskPoolBuilder` auf 1–2 Threads, **auf Hardware**, Langlauf (§B1/§B2)
6. `bevy_reflect` isoliert: `#[derive(Reflect)]`, `TypeRegistry`, `Box<dyn Reflect>`, Downcast
7. `bevy_platform::Instant` auf Hardware (§B6)
8. `bevy_app` mit manuellem `app.update()`-Loop (kein Runner)
9. Langlauf-/Speichertest (§B7)