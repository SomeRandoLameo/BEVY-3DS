| Crate | Einstufung | Begründung | getestet |
|---|---|---|---|
| bevy_ecs | Sicher (core, 1 Thread) | Core-ECS = reine Logik/Daten. Aber: nicht "platformfrei" — Entity-Allocator nutzt 64-bit-Atomics (Fallback, §B1), `thread_local!`, optionaler Parallel-Executor. | ✅ HW: spawn/despawn, `Query<&T>`/`Query<&mut T>`, `Res`/`ResMut`, `Schedule`+`.chain()`, `entity().get()` |
| bevy_math | Sicher | glam-Reexport + Bevy-Primitives. Zieht **glam 0.32** neben citro3ds glam 0.30 (koexistieren, §B8), `scalar`-Backend (kein NEON). `rand`-Feature baut auf 3DS (kein `getrandom`, §B9). | ✅ **488/488 Checks** (57 Testfn, Emu, `crates/bevy-math-check`) — quasi die ganze öffentliche API. Vollständige Liste → `crates/bevy-math-check/README.md`, Überblick §B9 |
| bevy_transform | Sicher (1 Thread) | Mathe + Hierarchie. Propagation nutzt `par_iter`/`ComputeTaskPool` — hat aber ohne `multi_threaded` **explizite serielle Fallbacks** (`serial`-Modul, `iter_mut`-Branches). Die laufen korrekt (§B10). | ✅ **12/12** (Emu, `crates/bevy-transform-check`) — Propagation 3 Ebenen + Rotation + Change-Detection, Transform/GlobalTransform-Mathe. §B10 |
| bevy_ptr | Sicher | keine Platform-Annahmen | ✅ baut (transitiv via bevy_ecs) |
| bevy_utils | Sicher | keine Platform-Annahmen | ✅ baut (transitiv) |
| bevy_platform | Sicher (mit Fallbacks) | **ist** die Platform-Abstraktion: Atomic-Shim (64-bit → `portable-atomic` Fallback), `Instant`, sync-Primitive. Baut für 3DS. | ✅ baut (transitiv) |
| bevy_derive | Sicher | Makros, keine Runtime-Abhängigkeit ||
| bevy_macro_utils | Sicher | Makros, keine Runtime-Abhängigkeit ||
| bevy_color | Sicher | reine Farbraum-Mathe ||
| bevy_time | Wahrscheinlich okay | Baut (`bevy_platform::Instant` → `std::time::Instant`). Ob `Instant::now()` auf **3DS-Hardware** liefert, ist UNGEPRÜFT — im Demo bewusst mit `svcGetSystemTick` umgangen (§B6). | Instant auf HW ✗ |
| bevy_app | Wahrscheinlich okay | Runner passt nicht, `App::update()` manuell nötig. **Kompiliert + linkt** für 3DS (transitiv via `bevy-transform-check`). `App`-Konstruktion / `update()`-Loop noch nicht funktionsgetestet. | ⚠️ baut + linkt |
| bevy_state | Wahrscheinlich okay | reine State-Machine-Logik ||
| bevy_diagnostic | Wahrscheinlich okay | evtl. OS-Metriken disablen ||
| bevy_asset | Riskant | Async-Loader, Thread-Pool-Abhängigkeit ||
| bevy_tasks | Riskant | **Nicht-optionale Dep von bevy_ecs** — baut + linkt schon jetzt (single-threaded Form). `multi_threaded` zieht `async-executor` + `concurrent-queue` und echte Threads → §B2. | ⚠️ baut single-thread; `multi_threaded` ✗ |
| bevy_reflect | Riskant | schwergewichtig, Compile-Zeit/Binary-Size. `uuid`-Feature-Pfade können `getrandom` ziehen → §"Plattform-Fakten". Gate für app/scene/state/asset. ||
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
  `Query<Entity, With<T>>`, Bulk-Spawn/Despawn. **Alles single-threaded.**
  9/9 Checks → **`crates/bevy-ecs-check/README.md`** (Funktion · Eingabe · Erwartet · Ausgabe).
- `bevy_math` 0.19.1, `default-features = false, features = ["std", "curve", "rand"]`:
  **488 Assertion-Checks** (57 Testfunktionen) im Emulator — nahezu die komplette
  öffentliche API. Siehe §B9 / `crates/bevy-math-check/README.md`. In `dove` selbst nur `["std"]` nötig.
- `bevy_transform` 0.19.1, `default-features = false, features = ["std", "bevy-support"]`:
  12 Tests im Emulator, siehe §B10 — inkl. der **Propagations-Pipeline** über eine
  echte Hierarchie (der §B2-Verdacht ist damit für `bevy_transform` entschärft).
- Transitiv mitgebaut **+ gelinkt** (nicht separat funktionsgetestet):
  `bevy_platform`, `bevy_ptr`, `bevy_utils`, `bevy_tasks` (single-threaded), `bevy_ecs_macros`,
  `bevy_derive`, `bevy_app` (via `bevy-transform-check`), `glam` 0.32.

Vehikel für weitere isolierte Tests: **`crates/bevy-ecs-check`**,
**`crates/bevy-math-check`**, **`crates/bevy-transform-check`** (jeweils ohne
citro3d im Baum, on-device via `./scripts/test-emulator.sh -p <crate>`).

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
`./scripts/test-emulator.sh -p bevy-transform-check`. **12 / 12 Testfunktionen, 26 / 26 Checks bestanden**, ε = 1e-4.

→ Vollständige Tabelle (Funktion · Eingabe · Erwartet · **tatsächliche Ausgabe** · OK):
**`crates/bevy-transform-check/README.md`**.

Die Propagations-Systeme werden direkt in ein `Schedule` auf einer `World` registriert
(`mark_dirty_trees` → `propagate_parent_transforms` → `sync_simple_transforms`, `.chain()`,
`StaticTransformOptimizations::Enabled` als Resource) — genau wie `TransformPlugin` es tut,
nur ohne `App`.

Funktion für Funktion:

| Test-Funktion | geprüfte APIs / Aussagen | Status |
|---|---|---|
| `transform_compose_and_point` | `Transform::from_xyz` / `from_rotation` / `with_scale`; `mul_transform` (Translation + Scale kombiniert); `transform_point` mit Rotation | ✅ |
| `transform_looking_at_and_basis` | `Transform::looking_at`; `forward()` == `-Z`, `up()` == `Y`, `right()` == `X` | ✅ |
| `transform_matrix_roundtrip` | `Transform::to_matrix()` · `Mat4::transform_point3` == `Transform::transform_point` | ✅ |
| `global_transform_ops` | `GlobalTransform::from(Transform)`; **`GlobalTransform * Transform`** (die Propagations-Compose-Op); `transform_point`; `to_scale_rotation_translation`; `compute_transform()`-Roundtrip | ✅ |
| `transform_point_trait` | `TransformPoint`-Trait auf `Transform` **und** `GlobalTransform` | ✅ |
| `sync_simple_transform_no_hierarchy` | `sync_simple_transforms` (serieller `iter_mut`-Branch): Entity nur mit `Transform` → `GlobalTransform` = lokal | ✅ |
| `propagate_three_level_translation` | volle Pipeline über root→mid→tip (`ChildOf`), `GlobalTransform.translation()` je Ebene = akkumuliert | ✅ |
| `propagate_with_parent_rotation` | Parent 90° um Z, Child lokal `+X` → Child-Welt ≈ `+Y` (Rotations-Komposition durch die Hierarchie) | ✅ |
| `propagation_reacts_to_parent_change` | 2. Schedule-Lauf nach `Transform`-Mutation am Parent → Child-`GlobalTransform` aktualisiert (Change-Detection) | ✅ |
| `childof_maintains_children_relationship` | `ChildOf` einfügen → Parent bekommt `Children` mit beiden Kindern (bevy_ecs-Relationship-Hooks auf 3DS) | ✅ |
| `transform_component_roundtrip` | `require(GlobalTransform, …)` greift beim Spawn; `Query<&mut Transform>` mutieren + zurücklesen | ✅ |
| `demo_runs` | `demo()`: 3-Ebenen-Rig aufbauen + propagieren → `(10, 5, 2)` | ✅ |

- **Serielle Fallbacks bestätigt:** ohne `multi_threaded` nimmt bevy_transform `mod serial` /
  die `#[cfg(not(feature = "multi_threaded"))] iter_mut`-Branches — **kein `ComputeTaskPool`**
  wird angefasst, kein `par_iter`-Panic. `bevy_ecs::par_iter` degradiert selbst zu seriell.
- **Bonus:** `bevy_app` 0.19.1 **kompiliert + linkt** für 3DS (transitiv). `App`/`update()`
  weiterhin ungetestet (§B6, §Empfohlene Reihenfolge).
- Nicht abgedeckt: `TransformHelper`, `BuildChildrenTransformExt` (`with_child` etc.),
  Orphan-Handling (`RemovedComponents<ChildOf>`-Pfad), `StaticTransformOptimizations`-Subtree-Skipping,
  sehr tiefe/breite Hierarchien, `multi_threaded`-Propagation.

### Erledigt

- ✅ `bevy_math` inkl. `rand`/`sampling` — §B9 (`bevy-math-check`, 488/488 Checks, quasi ganze API)
- ✅ `bevy_transform` inkl. Propagation, Change-Detection, `ChildOf`-Relationship — §B10 (`bevy-transform-check`, 12/12)
- ✅ `bevy_ecs` core single-threaded — §"Was verifiziert ist"
- ✅ `bevy_app` kompiliert + linkt (noch nicht `update()`-getestet)

### Empfohlene nächste Schritte (on-device via ein `bevy-*-check`)

1. `Commands` + `world.flush()` + `EntityCommands`
2. Events (`Events<T>`, Reader/Writer, Update-System über mehrere Frames)
3. Change Detection als eigener Test (`Added`/`Changed`/`Ref`, Tick-Verhalten)
4. `SystemParam`-derive, `Local`, exklusives System, `ParamSet`
5. **`multi_threaded`**: `Schedule` mit parallelem Executor + 2 nicht-konfligierenden Systemen,
   `TaskPoolBuilder` auf 1–2 Threads, **auf Hardware**, Langlauf (§B1/§B2) — der harte Block
6. `bevy_reflect` isoliert: `#[derive(Reflect)]`, `TypeRegistry`, `Box<dyn Reflect>`, Downcast
7. `bevy_platform::Instant` auf Hardware (§B6)
8. `bevy_app` mit manuellem `app.update()`-Loop (kein Runner)
9. `bevy_transform` mit `multi_threaded` (parallele Propagation, §B10-Rest)
10. Langlauf-/Speichertest (§B7)