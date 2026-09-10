# bevy-math-check

On-device check that **`bevy_math`** (and its `glam`) computes correctly on the Nintendo 3DS.

| | |
|---|---|
| Bevy-Einheit | `bevy_math` (zieht `glam` `0.32.1`) |
| Version | `0.19.1` |
| Features | `default-features = false`, `features = ["std", "curve"]` (kein `rand` → kein `getrandom`, kein `bevy_reflect`) |
| Target | `armv6k-nintendo-3ds` · glam-Backend **`scalar`** (kein SSE/NEON) · Trig/`sqrt`/`exp` aus devkitPro-newlib |
| Toleranz | ε = 1e-4 |
| Stand | 2026-09-10 · Azahar-Emulator · **14/14 Testfunktionen · 66/66 Checks bestanden** |

Ausführen: `./scripts/test-emulator.sh -p bevy-math-check`

## Ergebnis

Funktion · Eingabe · Erwartet · **tatsächliche Ausgabe** · OK. Reihenfolge = Testausführung
(alphabetisch nach Testfunktion). Die ε-Toleranz fängt die üblichen f32-Rundungen ab
(z. B. `ln(E) = 0.99999994`, `sin(PI/2)`-Rotation lässt `~-4.4e-8` statt `0` stehen).

### `bounding_volume_raycast`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `RayCast3d(+X) intersects Aabb3d @ x=5` |  | `true` | `true` | ✅ |
| `RayCast3d(+Y) misses Aabb3d @ x=5` |  | `false` | `false` | ✅ |
| `RayCast3d::aabb_intersection_at` | ray +X, aabb center 5 half 1 | `4` | `4` | ✅ |

### `cubic_bezier_segment`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `CubicSegment::new_bezier(..).position(0)` | control pts (0,0)(0,1)(1,1)(1,0) | `Vec2(0.0, 0.0)` | `Vec2(0.0, 0.0)` | ✅ |
| `CubicSegment::position(1)` | same | `Vec2(1.0, 0.0)` | `Vec2(1.0, 0.0)` | ✅ |
| `CubicSegment::position(0.5)` | same | `Vec2(0.5, 0.75)` | `Vec2(0.5, 0.75)` | ✅ |

### `curve_feature`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `EasingCurve(Linear, 0..10).sample(0.5)` |  | `5` | `5` | ✅ |
| `EasingCurve(QuadraticIn, 0..1).sample(0.5)` |  | `0.25` | `0.25` | ✅ |
| `FunctionCurve(\|t\| t^2+1 over [0,1]).sample(0.5)` |  | `1.25` | `1.25` | ✅ |
| `FunctionCurve::sample(2.0) outside interval` | t = 2.0 | `None` | `None` | ✅ |

### `demo_pipeline_runs`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `demo() — proj·view·model.project_point3` | MVP of (0.5, 0.5, 0) | `finite` | `finite` | ✅ |

### `directions_normalise_and_reject`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Dir3::new normalises` | (0,0,5) | `Vec3(0.0, 0.0, 1.0)` | `Vec3(0.0, 0.0, 1.0)` | ✅ |
| `Dir3::new rejects zero vector` | (0,0,0) | `Err` | `Err` | ✅ |
| `Dir2::new normalises` | (3,4) | `Vec2(0.6, 0.8)` | `Vec2(0.6, 0.8)` | ✅ |

### `float_ops_match_known_values` (newlib)

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `bevy_math::ops::sin` | PI/6 | `0.5` | `0.5` | ✅ |
| `bevy_math::ops::cos` | PI/6 | `0.8660254` | `0.8660254` | ✅ |
| `bevy_math::ops::tan` | PI/4 | `1` | `1` | ✅ |
| `bevy_math::ops::atan2` | (1, 1) | `0.7853982` | `0.7853982` | ✅ |
| `bevy_math::ops::sqrt` | 2 | `1.4142135` | `1.4142135` | ✅ |
| `bevy_math::ops::powf` | 2^10 | `1024` | `1024` | ✅ |
| `bevy_math::ops::exp` | 0 | `1` | `1` | ✅ |
| `bevy_math::ops::ln` | E | `1` | `0.99999994` | ✅ |
| `bevy_math::ops::cbrt` | 27 | `3` | `3` | ✅ |
| `bevy_math::ops::hypot` | (3, 4) | `5` | `5` | ✅ |
| `f32::sin` | PI/2 | `1` | `1` | ✅ |
| `f32::sqrt` | 9 | `3` | `3` | ✅ |

### `mat3_and_mat4_camera`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Mat3::from_scale_angle_translation.transform_point2(X)` | S=2, a=PI/2, T=(1,0) | `Vec2(1.0, 2.0)` | `Vec2(0.99999994, 2.0)` | ✅ |
| `Mat4::look_at_rh — eye maps to origin` | eye=(0,0,5) | `Vec3(0.0, 0.0, 0.0)` | `Vec3(0.0, 0.0, 0.0)` | ✅ |
| `Mat4::perspective_rh — near plane depth` | fov=PI/2, near=1, point z=-1 | `0` | `0` | ✅ |

### `mat4_transforms`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Mat4::from_translation.transform_point3` | T(10,0,0) * (0,0,0) | `Vec3(10.0, 0.0, 0.0)` | `Vec3(10.0, 0.0, 0.0)` | ✅ |
| `Mat4::from_translation.transform_vector3` | T(10,0,0) * vec Y | `Vec3(0.0, 1.0, 0.0)` | `Vec3(0.0, 1.0, 0.0)` | ✅ |
| `Mat4::from_rotation_z(PI/2).transform_point3(X)` |  | `Vec3(0.0, 1.0, 0.0)` | `Vec3(-4.371139e-8, 1.0, 0.0)` | ✅ |
| `Mat4::from_scale.transform_point3(ONE)` | S(2,3,4) | `Vec3(2.0, 3.0, 4.0)` | `Vec3(2.0, 3.0, 4.0)` | ✅ |
| `Mat4::from_scale_rotation_translation.transform_point3(X)` | S=2, Rz=PI/2, T=(1,1,0) | `Vec3(1.0, 3.0, 0.0)` | `Vec3(1.0000001, 3.0, 0.0)` | ✅ |
| `Mat4::inverse round-trip (M * M^-1)` | srt above | `IDENTITY` | Diagonale `0.99999994`, w-axis y `5.96e-8` (Rest 0) | ✅ |
| `Mat4::from_rotation_z.determinant` | PI/2 | `1` | `1` | ✅ |
| `Mat4::from_scale.determinant` | S(2,3,4) | `24` | `24` | ✅ |
| `Mat4 * Vec4` | T(10,0,0) * (0,0,0,1) | `Vec4(10.0, 0.0, 0.0, 1.0)` | `Vec4(10.0, 0.0, 0.0, 1.0)` | ✅ |

### `primitive_measurements`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Sphere::new(2).volume()` | r=2 | `33.510323` | `33.510323` | ✅ |
| `Sphere::new(2).area()` | r=2 | `50.265484` | `50.265484` | ✅ |
| `Cuboid::new(2,3,4).volume()` |  | `24` | `24` | ✅ |
| `Cuboid::new(2,3,4).area()` |  | `52` | `52` | ✅ |

### `quaternion_rotation`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Quat::from_axis_angle(Z, PI/2).mul_vec3(X)` |  | `Vec3(0.0, 1.0, 0.0)` | `Vec3(0.0, 0.99999994, 0.0)` | ✅ |
| `Quat::from_rotation_x(PI/2).mul_vec3(Y)` |  | `Vec3(0.0, 0.0, 1.0)` | `Vec3(0.0, 0.0, 0.99999994)` | ✅ |
| `Quat::from_rotation_z(PI/4) squared` |  | `Quat(0.0, 0.0, 0.70710677, 0.70710677)` | `Quat(0.0, 0.0, 0.7071068, 0.7071067)` | ✅ |
| `Quat::from_xyzw(1,2,3,4).normalize().length()` |  | `1` | `1` | ✅ |
| `Quat::slerp(IDENTITY, rot_z(PI/2), 0.5)` |  | `Quat(0.0, 0.0, 0.38268346, 0.9238795)` | `Quat(0.0, 0.0, 0.38268343, 0.92387956)` | ✅ |
| `Quat::from_euler -> to_euler (XYZ)` | (0.3, -0.4, 0.5) | `(0.3, -0.4, 0.5)` | `(0.30000004, -0.4, 0.5)` | ✅ |

### `rot2_isometry_ray`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Rot2::degrees(90) * Vec2::X` |  | `Vec2(0.0, 1.0)` | `Vec2(-4.371139e-8, 1.0)` | ✅ |
| `Isometry3d::new(T,Rz).transform_point(X)` | T=(1,2,3), Rz=PI/2 | `Vec3(1.0, 3.0, 3.0)` | `Vec3(1.0, 3.0, 3.0)` | ✅ |
| `Ray3d::new(0, Dir3::X).get_point(2.5)` |  | `Vec3(2.5, 0.0, 0.0)` | `Vec3(2.5, 0.0, 0.0)` | ✅ |

### `vec2_rotation_helpers`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Vec2::from_angle(a).rotate(X)` | a = PI/2 | `Vec2(0.0, 1.0)` | `Vec2(-4.371139e-8, 1.0)` | ✅ |
| `Vec2::from_angle(a).to_angle()` | a = 0.7 | `0.7` | `0.7` | ✅ |
| `Vec2::perp` | X | `Vec2(0.0, 1.0)` | `Vec2(-0.0, 1.0)` | ✅ |
| `Vec2::angle_to` | (1,1) -> (-1,1) | `1.5707964` | `1.5707963` | ✅ |

### `vec3a_layout_and_parity`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `core::mem::size_of::<Vec3A>()` |  | `16` | `16` | ✅ |
| `core::mem::align_of::<Vec3A>()` |  | `16` | `16` | ✅ |
| `Vec3A::cross == Vec3::cross` | (1.5,-2,3.25) x (-0.5,4,1) | `Vec3(-15.0, -3.125, 5.0)` | `Vec3(-15.0, -3.125, 5.0)` | ✅ |

### `vector_algebra`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Vec3::dot` | (1,2,3)·(4,5,6) | `32` | `32` | ✅ |
| `Vec3::cross` | (1,2,3)x(4,5,6) | `Vec3(-3.0, 6.0, -3.0)` | `Vec3(-3.0, 6.0, -3.0)` | ✅ |
| `Vec3::length` | (2,3,6) | `7` | `7` | ✅ |
| `Vec3::normalize().length()` | (1,2,3) | `1` | `0.99999994` | ✅ |
| `Vec3::distance` | (1,2,3)->(4,5,6) | `5.196152` | `5.196152` | ✅ |
| `Vec3::lerp` | 0 -> (10,10,10) @ t=0.25 | `Vec3(2.5, 2.5, 2.5)` | `Vec3(2.5, 2.5, 2.5)` | ✅ |
| `Vec4::length_squared` | (1,0,0,2) | `5` | `5` | ✅ |
| `Vec2 * f32` | (3,4) * 2 | `Vec2(6.0, 8.0)` | `Vec2(6.0, 8.0)` | ✅ |

## Nicht abgedeckt

Swizzles, `DVec`/`IVec`/`UVec`, `Affine2/3`, alle `EaseFunction`-Varianten, NURBS/`RationalCurve`,
`sampling` (braucht `rand`), `compass`, `FloatOrd`.
**Caveat:** nur Emulator — VFP-Rundungsmodi / Denormals / exakte NaN-Bitmuster auf echter
Hardware nicht 1:1 garantiert (für ε=1e-4 unkritisch). Siehe `../../port.md` §B9.
