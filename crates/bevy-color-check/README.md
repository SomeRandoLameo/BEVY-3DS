# bevy-color-check

On-device check that **`bevy_color`** — plain color-space math, no rendering — computes correctly on the Nintendo 3DS. Aims to touch every public color space, every conversion between them, and the shared color-op traits.

| | |
|---|---|
| Bevy-Einheit | `bevy_color` (zieht `bevy_math` mit `curve`) |
| Version | `0.19.1` |
| Features | `default-features = false`, `features = ["std"]` (= `alloc` + `bevy_math/std`; kein `bevy_reflect`, kein `serialize`, kein `wgpu-types`) |
| Target | `armv6k-nintendo-3ds` · Trig/`powf`/`cbrt` aus devkitPro-newlib |
| Toleranz | ε = 1e-4 (gleicher Raum) · 4e-3 (über eine Farbraum-Konversion, s.u.) |
| Stand | 2026-09-11 · Azahar-Emulator · **44/44 Testfunktionen · 381/381 Checks bestanden** |

Ausführen: `./scripts/test-emulator.sh -p bevy-color-check`

## Toleranz-Hinweis

`bevy_color`s eigene Tests vergleichen Konversionen mit ε=1e-3 auf gut konditionierten Primärfarben. Die 3DS zieht `cbrt`/`powf`/Trig aus devkitPro-newlib statt der Host-libm, und die Ketten `Hsla→Hsva→Hwba→Srgba`, `Laba↔Xyza`, `Oklaba↔LinearRgba` etc. verketten mehrere solcher Aufrufe — daher hier `CONV_EPS = 4e-3` für Checks, die eine Farbraum-Grenze überschreiten. Innerhalb eines Raums (Konstruktoren, `with_*`, `Mix`, `Hue`, `Gray`, Layout) gilt weiterhin ε=1e-4.

## Abgedeckt

**`Srgba`** — Konstanten, `new`/`rgb`/`with_*`, `hex`/`to_hex` (3/4/6/8-stellig + Fehlerpfade), `rgb_u8`/`rgba_u8`, `gamma_function`/`_inverse`, `Mix`/`Alpha`/`Luminance`/`Gray`/`EuclideanDistance`, `ColorToComponents`/`ColorToPacked`, componentwise `Add`/`Sub`/`Neg`/`Mul<f32>`/`Div`/`AddAssign` (via `impl_componentwise_vector_space!`), `VectorSpace::ZERO`, `StableInterpolate`.

**`LinearRgba`** — Konstanten (inkl. `NAN`), CIE-Luminanz-Gewichte, `with_luminance`/`darker`/`lighter` (distributiv), `as_u32`/`to_u8_array`/`from_u8_array` (+ Clamping), `Mix`/`Gray`/`Alpha`/`EuclideanDistance`, `ColorToComponents`, Vektor-Ops.

**`Hsla`/`Hsva`/`Hwba`** — Konstruktoren, `Hue`/`Saturation`-Traits (`with_hue`/`rotate_hue`/`set_hue` inkl. `rem_euclid`-Wrap), `Mix` mit kürzestem Hue-Pfad (bevy_colors `test_mix_wrap`), `Luminance`, `Gray`, `Hsla::sequential_dispersed` (Golden-Angle-Sequenz), paarweise Rückkonversionen.

**`Laba`/`Lcha`/`Oklaba`/`Oklcha`/`Xyza`** — Konstruktoren, `CIE_EPSILON`/`CIE_KAPPA`, `D65_WHITE`, `Luminance`, `EuclideanDistance` (Oklab/Oklch), `Hue` (Lcha/Oklcha), `Gray`, `sequential_dispersed`, `Laba↔Oklaba`/`Lcha↔Xyza`/`Hwba↔Lcha` Konversionsketten.

**Konversionsgraph** — jede der 5 Testfarben (schwarz/weiß/rot/grün/blau/grau, aus `bevy_color`s eigener `test_colors::TEST_COLORS`-Tabelle) durch `Srgba→LinearRgba`, `→Hsla`, `→Oklaba`, `→Xyza` sowie **alle 8 Räume** über `Srgba→Raum→LinearRgba` zurück.

**`Color`** (das typ-erasure Enum) — alle 10 Konstruktoren (`srgb`/`srgba`/`srgb_u8`/`srgb_u32`/…/`xyz`), `WHITE`/`BLACK`/`NONE`/`Default`, `to_srgba`/`to_linear`, `From<Konkreter Typ>` (via `derive_more`), `Alpha`/`Luminance`/`Hue`/`Saturation`/`Mix` (inkl. Delegation über `Oklcha` für Räume ohne native Definition), `TryStableInterpolate` (Ok bei gleicher Variante, `Err` bei Mismatch).

**`color_ops`/`color_range`** — `Alpha for f32`, `Gray::gray(0.0/1.0) == BLACK/WHITE` generisch über alle 10 Farbtypen, `ColorRange::at` (inkl. Clamping außerhalb `[0,1]`).

**`ColorCurve`** — `new` (Fehler bei < 2 Farben), `domain`, `sample_clamped`, `Curve::sample` (`None` außerhalb der Domain), `CurveExt::map`-Adaptor.

**`palettes`** — Stichproben aus `basic`, `css`, `tailwind`.

**Layout** — `size_of`/`align_of` jedes Farbtyps + `Color`.

## Ergebnis

Funktion · Eingabe · Erwartet · **tatsächliche Ausgabe** · OK. Reihenfolge = Testausführung (alphabetisch nach Modul/Funktion). Die ε-Toleranz fängt f32-Rundungen ab.

### `color_enum::constants_and_conversions`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Color::WHITE is LinearRgba` |  | `LinearRgba(LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 })` | `LinearRgba(LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 })` | ✅ |
|`Color::BLACK is LinearRgba` |  | `LinearRgba(LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 })` | `LinearRgba(LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 })` | ✅ |
|`Color::default() == WHITE` |  | `LinearRgba(LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 })` | `LinearRgba(LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 })` | ✅ |
|`Color::to_srgba` | srgb(1,0,0) | `Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Color::to_linear (WHITE)` |  | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`From<Srgba> for Color (derive_more)` |  | `Srgba(Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 })` | `Srgba(Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 })` | ✅ |
|`From<Hsla> for Color` |  | `Hsla(Hsla { hue: 120.0, saturation: 1.0, lightness: 0.5, alpha: 1.0 })` | `Hsla(Hsla { hue: 120.0, saturation: 1.0, lightness: 0.5, alpha: 1.0 })` | ✅ |
|`Hsla::from(Color) round-trips the variant` |  | `Hsla { hue: 120.0, saturation: 1.0, lightness: 0.5, alpha: 1.0 }` | `Hsla { hue: 120.0, saturation: 1.0, lightness: 0.5, alpha: 1.0 }` | ✅ |

### `color_enum::constructors_agree_with_their_concrete_type`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Color::srgb == Srgba::rgb` |  | `Srgba(Srgba { red: 0.1, green: 0.2, blue: 0.3, alpha: 1.0 })` | `Srgba(Srgba { red: 0.1, green: 0.2, blue: 0.3, alpha: 1.0 })` | ✅ |
|`Color::srgba` |  | `Srgba(Srgba { red: 0.1, green: 0.2, blue: 0.3, alpha: 0.4 })` | `Srgba(Srgba { red: 0.1, green: 0.2, blue: 0.3, alpha: 0.4 })` | ✅ |
|`Color::srgb_from_array` |  | `Srgba(Srgba { red: 0.1, green: 0.2, blue: 0.3, alpha: 1.0 })` | `Srgba(Srgba { red: 0.1, green: 0.2, blue: 0.3, alpha: 1.0 })` | ✅ |
|`Color::srgb_u8` |  | `Srgba(Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 })` | `Srgba(Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 })` | ✅ |
|`Color::srgba_u8` |  | `Srgba(Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 0.0 })` | `Srgba(Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 0.0 })` | ✅ |
|`Color::srgb_u32(0xff0000) -> red` |  | `Srgba(Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 })` | `Srgba(Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 })` | ✅ |
|`Color::srgba_u32(0xff000080)` |  | `Srgba(Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 0.5019608 })` | `Srgba(Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 0.5019608 })` | ✅ |
|`Color::linear_rgb` |  | `LinearRgba(LinearRgba { red: 0.1, green: 0.2, blue: 0.3, alpha: 1.0 })` | `LinearRgba(LinearRgba { red: 0.1, green: 0.2, blue: 0.3, alpha: 1.0 })` | ✅ |
|`Color::hsl` |  | `Hsla(Hsla { hue: 120.0, saturation: 1.0, lightness: 0.5, alpha: 1.0 })` | `Hsla(Hsla { hue: 120.0, saturation: 1.0, lightness: 0.5, alpha: 1.0 })` | ✅ |
|`Color::hsv` |  | `Hsva(Hsva { hue: 120.0, saturation: 1.0, value: 0.5, alpha: 1.0 })` | `Hsva(Hsva { hue: 120.0, saturation: 1.0, value: 0.5, alpha: 1.0 })` | ✅ |
|`Color::hwb` |  | `Hwba(Hwba { hue: 120.0, whiteness: 0.1, blackness: 0.2, alpha: 1.0 })` | `Hwba(Hwba { hue: 120.0, whiteness: 0.1, blackness: 0.2, alpha: 1.0 })` | ✅ |
|`Color::lab` |  | `Laba(Laba { lightness: 0.5, a: 0.1, b: -0.1, alpha: 1.0 })` | `Laba(Laba { lightness: 0.5, a: 0.1, b: -0.1, alpha: 1.0 })` | ✅ |
|`Color::lch` |  | `Lcha(Lcha { lightness: 0.5, chroma: 0.1, hue: 90.0, alpha: 1.0 })` | `Lcha(Lcha { lightness: 0.5, chroma: 0.1, hue: 90.0, alpha: 1.0 })` | ✅ |
|`Color::oklab` |  | `Oklaba(Oklaba { lightness: 0.5, a: 0.1, b: -0.1, alpha: 1.0 })` | `Oklaba(Oklaba { lightness: 0.5, a: 0.1, b: -0.1, alpha: 1.0 })` | ✅ |
|`Color::oklch` |  | `Oklcha(Oklcha { lightness: 0.5, chroma: 0.1, hue: 90.0, alpha: 1.0 })` | `Oklcha(Oklcha { lightness: 0.5, chroma: 0.1, hue: 90.0, alpha: 1.0 })` | ✅ |
|`Color::xyz` |  | `Xyza(Xyza { x: 0.3, y: 0.4, z: 0.5, alpha: 1.0 })` | `Xyza(Xyza { x: 0.3, y: 0.4, z: 0.5, alpha: 1.0 })` | ✅ |

### `color_enum::op_traits_delegate`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Color::alpha()` | srgba(_,_,_,0.4) | `0.4` | `0.4` | ✅ |
|`Color::with_alpha().alpha()` |  | `0.7` | `0.7` | ✅ |
|`Color::set_alpha` |  | `0.5` | `0.5` | ✅ |
|`Color::luminance (WHITE)` |  | `1` | `1` | ✅ |
|`Color::hue (native, Hsla)` | hsl(120,..) | `120.0` | `120.0` | ✅ |
|`Color::hue (no native def, Srgba) is finite` | converts via Oklch | `true` | `true` | ✅ |
|`Color::saturation (native, Hsla)` |  | `0.5` | `0.5` | ✅ |
|`Color::mix(srgb RED, srgb BLUE, 0.5)` |  | `Srgba(Srgba { red: 0.5, green: 0.0, blue: 0.5, alpha: 1.0 })` | `Srgba(Srgba { red: 0.5, green: 0.0, blue: 0.5, alpha: 1.0 })` | ✅ |

### `color_enum::try_stable_interpolate`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Color::try_interpolate_stable (same variant) is Ok` |  | `true` | `true` | ✅ |
|`Color::try_interpolate_stable (mismatched variants) is Err` | Srgba vs Hsla | `true` | `true` | ✅ |

### `conversions::derived_conversions`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Laba -> Oklaba -> Laba` |  | `Laba { lightness: 0.5666139, a: -0.42942643, b: 0.44837302, alpha: 1.0 }` | `Laba { lightness: 0.5666139, a: -0.42942673, b: 0.44837332, alpha: 1.0 }` | ✅ |
|`Lcha -> Xyza -> Lcha` |  | `Lcha { lightness: 0.5666139, chroma: 0.6208425, hue: 133.76352, alpha: 1.0 }` | `Lcha { lightness: 0.5666139, chroma: 0.62084246, hue: 133.76352, alpha: 1.0 }` | ✅ |
|`Hwba -> Lcha -> Hwba` |  | `Hwba { hue: 105.0, whiteness: 0.2, blackness: 0.39999998, alpha: 1.0 }` | `Hwba { hue: 104.99998, whiteness: 0.19999993, blackness: 0.40000004, alpha: 1.0 }` | ✅ |

### `conversions::every_space_round_trips_through_linear`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Srgba -> Hsla -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Hsva -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Hwba -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Laba -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Lcha -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Oklaba -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Oklcha -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Xyza -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Hsla -> LinearRgba` |  | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Hsva -> LinearRgba` |  | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Hwba -> LinearRgba` |  | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Laba -> LinearRgba` |  | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Lcha -> LinearRgba` |  | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Oklaba -> LinearRgba` |  | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 1.0000001, blue: 0.9999996, alpha: 1.0 }` | ✅ |
|`Srgba -> Oklcha -> LinearRgba` |  | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 1.0000001, blue: 0.9999996, alpha: 1.0 }` | ✅ |
|`Srgba -> Xyza -> LinearRgba` |  | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 1.0000001, blue: 1.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Hsla -> LinearRgba` |  | `LinearRgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Hsva -> LinearRgba` |  | `LinearRgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Hwba -> LinearRgba` |  | `LinearRgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Laba -> LinearRgba` |  | `LinearRgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.9999995, green: 8.760253e-8, blue: -2.2351742e-8, alpha: 1.0 }` | ✅ |
|`Srgba -> Lcha -> LinearRgba` |  | `LinearRgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.9999995, green: 8.760253e-8, blue: -2.2351742e-8, alpha: 1.0 }` | ✅ |
|`Srgba -> Oklaba -> LinearRgba` |  | `LinearRgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.9999999, green: 5.401671e-8, blue: -4.4703484e-8, alpha: 1.0 }` | ✅ |
|`Srgba -> Oklcha -> LinearRgba` |  | `LinearRgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.9999994, green: 2.1979213e-7, blue: 1.4901161e-8, alpha: 1.0 }` | ✅ |
|`Srgba -> Xyza -> LinearRgba` |  | `LinearRgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.99999976, green: 1.4778925e-7, blue: -1.6763806e-8, alpha: 1.0 }` | ✅ |
|`Srgba -> Hsla -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Hsva -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Hwba -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Laba -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: -2.6077032e-8, green: 1.0000001, blue: -1.4901161e-8, alpha: 1.0 }` | ✅ |
|`Srgba -> Lcha -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: -2.6077032e-8, green: 1.0000001, blue: -1.4901161e-8, alpha: 1.0 }` | ✅ |
|`Srgba -> Oklaba -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 1.0207295e-6, green: 0.9999996, blue: -2.0861626e-7, alpha: 1.0 }` | ✅ |
|`Srgba -> Oklcha -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 9.983778e-7, green: 0.99999964, blue: -3.5762787e-7, alpha: 1.0 }` | ✅ |
|`Srgba -> Xyza -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: -1.1175871e-8, green: 1.0, blue: -2.9802322e-8, alpha: 1.0 }` | ✅ |
|`Srgba -> Hsla -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Hsva -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Hwba -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Laba -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 5.9604645e-8, green: -2.2351742e-8, blue: 1.0000002, alpha: 1.0 }` | ✅ |
|`Srgba -> Lcha -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 2.682209e-7, green: -3.7252903e-8, blue: 0.99999976, alpha: 1.0 }` | ✅ |
|`Srgba -> Oklaba -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 1.4901161e-8, green: 0.0, blue: 0.9999999, alpha: 1.0 }` | ✅ |
|`Srgba -> Oklcha -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 7.4505806e-8, green: -2.9802322e-8, blue: 0.9999999, alpha: 1.0 }` | ✅ |
|`Srgba -> Xyza -> LinearRgba` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 5.9604645e-8, green: -1.1175871e-8, blue: 1.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Hsla -> LinearRgba` |  | `LinearRgba { red: 0.21404114, green: 0.21404114, blue: 0.21404114, alpha: 1.0 }` | `LinearRgba { red: 0.21404114, green: 0.21404114, blue: 0.21404114, alpha: 1.0 }` | ✅ |
|`Srgba -> Hsva -> LinearRgba` |  | `LinearRgba { red: 0.21404114, green: 0.21404114, blue: 0.21404114, alpha: 1.0 }` | `LinearRgba { red: 0.21404114, green: 0.21404114, blue: 0.21404114, alpha: 1.0 }` | ✅ |
|`Srgba -> Hwba -> LinearRgba` |  | `LinearRgba { red: 0.21404114, green: 0.21404114, blue: 0.21404114, alpha: 1.0 }` | `LinearRgba { red: 0.21404114, green: 0.21404114, blue: 0.21404114, alpha: 1.0 }` | ✅ |
|`Srgba -> Laba -> LinearRgba` |  | `LinearRgba { red: 0.21404114, green: 0.21404114, blue: 0.21404114, alpha: 1.0 }` | `LinearRgba { red: 0.21404114, green: 0.2140411, blue: 0.21404105, alpha: 1.0 }` | ✅ |
|`Srgba -> Lcha -> LinearRgba` |  | `LinearRgba { red: 0.21404114, green: 0.21404114, blue: 0.21404114, alpha: 1.0 }` | `LinearRgba { red: 0.21404114, green: 0.2140411, blue: 0.21404105, alpha: 1.0 }` | ✅ |
|`Srgba -> Oklaba -> LinearRgba` |  | `LinearRgba { red: 0.21404114, green: 0.21404114, blue: 0.21404114, alpha: 1.0 }` | `LinearRgba { red: 0.21404143, green: 0.21404102, blue: 0.2140411, alpha: 1.0 }` | ✅ |
|`Srgba -> Oklcha -> LinearRgba` |  | `LinearRgba { red: 0.21404114, green: 0.21404114, blue: 0.21404114, alpha: 1.0 }` | `LinearRgba { red: 0.21404143, green: 0.21404102, blue: 0.2140411, alpha: 1.0 }` | ✅ |
|`Srgba -> Xyza -> LinearRgba` |  | `LinearRgba { red: 0.21404114, green: 0.21404114, blue: 0.21404114, alpha: 1.0 }` | `LinearRgba { red: 0.21404114, green: 0.21404119, blue: 0.21404114, alpha: 1.0 }` | ✅ |

### `conversions::linear_to_srgba_matches_table`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`LinearRgba -> Srgba (black)` |  | `Srgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`LinearRgba -> Srgba (white)` |  | `Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `Srgba { red: 0.99999994, green: 0.99999994, blue: 0.99999994, alpha: 1.0 }` | ✅ |
|`LinearRgba -> Srgba (red)` |  | `Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 0.99999994, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`LinearRgba -> Srgba (green)` |  | `Srgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 0.0, green: 0.99999994, blue: 0.0, alpha: 1.0 }` | ✅ |
|`LinearRgba -> Srgba (blue)` |  | `Srgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | `Srgba { red: 0.0, green: 0.0, blue: 0.99999994, alpha: 1.0 }` | ✅ |
|`LinearRgba -> Srgba (gray)` |  | `Srgba { red: 0.5, green: 0.5, blue: 0.5, alpha: 1.0 }` | `Srgba { red: 0.5, green: 0.5, blue: 0.5, alpha: 1.0 }` | ✅ |

### `conversions::srgba_to_hsla_matches_table`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Srgba -> Hsla (black)` |  | `Hsla { hue: 0.0, saturation: 0.0, lightness: 0.0, alpha: 1.0 }` | `Hsla { hue: 0.0, saturation: 0.0, lightness: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Hsla (white)` |  | `Hsla { hue: 0.0, saturation: 0.0, lightness: 1.0, alpha: 1.0 }` | `Hsla { hue: 0.0, saturation: 0.0, lightness: 1.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Hsla (red)` |  | `Hsla { hue: 0.0, saturation: 1.0, lightness: 0.5, alpha: 1.0 }` | `Hsla { hue: 0.0, saturation: 1.0, lightness: 0.5, alpha: 1.0 }` | ✅ |
|`Srgba -> Hsla (green)` |  | `Hsla { hue: 120.0, saturation: 1.0, lightness: 0.5, alpha: 1.0 }` | `Hsla { hue: 120.0, saturation: 1.0, lightness: 0.5, alpha: 1.0 }` | ✅ |
|`Srgba -> Hsla (blue)` |  | `Hsla { hue: 240.0, saturation: 1.0, lightness: 0.5, alpha: 1.0 }` | `Hsla { hue: 240.0, saturation: 1.0, lightness: 0.5, alpha: 1.0 }` | ✅ |
|`Srgba -> Hsla (gray)` |  | `Hsla { hue: 0.0, saturation: 0.0, lightness: 0.5, alpha: 1.0 }` | `Hsla { hue: 0.0, saturation: 0.0, lightness: 0.5, alpha: 1.0 }` | ✅ |

### `conversions::srgba_to_linear_matches_table`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Srgba -> LinearRgba (black)` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba -> LinearRgba (white)` |  | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`Srgba -> LinearRgba (red)` |  | `LinearRgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba -> LinearRgba (green)` |  | `LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba -> LinearRgba (blue)` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`Srgba -> LinearRgba (gray)` |  | `LinearRgba { red: 0.21404114, green: 0.21404114, blue: 0.21404114, alpha: 1.0 }` | `LinearRgba { red: 0.21404114, green: 0.21404114, blue: 0.21404114, alpha: 1.0 }` | ✅ |

### `conversions::srgba_to_oklaba_matches_table`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Srgba -> Oklaba (black)` |  | `Oklaba { lightness: 0.0, a: 0.0, b: 0.0, alpha: 1.0 }` | `Oklaba { lightness: 0.0, a: 0.0, b: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Oklaba (white)` |  | `Oklaba { lightness: 1.0, a: 0.0, b: 5.9604645e-8, alpha: 1.0 }` | `Oklaba { lightness: 1.0, a: 0.0, b: 5.9604645e-8, alpha: 1.0 }` | ✅ |
|`Srgba -> Oklaba (red)` |  | `Oklaba { lightness: 0.6279554, a: 0.22486295, b: 0.1258463, alpha: 1.0 }` | `Oklaba { lightness: 0.6279554, a: 0.22486295, b: 0.1258463, alpha: 1.0 }` | ✅ |
|`Srgba -> Oklaba (green)` |  | `Oklaba { lightness: 0.8664396, a: -0.2338874, b: 0.1794985, alpha: 1.0 }` | `Oklaba { lightness: 0.8664396, a: -0.2338874, b: 0.1794985, alpha: 1.0 }` | ✅ |
|`Srgba -> Oklaba (blue)` |  | `Oklaba { lightness: 0.4520137, a: -0.032456964, b: -0.31152815, alpha: 1.0 }` | `Oklaba { lightness: 0.4520137, a: -0.032456964, b: -0.31152815, alpha: 1.0 }` | ✅ |
|`Srgba -> Oklaba (gray)` |  | `Oklaba { lightness: 0.5981807, a: 1.1920929e-7, b: 0.0, alpha: 1.0 }` | `Oklaba { lightness: 0.5981807, a: 1.1920929e-7, b: 0.0, alpha: 1.0 }` | ✅ |

### `conversions::srgba_to_xyza_matches_table`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Srgba -> Xyza (black)` |  | `Xyza { x: 0.0, y: 0.0, z: 0.0, alpha: 1.0 }` | `Xyza { x: 0.0, y: 0.0, z: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba -> Xyza (white)` |  | `Xyza { x: 0.95047, y: 1.0, z: 1.08883, alpha: 1.0 }` | `Xyza { x: 0.95047003, y: 1.0000001, z: 1.08883, alpha: 1.0 }` | ✅ |
|`Srgba -> Xyza (red)` |  | `Xyza { x: 0.4124564, y: 0.2126729, z: 0.0193339, alpha: 1.0 }` | `Xyza { x: 0.4124564, y: 0.2126729, z: 0.0193339, alpha: 1.0 }` | ✅ |
|`Srgba -> Xyza (green)` |  | `Xyza { x: 0.3575761, y: 0.7151522, z: 0.119192, alpha: 1.0 }` | `Xyza { x: 0.3575761, y: 0.7151522, z: 0.119192, alpha: 1.0 }` | ✅ |
|`Srgba -> Xyza (blue)` |  | `Xyza { x: 0.1804375, y: 0.072175, z: 0.9503041, alpha: 1.0 }` | `Xyza { x: 0.1804375, y: 0.072175, z: 0.9503041, alpha: 1.0 }` | ✅ |
|`Srgba -> Xyza (gray)` |  | `Xyza { x: 0.2034397, y: 0.21404117, z: 0.23305441, alpha: 1.0 }` | `Xyza { x: 0.2034397, y: 0.21404117, z: 0.23305441, alpha: 1.0 }` | ✅ |

### `cylindrical::cylindrical_spaces_roundtrip_srgba`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Srgba -> Hsla -> Srgba` |  | `Srgba { red: 0.25, green: 0.4, blue: 0.7, alpha: 1.0 }` | `Srgba { red: 0.25000003, green: 0.39999998, blue: 0.7, alpha: 1.0 }` | ✅ |
|`Srgba -> Hsva -> Srgba` |  | `Srgba { red: 0.25, green: 0.4, blue: 0.7, alpha: 1.0 }` | `Srgba { red: 0.25, green: 0.39999998, blue: 0.7, alpha: 1.0 }` | ✅ |
|`Srgba -> Hwba -> Srgba` |  | `Srgba { red: 0.25, green: 0.4, blue: 0.7, alpha: 1.0 }` | `Srgba { red: 0.25, green: 0.39999998, blue: 0.7, alpha: 1.0 }` | ✅ |

### `cylindrical::hsla_constructors_and_accessors`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Hsla::hsl sets alpha 1` |  | `1.0` | `1.0` | ✅ |
|`Hsla::new` |  | `Hsla { hue: 120.0, saturation: 0.5, lightness: 0.5, alpha: 1.0 }` | `Hsla { hue: 120.0, saturation: 0.5, lightness: 0.5, alpha: 1.0 }` | ✅ |
|`Hsla::with_saturation` |  | `0.25` | `0.25` | ✅ |
|`Hsla::with_lightness` |  | `0.25` | `0.25` | ✅ |
|`Hsla::default() (white: lightness 1)` |  | `Hsla { hue: 0.0, saturation: 0.0, lightness: 1.0, alpha: 1.0 }` | `Hsla { hue: 0.0, saturation: 0.0, lightness: 1.0, alpha: 1.0 }` | ✅ |

### `cylindrical::hsla_hue_and_saturation_traits`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Hsla::hue()` | hsl(180,1,.5) | `180.0` | `180.0` | ✅ |
|`Hsla::with_hue` |  | `270.0` | `270.0` | ✅ |
|`Hsla::rotate_hue(+90)` |  | `Hsla { hue: 270.0, saturation: 1.0, lightness: 0.5, alpha: 1.0 }` | `Hsla { hue: 270.0, saturation: 1.0, lightness: 0.5, alpha: 1.0 }` | ✅ |
|`Hsla::rotate_hue(-90) wraps via rem_euclid` |  | `Hsla { hue: 90.0, saturation: 1.0, lightness: 0.5, alpha: 1.0 }` | `Hsla { hue: 90.0, saturation: 1.0, lightness: 0.5, alpha: 1.0 }` | ✅ |
|`Hsla::rotate_hue(180)` |  | `Hsla { hue: 0.0, saturation: 1.0, lightness: 0.5, alpha: 1.0 }` | `Hsla { hue: 0.0, saturation: 1.0, lightness: 0.5, alpha: 1.0 }` | ✅ |
|`Hsla::rotate_hue(360) is identity` |  | `Hsla { hue: 180.0, saturation: 1.0, lightness: 0.5, alpha: 1.0 }` | `Hsla { hue: 180.0, saturation: 1.0, lightness: 0.5, alpha: 1.0 }` | ✅ |
|`Hsla::set_hue` |  | `200.0` | `200.0` | ✅ |
|`Hsla::saturation()` |  | `0.3` | `0.3` | ✅ |
|`Hsla::set_saturation` |  | `0.9` | `0.9` | ✅ |

### `cylindrical::hsla_luminance_gray_and_dispersed`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Hsla::luminance == lightness` |  | `0.5` | `0.5` | ✅ |
|`Hsla::darker clamps at 0` |  | `0.0` | `0.0` | ✅ |
|`Hsla::lighter clamps at 1` |  | `1.0` | `1.0` | ✅ |
|`Hsla::gray(0.0) == BLACK` |  | `Hsla { hue: 0.0, saturation: 0.0, lightness: 0.0, alpha: 1.0 }` | `Hsla { hue: 0.0, saturation: 0.0, lightness: 0.0, alpha: 1.0 }` | ✅ |
|`Hsla::gray(1.0) == WHITE` |  | `Hsla { hue: 0.0, saturation: 0.0, lightness: 1.0, alpha: 1.0 }` | `Hsla { hue: 0.0, saturation: 0.0, lightness: 1.0, alpha: 1.0 }` | ✅ |
|`Hsla::sequential_dispersed(0).hue` |  | `0.0` | `0.0` | ✅ |
|`Hsla::sequential_dispersed(1).hue` |  | `222.49225` | `222.49225` | ✅ |
|`Hsla::sequential_dispersed(2).hue` |  | `84.984474` | `84.984474` | ✅ |
|`Hsla::sequential_dispersed(3).hue` |  | `307.4767` | `307.4767` | ✅ |
|`Hsla::sequential_dispersed(4).hue` |  | `169.96895` | `169.96895` | ✅ |

### `cylindrical::hsla_mix_takes_shortest_hue_path`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Hsla::mix hue 10->350 @ 0.25` |  | `5` | `5` | ✅ |
|`Hsla::mix hue 10->350 @ 0.5 (through 0)` |  | `0` | `0` | ✅ |
|`Hsla::mix hue 10->350 @ 0.75` |  | `355` | `355` | ✅ |
|`Hsla::mix hue 350->10 @ 0.5` |  | `0` | `0` | ✅ |
|`Hsla::mix_assign hue 10->20 @ 0.5` |  | `15` | `15` | ✅ |

### `cylindrical::hsva_surface`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Hsva::hsv sets alpha 1` |  | `1.0` | `1.0` | ✅ |
|`Hsva::new` |  | `Hsva { hue: 120.0, saturation: 0.5, value: 0.5, alpha: 1.0 }` | `Hsva { hue: 120.0, saturation: 0.5, value: 0.5, alpha: 1.0 }` | ✅ |
|`Hsva::with_saturation` |  | `0.2` | `0.2` | ✅ |
|`Hsva::with_value` |  | `0.2` | `0.2` | ✅ |
|`Hsva::hue()/rotate_hue` |  | `Hsva { hue: 60.0, saturation: 1.0, value: 1.0, alpha: 1.0 }` | `Hsva { hue: 60.0, saturation: 1.0, value: 1.0, alpha: 1.0 }` | ✅ |
|`Hsva::saturation() trait` |  | `0.4` | `0.4` | ✅ |
|`Hsva::gray(0.0) == BLACK` |  | `Hsva { hue: 0.0, saturation: 0.0, value: 0.0, alpha: 1.0 }` | `Hsva { hue: 0.0, saturation: 0.0, value: 0.0, alpha: 1.0 }` | ✅ |
|`Hsva -> Hsla -> Hsva round-trip` |  | `Hsva { hue: 180.0, saturation: 0.5, value: 0.5, alpha: 1.0 }` | `Hsva { hue: 180.0, saturation: 0.5, value: 0.5, alpha: 1.0 }` | ✅ |

### `cylindrical::hwba_surface`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Hwba::hwb sets alpha 1` |  | `1.0` | `1.0` | ✅ |
|`Hwba::new` |  | `Hwba { hue: 120.0, whiteness: 0.2, blackness: 0.3, alpha: 1.0 }` | `Hwba { hue: 120.0, whiteness: 0.2, blackness: 0.3, alpha: 1.0 }` | ✅ |
|`Hwba::with_whiteness` |  | `0.7` | `0.7` | ✅ |
|`Hwba::with_blackness` |  | `0.7` | `0.7` | ✅ |
|`Hwba::hue()/rotate_hue` |  | `Hwba { hue: 60.0, whiteness: 0.0, blackness: 0.0, alpha: 1.0 }` | `Hwba { hue: 60.0, whiteness: 0.0, blackness: 0.0, alpha: 1.0 }` | ✅ |
|`Hwba::gray(0.0) == BLACK` |  | `Hwba { hue: 0.0, whiteness: 0.0, blackness: 1.0, alpha: 1.0 }` | `Hwba { hue: 0.0, whiteness: 0.0, blackness: 1.0, alpha: 1.0 }` | ✅ |
|`Srgba::RED -> Hwba whiteness` |  | `0` | `0` | ✅ |
|`Srgba::RED -> Hwba blackness` |  | `0` | `0` | ✅ |
|`Hwba -> Hsva -> Hwba round-trip` |  | `Hwba { hue: 200.0, whiteness: 0.2, blackness: 0.3, alpha: 1.0 }` | `Hwba { hue: 200.0, whiteness: 0.19999999, blackness: 0.3, alpha: 1.0 }` | ✅ |

### `gradient::construction_needs_at_least_two_colors`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`ColorCurve::new([RED]) is Err (needs >= 2 colors)` |  | `true` | `true` | ✅ |
|`ColorCurve::new([RED, BLUE]) is Ok` |  | `true` | `true` | ✅ |

### `gradient::curve_adaptors_compose`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`ColorCurve.map(brighten) @ 0.0` |  | `Srgba { red: 1.0, green: 0.5, blue: 0.5, alpha: 1.0 }` | `Srgba { red: 1.0, green: 0.5, blue: 0.5, alpha: 1.0 }` | ✅ |
|`ColorCurve.map(brighten) @ 1.0` |  | `Srgba { red: 0.5, green: 0.5, blue: 1.0, alpha: 1.0 }` | `Srgba { red: 0.5, green: 0.5, blue: 1.0, alpha: 1.0 }` | ✅ |

### `gradient::domain_and_sampling`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`ColorCurve::domain == [0, len-1]` | 3 colors | `Interval { start: 0.0, end: 2.0 }` | `Interval { start: 0.0, end: 2.0 }` | ✅ |
|`ColorCurve::sample_clamped(0.0)` |  | `Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`ColorCurve::sample_clamped(1.0)` |  | `Srgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`ColorCurve::sample_clamped(2.0)` |  | `Srgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | `Srgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`ColorCurve::sample_clamped(0.5) == RED.mix(LIME, 0.5)` |  | `Srgba { red: 0.5, green: 0.5, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 0.5, green: 0.5, blue: 0.0, alpha: 1.0 }` | ✅ |
|`ColorCurve::sample(-0.1) is None (outside domain)` |  | `None` | `None` | ✅ |
|`ColorCurve::sample(2.1) is None (outside domain)` |  | `None` | `None` | ✅ |
|`ColorCurve::sample(1.0) is Some(LIME)` |  | `Some(Srgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 })` | `Some(Srgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 })` | ✅ |

### `layout::type_layout`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`size/align Srgba` |  | `size / align` | `16 / 4` | ✅ |
|`size/align LinearRgba` |  | `size / align` | `16 / 4` | ✅ |
|`size/align Hsla` |  | `size / align` | `16 / 4` | ✅ |
|`size/align Hsva` |  | `size / align` | `16 / 4` | ✅ |
|`size/align Hwba` |  | `size / align` | `16 / 4` | ✅ |
|`size/align Laba` |  | `size / align` | `16 / 4` | ✅ |
|`size/align Lcha` |  | `size / align` | `16 / 4` | ✅ |
|`size/align Oklaba` |  | `size / align` | `16 / 4` | ✅ |
|`size/align Oklcha` |  | `size / align` | `16 / 4` | ✅ |
|`size/align Xyza` |  | `size / align` | `16 / 4` | ✅ |
|`size/align Color (enum)` |  | `size / align` | `20 / 4` | ✅ |
|`every *a color type is 4 x f32` |  | `16` | `16` | ✅ |

### `linear_rgba::constants_and_constructors`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`LinearRgba::RED` |  | `LinearRgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`LinearRgba::BLACK` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`LinearRgba::WHITE` |  | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`LinearRgba::NONE (alpha 0)` |  | `0.0` | `0.0` | ✅ |
|`LinearRgba::default() == WHITE` |  | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`LinearRgba::NAN is all-NaN` |  | `true` | `true` | ✅ |
|`LinearRgba::rgb sets alpha 1` |  | `1.0` | `1.0` | ✅ |
|`LinearRgba::with_red` |  | `0.5` | `0.5` | ✅ |
|`LinearRgba::with_green` |  | `0.5` | `0.5` | ✅ |
|`LinearRgba::with_blue` |  | `0.5` | `0.5` | ✅ |

### `linear_rgba::luminance_cie_weights`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`LinearRgba::luminance (WHITE = .2126+.7152+.0722)` |  | `1` | `1` | ✅ |
|`LinearRgba::luminance (RED)` |  | `0.2126` | `0.2126` | ✅ |
|`LinearRgba::luminance (GREEN)` |  | `0.7152` | `0.7152` | ✅ |
|`LinearRgba::luminance (BLUE)` |  | `0.0722` | `0.0722` | ✅ |
|`LinearRgba::with_luminance keeps target` | gray -> 0.5 | `0.5` | `0.5` | ✅ |
|`LinearRgba::darker distributive` |  | `0` | `0.0000000000000008881784` | ✅ |
|`LinearRgba::lighter distributive` |  | `0` | `0.0000000000000071054274` | ✅ |

### `linear_rgba::packing`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`LinearRgba::as_u32 (blue, opaque)` | A=MSB, R=LSB, LE | `4294901760` | `4294901760` | ✅ |
|`LinearRgba::to_u8_array` | BLUE | `[0, 0, 255, 255]` | `[0, 0, 255, 255]` | ✅ |
|`LinearRgba::to_u8_array_no_alpha` | cyan | `[0, 255, 255]` | `[0, 255, 255]` | ✅ |
|`LinearRgba::from_u8_array` | [255,0,0,255] | `LinearRgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`LinearRgba::from_u8_array_no_alpha` | [255,255,0] | `LinearRgba { red: 1.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`LinearRgba::to_u8_array clamps` | rgb(0,100,-100) | `[0, 255, 0]` | `[0, 255, 0]` | ✅ |

### `linear_rgba::traits`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`LinearRgba::mix(RED, BLUE, 0.5)` |  | `LinearRgba { red: 0.5, green: 0.0, blue: 0.5, alpha: 1.0 }` | `LinearRgba { red: 0.5, green: 0.0, blue: 0.5, alpha: 1.0 }` | ✅ |
|`LinearRgba::distance_squared (BLACK<->WHITE)` |  | `3` | `3` | ✅ |
|`LinearRgba::with_alpha` |  | `0.25` | `0.25` | ✅ |
|`LinearRgba::gray(0.0) == BLACK` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`LinearRgba::gray(1.0) == WHITE` |  | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`LinearRgba::gray(0.5)` |  | `LinearRgba { red: 0.5, green: 0.5, blue: 0.5, alpha: 1.0 }` | `LinearRgba { red: 0.5, green: 0.5, blue: 0.5, alpha: 1.0 }` | ✅ |
|`LinearRgba::to_vec3` | RED | `Vec3(1.0, 0.0, 0.0)` | `Vec3(1.0, 0.0, 0.0)` | ✅ |
|`LinearRgba::from_vec3 round-trip` |  | `LinearRgba { red: 0.2, green: 0.4, blue: 0.6, alpha: 1.0 }` | `LinearRgba { red: 0.2, green: 0.4, blue: 0.6, alpha: 1.0 }` | ✅ |
|`LinearRgba + LinearRgba` | RED+GREEN | `LinearRgba { red: 1.0, green: 1.0, blue: 0.0, alpha: 2.0 }` | `LinearRgba { red: 1.0, green: 1.0, blue: 0.0, alpha: 2.0 }` | ✅ |
|`LinearRgba * f32` | RED*0.5 | `LinearRgba { red: 0.5, green: 0.0, blue: 0.0, alpha: 0.5 }` | `LinearRgba { red: 0.5, green: 0.0, blue: 0.0, alpha: 0.5 }` | ✅ |
|`LinearRgba VectorSpace::ZERO` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 }` | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 }` | ✅ |

### `ops::alpha_for_f32`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`f32::alpha() is itself` | 0.5 | `0.5` | `0.5` | ✅ |
|`f32::with_alpha() replaces the value` | 0.5 -> 0.9 | `0.9` | `0.9` | ✅ |
|`f32::set_alpha` |  | `0.8` | `0.8` | ✅ |
|`f32::is_fully_opaque (1.0)` |  | `true` | `true` | ✅ |
|`f32::is_fully_transparent (0.0)` |  | `true` | `true` | ✅ |

### `ops::color_range`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`(RED..BLUE).at(-0.5) clamps to start` |  | `Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`(RED..BLUE).at(0.0)` |  | `Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`(RED..BLUE).at(0.5)` |  | `Srgba { red: 0.5, green: 0.0, blue: 0.5, alpha: 1.0 }` | `Srgba { red: 0.5, green: 0.0, blue: 0.5, alpha: 1.0 }` | ✅ |
|`(RED..BLUE).at(1.0)` |  | `Srgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | `Srgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`(RED..BLUE).at(1.5) clamps to end` |  | `Srgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | `Srgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`(LinearRgba RED..BLUE).at(0.5)` |  | `LinearRgba { red: 0.5, green: 0.0, blue: 0.5, alpha: 1.0 }` | `LinearRgba { red: 0.5, green: 0.0, blue: 0.5, alpha: 1.0 }` | ✅ |

### `ops::gray_black_and_white_every_space`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Srgba::gray(0.0) == BLACK` |  | `Srgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba::gray(1.0) == WHITE` |  | `Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`LinearRgba::gray(0.0) == BLACK` |  | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`LinearRgba::gray(1.0) == WHITE` |  | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`Hsla::gray(0.0) == BLACK` |  | `Hsla { hue: 0.0, saturation: 0.0, lightness: 0.0, alpha: 1.0 }` | `Hsla { hue: 0.0, saturation: 0.0, lightness: 0.0, alpha: 1.0 }` | ✅ |
|`Hsla::gray(1.0) == WHITE` |  | `Hsla { hue: 0.0, saturation: 0.0, lightness: 1.0, alpha: 1.0 }` | `Hsla { hue: 0.0, saturation: 0.0, lightness: 1.0, alpha: 1.0 }` | ✅ |
|`Hsva::gray(0.0) == BLACK` |  | `Hsva { hue: 0.0, saturation: 0.0, value: 0.0, alpha: 1.0 }` | `Hsva { hue: 0.0, saturation: 0.0, value: 0.0, alpha: 1.0 }` | ✅ |
|`Hsva::gray(1.0) == WHITE` |  | `Hsva { hue: 0.0, saturation: 0.0, value: 1.0, alpha: 1.0 }` | `Hsva { hue: 0.0, saturation: 0.0, value: 1.0, alpha: 1.0 }` | ✅ |
|`Hwba::gray(0.0) == BLACK` |  | `Hwba { hue: 0.0, whiteness: 0.0, blackness: 1.0, alpha: 1.0 }` | `Hwba { hue: 0.0, whiteness: 0.0, blackness: 1.0, alpha: 1.0 }` | ✅ |
|`Hwba::gray(1.0) == WHITE` |  | `Hwba { hue: 0.0, whiteness: 1.0, blackness: 0.0, alpha: 1.0 }` | `Hwba { hue: 0.0, whiteness: 1.0, blackness: 0.0, alpha: 1.0 }` | ✅ |
|`Laba::gray(0.0) == BLACK` |  | `Laba { lightness: 0.0, a: 0.0, b: 0.0, alpha: 1.0 }` | `Laba { lightness: 0.0, a: 0.0, b: 0.0, alpha: 1.0 }` | ✅ |
|`Laba::gray(1.0) == WHITE` |  | `Laba { lightness: 1.0, a: 0.0, b: 0.0, alpha: 1.0 }` | `Laba { lightness: 1.0, a: 0.0, b: 0.0, alpha: 1.0 }` | ✅ |
|`Lcha::gray(0.0) == BLACK` |  | `Lcha { lightness: 0.0, chroma: 0.0, hue: 1.36603785e-5, alpha: 1.0 }` | `Lcha { lightness: 0.0, chroma: 0.0, hue: 1.36603785e-5, alpha: 1.0 }` | ✅ |
|`Lcha::gray(1.0) == WHITE` |  | `Lcha { lightness: 1.0, chroma: 0.0, hue: 1.36603785e-5, alpha: 1.0 }` | `Lcha { lightness: 1.0, chroma: 0.0, hue: 1.36603785e-5, alpha: 1.0 }` | ✅ |
|`Oklaba::gray(0.0) == BLACK` |  | `Oklaba { lightness: 0.0, a: 0.0, b: 0.0, alpha: 1.0 }` | `Oklaba { lightness: 0.0, a: 0.0, b: 0.0, alpha: 1.0 }` | ✅ |
|`Oklaba::gray(1.0) == WHITE` |  | `Oklaba { lightness: 1.0, a: 0.0, b: 5.9604645e-8, alpha: 1.0 }` | `Oklaba { lightness: 1.0, a: 0.0, b: 5.9604645e-8, alpha: 1.0 }` | ✅ |
|`Oklcha::gray(0.0) == BLACK` |  | `Oklcha { lightness: 0.0, chroma: 0.0, hue: 0.0, alpha: 1.0 }` | `Oklcha { lightness: 0.0, chroma: 0.0, hue: 0.0, alpha: 1.0 }` | ✅ |
|`Oklcha::gray(1.0) == WHITE` |  | `Oklcha { lightness: 1.0, chroma: 5.9604645e-8, hue: 90.0, alpha: 1.0 }` | `Oklcha { lightness: 1.0, chroma: 5.9604645e-8, hue: 90.0, alpha: 1.0 }` | ✅ |
|`Xyza::gray(0.0) == BLACK` |  | `Xyza { x: 0.0, y: 0.0, z: 0.0, alpha: 1.0 }` | `Xyza { x: 0.0, y: 0.0, z: 0.0, alpha: 1.0 }` | ✅ |
|`Xyza::gray(1.0) == WHITE` |  | `Xyza { x: 0.95047, y: 1.0, z: 1.08883, alpha: 1.0 }` | `Xyza { x: 0.95047, y: 1.0, z: 1.08883, alpha: 1.0 }` | ✅ |

### `palettes::basic_palette`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`basic::RED` |  | `Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`basic::BLACK` |  | `Srgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`basic::WHITE` |  | `Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`basic::GRAY (VGA 50.19%)` |  | `Srgba { red: 0.5019608, green: 0.5019608, blue: 0.5019608, alpha: 1.0 }` | `Srgba { red: 0.5019608, green: 0.5019608, blue: 0.5019608, alpha: 1.0 }` | ✅ |
|`basic::LIME` |  | `Srgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | ✅ |

### `palettes::css_palette`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`css::REBECCA_PURPLE` |  | `Srgba { red: 0.4, green: 0.2, blue: 0.6, alpha: 1.0 }` | `Srgba { red: 0.4, green: 0.2, blue: 0.6, alpha: 1.0 }` | ✅ |
|`css::ALICE_BLUE` |  | `Srgba { red: 0.941, green: 0.973, blue: 1.0, alpha: 1.0 }` | `Srgba { red: 0.941, green: 0.973, blue: 1.0, alpha: 1.0 }` | ✅ |
|`css::WHITE_SMOKE` |  | `Srgba { red: 0.961, green: 0.961, blue: 0.961, alpha: 1.0 }` | `Srgba { red: 0.961, green: 0.961, blue: 0.961, alpha: 1.0 }` | ✅ |

### `palettes::tailwind_palette`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`tailwind::BLUE_500` |  | `Srgba { red: 0.23137255, green: 0.50980395, blue: 0.9647059, alpha: 1.0 }` | `Srgba { red: 0.23137255, green: 0.50980395, blue: 0.9647059, alpha: 1.0 }` | ✅ |
|`tailwind::RED_500` |  | `Srgba { red: 0.9372549, green: 0.26666668, blue: 0.26666668, alpha: 1.0 }` | `Srgba { red: 0.9372549, green: 0.26666668, blue: 0.26666668, alpha: 1.0 }` | ✅ |
|`tailwind::SLATE_50` |  | `Srgba { red: 0.972549, green: 0.98039216, blue: 0.9882353, alpha: 1.0 }` | `Srgba { red: 0.972549, green: 0.98039216, blue: 0.9882353, alpha: 1.0 }` | ✅ |
|`tailwind::GRAY_950` |  | `Srgba { red: 0.011764706, green: 0.02745098, blue: 0.07058824, alpha: 1.0 }` | `Srgba { red: 0.011764706, green: 0.02745098, blue: 0.07058824, alpha: 1.0 }` | ✅ |

### `perceptual::laba_surface`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Laba::lab sets alpha 1` |  | `1.0` | `1.0` | ✅ |
|`Laba::new` |  | `Laba { lightness: 0.5, a: 0.1, b: -0.1, alpha: 1.0 }` | `Laba { lightness: 0.5, a: 0.1, b: -0.1, alpha: 1.0 }` | ✅ |
|`Laba::with_lightness` |  | `0.2` | `0.2` | ✅ |
|`Laba::CIE_EPSILON == 216/24389` |  | `0.008856452` | `0.008856452` | ✅ |
|`Laba::CIE_KAPPA == 24389/27` |  | `903.2963` | `903.2963` | ✅ |
|`Laba::luminance == lightness` |  | `0.7` | `0.7` | ✅ |
|`Laba::gray(0.0) == BLACK` |  | `Laba { lightness: 0.0, a: 0.0, b: 0.0, alpha: 1.0 }` | `Laba { lightness: 0.0, a: 0.0, b: 0.0, alpha: 1.0 }` | ✅ |
|`Laba::gray(1.0) == WHITE` |  | `Laba { lightness: 1.0, a: 0.0, b: 0.0, alpha: 1.0 }` | `Laba { lightness: 1.0, a: 0.0, b: 0.0, alpha: 1.0 }` | ✅ |
|`Laba::mix @ 0.5` |  | `Laba { lightness: 0.5, a: 0.0, b: 0.0, alpha: 1.0 }` | `Laba { lightness: 0.5, a: 0.0, b: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba::RED -> Laba lightness` |  | `0.532408` | `0.5324079` | ✅ |
|`Srgba -> Laba -> Srgba (red)` |  | `Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 0.9999997, green: 1.1318247e-6, blue: -2.2351742e-8, alpha: 1.0 }` | ✅ |

### `perceptual::lcha_surface`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Lcha::lch sets alpha 1` |  | `1.0` | `1.0` | ✅ |
|`Lcha::new` |  | `Lcha { lightness: 0.5, chroma: 0.1, hue: 90.0, alpha: 1.0 }` | `Lcha { lightness: 0.5, chroma: 0.1, hue: 90.0, alpha: 1.0 }` | ✅ |
|`Lcha::with_chroma` |  | `0.3` | `0.3` | ✅ |
|`Lcha::with_lightness` |  | `0.2` | `0.2` | ✅ |
|`Lcha::luminance == lightness` |  | `0.4` | `0.4` | ✅ |
|`Lcha::hue()/rotate_hue` |  | `120.0` | `120.0` | ✅ |
|`Lcha::gray(0.0) == BLACK` |  | `Lcha { lightness: 0.0, chroma: 0.0, hue: 1.36603785e-5, alpha: 1.0 }` | `Lcha { lightness: 0.0, chroma: 0.0, hue: 1.36603785e-5, alpha: 1.0 }` | ✅ |
|`Lcha::sequential_dispersed(0).hue` |  | `0.0` | `0.0` | ✅ |
|`Lcha::sequential_dispersed(1).hue` |  | `222.49225` | `222.49225` | ✅ |
|`Lcha::sequential_dispersed(2).hue` |  | `84.984474` | `84.984474` | ✅ |
|`Lcha -> Laba -> Lcha round-trip` |  | `Lcha { lightness: 0.6, chroma: 0.4, hue: 120.0, alpha: 1.0 }` | `Lcha { lightness: 0.6, chroma: 0.4, hue: 120.00001, alpha: 1.0 }` | ✅ |

### `perceptual::oklaba_surface`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Oklaba::lab sets alpha 1` |  | `1.0` | `1.0` | ✅ |
|`Oklaba::new` |  | `Oklaba { lightness: 0.5, a: 0.1, b: -0.1, alpha: 1.0 }` | `Oklaba { lightness: 0.5, a: 0.1, b: -0.1, alpha: 1.0 }` | ✅ |
|`Oklaba::with_lightness` |  | `0.2` | `0.2` | ✅ |
|`Oklaba::with_a` |  | `0.3` | `0.3` | ✅ |
|`Oklaba::with_b` |  | `0.3` | `0.3` | ✅ |
|`Oklaba::gray(0.0) == BLACK` |  | `Oklaba { lightness: 0.0, a: 0.0, b: 0.0, alpha: 1.0 }` | `Oklaba { lightness: 0.0, a: 0.0, b: 0.0, alpha: 1.0 }` | ✅ |
|`Oklaba::distance (BLACK<->WHITE)` | = L difference | `1` | `1` | ✅ |
|`Oklaba::mix @ 0.5` |  | `Oklaba { lightness: 0.5, a: 0.0, b: 2.9802322e-8, alpha: 1.0 }` | `Oklaba { lightness: 0.5, a: 0.0, b: 2.9802322e-8, alpha: 1.0 }` | ✅ |
|`LinearRgba::RED -> Oklaba` |  | `Oklaba { lightness: 0.6279554, a: 0.22486295, b: 0.1258463, alpha: 1.0 }` | `Oklaba { lightness: 0.6279554, a: 0.22486295, b: 0.1258463, alpha: 1.0 }` | ✅ |
|`LinearRgba -> Oklaba -> LinearRgba (red)` |  | `LinearRgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `LinearRgba { red: 0.9999999, green: 5.401671e-8, blue: -4.4703484e-8, alpha: 1.0 }` | ✅ |

### `perceptual::oklcha_surface`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Oklcha::lch sets alpha 1` |  | `1.0` | `1.0` | ✅ |
|`Oklcha::new` |  | `Oklcha { lightness: 0.5, chroma: 0.1, hue: 90.0, alpha: 1.0 }` | `Oklcha { lightness: 0.5, chroma: 0.1, hue: 90.0, alpha: 1.0 }` | ✅ |
|`Oklcha::with_lightness` |  | `0.2` | `0.2` | ✅ |
|`Oklcha::with_chroma` |  | `0.3` | `0.3` | ✅ |
|`Oklcha::hue()/rotate_hue` |  | `120.0` | `120.0` | ✅ |
|`Oklcha::gray(0.0) == BLACK` |  | `Oklcha { lightness: 0.0, chroma: 0.0, hue: 0.0, alpha: 1.0 }` | `Oklcha { lightness: 0.0, chroma: 0.0, hue: 0.0, alpha: 1.0 }` | ✅ |
|`Oklcha::distance (RED vs BLUE) > 0` |  | `true` | `true` | ✅ |
|`Oklcha -> Oklaba -> Oklcha round-trip` |  | `Oklcha { lightness: 0.7, chroma: 0.2, hue: 200.0, alpha: 1.0 }` | `Oklcha { lightness: 0.7, chroma: 0.2, hue: 200.0, alpha: 1.0 }` | ✅ |

### `perceptual::xyza_surface`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Xyza::xyz sets alpha 1` |  | `1.0` | `1.0` | ✅ |
|`Xyza::new` |  | `Xyza { x: 0.3, y: 0.4, z: 0.5, alpha: 1.0 }` | `Xyza { x: 0.3, y: 0.4, z: 0.5, alpha: 1.0 }` | ✅ |
|`Xyza::with_x` |  | `0.9` | `0.9` | ✅ |
|`Xyza::with_y` |  | `0.9` | `0.9` | ✅ |
|`Xyza::with_z` |  | `0.9` | `0.9` | ✅ |
|`Xyza::D65_WHITE` |  | `Xyza { x: 0.95047, y: 1.0, z: 1.08883, alpha: 1.0 }` | `Xyza { x: 0.95047, y: 1.0, z: 1.08883, alpha: 1.0 }` | ✅ |
|`Xyza::luminance == y` |  | `0.42` | `0.42` | ✅ |
|`Xyza::gray(0.0) == BLACK` |  | `Xyza { x: 0.0, y: 0.0, z: 0.0, alpha: 1.0 }` | `Xyza { x: 0.0, y: 0.0, z: 0.0, alpha: 1.0 }` | ✅ |
|`Xyza::mix @ 0.5` |  | `Xyza { x: 0.475235, y: 0.5, z: 0.544415, alpha: 1.0 }` | `Xyza { x: 0.475235, y: 0.5, z: 0.544415, alpha: 1.0 }` | ✅ |
|`LinearRgba::WHITE -> Xyza ~ D65_WHITE` |  | `Xyza { x: 0.95047, y: 1.0, z: 1.08883, alpha: 1.0 }` | `Xyza { x: 0.95047003, y: 1.0000001, z: 1.08883, alpha: 1.0 }` | ✅ |
|`LinearRgba -> Xyza -> LinearRgba (white)` |  | `LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `LinearRgba { red: 1.0, green: 1.0000001, blue: 1.0, alpha: 1.0 }` | ✅ |

### `srgba::color_op_traits`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Srgba::mix(RED, BLUE, 0.5)` |  | `Srgba { red: 0.5, green: 0.0, blue: 0.5, alpha: 1.0 }` | `Srgba { red: 0.5, green: 0.0, blue: 0.5, alpha: 1.0 }` | ✅ |
|`Srgba::luminance (WHITE)` |  | `1` | `1` | ✅ |
|`Srgba::luminance (BLACK)` |  | `0` | `0` | ✅ |
|`Srgba::darker(0.1).darker(0.1) ~ darker(0.2)` | distributive | `0` | `0.0000000000000033861802` | ✅ |
|`Srgba::with_alpha / alpha()` | 0.25 | `0.25` | `0.25` | ✅ |
|`Srgba::is_fully_opaque (WHITE)` |  | `true` | `true` | ✅ |
|`Srgba::is_fully_transparent (NONE)` |  | `true` | `true` | ✅ |
|`Srgba::set_alpha` | WHITE -> 0 | `0` | `0` | ✅ |
|`Srgba::distance_squared (BLACK<->WHITE)` |  | `3` | `3` | ✅ |
|`Srgba::distance (BLACK<->WHITE)` |  | `1.7320508` | `1.7320508` | ✅ |
|`Srgba::gray(0.0) == BLACK` |  | `Srgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba::gray(1.0) == WHITE` |  | `Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | ✅ |

### `srgba::components_and_vector_space`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Srgba::to_f32_array` | RED | `[1.0, 0.0, 0.0, 1.0]` | `[1.0, 0.0, 0.0, 1.0]` | ✅ |
|`Srgba::to_f32_array_no_alpha` | RED | `[1.0, 0.0, 0.0]` | `[1.0, 0.0, 0.0]` | ✅ |
|`Srgba::to_vec4` | RED | `Vec4(1.0, 0.0, 0.0, 1.0)` | `Vec4(1.0, 0.0, 0.0, 1.0)` | ✅ |
|`Srgba::from_vec4 round-trip` | RED | `Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba::from_f32_array_no_alpha sets alpha 1` |  | `1.0` | `1.0` | ✅ |
|`Srgba + Srgba (incl. alpha)` | RED+GREEN | `Srgba { red: 1.0, green: 1.0, blue: 0.0, alpha: 2.0 }` | `Srgba { red: 1.0, green: 1.0, blue: 0.0, alpha: 2.0 }` | ✅ |
|`Srgba - Srgba` | WHITE-RED | `Srgba { red: 0.0, green: 1.0, blue: 1.0, alpha: 0.0 }` | `Srgba { red: 0.0, green: 1.0, blue: 1.0, alpha: 0.0 }` | ✅ |
|`Srgba * f32` | RED*0.5 | `Srgba { red: 0.5, green: 0.0, blue: 0.0, alpha: 0.5 }` | `Srgba { red: 0.5, green: 0.0, blue: 0.0, alpha: 0.5 }` | ✅ |
|`f32 * Srgba` | 0.5*RED | `Srgba { red: 0.5, green: 0.0, blue: 0.0, alpha: 0.5 }` | `Srgba { red: 0.5, green: 0.0, blue: 0.0, alpha: 0.5 }` | ✅ |
|`Srgba / f32` | RED/2 | `Srgba { red: 0.5, green: 0.0, blue: 0.0, alpha: 0.5 }` | `Srgba { red: 0.5, green: 0.0, blue: 0.0, alpha: 0.5 }` | ✅ |
|`-Srgba (Neg)` | -RED | `Srgba { red: -1.0, green: 0.0, blue: 0.0, alpha: -1.0 }` | `Srgba { red: -1.0, green: -0.0, blue: -0.0, alpha: -1.0 }` | ✅ |
|`Srgba VectorSpace::ZERO` |  | `Srgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 }` | `Srgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 }` | ✅ |
|`Srgba::add_assign` | RED+=GREEN | `Srgba { red: 1.0, green: 1.0, blue: 0.0, alpha: 2.0 }` | `Srgba { red: 1.0, green: 1.0, blue: 0.0, alpha: 2.0 }` | ✅ |
|`Srgba::interpolate_stable(BLACK, WHITE, 0.5)` |  | `Srgba { red: 0.5, green: 0.5, blue: 0.5, alpha: 1.0 }` | `Srgba { red: 0.5, green: 0.5, blue: 0.5, alpha: 1.0 }` | ✅ |

### `srgba::constants_and_constructors`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Srgba::RED` |  | `Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba::GREEN` |  | `Srgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba::BLUE` |  | `Srgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | `Srgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`Srgba::BLACK` |  | `Srgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba::WHITE` |  | `Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`Srgba::NONE (alpha 0)` |  | `0.0` | `0.0` | ✅ |
|`Srgba::rgb sets alpha 1` | rgb(0.2,0.4,0.6) | `1.0` | `1.0` | ✅ |
|`Srgba::default() == WHITE` |  | `Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | `Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }` | ✅ |
|`Srgba::with_red` | WHITE.with_red(0) | `0.0` | `0.0` | ✅ |
|`Srgba::with_green` | WHITE.with_green(0) | `0.0` | `0.0` | ✅ |
|`Srgba::with_blue` | WHITE.with_blue(0) | `0.0` | `0.0` | ✅ |

### `srgba::gamma_correction`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Srgba::gamma_function(0.0)` |  | `0` | `0` | ✅ |
|`Srgba::gamma_function(1.0)` |  | `1` | `1` | ✅ |
|`Srgba::gamma_function_inverse(1.0)` |  | `1` | `0.99999994` | ✅ |
|`Srgba::gamma_function(0.5) (gamma branch)` |  | `0.21404114` | `0.21404114` | ✅ |
|`gamma_function round-trip` | 0.37 | `0.37` | `0.37` | ✅ |
|`Srgba::gamma_function(0.03) (linear branch)` |  | `0.0023219814` | `0.0023219814` | ✅ |

### `srgba::hex_parsing`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Srgba::hex("FFF")` |  | `Ok(Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 })` | `Ok(Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 })` | ✅ |
|`Srgba::hex("FFFF")` |  | `Ok(Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 })` | `Ok(Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 })` | ✅ |
|`Srgba::hex("FFFFFF")` |  | `Ok(Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 })` | `Ok(Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 })` | ✅ |
|`Srgba::hex("FFFFFFFF")` |  | `Ok(Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 })` | `Ok(Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 })` | ✅ |
|`Srgba::hex("#FFFFFF") strips '#'` |  | `Ok(Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 })` | `Ok(Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 })` | ✅ |
|`Srgba::hex("000")` |  | `Ok(Srgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 })` | `Ok(Srgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 })` | ✅ |
|`Srgba::hex("03a9f4")` |  | `Ok(Srgba { red: 0.011764706, green: 0.6627451, blue: 0.95686275, alpha: 1.0 })` | `Ok(Srgba { red: 0.011764706, green: 0.6627451, blue: 0.95686275, alpha: 1.0 })` | ✅ |
|`Srgba::hex("#f2a") (RGB nibble expand)` |  | `Ok(Srgba { red: 1.0, green: 0.13333334, blue: 0.6666667, alpha: 1.0 })` | `Ok(Srgba { red: 1.0, green: 0.13333334, blue: 0.6666667, alpha: 1.0 })` | ✅ |
|`Srgba::hex("12345678")` |  | `Ok(Srgba { red: 0.07058824, green: 0.20392157, blue: 0.3372549, alpha: 0.47058824 })` | `Ok(Srgba { red: 0.07058824, green: 0.20392157, blue: 0.3372549, alpha: 0.47058824 })` | ✅ |
|`Srgba::hex("yy") -> Length err` |  | `Err(Length)` | `Err(Length)` | ✅ |
|`Srgba::hex("#ff") -> Length err` |  | `Err(Length)` | `Err(Length)` | ✅ |
|`Srgba::hex("yyy") -> Parse err` |  | `Parse(_)` | `Parse(_)` | ✅ |

### `srgba::hex_roundtrip_and_u8`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Srgba::to_hex (opaque)` | rgb_u8(3,169,244) | `#03A9F4` | `#03A9F4` | ✅ |
|`Srgba::to_hex (with alpha)` | rgba_u8(1,2,3,4) | `#01020304` | `#01020304` | ✅ |
|`Srgba::rgb_u8(255,0,0) == RED` |  | `Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba::to_u8_array` | BLUE | `[0, 0, 255, 255]` | `[0, 0, 255, 255]` | ✅ |
|`Srgba::to_u8_array_no_alpha` | GREEN | `[0, 255, 0]` | `[0, 255, 0]` | ✅ |
|`Srgba::from_u8_array` | [255,0,0,255] | `Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | `Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }` | ✅ |
|`Srgba::to_u8_array clamps out-of-range` | rgb(2,-1,0.5) | `[255, 0, 128]` | `[255, 0, 128]` | ✅ |

### `srgba::srgba_linear_roundtrip`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Srgba->LinearRgba green (0.5 -> ~0.214)` |  | `0.21404114` | `0.21404114` | ✅ |
|`Srgba->LinearRgba keeps alpha` |  | `1` | `1` | ✅ |
|`Srgba->LinearRgba->Srgba round-trip` |  | `Srgba { red: 0.0, green: 0.5, blue: 1.0, alpha: 1.0 }` | `Srgba { red: 0.0, green: 0.5, blue: 0.99999994, alpha: 1.0 }` | ✅ |

## Nicht abgedeckt

`bevy_reflect`, `serialize` (serde), `wgpu-types`-Interop, `encase`/`ShaderType` (nur für `LinearRgba`, GPU-Uniform-Pfad), vollständige `palettes::css`/`palettes::tailwind`-Tabellen (nur Stichproben — die Konstanten sind reine Daten).

**Caveat:** nur Emulator. Siehe `../../port.md` §B10.
