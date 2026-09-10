# bevy-math-check

On-device check that **`bevy_math`** — and its `glam` 0.32 — computes correctly on the Nintendo 3DS. Aims to touch the whole public API surface.

| | |
|---|---|
| Bevy-Einheit | `bevy_math` (zieht `glam` `0.32.1`, `rand` `0.10`) |
| Version | `0.19.1` |
| Features | `default-features = false`, `features = ["std", "curve", "rand"]` (kein `bevy_reflect`; `rand` ohne default-features zieht **kein** `getrandom`) |
| Target | `armv6k-nintendo-3ds` · glam-Backend **`scalar`** (kein SSE/NEON) · Trig/`sqrt`/`exp`/`f64` aus devkitPro-newlib |
| Toleranz | ε = 1e-4 (f32) · 1e-9 (f64) |
| Stand | 2026-09-10 · Azahar-Emulator · **57/57 Testfunktionen · 488/488 Checks bestanden** |

Ausführen: `./scripts/test-emulator.sh -p bevy-math-check`

## Abgedeckt

`bevy_math::ops` (alle 32 Funktionen) · glam `Vec2/3/3A/4` `DVec2/3/4` `IVec/UVec/I64Vec/U64Vec/BVec` `Mat2/3/3A/4` `DMat4` `Quat` `DQuat` `Affine2/3A` `FloatExt` `EulerRot` · bevy_math `Dir2/3` `Rot2` `Isometry2d/3d` `Ray2d/3d` `Rect/IRect/URect` · `bounding` (`Aabb2d/3d`, `BoundingCircle/Sphere`, `BoundingVolume`, `IntersectsVolume`, `RayCast2d/3d`, `AabbCast2d`, `BoundingCircleCast`, `BoundingSphereCast`) · Primitives 2D + 3D (`Measured2d/3d`, `Bounded2d/3d`) · `curve` (`Curve` + Adaptoren, **alle 39 `EaseFunction`-Varianten**) · `cubic_splines` (Bezier/Hermite/Cardinal/BSpline/Nurbs) · `sampling` (`ShapeSample` interior/boundary, Determinismus) · `compass` · `FloatOrd` · `AspectRatio` · `StableInterpolate` · `size_of`/`align_of` jedes Typs.

## Ergebnis

Funktion · Eingabe · Erwartet · **tatsächliche Ausgabe** · OK. Reihenfolge = Testausführung. Die ε-Toleranz fängt f32/f64-Rundungen ab (z. B. `ln(E)=0.99999994`, `sin(PI/2)`-Rotation lässt `~-4.4e-8` stehen).

### `affine::affine2`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Affine2::from_scale_angle_translation.transform_point2(X)` | S=2, a=PI/2, T=(1,0) | `Vec2(1.0, 2.0)` | `Vec2(0.99999994, 2.0)` | ✅ |
| `Affine2::transform_vector2 (no translation)` | same, X | `Vec2(0.0, 2.0)` | `Vec2(-8.742278e-8, 2.0)` | ✅ |
| `Affine2::inverse round-trip` | a.inverse().transform_point2(a * X) | `Vec2(1.0, 0.0)` | `Vec2(1.0, 0.0)` | ✅ |
| `Affine2::from_translation` | (3,4) * (0,0) | `Vec2(3.0, 4.0)` | `Vec2(3.0, 4.0)` | ✅ |
| `Affine2::IDENTITY.is_finite` |  | `true` | `true` | ✅ |

### `affine::affine3a`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Affine3A::from_scale_rotation_translation.transform_point3(X)` | S=2, Rz=PI/2, T=(1,1,0) | `Vec3(1.0, 3.0, 0.0)` | `Vec3(1.0000001, 3.0, 0.0)` | ✅ |
| `Affine3A::transform_vector3` | same, vec X | `Vec3(0.0, 2.0, 0.0)` | `Vec3(1.1920929e-7, 1.9999999, 0.0)` | ✅ |
| `Affine3A::inverse round-trip` | a.inverse() ∘ a on X | `Vec3(1.0, 0.0, 0.0)` | `Vec3(0.99999994, 0.0, 0.0)` | ✅ |
| `Affine3A::to_scale_rotation_translation — scale` | a | `Vec3(2.0, 2.0, 2.0)` | `Vec3(1.9999999, 1.9999999, 2.0)` | ✅ |
| `Affine3A::to_scale_rotation_translation — translation` | a | `Vec3(1.0, 1.0, 0.0)` | `Vec3(1.0, 1.0, 0.0)` | ✅ |
| `Affine3A::to_scale_rotation_translation — rotation` | a | `true` | `true` | ✅ |
| `Affine3A::from_mat4(Mat4::from_translation)` | (5,0,0) | `Vec3(5.0, 0.0, 0.0)` | `Vec3(5.0, 0.0, 0.0)` | ✅ |

### `bounding::aabb2d_bounding_volume`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Aabb2d::center` | c=0 h=(2,1) | `Vec2(0.0, 0.0)` | `Vec2(0.0, 0.0)` | ✅ |
| `Aabb2d::half_size` | c=0 h=(2,1) | `Vec2(2.0, 1.0)` | `Vec2(2.0, 1.0)` | ✅ |
| `Aabb2d::visible_area (= area = w·h)` | h=(2,1) -> 4x2 | `8` | `8` | ✅ |
| `Aabb2d::contains (inner)` | h=(2,1) contains h=(1,0.5) | `true` | `true` | ✅ |
| `Aabb2d::merge` | h=(1,1)@0 + h=(1,1)@(4,0) | `Vec2(3.0, 1.0)` | `Vec2(3.0, 1.0)` | ✅ |
| `Aabb2d::grow` | h=(2,1) by (1,1) | `Vec2(3.0, 2.0)` | `Vec2(3.0, 2.0)` | ✅ |
| `Aabb2d::shrink` | h=(2,1) by (0.5,0.5) | `Vec2(1.5, 0.5)` | `Vec2(1.5, 0.5)` | ✅ |
| `Aabb2d::closest_point (outside)` | h=(2,1), p=(5,0) | `Vec2(2.0, 0.0)` | `Vec2(2.0, 0.0)` | ✅ |

### `bounding::aabb3d_and_sphere`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Aabb3d::center` | c=0 h=2 | `Vec3(0.0, 0.0, 0.0)` | `Vec3(0.0, 0.0, 0.0)` | ✅ |
| `Aabb3d::half_size` | c=0 h=2 | `Vec3(2.0, 2.0, 2.0)` | `Vec3(2.0, 2.0, 2.0)` | ✅ |
| `Aabb3d::merge half_size` | h=1@0 + h=1@(4,0,0) | `Vec3(3.0, 1.0, 1.0)` | `Vec3(3.0, 1.0, 1.0)` | ✅ |
| `BoundingSphere::radius` | r=3 | `3` | `3` | ✅ |
| `BoundingSphere::visible_area` | r=3 | `56.548668` | `56.548668` | ✅ |
| `BoundingSphere::contains` | r=3 contains r=1 | `true` | `true` | ✅ |

### `bounding::intersections_2d`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Aabb2d ∩ Aabb2d (overlap)` |  | `true` | `true` | ✅ |
| `Aabb2d ∩ Aabb2d (disjoint)` |  | `false` | `false` | ✅ |
| `Aabb2d ∩ BoundingCircle` |  | `true` | `true` | ✅ |
| `BoundingCircle ∩ BoundingCircle` |  | `true` | `true` | ✅ |

### `bounding::intersections_and_casts_3d`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `RayCast3d(+X) intersects Aabb3d @ x=5` |  | `true` | `true` | ✅ |
| `RayCast3d(+Y) misses Aabb3d @ x=5` |  | `false` | `false` | ✅ |
| `RayCast3d::aabb_intersection_at` | ray +X, aabb center 5 half 1 | `4` | `4` | ✅ |
| `RayCast3d::sphere_intersection_at` | ray +Y, sphere @ y=6 r=2 | `4` | `4` | ✅ |
| `Aabb3d ∩ Aabb3d` |  | `true` | `true` | ✅ |
| `BoundingSphere ∩ Aabb3d` |  | `true` | `true` | ✅ |
| `AabbCast2d::aabb_collision_at` | moving box +X toward box @ x=5 | `4` | `4` | ✅ |
| `BoundingCircleCast::circle_collision_at` | moving circle +X toward circle @ x=5 | `4` | `4` | ✅ |
| `BoundingSphereCast::sphere_collision_at` | moving sphere +Z toward sphere @ z=5 | `4` | `4` | ✅ |
| `RayCast2d::circle_intersection_at` | ray +X, circle @ x=5 r=1 | `4` | `4` | ✅ |

### `compass_ord::aspect_ratio`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `AspectRatio::try_new(16, 9).ratio()` |  | `1.7777778` | `1.7777778` | ✅ |
| `AspectRatio::try_from_pixels(1920, 1080).ratio()` |  | `1.7777778` | `1.7777778` | ✅ |
| `AspectRatio::inverse` | 16:9 -> 9:16 | `0.5625` | `0.5625` | ✅ |
| `AspectRatio::is_landscape` | 16:9 | `true` | `true` | ✅ |
| `AspectRatio::is_portrait` | 9:16 | `true` | `true` | ✅ |
| `AspectRatio::is_square` | try_new(5,5) | `true` | `true` | ✅ |
| `AspectRatio::try_new(1, 0)` | zero height | `Err` | `Err` | ✅ |

### `compass_ord::compass`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Dir2 -> CompassQuadrant (North)` | Dir2::Y | `North` | `North` | ✅ |
| `Dir2 -> CompassQuadrant (East)` | Dir2::X | `East` | `East` | ✅ |
| `CompassQuadrant::opposite` | North | `South` | `South` | ✅ |
| `CompassQuadrant -> Dir2 (North)` |  | `Vec2(0.0, 1.0)` | `Vec2(0.0, 1.0)` | ✅ |
| `CompassQuadrant::is_in_direction` | East, origin 0, candidate (5,0.1) | `true` | `true` | ✅ |
| `Dir2 -> CompassOctant (NorthEast)` | Dir2::from_xy(1,1) | `NorthEast` | `NorthEast` | ✅ |
| `CompassOctant::opposite` | NorthEast | `SouthWest` | `SouthWest` | ✅ |
| `CompassOctant -> Dir2 (East)` |  | `Vec2(1.0, 0.0)` | `Vec2(1.0, 0.0)` | ✅ |

### `compass_ord::float_ord`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `FloatOrd::cmp` | 1.0 vs 2.0 | `Less` | `Less` | ✅ |
| `FloatOrd::cmp — NaN sorts as least` | NaN vs 1e9 | `Less` | `Less` | ✅ |
| `FloatOrd::cmp — NaN == NaN` |  | `Equal` | `Equal` | ✅ |
| `FloatOrd == FloatOrd (NaN)` |  | `true` | `true` | ✅ |
| `sort [3, NaN, -1, 0.5] — NaN first` |  | `true` | `true` | ✅ |
| `...then [-1, 0.5, 3]` |  | `[-1.0, 0.5, 3.0]` | `[-1.0, 0.5, 3.0]` | ✅ |
| `FloatOrd::neg` | -FloatOrd(2.5) | `FloatOrd(-2.5)` | `FloatOrd(-2.5)` | ✅ |

### `curves::cubic_spline_generators`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `CubicSegment::position(0.5)` | bezier (0,0)(0,1)(1,1)(1,0) | `Vec2(0.5, 0.75)` | `Vec2(0.5, 0.75)` | ✅ |
| `CubicSegment::velocity(0)` | same | `Vec2(0.0, 3.0)` | `Vec2(0.0, 3.0)` | ✅ |
| `CubicSegment::acceleration(0) = 6·(P0-2P1+P2)` | same | `Vec2(6.0, -6.0)` | `Vec2(6.0, -6.0)` | ✅ |
| `CubicBezier::to_curve.position(0)` | 4 control points | `Vec2(0.0, 0.0)` | `Vec2(0.0, 0.0)` | ✅ |
| `CubicBezier::to_curve.position(1)` | same | `Vec2(4.0, 0.0)` | `Vec2(4.0, 0.0)` | ✅ |
| `CubicHermite::to_curve endpoints` | p0=0 p1=(2,0) | `Vec2(0.0, 0.0)` | `Vec2(0.0, 0.0)` | ✅ |
| `CubicCardinalSpline::to_curve finite` |  | `true` | `true` | ✅ |
| `CubicBSpline::to_curve finite` |  | `true` | `true` | ✅ |
| `CubicNurbs::to_curve finite (uniform weights)` |  | `true` | `true` | ✅ |

### `curves::curve_trait_and_adaptors`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Curve::sample(0.5) on FunctionCurve` | |t| t*10 over [0,1] | `5` | `5` | ✅ |
| `Curve::sample(2.0) outside domain` |  | `None` | `None` | ✅ |
| `Curve::sample_clamped(2.0)` | clamps to 1.0 | `10` | `10` | ✅ |
| `Curve::map` | (|t| t*10).map(|v| v*2) @ 0.5 | `10` | `10` | ✅ |
| `Curve::reverse` | (|t| t).reverse() @ 0.25 | `0.75` | `0.75` | ✅ |
| `Curve::repeat(1) domain length` |  | `2` | `2` | ✅ |
| `Curve::ping_pong @ 1.5 (going back)` |  | `0.5` | `0.5` | ✅ |
| `Curve::reparametrize_linear to [0,10] @ 5` |  | `0.5` | `0.5` | ✅ |
| `Curve::samples(5)` | |t| t*4 over [0,1] | `[0.0, 1.0, 2.0, 3.0, 4.0]` | `[0.0, 1.0, 2.0, 3.0, 4.0]` | ✅ |
| `Interval::UNIT bounds` |  | `(0.0, 1.0)` | `(0.0, 1.0)` | ✅ |
| `Interval::length` | [2,7] | `5` | `5` | ✅ |

### `curves::ease_functions_all_variants`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `EaseFunction::Linear` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.5000 / 1.0000` | ✅ |
| `EaseFunction::QuadraticIn` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.2500 / 1.0000` | ✅ |
| `EaseFunction::QuadraticOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.7500 / 1.0000` | ✅ |
| `EaseFunction::QuadraticInOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.5000 / 1.0000` | ✅ |
| `EaseFunction::CubicIn` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.1250 / 1.0000` | ✅ |
| `EaseFunction::CubicOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.8750 / 1.0000` | ✅ |
| `EaseFunction::CubicInOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.5000 / 1.0000` | ✅ |
| `EaseFunction::QuarticIn` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.0625 / 1.0000` | ✅ |
| `EaseFunction::QuarticOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.9375 / 1.0000` | ✅ |
| `EaseFunction::QuarticInOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.5000 / 1.0000` | ✅ |
| `EaseFunction::QuinticIn` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.0312 / 1.0000` | ✅ |
| `EaseFunction::QuinticOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.9688 / 1.0000` | ✅ |
| `EaseFunction::QuinticInOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.5000 / 1.0000` | ✅ |
| `EaseFunction::SmoothStepIn` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.3125 / 1.0000` | ✅ |
| `EaseFunction::SmoothStepOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.6875 / 1.0000` | ✅ |
| `EaseFunction::SmoothStep` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.5000 / 1.0000` | ✅ |
| `EaseFunction::SmootherStepIn` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.2070 / 1.0000` | ✅ |
| `EaseFunction::SmootherStepOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.7930 / 1.0000` | ✅ |
| `EaseFunction::SmootherStep` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.5000 / 1.0000` | ✅ |
| `EaseFunction::SineIn` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.2929 / 1.0000` | ✅ |
| `EaseFunction::SineOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.7071 / 1.0000` | ✅ |
| `EaseFunction::SineInOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.5000 / 1.0000` | ✅ |
| `EaseFunction::CircularIn` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.1340 / 1.0000` | ✅ |
| `EaseFunction::CircularOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.8660 / 1.0000` | ✅ |
| `EaseFunction::CircularInOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.5000 / 1.0000` | ✅ |
| `EaseFunction::ExponentialIn` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.0303 / 1.0000` | ✅ |
| `EaseFunction::ExponentialOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.9697 / 1.0000` | ✅ |
| `EaseFunction::ExponentialInOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.5000 / 1.0000` | ✅ |
| `EaseFunction::ElasticIn` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `-0.0005 / -0.0156 / 1.0000` | ✅ |
| `EaseFunction::ElasticOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 1.0156 / 1.0005` | ✅ |
| `EaseFunction::ElasticInOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0001 / 0.5000 / 0.9999` | ✅ |
| `EaseFunction::BackIn` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / -0.0877 / 1.0000` | ✅ |
| `EaseFunction::BackOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 1.0877 / 1.0000` | ✅ |
| `EaseFunction::BackInOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.5000 / 1.0000` | ✅ |
| `EaseFunction::BounceIn` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.2812 / 1.0000` | ✅ |
| `EaseFunction::BounceOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.7188 / 1.0000` | ✅ |
| `EaseFunction::BounceInOut` | sample @ 0.0 / 0.5 / 1.0 | `0.0 / finite / 1.0` | `0.0000 / 0.5000 / 1.0000` | ✅ |
| `EaseFunction::Linear @ 0.5` |  | `0.5` | `0.5` | ✅ |
| `EaseFunction::QuadraticIn @ 0.5` |  | `0.25` | `0.25` | ✅ |
| `EaseFunction::SmoothStep @ 0.5` |  | `0.5` | `0.5` | ✅ |

### `directions::dir2`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Dir2::new normalises` | (3,4) | `Vec2(0.6, 0.8)` | `Vec2(0.6, 0.8)` | ✅ |
| `Dir2::new rejects zero` | (0,0) | `Err` | `Err` | ✅ |
| `Dir2::new rejects NaN` | (NaN,0) | `Err` | `Err` | ✅ |
| `Dir2::from_angle(PI/2)` |  | `Vec2(0.0, 1.0)` | `Vec2(-4.371139e-8, 1.0)` | ✅ |
| `Dir2::slerp(X, Y, 0.5)` |  | `Vec2(0.70710677, 0.70710677)` | `Vec2(0.7071068, 0.70710677)` | ✅ |
| `Dir2::rotation_to(X, Y).as_radians()` |  | `1.5707964` | `1.5707964` | ✅ |
| `-Dir2 (Neg)` | -X | `Vec2(-1.0, 0.0)` | `Vec2(-1.0, -0.0)` | ✅ |

### `directions::dir3`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Dir3::new normalises` | (0,0,5) | `Vec3(0.0, 0.0, 1.0)` | `Vec3(0.0, 0.0, 1.0)` | ✅ |
| `Dir3::new rejects zero` | (0,0,0) | `Err` | `Err` | ✅ |
| `Dir3::new_unchecked` | (1,0,0) | `Vec3(1.0, 0.0, 0.0)` | `Vec3(1.0, 0.0, 0.0)` | ✅ |
| `Dir3::new_and_length — length` | (0,3,4) | `5` | `5` | ✅ |
| `Dir3::new_and_length — dir` | (0,3,4) | `Vec3(0.0, 0.6, 0.8)` | `Vec3(0.0, 0.6, 0.8)` | ✅ |
| `Dir3::slerp(X, Y, 0.5)` |  | `Vec3(0.70710677, 0.70710677, 0.0)` | `Vec3(0.7071069, 0.70710677, 0.0)` | ✅ |
| `Dir3::fast_renormalize` | (1.001,0,0) | `Vec3(1.0, 0.0, 0.0)` | `Vec3(0.9999985, 0.0, 0.0)` | ✅ |
| `Quat::from_rotation_arc(X, Z) * X ≈ Z` |  | `Vec3(0.0, 0.0, 1.0)` | `Vec3(0.0, 0.0, 0.99999994)` | ✅ |

### `directions::rot2`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Rot2::degrees(90) * X` |  | `Vec2(0.0, 1.0)` | `Vec2(-4.371139e-8, 1.0)` | ✅ |
| `Rot2::radians(PI/2) * X` |  | `Vec2(0.0, 1.0)` | `Vec2(-4.371139e-8, 1.0)` | ✅ |
| `Rot2::degrees(90).as_radians()` |  | `1.5707964` | `1.5707964` | ✅ |
| `Rot2::radians(PI/2).as_degrees()` |  | `90` | `90` | ✅ |
| `Rot2::radians(PI/2).as_turn_fraction()` |  | `0.25` | `0.25` | ✅ |
| `Rot2 * Rot2 (compose)` | 45° * 45° | `Rot2 { cos: -4.371139e-8, sin: 1.0 }` | `Rot2 { cos: 0.0, sin: 0.99999994 }` | ✅ |
| `Rot2::inverse` | 90°.inverse() * Y | `Vec2(1.0, 0.0)` | `Vec2(1.0, -4.371139e-8)` | ✅ |
| `Rot2::angle_to` | 0° -> 90° | `1.5707964` | `1.5707964` | ✅ |
| `Rot2::slerp(0°, 90°, 0.5)` |  | `Rot2 { cos: 0.70710677, sin: 0.70710677 }` | `Rot2 { cos: 0.70710677, sin: 0.70710677 }` | ✅ |
| `Rot2::nlerp(0°, 90°, 0.5)` |  | `Rot2 { cos: 0.70710677, sin: 0.70710677 }` | `Rot2 { cos: 0.70710677, sin: 0.70710677 }` | ✅ |
| `Rot2::is_normalized` | from_sin_cos(sin,cos of 30°) | `true` | `true` | ✅ |

### `dvec::dquat_and_dmat`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `DQuat::from_rotation_z(PI/2).mul_vec3(X).y` |  | `1` | `1` | ✅ |
| `DQuat::length after normalize` | from_xyzw(1,2,3,4) | `1` | `0.9999999999999999` | ✅ |
| `DMat4::from_translation.transform_point3.x` | T(10,0,0)*(0,0,0) | `10` | `10` | ✅ |
| `DMat4::from_scale.determinant` | S(2,3,4) | `24` | `24` | ✅ |
| `DMat4::inverse round-trip diagonal` | srt | `1` | `1` | ✅ |

### `dvec::dvec_math`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `DVec2::dot` | (1,2)·(3,4) | `11` | `11` | ✅ |
| `DVec3::length` | (2,3,6) | `7` | `7` | ✅ |
| `DVec3::normalize().length()` | (1,2,3) | `1` | `1` | ✅ |
| `DVec3::cross.x` | (1,0,0)x(0,1,0) | `0` | `0` | ✅ |
| `DVec3::cross.z` | (1,0,0)x(0,1,0) | `1` | `1` | ✅ |
| `DVec4::length` | (0,0,3,4) | `5` | `5` | ✅ |
| `DVec2::distance` | (0,0)->(3,4) | `5` | `5` | ✅ |
| `DVec3 -> Vec3 (as_vec3)` | (1.5,-2,3) | `Vec3(1.5, -2.0, 3.0)` | `Vec3(1.5, -2.0, 3.0)` | ✅ |

### `float_ext::bevy_math_common_traits`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `VectorSpace::lerp on Vec3` | 0..(4,4,4) @ 0.5 | `Vec3(2.0, 2.0, 2.0)` | `Vec3(2.0, 2.0, 2.0)` | ✅ |
| `ScalarField::recip on f32` | 4 | `0.25` | `0.25` | ✅ |
| `NormedVectorSpace::norm on Vec3` | (2,3,6) | `7` | `7` | ✅ |
| `NormedVectorSpace::distance on Vec3` | (0,0,0)->(0,3,4) | `5` | `5` | ✅ |

### `float_ext::glam_float_ext`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `f32::lerp (FloatExt)` | 0..10 @ 0.25 | `2.5` | `2.5` | ✅ |
| `f32::inverse_lerp` | 0..10, v=2.5 | `0.25` | `0.25` | ✅ |
| `f32::remap` | [0,1]->[10,20] @ 0.5 | `15` | `15` | ✅ |

### `ivec::bvec`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `BVec3: size_of` |  | `3` | `3` | ✅ |
| `IVec3::cmplt -> BVec3` | (1,5,3) < (4,2,6) | `BVec3(0xffffffff, 0x0, 0xffffffff)` | `BVec3(0xffffffff, 0x0, 0xffffffff)` | ✅ |
| `BVec3::any` | (true,false,true) | `true` | `true` | ✅ |
| `BVec3::all` | (true,false,true) | `false` | `false` | ✅ |
| `BVec3::bitmask` | (true,false,true) | `5` | `5` | ✅ |
| `Vec3::select via mask` | mask(t,f,t) a=(1,1,1) b=(9,9,9) | `Vec3(1.0, 9.0, 1.0)` | `Vec3(1.0, 9.0, 1.0)` | ✅ |

### `ivec::ivec_math`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `IVec2 + IVec2` | (1,2)+(3,4) | `IVec2(4, 6)` | `IVec2(4, 6)` | ✅ |
| `IVec3::dot` | (1,2,3)·(4,5,6) | `32` | `32` | ✅ |
| `IVec3::cross` | X x Y | `IVec3(0, 0, 1)` | `IVec3(0, 0, 1)` | ✅ |
| `IVec3::abs` | (-1,2,-3) | `IVec3(1, 2, 3)` | `IVec3(1, 2, 3)` | ✅ |
| `IVec3::min` | (1,5,3),(4,2,6) | `IVec3(1, 2, 3)` | `IVec3(1, 2, 3)` | ✅ |
| `IVec3::max` | (1,5,3),(4,2,6) | `IVec3(4, 5, 6)` | `IVec3(4, 5, 6)` | ✅ |
| `IVec3 -> Vec3 (as_vec3)` | (1,2,3) | `Vec3(1.0, 2.0, 3.0)` | `Vec3(1.0, 2.0, 3.0)` | ✅ |
| `Vec3 -> IVec3 (as_ivec3, truncating)` | (1.9,-2.9,3.1) | `IVec3(1, -2, 3)` | `IVec3(1, -2, 3)` | ✅ |

### `ivec::uvec_i64_u64`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `UVec3 + UVec3` | (1,2,3)+(4,5,6) | `UVec3(5, 7, 9)` | `UVec3(5, 7, 9)` | ✅ |
| `UVec3::element_sum` | (1,2,3) | `6` | `6` | ✅ |
| `I64Vec3::dot` | (1,2,3)·(4,5,6) | `32` | `32` | ✅ |
| `U64Vec3 max_element` | (3,9,1) | `9` | `9` | ✅ |

### `layout::bevy_math_type_layout`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `size/align Dir2` |  | `size / align` | `8 / 4` | ✅ |
| `size/align Dir3` |  | `size / align` | `12 / 4` | ✅ |
| `size/align Dir3A` |  | `size / align` | `16 / 16` | ✅ |
| `size/align Rot2` |  | `size / align` | `8 / 4` | ✅ |
| `size/align Isometry2d` |  | `size / align` | `16 / 4` | ✅ |
| `size/align Isometry3d` |  | `size / align` | `32 / 16` | ✅ |
| `size/align Ray2d` |  | `size / align` | `16 / 4` | ✅ |
| `size/align Ray3d` |  | `size / align` | `24 / 4` | ✅ |
| `size/align Circle` |  | `size / align` | `4 / 4` | ✅ |
| `size/align Sphere` |  | `size / align` | `4 / 4` | ✅ |
| `size/align Cuboid` |  | `size / align` | `12 / 4` | ✅ |
| `size/align Aabb2d` |  | `size / align` | `16 / 4` | ✅ |
| `size/align Aabb3d` |  | `size / align` | `32 / 16` | ✅ |
| `size/align BoundingCircle` |  | `size / align` | `12 / 4` | ✅ |
| `size/align BoundingSphere` |  | `size / align` | `32 / 16` | ✅ |

### `layout::glam_type_layout`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `size/align Vec2` |  | `size / align` | `8 / 4` | ✅ |
| `size/align Vec3` |  | `size / align` | `12 / 4` | ✅ |
| `size/align Vec3A` |  | `size / align` | `16 / 16` | ✅ |
| `size/align Vec4` |  | `size / align` | `16 / 16` | ✅ |
| `size/align IVec2` |  | `size / align` | `8 / 4` | ✅ |
| `size/align IVec3` |  | `size / align` | `12 / 4` | ✅ |
| `size/align IVec4` |  | `size / align` | `16 / 4` | ✅ |
| `size/align UVec3` |  | `size / align` | `12 / 4` | ✅ |
| `size/align DVec2` |  | `size / align` | `16 / 8` | ✅ |
| `size/align DVec3` |  | `size / align` | `24 / 8` | ✅ |
| `size/align DVec4` |  | `size / align` | `32 / 8` | ✅ |
| `size/align Mat2` |  | `size / align` | `16 / 16` | ✅ |
| `size/align Mat3` |  | `size / align` | `36 / 4` | ✅ |
| `size/align Mat3A` |  | `size / align` | `48 / 16` | ✅ |
| `size/align Mat4` |  | `size / align` | `64 / 16` | ✅ |
| `size/align DMat4` |  | `size / align` | `128 / 8` | ✅ |
| `size/align Quat` |  | `size / align` | `16 / 16` | ✅ |
| `size/align DQuat` |  | `size / align` | `32 / 8` | ✅ |
| `size/align Affine2` |  | `size / align` | `32 / 16` | ✅ |
| `size/align Affine3A` |  | `size / align` | `64 / 16` | ✅ |

### `mat::mat2`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Mat2::from_angle(PI/2) * X` |  | `Vec2(0.0, 1.0)` | `Vec2(-4.371139e-8, 1.0)` | ✅ |
| `Mat2::from_angle(PI/2).determinant()` |  | `1` | `1` | ✅ |
| `Mat2::inverse round-trip` | from_angle(PI/2) | `Mat2 { x_axis: Vec2(1.0, 0.0), y_axis: Vec2(0.0, 1.0) }` | `Mat2 { x_axis: Vec2(1.0, 0.0), y_axis: Vec2(0.0, 1.0) }` | ✅ |
| `Mat2::from_scale_angle * X` | S=(2,3), a=0 | `Vec2(2.0, 0.0)` | `Vec2(2.0, 0.0)` | ✅ |
| `Mat2::from_diagonal` | (2,3) * (1,1) | `Vec2(2.0, 3.0)` | `Vec2(2.0, 3.0)` | ✅ |
| `Mat2::transpose` | cols->rows | `Mat2 { x_axis: Vec2(1.0, 3.0), y_axis: Vec2(2.0, 4.0) }` | `Mat2 { x_axis: Vec2(1.0, 3.0), y_axis: Vec2(2.0, 4.0) }` | ✅ |

### `mat::mat3`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Mat3::from_scale_angle_translation.transform_point2(X)` | S=2, a=PI/2, T=(1,0) | `Vec2(1.0, 2.0)` | `Vec2(0.99999994, 2.0)` | ✅ |
| `Mat3::transform_vector2 (ignores translation)` | same, vec X | `Vec2(0.0, 2.0)` | `Vec2(-8.742278e-8, 2.0)` | ✅ |
| `Mat3::from_rotation_z(PI/2) * X (as Vec3)` |  | `Vec3(0.0, 1.0, 0.0)` | `Vec3(-4.371139e-8, 1.0, 0.0)` | ✅ |
| `Mat3::from_quat(rot_z(PI/2)) * X` |  | `Vec3(0.0, 1.0, 0.0)` | `Vec3(5.9604645e-8, 0.99999994, 0.0)` | ✅ |
| `Mat3::from_scale((2,3)).determinant()` |  | `6` | `6` | ✅ |
| `Mat3::inverse round-trip` | scale+rot | `Mat3 { x_axis: Vec3(1.0, 0.0, 0.0), y_axis: Vec3(0.0, 1.0, 0.0), z_axis: Vec3(0.0, 0.0, 1.0) }` | `Mat3 { x_axis: Vec3(1.0, 0.0, 0.0), y_axis: Vec3(0.0, 1.0, 0.0), z_axis: Vec3(0.0, 0.0, 1.0) }` | ✅ |
| `Mat3::transpose` | rot_z(PI/2) | `Mat3 { x_axis: Vec3(-4.371139e-8, -1.0, 0.0), y_axis: Vec3(1.0, -4.371139e-8, 0.0), z_axis: Vec3(0.0, 0.0, 1.0) }` | `Mat3 { x_axis: Vec3(-4.371139e-8, -1.0, 0.0), y_axis: Vec3(1.0, -4.371139e-8, 0.0), z_axis: Vec3(0.0, 0.0, 1.0) }` | ✅ |
| `Mat3::col(0)` | from_cols e_i | `Vec3(1.0, 0.0, 0.0)` | `Vec3(1.0, 0.0, 0.0)` | ✅ |

### `mat::mat3a`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Mat3A::from_rotation_z(PI/2) * X` |  | `Vec3(0.0, 1.0, 0.0)` | `Vec3(-4.371139e-8, 1.0, 0.0)` | ✅ |
| `Mat3A::from_scale((2,3)).determinant()` |  | `6` | `6` | ✅ |
| `Mat3A: size_of` |  | `48` | `48` | ✅ |

### `mat::mat4_algebra_and_camera`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Mat4::determinant` | S(2,3,4) | `24` | `24` | ✅ |
| `Mat4::from_rotation_z.determinant` | PI/2 | `1` | `1` | ✅ |
| `Mat4::inverse round-trip (M*M^-1)` | srt | `Mat4 { x_axis: Vec4(1.0, 0.0, 0.0, 0.0), y_axis: Vec4(0.0, 1.0, 0.0, 0.0), z_axis: Vec4(0.0, 0.0, 1.0, 0.0), w_axis: Vec4(0.0, 0.0, 0.0, 1.0) }` | `Mat4 { x_axis: Vec4(0.99999994, 0.0, 0.0, 0.0), y_axis: Vec4(0.0, 0.99999994, 0.0, 0.0), z_axis: Vec4(0.0, 0.0, 1.0, 0.0), w_axis: Vec4(0.0, 5.9604645e-8, 0.0, 1.0) }` | ✅ |
| `Mat4::transpose` | rot_z(PI/2) | `Mat4 { x_axis: Vec4(-4.371139e-8, -1.0, 0.0, 0.0), y_axis: Vec4(1.0, -4.371139e-8, 0.0, 0.0), z_axis: Vec4(0.0, 0.0, 1.0, 0.0), w_axis: Vec4(0.0, 0.0, 0.0, 1.0) }` | `Mat4 { x_axis: Vec4(-4.371139e-8, -1.0, 0.0, 0.0), y_axis: Vec4(1.0, -4.371139e-8, 0.0, 0.0), z_axis: Vec4(0.0, 0.0, 1.0, 0.0), w_axis: Vec4(0.0, 0.0, 0.0, 1.0) }` | ✅ |
| `Mat4 * Vec4` | T(10,0,0) * (0,0,0,1) | `Vec4(10.0, 0.0, 0.0, 1.0)` | `Vec4(10.0, 0.0, 0.0, 1.0)` | ✅ |
| `Mat4::to_scale_rotation_translation — scale` | srt | `Vec3(2.0, 2.0, 2.0)` | `Vec3(1.9999999, 1.9999999, 2.0)` | ✅ |
| `Mat4::to_scale_rotation_translation — translation` | srt | `Vec3(1.0, 1.0, 1.0)` | `Vec3(1.0, 1.0, 1.0)` | ✅ |
| `Mat4::to_scale_rotation_translation — rotation ≈ rot_z(PI/2)` | srt | `true` | `true` | ✅ |
| `Mat4::look_at_rh — eye -> origin` | eye=(0,0,5) | `Vec3(0.0, 0.0, 0.0)` | `Vec3(0.0, 0.0, 0.0)` | ✅ |
| `Mat4::perspective_rh — near plane -> z≈0` | fov=PI/2, near=1, z=-1 | `0` | `0` | ✅ |
| `Mat4::orthographic_rh center (z -5, [0,1] depth)` | near=0 far=10 | `Vec3(0.0, 0.0, 0.5)` | `Vec3(0.0, 0.0, 0.5)` | ✅ |

### `mat::mat4_constructors_and_transforms`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Mat4::from_translation.transform_point3` | T(10,0,0)*(0,0,0) | `Vec3(10.0, 0.0, 0.0)` | `Vec3(10.0, 0.0, 0.0)` | ✅ |
| `Mat4::from_translation.transform_vector3` | T(10,0,0)*vec Y | `Vec3(0.0, 1.0, 0.0)` | `Vec3(0.0, 1.0, 0.0)` | ✅ |
| `Mat4::from_rotation_z(PI/2).transform_point3(X)` |  | `Vec3(0.0, 1.0, 0.0)` | `Vec3(-4.371139e-8, 1.0, 0.0)` | ✅ |
| `Mat4::from_axis_angle(Z, PI/2).transform_point3(X)` |  | `Vec3(0.0, 1.0, 0.0)` | `Vec3(-4.371139e-8, 1.0, 0.0)` | ✅ |
| `Mat4::from_scale.transform_point3(ONE)` | S(2,3,4) | `Vec3(2.0, 3.0, 4.0)` | `Vec3(2.0, 3.0, 4.0)` | ✅ |
| `Mat4::from_quat(rot_y(PI/2)).transform_point3(Z)` |  | `Vec3(1.0, 0.0, 0.0)` | `Vec3(0.99999994, 0.0, 5.9604645e-8)` | ✅ |
| `Mat4::from_scale_rotation_translation.transform_point3(X)` | S=2,Rz=PI/2,T=(1,1,0) | `Vec3(1.0, 3.0, 0.0)` | `Vec3(1.0000001, 3.0, 0.0)` | ✅ |

### `ops_mod::ops_exp_log`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `ops::exp` | 0 | `1` | `1` | ✅ |
| `ops::exp` | 1 | `2.7182817` | `2.7182817` | ✅ |
| `ops::exp2` | 10 | `1024` | `1024` | ✅ |
| `ops::exp_m1` | 0 | `0` | `0` | ✅ |
| `ops::ln` | E | `1` | `0.99999994` | ✅ |
| `ops::ln_1p` | 0 | `0` | `0` | ✅ |
| `ops::log2` | 8 | `3` | `3` | ✅ |
| `ops::log10` | 1000 | `3` | `3` | ✅ |
| `ops::ln` | 2 | `0.6931472` | `0.6931472` | ✅ |

### `ops_mod::ops_hyperbolic`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `ops::sinh` | 0 | `0` | `0` | ✅ |
| `ops::cosh` | 0 | `1` | `1` | ✅ |
| `ops::tanh` | 0 | `0` | `0` | ✅ |
| `ops::sinh` | 1 | `1.1752012` | `1.1752012` | ✅ |
| `ops::cosh` | 1 | `1.5430806` | `1.5430807` | ✅ |
| `ops::asinh` | sinh(1) | `1` | `1` | ✅ |
| `ops::acosh` | cosh(1) | `1` | `1` | ✅ |
| `ops::atanh` | tanh(0.5) | `0.5` | `0.5` | ✅ |

### `ops_mod::ops_powers_roots`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `ops::powf` | 2^10 | `1024` | `1024` | ✅ |
| `ops::powf` | 9^0.5 | `3` | `3` | ✅ |
| `ops::sqrt` | 2 | `1.4142135` | `1.4142135` | ✅ |
| `ops::sqrt` | 144 | `12` | `12` | ✅ |
| `ops::cbrt` | 27 | `3` | `3` | ✅ |
| `ops::cbrt` | -8 | `-2` | `-2` | ✅ |
| `ops::hypot` | (3, 4) | `5` | `5` | ✅ |

### `ops_mod::ops_rounding_and_misc`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `ops::abs` | -3.5 | `3.5` | `3.5` | ✅ |
| `ops::floor` | 2.7 | `2` | `2` | ✅ |
| `ops::floor` | -2.1 | `-3` | `-3` | ✅ |
| `ops::ceil` | 2.1 | `3` | `3` | ✅ |
| `ops::round` | 2.5 | `3` | `3` | ✅ |
| `ops::round` | -2.5 | `-3` | `-3` | ✅ |
| `ops::fract` | 2.75 | `0.75` | `0.75` | ✅ |
| `ops::copysign` | (3, -1) | `-3` | `-3` | ✅ |
| `ops::rem_euclid` | (-1, 3) | `2` | `2` | ✅ |

### `ops_mod::ops_trigonometric`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `ops::sin` | PI/6 | `0.5` | `0.5` | ✅ |
| `ops::cos` | PI/3 | `0.5` | `0.49999997` | ✅ |
| `ops::tan` | PI/4 | `1` | `1` | ✅ |
| `ops::sin_cos.0` | PI/6 | `0.5` | `0.5` | ✅ |
| `ops::sin_cos.1` | PI/6 | `0.8660254` | `0.8660254` | ✅ |
| `ops::asin` | 0.5 | `0.5235988` | `0.5235988` | ✅ |
| `ops::acos` | 0.5 | `1.0471976` | `1.0471976` | ✅ |
| `ops::atan` | 1 | `0.7853982` | `0.7853982` | ✅ |
| `ops::atan2` | (1, 1) | `0.7853982` | `0.7853982` | ✅ |
| `ops::atan2` | (1, -1) | `2.3561945` | `2.3561945` | ✅ |

### `primitives2d::bounded_2d`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Circle::new(3).aabb_2d — half_size` | identity | `Vec2(3.0, 3.0)` | `Vec2(3.0, 3.0)` | ✅ |
| `Rectangle::new(4,2).aabb_2d — half_size` | identity | `Vec2(2.0, 1.0)` | `Vec2(2.0, 1.0)` | ✅ |
| `Circle::new(3).bounding_circle — radius` | identity | `3` | `3` | ✅ |
| `Triangle2d.aabb_2d — half_size` | (-1,0)(1,0)(0,2) | `Vec2(1.0, 1.0)` | `Vec2(1.0, 1.0)` | ✅ |
| `Triangle2d.aabb_2d — center` | same | `Vec2(0.0, 1.0)` | `Vec2(0.0, 1.0)` | ✅ |

### `primitives2d::measured_2d`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Circle::new(2).area()` | r=2 | `12.566371` | `12.566371` | ✅ |
| `Circle::new(2).perimeter()` | r=2 | `12.566371` | `12.566371` | ✅ |
| `Ellipse::new(3, 2).area()` | a=3 b=2 | `18.849556` | `18.849556` | ✅ |
| `Rectangle::new(4, 2).area()` |  | `8` | `8` | ✅ |
| `Rectangle::new(4, 2).perimeter()` |  | `12` | `12` | ✅ |
| `Rhombus::new(4, 2).area()` | diagonals 4,2 | `4` | `4` | ✅ |
| `Triangle2d area (3-4-5)` | (0,0)(3,0)(0,4) | `6` | `6` | ✅ |
| `Triangle2d perimeter (3-4-5)` |  | `12` | `12` | ✅ |
| `Annulus::new(1, 2).area()` | π(4-1) | `9.424778` | `9.424778` | ✅ |
| `RegularPolygon::new(1, 4).area()` | square, circumradius 1 | `2` | `2` | ✅ |
| `CircularSector::from_turns quarter area` | r=2, quarter | `3.1415927` | `3.1415927` | ✅ |
| `Capsule2d::new(1, 2).area()` | r=1, len=2 | `7.141593` | `7.141593` | ✅ |

### `primitives2d::primitive_2d_helpers`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Circle::diameter` | r=2.5 | `5` | `5` | ✅ |
| `Circle::closest_point (inside -> unchanged)` | r=2, point (1,0) | `Vec2(1.0, 0.0)` | `Vec2(1.0, 0.0)` | ✅ |
| `RegularPolygon::circumradius` | new(2, 6) | `2` | `2` | ✅ |
| `RegularPolygon::sides` | new(2, 6) | `6` | `6` | ✅ |

### `primitives3d::bounded_3d`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Sphere::new(3).aabb_3d — half_size` | identity | `Vec3(3.0, 3.0, 3.0)` | `Vec3(3.0, 3.0, 3.0)` | ✅ |
| `Cuboid::new(4,2,6).aabb_3d — half_size` | identity | `Vec3(2.0, 1.0, 3.0)` | `Vec3(2.0, 1.0, 3.0)` | ✅ |
| `Sphere::new(3).bounding_sphere — radius` | identity | `3` | `3` | ✅ |
| `Cuboid::new(2,2,2).bounding_sphere — radius` | half-diagonal | `1.7320508` | `1.7320508` | ✅ |

### `primitives3d::measured_3d`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Sphere::new(2).volume()` | r=2 | `33.510323` | `33.510323` | ✅ |
| `Sphere::new(2).area()` | r=2 | `50.265484` | `50.265484` | ✅ |
| `Cuboid::new(2, 3, 4).volume()` |  | `24` | `24` | ✅ |
| `Cuboid::new(2, 3, 4).area()` |  | `52` | `52` | ✅ |
| `Cylinder::new(2, 4).volume()` | r=2 h=4 | `50.265484` | `50.265484` | ✅ |
| `Cylinder::new(2, 4).area()` | r=2 h=4 | `75.398224` | `75.398224` | ✅ |
| `Cone::new(2, 3).volume()` | r=2 h=3 | `12.566371` | `12.566371` | ✅ |
| `Capsule3d::new(1, 2).volume()` | r=1 len=2 | `10.471975` | `10.471976` | ✅ |
| `Torus::new(1, 3).volume()` | inner=1 outer=3 -> minor=1 major=2 | `39.47842` | `39.47842` | ✅ |
| `Tetrahedron unit volume` | (0,0,0)(1,0,0)(0,1,0)(0,0,1) | `0.16666667` | `0.16666667` | ✅ |

### `quat::quat_constructors`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Quat::from_axis_angle(Z, PI/2) * X` |  | `Vec3(0.0, 1.0, 0.0)` | `Vec3(0.0, 0.99999994, 0.0)` | ✅ |
| `Quat::from_rotation_x(PI/2) * Y` |  | `Vec3(0.0, 0.0, 1.0)` | `Vec3(0.0, 0.0, 0.99999994)` | ✅ |
| `Quat::from_rotation_y(PI/2) * Z` |  | `Vec3(1.0, 0.0, 0.0)` | `Vec3(0.99999994, 0.0, 0.0)` | ✅ |
| `Quat::from_scaled_axis(Z*PI/2) * X` |  | `Vec3(0.0, 1.0, 0.0)` | `Vec3(0.0, 0.99999994, 0.0)` | ✅ |
| `Quat::from_mat3(Mat3::from_rotation_z(PI/2))` |  | `Quat(0.0, 0.0, 0.70710677, 0.70710677)` | `Quat(0.0, 0.0, 0.70710677, 0.70710677)` | ✅ |
| `Quat::from_mat4(Mat4::from_rotation_z(PI/2))` |  | `Quat(0.0, 0.0, 0.70710677, 0.70710677)` | `Quat(0.0, 0.0, 0.70710677, 0.70710677)` | ✅ |
| `Quat::from_rotation_arc(X, Y) * X` |  | `Vec3(0.0, 1.0, 0.0)` | `Vec3(0.0, 0.99999994, 0.0)` | ✅ |
| `Quat::from_xyzw / from_array round-trip` | (0,0,0,1) | `Quat(0.0, 0.0, 0.0, 1.0)` | `Quat(0.0, 0.0, 0.0, 1.0)` | ✅ |

### `quat::quat_interpolation_and_decompose`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Quat::slerp(IDENTITY, rot_z(PI/2), 0.5)` |  | `Quat(0.0, 0.0, 0.38268346, 0.9238795)` | `Quat(0.0, 0.0, 0.38268343, 0.92387956)` | ✅ |
| `Quat::lerp(IDENTITY, rot_z(PI/2), 0.5).normalize()` |  | `Quat(0.0, 0.0, 0.38268346, 0.9238795)` | `Quat(0.0, 0.0, 0.38268343, 0.92387956)` | ✅ |
| `Quat::angle_between` | IDENTITY, rot_z(PI/2) | `1.5707964` | `1.5707964` | ✅ |
| `Quat::to_axis_angle — axis` | from_axis_angle(Z, 1.0) | `Vec3(0.0, 0.0, 1.0)` | `Vec3(0.0, 0.0, 1.0)` | ✅ |
| `Quat::to_axis_angle — angle` | from_axis_angle(Z, 1.0) | `1` | `1` | ✅ |
| `Quat::to_scaled_axis` | from_scaled_axis(Z*1.2) | `Vec3(0.0, 0.0, 1.2)` | `Vec3(0.0, 0.0, 1.2)` | ✅ |
| `Quat::from_euler XYZ -> to_euler XYZ` | (0.3,-0.4,0.5) | `(0.3, -0.4, 0.5)` | `(0.30000004, -0.4, 0.5)` | ✅ |
| `Quat::from_euler ZYX -> to_euler ZYX` | (0.2,0.1,-0.3) | `(0.2, 0.1, -0.3)` | `(0.20000002, 0.099999994, -0.3)` | ✅ |
| `Quat::xyz of rot_z(PI/2)` | ≈(0,0,0.707) | `true` | `true` | ✅ |
| `Quat::to_array` | IDENTITY | `[0.0, 0.0, 0.0, 1.0]` | `[0.0, 0.0, 0.0, 1.0]` | ✅ |

### `quat::quat_operations`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Quat * Quat (compose)` | rot_z(PI/4)^2 | `Quat(0.0, 0.0, 0.70710677, 0.70710677)` | `Quat(0.0, 0.0, 0.7071068, 0.7071067)` | ✅ |
| `Quat::mul_quat` | rot_z(PI/4).mul_quat(rot_z(PI/4)) | `Quat(0.0, 0.0, 0.70710677, 0.70710677)` | `Quat(0.0, 0.0, 0.7071068, 0.7071067)` | ✅ |
| `Quat::dot` | IDENTITY·IDENTITY | `1` | `1` | ✅ |
| `Quat::length (normalized)` | rot_z(PI/4) | `1` | `1` | ✅ |
| `Quat::length_squared` | rot_z(PI/4) | `1` | `1` | ✅ |
| `Quat::conjugate` | rot_z(PI/2).conjugate() * Y | `Vec3(1.0, 0.0, 0.0)` | `Vec3(0.99999994, 0.0, 0.0)` | ✅ |
| `Quat::inverse` | rot_z(PI/2).inverse() * Y | `Vec3(1.0, 0.0, 0.0)` | `Vec3(0.99999994, 0.0, 0.0)` | ✅ |
| `Quat::normalize().length()` | from_xyzw(1,2,3,4) | `1` | `1` | ✅ |
| `Quat::is_normalized` | rot_z(PI/4) | `true` | `true` | ✅ |
| `Quat::is_near_identity` | rot_z(1e-5) | `true` | `true` | ✅ |
| `Quat::is_finite` | IDENTITY | `true` | `true` | ✅ |

### `rects::irect_urect`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `IRect::from_corners` | (3,1),(0,4) | `IRect { min: IVec2(0, 1), max: IVec2(3, 4) }` | `IRect { min: IVec2(0, 1), max: IVec2(3, 4) }` | ✅ |
| `IRect::width/height` | (0,0)-(5,3) | `(5, 3)` | `(5, 3)` | ✅ |
| `URect::center` | (0,0)-(4,4) | `UVec2(2, 2)` | `UVec2(2, 2)` | ✅ |

### `rects::isometries_and_rays`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Isometry2d::transform_point` | T=(1,2), R=90°, point X | `Vec2(1.0, 3.0)` | `Vec2(0.99999994, 3.0)` | ✅ |
| `Isometry2d::inverse_transform_point round-trip` |  | `Vec2(1.0, 0.0)` | `Vec2(1.0, 1.5893256e-8)` | ✅ |
| `Isometry2d * Isometry2d compose` |  | `Vec2(-2.0, 2.9999998)` | `Vec2(-2.0, 3.0)` | ✅ |
| `Isometry3d::transform_point` | T=(1,2,3), Rz=PI/2, point X | `Vec3(1.0, 3.0, 3.0)` | `Vec3(1.0, 3.0, 3.0)` | ✅ |
| `Isometry3d::inverse round-trip` |  | `Vec3(1.0, 0.0, 0.0)` | `Vec3(0.9999999, 0.0, 0.0)` | ✅ |
| `Ray2d::get_point` | origin 0, dir Y, d=3 | `Vec2(0.0, 3.0)` | `Vec2(0.0, 3.0)` | ✅ |
| `Ray3d::get_point` | origin 0, dir X, d=2.5 | `Vec3(2.5, 0.0, 0.0)` | `Vec3(2.5, 0.0, 0.0)` | ✅ |

### `rects::rect_family`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Rect::width` | (0,0)-(4,2) | `4` | `4` | ✅ |
| `Rect::height` | (0,0)-(4,2) | `2` | `2` | ✅ |
| `Rect::area` | (0,0)-(4,2) | `8` | `8` | ✅ |
| `Rect::center` | (0,0)-(4,2) | `Vec2(2.0, 1.0)` | `Vec2(2.0, 1.0)` | ✅ |
| `Rect::size` | (0,0)-(4,2) | `Vec2(4.0, 2.0)` | `Vec2(4.0, 2.0)` | ✅ |
| `Rect::contains` | (0,0)-(4,2) has (1,1) | `true` | `true` | ✅ |
| `Rect::from_center_size` | c=(0,0), s=(2,2) | `Rect { min: Vec2(-1.0, -1.0), max: Vec2(1.0, 1.0) }` | `Rect { min: Vec2(-1.0, -1.0), max: Vec2(1.0, 1.0) }` | ✅ |
| `Rect::union_point` | (0,0)-(1,1) ∪ (3,3) | `Rect { min: Vec2(0.0, 0.0), max: Vec2(3.0, 3.0) }` | `Rect { min: Vec2(0.0, 0.0), max: Vec2(3.0, 3.0) }` | ✅ |
| `Rect::intersect` | (0,0)-(2,2) ∩ (1,1)-(3,3) | `Rect { min: Vec2(1.0, 1.0), max: Vec2(2.0, 2.0) }` | `Rect { min: Vec2(1.0, 1.0), max: Vec2(2.0, 2.0) }` | ✅ |
| `Rect::inflate` | (0,0)-(2,2) by 1 | `Rect { min: Vec2(-1.0, -1.0), max: Vec2(3.0, 3.0) }` | `Rect { min: Vec2(-1.0, -1.0), max: Vec2(3.0, 3.0) }` | ✅ |
| `Rect::is_empty` | (1,1)-(1,1) | `true` | `true` | ✅ |
| `Rect::as_irect` | (0.2,0.9)-(3.7,4.1) | `IRect { min: IVec2(0, 0), max: IVec2(3, 4) }` | `IRect { min: IVec2(0, 0), max: IVec2(3, 4) }` | ✅ |

### `sampling::sampling_is_deterministic`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Same seed -> same sample (Sphere::sample_interior)` | seed 42 | `Vec3(0.2632629, -0.12307412, 0.78335136)` | `Vec3(0.2632629, -0.12307412, 0.78335136)` | ✅ |
| `Same seed -> same sample (Rectangle::sample_boundary)` | seed 7 | `Vec2(1.0, 0.5277977)` | `Vec2(1.0, 0.5277977)` | ✅ |

### `sampling::shape_sample_2d`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Rectangle::sample_interior — 500 points inside [-1,1]x[-2,2]` | seed 0x1234… | `true` | `true` | ✅ |
| `Rectangle::sample_boundary — 500 points on the perimeter` |  | `true` | `true` | ✅ |
| `Circle::sample_interior — 500 points within radius 3` |  | `true` | `true` | ✅ |
| `Circle::sample_boundary — 500 points on radius 3` |  | `true` | `true` | ✅ |

### `sampling::shape_sample_3d`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Cuboid::sample_interior — 500 points inside half-extents (1,2,3)` |  | `true` | `true` | ✅ |
| `Sphere::sample_interior — 500 points within radius 5` |  | `true` | `true` | ✅ |
| `Sphere::sample_boundary — 500 points on radius 5` |  | `true` | `true` | ✅ |

### `stable_interp::smooth_nudge`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Vec3::smooth_nudge converges to target` | target (10,0,0), 1000 steps | `Vec3(10.0, 0.0, 0.0)` | `Vec3(9.999994, 0.0, 0.0)` | ✅ |

### `stable_interp::stable_interpolate_impls`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Vec3::interpolate_stable` | 0..(4,4,4) @ 0.25 | `Vec3(1.0, 1.0, 1.0)` | `Vec3(1.0, 1.0, 1.0)` | ✅ |
| `Quat::interpolate_stable` | IDENTITY..rot_z(PI/2) @ 0.5 | `Quat(0.0, 0.0, 0.38268346, 0.9238795)` | `Quat(0.0, 0.0, 0.38268343, 0.92387956)` | ✅ |
| `Dir3::interpolate_stable` | X..Y @ 0.5 | `Vec3(0.70710677, 0.70710677, 0.0)` | `Vec3(0.7071069, 0.70710677, 0.0)` | ✅ |
| `Rot2::interpolate_stable` | 0°..90° @ 0.5 | `Rot2 { cos: 0.70710677, sin: 0.70710677 }` | `Rot2 { cos: 0.70710677, sin: 0.70710677 }` | ✅ |
| `Isometry3d from interpolated parts — translation` | 0..(10,0,0) @ 0.5 | `Vec3(5.0, 0.0, 0.0)` | `Vec3(5.0, 0.0, 0.0)` | ✅ |

### `vectors::vec2_arithmetic_and_products`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Vec2 + Vec2` | (1,2)+(3,4) | `Vec2(4.0, 6.0)` | `Vec2(4.0, 6.0)` | ✅ |
| `Vec2 - Vec2` | (3,4)-(1,2) | `Vec2(2.0, 2.0)` | `Vec2(2.0, 2.0)` | ✅ |
| `Vec2 * f32` | (3,4)*2 | `Vec2(6.0, 8.0)` | `Vec2(6.0, 8.0)` | ✅ |
| `Vec2 / f32` | (6,8)/2 | `Vec2(3.0, 4.0)` | `Vec2(3.0, 4.0)` | ✅ |
| `Vec2::dot` | (1,2)·(3,4) | `11` | `11` | ✅ |
| `Vec2::perp_dot` | (1,0)⟂·(0,1) | `1` | `1` | ✅ |
| `Vec2::perp` | (1,0) | `Vec2(0.0, 1.0)` | `Vec2(-0.0, 1.0)` | ✅ |

### `vectors::vec2_componentwise`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Vec2::abs` | (-1,2) | `Vec2(1.0, 2.0)` | `Vec2(1.0, 2.0)` | ✅ |
| `Vec2::signum` | (-3,0.5) | `Vec2(-1.0, 1.0)` | `Vec2(-1.0, 1.0)` | ✅ |
| `Vec2::floor` | (1.7,-1.2) | `Vec2(1.0, -2.0)` | `Vec2(1.0, -2.0)` | ✅ |
| `Vec2::ceil` | (1.2,-1.7) | `Vec2(2.0, -1.0)` | `Vec2(2.0, -1.0)` | ✅ |
| `Vec2::round` | (1.5,2.4) | `Vec2(2.0, 2.0)` | `Vec2(2.0, 2.0)` | ✅ |
| `Vec2::fract` | (1.75,-0.25) | `Vec2(0.75, -0.25)` | `Vec2(0.75, -0.25)` | ✅ |
| `Vec2::trunc` | (1.9,-1.9) | `Vec2(1.0, -1.0)` | `Vec2(1.0, -1.0)` | ✅ |
| `Vec2::recip` | (2,4) | `Vec2(0.5, 0.25)` | `Vec2(0.5, 0.25)` | ✅ |
| `Vec2::min` | (1,4),(3,2) | `Vec2(1.0, 2.0)` | `Vec2(1.0, 2.0)` | ✅ |
| `Vec2::max` | (1,4),(3,2) | `Vec2(3.0, 4.0)` | `Vec2(3.0, 4.0)` | ✅ |
| `Vec2::clamp` | (5,-5) in [-1,1] | `Vec2(1.0, -1.0)` | `Vec2(1.0, -1.0)` | ✅ |
| `Vec2::min_element` | (3,-2) | `-2` | `-2` | ✅ |
| `Vec2::max_element` | (3,-2) | `3` | `3` | ✅ |
| `Vec2::element_sum` | (3,4) | `7` | `7` | ✅ |
| `Vec2::element_product` | (3,4) | `12` | `12` | ✅ |
| `Vec2::extend` | (1,2) z=3 | `Vec3(1.0, 2.0, 3.0)` | `Vec3(1.0, 2.0, 3.0)` | ✅ |
| `Vec2::to_array` | (1,2) | `[1.0, 2.0]` | `[1.0, 2.0]` | ✅ |
| `Vec2::from_array` | [1,2] | `Vec2(1.0, 2.0)` | `Vec2(1.0, 2.0)` | ✅ |
| `Vec2::with_x` | (1,2).with_x(9) | `Vec2(9.0, 2.0)` | `Vec2(9.0, 2.0)` | ✅ |

### `vectors::vec2_geometry`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Vec2::length` | (3,4) | `5` | `5` | ✅ |
| `Vec2::length_squared` | (3,4) | `25` | `25` | ✅ |
| `Vec2::length_recip` | (3,4) | `0.2` | `0.2` | ✅ |
| `Vec2::distance` | (0,0)->(3,4) | `5` | `5` | ✅ |
| `Vec2::distance_squared` | (0,0)->(3,4) | `25` | `25` | ✅ |
| `Vec2::normalize().length()` | (3,4) | `1` | `1` | ✅ |
| `Vec2::is_normalized` | (0.6,0.8) | `true` | `true` | ✅ |
| `Vec2::normalize_or_zero` | (0,0) | `Vec2(0.0, 0.0)` | `Vec2(0.0, 0.0)` | ✅ |
| `Vec2::project_onto` | (2,2) onto X | `Vec2(2.0, 0.0)` | `Vec2(2.0, 0.0)` | ✅ |
| `Vec2::reject_from` | (2,2) from X | `Vec2(0.0, 2.0)` | `Vec2(0.0, 2.0)` | ✅ |
| `Vec2::lerp` | 0..(4,4) @ 0.25 | `Vec2(1.0, 1.0)` | `Vec2(1.0, 1.0)` | ✅ |
| `Vec2::midpoint` | (0,0),(4,2) | `Vec2(2.0, 1.0)` | `Vec2(2.0, 1.0)` | ✅ |
| `Vec2::move_towards` | (0,0)->(10,0) by 3 | `Vec2(3.0, 0.0)` | `Vec2(3.0, 0.0)` | ✅ |
| `Vec2::angle_to` | (1,0)->(0,1) | `1.5707964` | `1.5707963` | ✅ |
| `Vec2::to_angle` | (0,1) | `1.5707964` | `1.5707964` | ✅ |
| `Vec2::from_angle` | PI/2 | `Vec2(0.0, 1.0)` | `Vec2(-4.371139e-8, 1.0)` | ✅ |
| `Vec2::rotate` | from_angle(PI/2).rotate(X) | `Vec2(0.0, 1.0)` | `Vec2(-4.371139e-8, 1.0)` | ✅ |

### `vectors::vec3_componentwise_and_convert`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Vec3::abs` | (-1,2,-3) | `Vec3(1.0, 2.0, 3.0)` | `Vec3(1.0, 2.0, 3.0)` | ✅ |
| `Vec3::signum` | (-2,0,3) | `Vec3(-1.0, 1.0, 1.0)` | `Vec3(-1.0, 1.0, 1.0)` | ✅ |
| `Vec3::min` | (1,5,3),(4,2,6) | `Vec3(1.0, 2.0, 3.0)` | `Vec3(1.0, 2.0, 3.0)` | ✅ |
| `Vec3::max` | (1,5,3),(4,2,6) | `Vec3(4.0, 5.0, 6.0)` | `Vec3(4.0, 5.0, 6.0)` | ✅ |
| `Vec3::clamp` | (9,-9,0.5) in [-1,1] | `Vec3(1.0, -1.0, 0.5)` | `Vec3(1.0, -1.0, 0.5)` | ✅ |
| `Vec3::min_element` | (3,-2,5) | `-2` | `-2` | ✅ |
| `Vec3::max_element` | (3,-2,5) | `5` | `5` | ✅ |
| `Vec3::element_sum` | (1,2,3) | `6` | `6` | ✅ |
| `Vec3::element_product` | (1,2,3) | `6` | `6` | ✅ |
| `Vec3::extend` | (1,2,3) w=4 | `Vec4(1.0, 2.0, 3.0, 4.0)` | `Vec4(1.0, 2.0, 3.0, 4.0)` | ✅ |
| `Vec3::truncate` | (1,2,3) | `Vec2(1.0, 2.0)` | `Vec2(1.0, 2.0)` | ✅ |
| `Vec3::to_array` | (1,2,3) | `[1.0, 2.0, 3.0]` | `[1.0, 2.0, 3.0]` | ✅ |
| `Vec3::any_orthonormal_pair — orthonormal` | Z | `true` | `true` | ✅ |
| `Vec3::is_finite` | (1,2,3) | `true` | `true` | ✅ |
| `Vec3::is_nan` | (NaN,0,0) | `true` | `true` | ✅ |

### `vectors::vec3_core`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Vec3::dot` | (1,2,3)·(4,5,6) | `32` | `32` | ✅ |
| `Vec3::cross` | (1,2,3)x(4,5,6) | `Vec3(-3.0, 6.0, -3.0)` | `Vec3(-3.0, 6.0, -3.0)` | ✅ |
| `Vec3::length` | (2,3,6) | `7` | `7` | ✅ |
| `Vec3::length_squared` | (1,2,2) | `9` | `9` | ✅ |
| `Vec3::length_recip` | (0,0,4) | `0.25` | `0.25` | ✅ |
| `Vec3::normalize().length()` | (1,2,3) | `1` | `0.99999994` | ✅ |
| `Vec3::distance` | (1,2,3)->(4,5,6) | `5.196152` | `5.196152` | ✅ |
| `Vec3::distance_squared` | (1,2,3)->(4,5,6) | `27` | `27` | ✅ |
| `Vec3::angle_between` | X,Y | `1.5707964` | `1.5707963` | ✅ |
| `Vec3::project_onto` | (1,1,1) onto X | `Vec3(1.0, 0.0, 0.0)` | `Vec3(1.0, 0.0, 0.0)` | ✅ |
| `Vec3::reject_from` | (1,1,1) from X | `Vec3(0.0, 1.0, 1.0)` | `Vec3(0.0, 1.0, 1.0)` | ✅ |
| `Vec3::lerp` | 0..(4,4,4) @ 0.25 | `Vec3(1.0, 1.0, 1.0)` | `Vec3(1.0, 1.0, 1.0)` | ✅ |
| `Vec3::reflect` | (1,-1,0) about Y | `Vec3(1.0, 1.0, 0.0)` | `Vec3(1.0, 1.0, 0.0)` | ✅ |
| `Vec3::slerp` | X..Y @ 0.5 | `Vec3(0.70710677, 0.70710677, 0.0)` | `Vec3(0.70710677, 0.70710677, 0.0)` | ✅ |

### `vectors::vec3a_specifics`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Vec3A: size_of` |  | `16` | `16` | ✅ |
| `Vec3A: align_of` |  | `16` | `16` | ✅ |
| `Vec3A::cross matches Vec3::cross` | (1.5,-2,3.25) x (-0.5,4,1) | `Vec3(-15.0, -3.125, 5.0)` | `Vec3(-15.0, -3.125, 5.0)` | ✅ |
| `Vec3A::dot matches Vec3::dot` | same | `-5.5` | `-5.5` | ✅ |
| `Vec3::to_vec3a round-trip` | (1.5,-2,3.25) | `Vec3(1.5, -2.0, 3.25)` | `Vec3(1.5, -2.0, 3.25)` | ✅ |
| `Vec3A::lerp` | 0..(4,4,4)@0.5 | `Vec3A(2.0, 2.0, 2.0)` | `Vec3A(2.0, 2.0, 2.0)` | ✅ |

### `vectors::vec4_specifics`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
| `Vec4::dot` | (1,2,3,4)·self | `30` | `30` | ✅ |
| `Vec4::length` | (0,0,3,4) | `5` | `5` | ✅ |
| `Vec4::length_squared` | (1,2,3,4) | `30` | `30` | ✅ |
| `Vec4::truncate` | (1,2,3,4) | `Vec3(1.0, 2.0, 3.0)` | `Vec3(1.0, 2.0, 3.0)` | ✅ |
| `Vec4::min_element` | (1,2,3,4) | `1.0` | `1.0` | ✅ |
| `Vec4::max_element` | (1,2,3,4) | `4.0` | `4.0` | ✅ |
| `Vec4::element_sum` | (1,2,3,4) | `10` | `10` | ✅ |
| `Vec4::abs` | (-1,2,-3,4) | `Vec4(1.0, 2.0, 3.0, 4.0)` | `Vec4(1.0, 2.0, 3.0, 4.0)` | ✅ |
| `Vec4::lerp` | 0..(4,4,4,4)@0.25 | `Vec4(1.0, 1.0, 1.0, 1.0)` | `Vec4(1.0, 1.0, 1.0, 1.0)` | ✅ |
| `Vec4::to_array` | (1,2,3,4) | `[1.0, 2.0, 3.0, 4.0]` | `[1.0, 2.0, 3.0, 4.0]` | ✅ |

## Nicht abgedeckt

Alle ~200 Swizzle-Methoden pro Vektor (Makro-generiert; eine geprüft ⇒ alle), `I16Vec`/`I8Vec`/`U16Vec`/`U8Vec`/`USizeVec`, `mint`-Interop (Feature aus), `mesh_sampling` (`alloc`), `bevy_reflect`-Integration (Feature aus).

**Caveat:** nur Emulator — VFP-Rundungsmodi / Denormals / exakte NaN-Bitmuster auf echter Hardware nicht 1:1 garantiert (für die Toleranzen unkritisch). Siehe `../../port.md` §B9.
