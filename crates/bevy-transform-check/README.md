# bevy-transform-check

On-device check that **`bevy_transform`** works on the Nintendo 3DS — the `Transform`/`GlobalTransform` math **and** the ECS side. Aims to touch every public function.

| | |
|---|---|
| Bevy-Einheit | `bevy_transform` (zieht `bevy_ecs`, `bevy_app`, `bevy_math`) |
| Version | `0.19.1` |
| Features | `default-features = false`, `features = ["std", "bevy-support"]` — **kein** `bevy_reflect`, **kein** `multi_threaded` |
| Target | `armv6k-nintendo-3ds` |
| Toleranz | ε = 1e-4 |
| Stand | 2026-09-10 · Azahar-Emulator · **38/38 Testfunktionen · 129/129 Checks bestanden** |

Ausführen: `./scripts/test-emulator.sh -p bevy-transform-check`

## Abgedeckt

**`Transform`** — alle Konstruktoren/Builder (`from_xyz`/`from_translation`/`from_rotation`/`from_scale`/`from_matrix`/`from_isometry`/`with_*`/`IDENTITY`/`Default`), lokale Achsen (`local_x/y/z`, `forward`/`back`/`up`/`down`/`left`/`right` → `Dir3`), Komposition (`mul_transform`, `*` für `Transform`/`GlobalTransform`/`Vec3`, `transform_point`), Rotation (`rotate`/`rotate_x/y/z`/`rotate_axis`, `rotate_local*`, `rotate_around`, `translate_around`), Ausrichtung (`look_at`/`looking_at`/`look_to`/`looking_to`/`align`/`aligned_by`), Konversion (`to_matrix`/`compute_affine`/`to_isometry` + Round-Trips), `is_finite`.

**`GlobalTransform`** — `from_xyz/translation/rotation/scale`, `from(Mat4)`, `from(Transform)`, `translation`/`translation_vec3a`/`rotation`/`scale`, `to_scale_rotation_translation`, `compute_transform`, `to_matrix`/`affine`/`to_isometry`, `transform_point`, `reparented_to`, `radius_vec3a`, `mul_transform`, `*`-Operatoren.

**`TransformPoint`** auf `Transform`/`GlobalTransform`/`Mat4`/`Affine3A` (+ generisch).

**ECS** — `Transform` als Component + `require(GlobalTransform, TransformTreeChanged)`, `Query<&mut Transform>` + Change-Detection, `TransformTreeChanged`.

**Propagation** — `mark_dirty_trees` → `propagate_parent_transforms` → `sync_simple_transforms` (serielle Fallbacks) über 3-Ebenen- und 21-Ebenen-Hierarchien, Rotations-Komposition, Reaktion auf Parent-Änderung, `ChildOf`→`Children`, `StaticTransformOptimizations` (Enabled/Disabled).

**`TransformHelper`** (`SystemParam`, via `run_system_once`) — `compute_global_transform` ohne Propagations-Lauf + Fehlerpfad.

**`BuildChildrenTransformExt`** — `set_parent_in_place` / `remove_parent_in_place` (Weltposition bleibt, `Transform` wird nachgezogen).

**`TransformPlugin`** unter echter `bevy_app::App` — `App::update()` propagiert; folgt Parent über mehrere Updates. (⇒ **`bevy_app::App::update()` läuft auf dem 3DS.**)

**Layout** — `size_of`/`align_of` von `Transform`, `GlobalTransform`, `TransformTreeChanged`.

## Ergebnis

Funktion · Eingabe · Erwartet · **tatsächliche Ausgabe** · OK.

### `commands_ext::remove_parent_in_place_preserves_world_position`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `EntityWorldMut::remove_parent_in_place — world position kept` | child was at world (12,0,0) | `Vec3(12.0, 0.0, 0.0)` | `Vec3(12.0, 0.0, 0.0)` | ✅ |
| `… and local Transform is now the world transform (12,0,0)` |  | `Vec3(12.0, 0.0, 0.0)` | `Vec3(12.0, 0.0, 0.0)` | ✅ |
| `… ChildOf removed` |  | `false` | `false` | ✅ |

### `commands_ext::set_parent_in_place_preserves_world_position`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `EntityWorldMut::set_parent_in_place — world position kept` | child was at world (3,0,0), new parent at (10,0,0) | `Vec3(3.0, 0.0, 0.0)` | `Vec3(3.0, 0.0, 0.0)` | ✅ |
| `… and its local Transform is now (-7,0,0)` |  | `Vec3(-7.0, 0.0, 0.0)` | `Vec3(-7.0, 0.0, 0.0)` | ✅ |
| `… ChildOf(parent) inserted` |  | `true` | `true` | ✅ |

### `ecs_components::query_and_mutate`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Query<&mut Transform> mutate + read back` | T(1,2,3), x += 9 | `Vec3(10.0, 2.0, 3.0)` | `Vec3(10.0, 2.0, 3.0)` | ✅ |
| `Changed<Transform> sees the mutation` | after get_mut | `1` | `1` | ✅ |

### `ecs_components::required_components_and_spawn`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Transform requires GlobalTransform (auto-inserted on spawn)` | spawn(Transform) | `true` | `true` | ✅ |
| `Transform requires TransformTreeChanged` | spawn(Transform) | `true` | `true` | ✅ |
| `GlobalTransform starts at default` | before any propagation | `GlobalTransform(Affine3A { matrix3: Mat3A { x_axis: Vec3A(1.0, 0.0, 0.0), y_axis: Vec3A(0.0, 1.0, 0.0), z_axis: Vec3A(0.0, 0.0, 1.0) }, translation: Vec3A(0.0, 0.0, 0.0) })` | `GlobalTransform(Affine3A { matrix3: Mat3A { x_axis: Vec3A(1.0, 0.0, 0.0), y_axis: Vec3A(0.0, 1.0, 0.0), z_axis: Vec3A(0.0, 0.0, 1.0) }, translation: Vec3A(0.0, 0.0, 0.0) })` | ✅ |

### `ecs_components::transform_tree_changed_component`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `TransformTreeChanged is a unit component` |  | `TransformTreeChanged` | `TransformTreeChanged` | ✅ |
| `TransformTreeChanged present + mutable` | spawn(Transform) | `true` | `true` | ✅ |

### `global_transform::constructors_and_accessors`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `GlobalTransform::from_xyz` | (1,2,3) | `Vec3(1.0, 2.0, 3.0)` | `Vec3(1.0, 2.0, 3.0)` | ✅ |
| `GlobalTransform::from_translation` | (4,5,6) | `Vec3(4.0, 5.0, 6.0)` | `Vec3(4.0, 5.0, 6.0)` | ✅ |
| `GlobalTransform::from_rotation` | rot_z(PI/2) * X | `Vec3(0.0, 1.0, 0.0)` | `Vec3(5.9604645e-8, 0.99999994, 0.0)` | ✅ |
| `GlobalTransform::from_scale` | (2,3,4) | `Vec3(2.0, 3.0, 4.0)` | `Vec3(2.0, 3.0, 4.0)` | ✅ |
| `GlobalTransform::default() == from(IDENTITY)` |  | `GlobalTransform(Affine3A { matrix3: Mat3A { x_axis: Vec3A(1.0, 0.0, 0.0), y_axis: Vec3A(0.0, 1.0, 0.0), z_axis: Vec3A(0.0, 0.0, 1.0) }, translation: Vec3A(0.0, 0.0, 0.0) })` | `GlobalTransform(Affine3A { matrix3: Mat3A { x_axis: Vec3A(1.0, 0.0, 0.0), y_axis: Vec3A(0.0, 1.0, 0.0), z_axis: Vec3A(0.0, 0.0, 1.0) }, translation: Vec3A(0.0, 0.0, 0.0) })` | ✅ |
| `GlobalTransform::translation` | T(5,6,7) S=3 Rz | `Vec3(5.0, 6.0, 7.0)` | `Vec3(5.0, 6.0, 7.0)` | ✅ |
| `GlobalTransform::translation_vec3a` | same | `Vec3A(5.0, 6.0, 7.0)` | `Vec3A(5.0, 6.0, 7.0)` | ✅ |
| `GlobalTransform::scale` | same | `Vec3(3.0, 3.0, 3.0)` | `Vec3(2.9999998, 2.9999998, 3.0)` | ✅ |
| `GlobalTransform::rotation` | same | `Quat(0.0, 0.0, 0.70710677, 0.70710677)` | `Quat(0.0, 0.0, 0.7071067, 0.70710677)` | ✅ |

### `global_transform::decompose_and_convert`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `GlobalTransform::to_scale_rotation_translation — translation` |  | `Vec3(-2.0, 4.0, 1.0)` | `Vec3(-2.0, 4.0, 1.0)` | ✅ |
| `… — scale` |  | `Vec3(2.0, 2.0, 2.0)` | `Vec3(2.0, 2.0, 2.0)` | ✅ |
| `… — rotation` |  | `true` | `true` | ✅ |
| `GlobalTransform::compute_transform() round-trip — translation` |  | `Vec3(-2.0, 4.0, 1.0)` | `Vec3(-2.0, 4.0, 1.0)` | ✅ |
| `GlobalTransform::to_matrix().transform_point3 == transform_point` | point X | `Vec3(-0.2448349, 4.0, 0.0411489)` | `Vec3(-0.2448349, 4.0, 0.0411489)` | ✅ |
| `GlobalTransform::affine().transform_point3 == transform_point` | point Y | `Vec3(-2.0, 6.0, 1.0)` | `Vec3(-2.0, 6.0, 1.0)` | ✅ |
| `GlobalTransform::to_isometry().transform_point == transform_point` | point X | `Vec3(1.0, 3.0, 3.0)` | `Vec3(1.0, 3.0, 3.0)` | ✅ |

### `global_transform::from_mat4_and_operators`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `GlobalTransform::from(Mat4)` | T(9,0,0) | `Vec3(9.0, 0.0, 0.0)` | `Vec3(9.0, 0.0, 0.0)` | ✅ |
| `GlobalTransform * GlobalTransform — translation` | G(1,0,0) * G(0,2,0) | `Vec3(1.0, 2.0, 0.0)` | `Vec3(1.0, 2.0, 0.0)` | ✅ |
| `GlobalTransform * Transform — translation` | G(1,0,0) * T(0,2,0) | `Vec3(1.0, 2.0, 0.0)` | `Vec3(1.0, 2.0, 0.0)` | ✅ |
| `GlobalTransform * Vec3 == transform_point` | point Z | `Vec3(1.0, 0.0, 1.0)` | `Vec3(1.0, 0.0, 1.0)` | ✅ |
| `GlobalTransform::mul_transform` | same as * | `Vec3(1.0, 2.0, 0.0)` | `Vec3(1.0, 2.0, 0.0)` | ✅ |

### `global_transform::reparented_to_and_radius`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `GlobalTransform::reparented_to — local translation` | child (10,0,0) under parent (4,0,0) | `Vec3(6.0, 0.0, 0.0)` | `Vec3(6.0, 0.0, 0.0)` | ✅ |
| `GlobalTransform::radius_vec3a(unit extents)` | IDENTITY | `1.7320508` | `1.7320508` | ✅ |

### `helper::compute_global_transform_via_helper`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `TransformHelper::compute_global_transform (no propagation run)` | root(10,0,0)/mid(0,5,0)/tip(0,0,2) | `Vec3(10.0, 5.0, 2.0)` | `Vec3(10.0, 5.0, 2.0)` | ✅ |
| `TransformHelper — rotation composes through ancestors` | root Rz(PI/2), child (1,0,0) | `Vec3(0.0, 1.0, 0.0)` | `Vec3(5.9604645e-8, 0.99999994, 0.0)` | ✅ |
| `TransformHelper — Err for entity without Transform` | spawn_empty() | `true` | `true` | ✅ |

### `layout::type_layout`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `size/align Transform` |  | `size / align` | `48 / 16` | ✅ |
| `size/align GlobalTransform` |  | `size / align` | `64 / 16` | ✅ |
| `size/align TransformTreeChanged` |  | `size / align` | `0 / 1` | ✅ |

### `plugin::transform_plugin_propagates_on_update`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `TransformPlugin: GlobalTransform after App::update()` | root(10,0,0)/child(0,5,0) | `Vec3(10.0, 5.0, 0.0)` | `Vec3(10.0, 5.0, 0.0)` | ✅ |

### `plugin::transform_plugin_reacts_across_updates`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `TransformPlugin: rotation compose after update` | root Rz(PI/2) | `Vec3(0.0, 1.0, 0.0)` | `Vec3(5.9604645e-8, 0.99999994, 0.0)` | ✅ |
| `TransformPlugin: child follows root across a 2nd update` | root translated to (0,0,7) | `Vec3(0.0, 1.0, 7.0)` | `Vec3(5.9604645e-8, 0.99999994, 7.0)` | ✅ |

### `plugin::transform_systems_set_exists`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `TransformSystems::Propagate is Eq/Hash-able` |  | `Propagate` | `Propagate` | ✅ |

### `propagation::childof_maintains_children`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `ChildOf(parent) -> parent gets Children (bevy_ecs relationship hooks)` | 2 children spawned | `len 2, contains both` | `len 2, contains both = true` | ✅ |

### `propagation::deep_hierarchy`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `propagate — 21-deep chain of T(1,0,0)` | each level +1 on x | `Vec3(21.0, 0.0, 0.0)` | `Vec3(21.0, 0.0, 0.0)` | ✅ |

### `propagation::reacts_to_parent_change`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `propagate — child after 1st run` | root at origin | `Vec3(1.0, 1.0, 0.0)` | `Vec3(1.0, 1.0, 0.0)` | ✅ |
| `propagate — child updates after parent moved (change detection)` | root -> (100,0,0), 2nd run | `Vec3(101.0, 1.0, 0.0)` | `Vec3(101.0, 1.0, 0.0)` | ✅ |

### `propagation::rotation_composes_through_hierarchy`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `propagate — rotation compose` | root Rz(PI/2), child local (1,0,0) | `Vec3(0.0, 1.0, 0.0)` | `Vec3(5.9604645e-8, 0.99999994, 0.0)` | ✅ |

### `propagation::static_optimizations_toggle`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `propagate with StaticTransformOptimizations::Disabled` | root(2,0,0)/child(3,0,0) | `Vec3(5.0, 0.0, 0.0)` | `Vec3(5.0, 0.0, 0.0)` | ✅ |
| `StaticTransformOptimizations::Enabled.is_enabled()` |  | `true` | `true` | ✅ |
| `StaticTransformOptimizations::default() == Enabled` |  | `Enabled` | `Enabled` | ✅ |

### `propagation::sync_simple_no_hierarchy`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `sync_simple_transforms (serial iter_mut)` | entity with only Transform(4,-5,6) | `Vec3(4.0, -5.0, 6.0)` | `Vec3(4.0, -5.0, 6.0)` | ✅ |

### `propagation::three_level_translation`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `propagate — root` | T(10,0,0) | `Vec3(10.0, 0.0, 0.0)` | `Vec3(10.0, 0.0, 0.0)` | ✅ |
| `propagate — mid (child of root)` | T(0,5,0) | `Vec3(10.0, 5.0, 0.0)` | `Vec3(10.0, 5.0, 0.0)` | ✅ |
| `propagate — tip (grandchild)` | T(0,0,2) | `Vec3(10.0, 5.0, 2.0)` | `Vec3(10.0, 5.0, 2.0)` | ✅ |

### `transform_basis::identity_basis`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Transform::local_x (identity)` |  | `Vec3(1.0, 0.0, 0.0)` | `Vec3(1.0, 0.0, 0.0)` | ✅ |
| `Transform::local_y (identity)` |  | `Vec3(0.0, 1.0, 0.0)` | `Vec3(0.0, 1.0, 0.0)` | ✅ |
| `Transform::local_z (identity)` |  | `Vec3(0.0, 0.0, 1.0)` | `Vec3(0.0, 0.0, 1.0)` | ✅ |
| `Transform::forward (identity) = -Z` |  | `Vec3(0.0, 0.0, -1.0)` | `Vec3(-0.0, -0.0, -1.0)` | ✅ |
| `Transform::back (identity) = +Z` |  | `Vec3(0.0, 0.0, 1.0)` | `Vec3(0.0, 0.0, 1.0)` | ✅ |
| `Transform::right (identity) = +X` |  | `Vec3(1.0, 0.0, 0.0)` | `Vec3(1.0, 0.0, 0.0)` | ✅ |
| `Transform::left (identity) = -X` |  | `Vec3(-1.0, 0.0, 0.0)` | `Vec3(-1.0, -0.0, -0.0)` | ✅ |
| `Transform::up (identity) = +Y` |  | `Vec3(0.0, 1.0, 0.0)` | `Vec3(0.0, 1.0, 0.0)` | ✅ |
| `Transform::down (identity) = -Y` |  | `Vec3(0.0, -1.0, 0.0)` | `Vec3(-0.0, -1.0, -0.0)` | ✅ |

### `transform_basis::rotated_basis`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Transform::local_x after rot_y(PI/2)` |  | `Vec3(0.0, 0.0, -1.0)` | `Vec3(0.0, 0.0, -0.99999994)` | ✅ |
| `Transform::local_z after rot_y(PI/2)` |  | `Vec3(1.0, 0.0, 0.0)` | `Vec3(0.99999994, 0.0, 0.0)` | ✅ |
| `Transform::forward after rot_y(PI/2)` |  | `Vec3(-1.0, 0.0, 0.0)` | `Vec3(-0.99999994, -0.0, -0.0)` | ✅ |

### `transform_compose::mul_transform_and_operators`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Transform::mul_transform — translation` | T(1,2,3) * (T(0,0,1), S=2) | `Vec3(1.0, 2.0, 4.0)` | `Vec3(1.0, 2.0, 4.0)` | ✅ |
| `Transform::mul_transform — scale` | same | `Vec3(2.0, 2.0, 2.0)` | `Vec3(2.0, 2.0, 2.0)` | ✅ |
| `Transform * Transform == mul_transform` | same | `Transform { translation: Vec3(1.0, 2.0, 4.0), rotation: Quat(0.0, 0.0, 0.0, 1.0), scale: Vec3(2.0, 2.0, 2.0) }` | `Transform { translation: Vec3(1.0, 2.0, 4.0), rotation: Quat(0.0, 0.0, 0.0, 1.0), scale: Vec3(2.0, 2.0, 2.0) }` | ✅ |
| `Transform::mul_transform composes rotation` | Rz(PI/2) * T(1,0,0) | `Vec3(0.0, 1.0, 0.0)` | `Vec3(0.0, 0.99999994, 0.0)` | ✅ |

### `transform_compose::transform_point_and_vec3_mul`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Transform::transform_point` | T(10,0,0) S=2 * point (1,0,0) | `Vec3(12.0, 0.0, 0.0)` | `Vec3(12.0, 0.0, 0.0)` | ✅ |
| `Transform * Vec3 == transform_point` | same | `Vec3(12.0, 0.0, 0.0)` | `Vec3(12.0, 0.0, 0.0)` | ✅ |
| `Transform::transform_point with rotation` | Rz(PI/2) * (1,0,0) | `Vec3(0.0, 1.0, 0.0)` | `Vec3(0.0, 0.99999994, 0.0)` | ✅ |

### `transform_compose::transform_times_global_transform`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Transform * GlobalTransform — translation` | T(5,0,0) * G(T(0,1,0)) | `Vec3(5.0, 1.0, 0.0)` | `Vec3(5.0, 1.0, 0.0)` | ✅ |

### `transform_convert::matrix_round_trip`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Transform -> to_matrix -> from_matrix — translation` |  | `Vec3(5.0, 6.0, 7.0)` | `Vec3(5.0, 6.0, 7.0)` | ✅ |
| `… — scale` |  | `Vec3(2.0, 2.0, 2.0)` | `Vec3(2.0, 2.0, 2.0)` | ✅ |
| `… — rotation` |  | `true` | `true` | ✅ |

### `transform_convert::to_isometry_round_trip`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Transform::to_isometry -> from_isometry — translation` | T(1,2,3), Rz(PI/2) | `Vec3(1.0, 2.0, 3.0)` | `Vec3(1.0, 2.0, 3.0)` | ✅ |
| `Transform::to_isometry -> from_isometry — rotation` | same | `Quat(0.0, 0.0, 0.70710677, 0.70710677)` | `Quat(0.0, 0.0, 0.70710677, 0.70710677)` | ✅ |
| `to_isometry().transform_point == transform_point` | point X | `Vec3(1.0, 3.0, 3.0)` | `Vec3(1.0, 3.0, 3.0)` | ✅ |

### `transform_convert::to_matrix_and_affine`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Transform::to_matrix().transform_point3 == Transform::transform_point` | T=(3,-1,2), Ry=0.6, S=1.5, point X | `Vec3(4.2380037, -1.0, 1.1530362)` | `Vec3(4.2380037, -1.0, 1.1530362)` | ✅ |
| `Transform::compute_affine().transform_point3 == transform_point` | same | `Vec3(3.0, 0.5000001, 2.0)` | `Vec3(3.0, 0.5, 2.0)` | ✅ |

### `transform_ctors::builder_methods`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Transform::with_translation` | IDENTITY.with_translation((1,2,3)) | `Vec3(1.0, 2.0, 3.0)` | `Vec3(1.0, 2.0, 3.0)` | ✅ |
| `Transform::with_rotation` | …with_rotation(rot_y(0.3)) | `Quat(0.0, 0.14943814, 0.0, 0.9887711)` | `Quat(0.0, 0.14943814, 0.0, 0.9887711)` | ✅ |
| `Transform::with_scale` | …with_scale(5) | `Vec3(5.0, 5.0, 5.0)` | `Vec3(5.0, 5.0, 5.0)` | ✅ |
| `Transform::is_finite` | the built transform | `true` | `true` | ✅ |
| `Transform::is_finite (NaN translation)` | translation.x = NaN | `false` | `false` | ✅ |

### `transform_ctors::constructors`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Transform::IDENTITY` |  | `Transform { translation: Vec3(0.0, 0.0, 0.0), rotation: Quat(0.0, 0.0, 0.0, 1.0), scale: Vec3(1.0, 1.0, 1.0) }` | `Transform { translation: Vec3(0.0, 0.0, 0.0), rotation: Quat(0.0, 0.0, 0.0, 1.0), scale: Vec3(1.0, 1.0, 1.0) }` | ✅ |
| `Transform::default() == IDENTITY` |  | `Transform { translation: Vec3(0.0, 0.0, 0.0), rotation: Quat(0.0, 0.0, 0.0, 1.0), scale: Vec3(1.0, 1.0, 1.0) }` | `Transform { translation: Vec3(0.0, 0.0, 0.0), rotation: Quat(0.0, 0.0, 0.0, 1.0), scale: Vec3(1.0, 1.0, 1.0) }` | ✅ |
| `Transform::from_xyz` | (1,2,3) | `Vec3(1.0, 2.0, 3.0)` | `Vec3(1.0, 2.0, 3.0)` | ✅ |
| `Transform::from_translation` | (4,5,6) | `Vec3(4.0, 5.0, 6.0)` | `Vec3(4.0, 5.0, 6.0)` | ✅ |
| `Transform::from_rotation` | rot_z(PI/2) | `Vec3(0.0, 1.0, 0.0)` | `Vec3(0.0, 0.99999994, 0.0)` | ✅ |
| `Transform::from_scale` | (2,3,4) | `Vec3(2.0, 3.0, 4.0)` | `Vec3(2.0, 3.0, 4.0)` | ✅ |
| `Transform::from_matrix — translation` | SRT matrix | `Vec3(1.0, 1.0, 0.0)` | `Vec3(1.0, 1.0, 0.0)` | ✅ |
| `Transform::from_matrix — scale` | SRT matrix | `Vec3(2.0, 2.0, 2.0)` | `Vec3(1.9999999, 1.9999999, 2.0)` | ✅ |
| `Transform::from_matrix — rotation` | SRT matrix | `Quat(0.0, 0.0, 0.70710677, 0.70710677)` | `Quat(0.0, 0.0, 0.7071067, 0.70710677)` | ✅ |
| `Transform::from_isometry — translation` |  | `Vec3(3.0, 4.0, 5.0)` | `Vec3(3.0, 4.0, 5.0)` | ✅ |
| `Transform::from_isometry — rotation` |  | `Quat(0.24740396, 0.0, 0.0, 0.9689124)` | `Quat(0.24740396, 0.0, 0.0, 0.9689124)` | ✅ |
| `Transform::from_isometry — scale is ONE` |  | `Vec3(1.0, 1.0, 1.0)` | `Vec3(1.0, 1.0, 1.0)` | ✅ |

### `transform_ctors::transform_from_global`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Transform::from(GlobalTransform) — translation` | G(T(7,8,9), S=2) | `Vec3(7.0, 8.0, 9.0)` | `Vec3(7.0, 8.0, 9.0)` | ✅ |
| `Transform::from(GlobalTransform) — scale` | same | `Vec3(2.0, 2.0, 2.0)` | `Vec3(2.0, 2.0, 2.0)` | ✅ |

### `transform_look::align`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Transform::align(X, Z, X, Y) — X maps to Z` |  | `Vec3(0.0, 0.0, 1.0)` | `Vec3(0.0, 0.0, 0.99999994)` | ✅ |
| `Transform::aligned_by(X, Y, (1,1,0), Z) — main axis image ≈ Y` |  | `Vec3(0.0, 1.0, 0.0)` | `Vec3(-2.980233e-8, 1.0, 0.0)` | ✅ |

### `transform_look::look_at_and_to`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Transform::looking_at(origin) — forward points at target` | eye (0,0,5) | `Vec3(0.0, 0.0, -1.0)` | `Vec3(-0.0, -0.0, -1.0)` | ✅ |
| `Transform::looking_at — up preserved` |  | `Vec3(0.0, 1.0, 0.0)` | `Vec3(0.0, 1.0, 0.0)` | ✅ |
| `Transform::look_at (mut) — forward` | target -Z | `Vec3(0.0, 0.0, -1.0)` | `Vec3(-0.0, -0.0, -1.0)` | ✅ |
| `Transform::looking_to(-X) — forward` |  | `Vec3(-1.0, 0.0, 0.0)` | `Vec3(-0.99999994, -0.0, -0.0)` | ✅ |
| `Transform::look_to(+Y) — forward` |  | `Vec3(0.0, 1.0, 0.0)` | `Vec3(-0.0, 0.99999994, -0.0)` | ✅ |

### `transform_point_trait::transform_point_trait_impls`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `TransformPoint on Transform` | T(10,0,0) * (0,0,0) | `Vec3(10.0, 0.0, 0.0)` | `Vec3(10.0, 0.0, 0.0)` | ✅ |
| `TransformPoint on GlobalTransform` | G(T(10,0,0)) * (0,0,0) | `Vec3(10.0, 0.0, 0.0)` | `Vec3(10.0, 0.0, 0.0)` | ✅ |
| `TransformPoint on Mat4` | T(10,0,0) * (0,0,0) | `Vec3(10.0, 0.0, 0.0)` | `Vec3(10.0, 0.0, 0.0)` | ✅ |
| `TransformPoint on Affine3A` | T(10,0,0) * (0,0,0) | `Vec3(10.0, 0.0, 0.0)` | `Vec3(10.0, 0.0, 0.0)` | ✅ |
| `TransformPoint (generic) on rotated Transform` | Rz(PI/2) * X | `Vec3(0.0, 1.0, 0.0)` | `Vec3(0.0, 0.99999994, 0.0)` | ✅ |

### `transform_rotate::orbit_around_a_point`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Transform::translate_around(origin, rot_z(PI/2))` | start (1,0,0) | `Vec3(0.0, 1.0, 0.0)` | `Vec3(0.0, 0.99999994, 0.0)` | ✅ |
| `Transform::rotate_around — translation orbits` | start (1,0,0) | `Vec3(0.0, 1.0, 0.0)` | `Vec3(0.0, 0.99999994, 0.0)` | ✅ |
| `Transform::rotate_around — also spins the rotation` |  | `Quat(0.0, 0.0, 0.70710677, 0.70710677)` | `Quat(0.0, 0.0, 0.70710677, 0.70710677)` | ✅ |

### `transform_rotate::rotate_local_space`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Transform::rotate_local_x changes rotation` | started at rot_z(PI/2) | `true` | `true` | ✅ |
| `Transform::rotate_local(rot_y(PI/2)) from IDENTITY` |  | `Quat(0.0, 0.70710677, 0.0, 0.70710677)` | `Quat(0.0, 0.70710677, 0.0, 0.70710677)` | ✅ |
| `Transform::rotate_local_axis(Y, PI/2) from IDENTITY` |  | `Quat(0.0, 0.70710677, 0.0, 0.70710677)` | `Quat(0.0, 0.70710677, 0.0, 0.70710677)` | ✅ |
| `Transform::rotate_local_z(PI/2) from IDENTITY` |  | `Quat(0.0, 0.0, 0.70710677, 0.70710677)` | `Quat(0.0, 0.0, 0.70710677, 0.70710677)` | ✅ |

### `transform_rotate::rotate_world_space`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Transform::rotate(rot_z(PI/2))` | IDENTITY | `Quat(0.0, 0.0, 0.70710677, 0.70710677)` | `Quat(0.0, 0.0, 0.70710677, 0.70710677)` | ✅ |
| `Transform::rotate_x(PI/2) * local +Y = +Z` |  | `Vec3(0.0, 0.0, 1.0)` | `Vec3(0.0, 0.0, 0.99999994)` | ✅ |
| `Transform::rotate_y(PI/2) * local +Z = +X` |  | `Vec3(1.0, 0.0, 0.0)` | `Vec3(0.99999994, 0.0, 0.0)` | ✅ |
| `Transform::rotate_z(PI/2) * local +X = +Y` |  | `Vec3(0.0, 1.0, 0.0)` | `Vec3(0.0, 0.99999994, 0.0)` | ✅ |
| `Transform::rotate_axis(Z, PI/2)` |  | `Quat(0.0, 0.0, 0.70710677, 0.70710677)` | `Quat(0.0, 0.0, 0.70710677, 0.70710677)` | ✅ |

## Nicht abgedeckt

`multi_threaded`-Propagation (paralleler Pfad — der harte §B2-Block), sehr breite Hierarchien, `TransformPlugin` mit anderen Bevy-Plugins kombiniert, Serialisierung.

**Caveat:** nur Emulator. Siehe `../../port.md` §B10.
