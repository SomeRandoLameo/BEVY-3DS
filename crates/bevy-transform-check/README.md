# bevy-transform-check

On-device check that **`bevy_transform`** — the `Transform`/`GlobalTransform` math **and** the
ECS propagation pipeline — works on the Nintendo 3DS.

| | |
|---|---|
| Bevy-Einheit | `bevy_transform` |
| Version | `0.19.1` |
| Features | `default-features = false`, `features = ["std", "bevy-support"]` (zieht `bevy_ecs` + `bevy_app`; **kein** `bevy_reflect`, **kein** `multi_threaded`) |
| Target | `armv6k-nintendo-3ds` |
| Toleranz | ε = 1e-4 |
| Stand | 2026-09-10 · Azahar-Emulator · **12/12 Testfunktionen, 26/26 Checks bestanden** |

Ausführen: `./scripts/test-emulator.sh -p bevy-transform-check`

Die Propagations-Systeme werden 1:1 wie in `TransformPlugin` in ein `Schedule` auf einer
`World` gehängt (`mark_dirty_trees` → `propagate_parent_transforms` → `sync_simple_transforms`,
`.chain()`, `StaticTransformOptimizations::Enabled`), nur ohne `App`. Ohne `multi_threaded`
greifen die **seriellen Fallbacks** (`mod serial`, `iter_mut`-Branches) — **kein
`ComputeTaskPool` wird angefasst.**

## Ergebnis

Jede Zeile: geprüfte Funktion · Eingabe · Erwartung · tatsächliche Ausgabe · Status.

### `Transform` / `GlobalTransform` — Mathe

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Transform::mul_transform` — translation | T(1,2,3) * (T(0,0,1), S=2) | `Vec3(1.0, 2.0, 4.0)` | `Vec3(1.0, 2.0, 4.0)` | ✅ |
| `Transform::mul_transform` — scale | dito | `Vec3(2.0, 2.0, 2.0)` | `Vec3(2.0, 2.0, 2.0)` | ✅ |
| `Transform::from_rotation(Rz PI/2).transform_point(X)` | | `Vec3(0.0, 1.0, 0.0)` | `Vec3(0.0, 0.99999994, 0.0)` | ✅ |
| `Transform::looking_at(-Z).forward()` | | `Vec3(0.0, 0.0, -1.0)` | `Vec3(-0.0, -0.0, -1.0)` | ✅ |
| `Transform::looking_at(-Z).up()` | | `Vec3(0.0, 1.0, 0.0)` | `Vec3(0.0, 1.0, 0.0)` | ✅ |
| `Transform::looking_at(-Z).right()` | | `Vec3(1.0, 0.0, 0.0)` | `Vec3(1.0, 0.0, 0.0)` | ✅ |
| `Transform::to_matrix().transform_point3` == `Transform::transform_point` | T=(3,-1,2), Ry=0.6, S=1.5, point X | `Vec3(4.2380037, -1.0, 1.1530362)` | `Vec3(4.2380037, -1.0, 1.1530362)` | ✅ |
| `GlobalTransform * Transform` (Propagations-Compose-Op) | G(T(1,0,0)) * T(0,2,0) | `Vec3(1.0, 2.0, 0.0)` | `Vec3(1.0, 2.0, 0.0)` | ✅ |
| `GlobalTransform::transform_point` | G(T(1,0,0)) * point Z | `Vec3(1.0, 0.0, 1.0)` | `Vec3(1.0, 0.0, 1.0)` | ✅ |
| `GlobalTransform::to_scale_rotation_translation` — translation | T(5,6,7), S=3 | `Vec3(5.0, 6.0, 7.0)` | `Vec3(5.0, 6.0, 7.0)` | ✅ |
| `GlobalTransform::to_scale_rotation_translation` — scale | dito | `Vec3(3.0, 3.0, 3.0)` | `Vec3(3.0, 3.0, 3.0)` | ✅ |
| `GlobalTransform::to_scale_rotation_translation` — rotation ≈ identity | dito | `near-identity` | `near-identity` | ✅ |
| `GlobalTransform::from(t).compute_transform()` round-trip | T(-2,4,1) | `Vec3(-2.0, 4.0, 1.0)` | `Vec3(-2.0, 4.0, 1.0)` | ✅ |
| `TransformPoint::transform_point` auf `Transform` | T(10,0,0) * (0,0,0) | `Vec3(10.0, 0.0, 0.0)` | `Vec3(10.0, 0.0, 0.0)` | ✅ |
| `TransformPoint::transform_point` auf `GlobalTransform` | G(T(10,0,0)) * (0,0,0) | `Vec3(10.0, 0.0, 0.0)` | `Vec3(10.0, 0.0, 0.0)` | ✅ |

### ECS-Propagations-Pipeline

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `sync_simple_transforms` (serieller `iter_mut`-Branch) | Entity nur mit Transform(4,-5,6), 1 Schedule-Run | `Vec3(4.0, -5.0, 6.0)` | `Vec3(4.0, -5.0, 6.0)` | ✅ |
| `propagate`: root `GlobalTransform` | root T(10,0,0) | `Vec3(10.0, 0.0, 0.0)` | `Vec3(10.0, 0.0, 0.0)` | ✅ |
| `propagate`: mid (Kind von root) `GlobalTransform` | mid T(0,5,0) | `Vec3(10.0, 5.0, 0.0)` | `Vec3(10.0, 5.0, 0.0)` | ✅ |
| `propagate`: tip (Enkel) `GlobalTransform` | tip T(0,0,2) | `Vec3(10.0, 5.0, 2.0)` | `Vec3(10.0, 5.0, 2.0)` | ✅ |
| `propagate`: Rotation komponiert durch die Hierarchie | root Rz(PI/2), child local T(1,0,0) | `Vec3(0.0, 1.0, 0.0)` | `Vec3(5.96e-8, 0.99999994, 0.0)` | ✅ |
| `propagate`: child nach 1. Run | root im Ursprung | `Vec3(1.0, 1.0, 0.0)` | `Vec3(1.0, 1.0, 0.0)` | ✅ |
| `propagate`: child aktualisiert nach `Transform`-Mutation am Parent (Change Detection) | root → (100,0,0), 2. Run | `Vec3(101.0, 1.0, 0.0)` | `Vec3(101.0, 1.0, 0.0)` | ✅ |
| `ChildOf(parent)` → Parent bekommt `Children` (bevy_ecs-Relationship-Hooks) | 2 Kinder mit `ChildOf(parent)` spawnen | len 2, enthält beide | `len 2, contains both = true` | ✅ |
| `Transform::require(GlobalTransform)` beim Spawn | `spawn(Transform)` | `GlobalTransform` vorhanden | `present` | ✅ |
| `Query<&mut Transform>` mutieren + zurücklesen | T(1,2,3), x += 9 | `Vec3(10.0, 2.0, 3.0)` | `Vec3(10.0, 2.0, 3.0)` | ✅ |
| `demo()` — 3-Ebenen-Rig, propagiert, tip-Translation | root(10,0,0)/mid(0,5,0)/tip(0,0,2) | `Vec3(10.0, 5.0, 2.0)` | `Vec3(10.0, 5.0, 2.0)` | ✅ |

## Bonus

`bevy_app` `0.19.1` **kompiliert + linkt** für das 3DS-Target (transitiv über `bevy-support`).
`App`-Konstruktion / `update()`-Loop noch nicht funktionsgetestet.

## Nicht abgedeckt

`TransformHelper`, `BuildChildrenTransformExt` (`with_child` etc.), Orphan-Handling
(`RemovedComponents<ChildOf>`-Pfad), `StaticTransformOptimizations`-Subtree-Skipping, sehr
tiefe/breite Hierarchien, **`multi_threaded`-Propagation**. Siehe `../../port.md` §B10.
