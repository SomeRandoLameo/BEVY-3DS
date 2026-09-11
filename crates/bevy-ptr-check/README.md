# bevy-ptr-check

On-device check that **`bevy_ptr`** — the type-erased raw-pointer toolkit `bevy_ecs` builds its dynamic component storage on — behaves correctly on the Nintendo 3DS. Aims to touch every public item.

| | |
|---|---|
| Bevy-Einheit | `bevy_ptr` (keine Dependencies, keine Features — reiner `#![no_std]`-Zeigercode) |
| Version | `0.19.1` |
| Features | keine (die Crate hat schlicht keine) |
| Target | `armv6k-nintendo-3ds` |
| Toleranz | exakte Werte-/Adressvergleiche — keine Fließkomma-Toleranz nötig |
| Stand | 2026-09-11 · Azahar-Emulator · **39/39 Testfunktionen · 69/69 Checks bestanden** (+ 1 `#[should_panic]`-Test ohne Tabellenzeile) |

Ausführen: `./scripts/test-emulator.sh -p bevy-ptr-check`

## Warum diese Crate überhaupt testen?

`bevy_ptr` hat keine Features und keine Dependencies — auf den ersten Blick "kann da nichts kaputt gehen". Aber es ist **fast ausschließlich `unsafe`-Code**, der rohe Zeigerarithmetik, Typ-Erasure und (De-)Alignment macht — genau die Art Code, bei der sich Plattformunterschiede (Zeigergröße, Alignment-Anforderungen, ob unausgerichtete Zugriffe überhaupt funktionieren) bemerkbar machen könnten. `armv6k` (ARM11, die 3DS-CPU) ist 32-bit und unterstützt unausgerichtete Lese-/Schreibzugriffe für die meisten Load/Store-Instruktionen — aber das ist genau die Annahme, die `bevy_ptr`s `Aligned`/`Unaligned`-Unterscheidung kodiert, und die hier tatsächlich durch einen echten unausgerichteten Read/Write-Test verifiziert wird (`checks::alignment`), statt nur angenommen zu werden.

## Abgedeckt

**`ConstNonNull<T>`** — `new` (Some/None für null/non-null), `new_unchecked`, `as_ref`, `From<NonNull<T>>`/`From<&T>`/`From<&mut T>`, `Copy`/`Clone`.

**`Ptr<'a, A>`** — `from(&T)`, `deref::<T>()`, `as_ptr()`, `byte_offset`/`byte_add` (Indizierung in ein Array), `to_unaligned()`, `assert_unique()` → `PtrMut` (über einen echt exklusiven, `ManuallyDrop`-gehaltenen Wert, wie `bevy_ptr` es intern selbst rechtfertigt).

**`PtrMut<'a, A>`** — `from(&mut T)`, `deref_mut::<T>()`, `as_ptr()`, `reborrow()` (kürzer-lebige Sub-Ausleihe, Mutation bleibt sichtbar), `as_ref()` → `Ptr`, `promote()` → `OwningPtr`.

**`OwningPtr<'a, A>`** — `make()` (die sichere Konstruktion via Closure), `read::<T>()` (kein Doppel-Drop), `drop_as::<T>()` (läuft der Destruktor wirklich, per `DropTracker`-Zähler verifiziert), `cast::<T>()` → `MovingPtr`, `as_ref`/`as_mut`, `OwningPtr<Unaligned>::read_unaligned::<T>()`.

**`MovingPtr<'a, T, A>`** + die Makros `move_as_ptr!`/`deconstruct_moving_ptr!` — `read()`, `write_to()` (schreibt ohne das Ziel vorher zu droppen), `assign_to()` (droppt das alte Ziel, verschiebt den neuen Wert rein — per `DropTracker` gegengeprüft), `Drop`-Verhalten wenn ein `MovingPtr` nie konsumiert wird (der Pointee wird trotzdem korrekt gedroppt), Struct-/Tuple-/`MaybeUninit`-Dekonstruktion (adaptiert aus `bevy_ptr`s eigenen Doku-Beispielen), `partial_move()`, `From<MovingPtr> for OwningPtr`.

**`ThinSlicePtr<'a, T>`** — `From<&[T]>`, `get_unchecked`, `as_slice_unchecked`, `Copy`/`Clone`; die `UnsafeCell<T>`-Spezialisierung: `as_mut_slice_unchecked` (Mutation durch eine geteilte Slice hindurch), `cast`.

**`UnsafeCellDeref`** — `deref`, `deref_mut`, `read` (für `Copy`-Typen).

**Alignment (`checks::alignment`)** — ein echter unausgerichteter 4-Byte-Read (`OwningPtr<Unaligned>::read_unaligned::<u32>()`) über einen absichtlich fehlausgerichteten Byte-Puffer, der auf `armv6k` den korrekten Wert liefert; plus ein `#[should_panic]`-Test, der bestätigt, dass der `Aligned`-Pfad (`Ptr::deref`) einen fehlausgerichteten Zeiger im Debug-Build tatsächlich per Assertion ablehnt statt ihn stillschweigend zu akzeptieren.

**Layout** — `size_of`/`align_of` von `Ptr`, `PtrMut`, `OwningPtr`, `ConstNonNull<u32>`, `ThinSlicePtr<u32>`; Bestätigung, dass `Ptr`/`PtrMut`/`OwningPtr`/`ConstNonNull` exakt zeigergroß sind (kein Tag-Overhead, `repr(transparent)` über `NonNull<u8>`).

## Ergebnis

Funktion · Eingabe · Erwartet · **tatsächliche Ausgabe** · OK. Reihenfolge = Testausführung (alphabetisch nach Modul/Funktion). Ein Test (`alignment::aligned_deref_panics_in_debug_on_a_misaligned_pointer`) hat keine Tabellenzeile — er prüft nur, dass `Ptr::deref` auf einem absichtlich fehlausgerichteten Zeiger im Debug-Build panickt (`#[should_panic(expected = "not aligned")]`).

### `alignment::to_unaligned_lets_a_misaligned_ptr_still_be_constructed`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Ptr::to_unaligned() keeps the (misaligned) address` |  | `138429201` | `138429201` | ✅ |

### `alignment::unaligned_read_across_a_misaligned_offset_returns_the_right_value`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`test setup: offset+1 really is misaligned for u32` |  | `true` | `true` | ✅ |
|`OwningPtr<Unaligned>::read_unaligned::<u32>() on a misaligned ARM load` |  | `3735928559` | `3735928559` | ✅ |

### `const_non_null::from_conversions`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`ConstNonNull::from(&T)` |  | `5` | `5` | ✅ |
|`ConstNonNull::from(&mut T)` |  | `6` | `6` | ✅ |
|`ConstNonNull::from(NonNull<T>)` |  | `9` | `9` | ✅ |

### `const_non_null::is_copy_and_clone`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`ConstNonNull is Copy — original still usable` |  | `1` | `1` | ✅ |
|`ConstNonNull::clone() reads the same value` |  | `1` | `1` | ✅ |

### `const_non_null::new_rejects_null_accepts_non_null`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`ConstNonNull::new(&x) is Some` |  | `true` | `true` | ✅ |
|`ConstNonNull::new(null) is None` |  | `true` | `true` | ✅ |

### `const_non_null::new_unchecked_and_as_ref_read_through`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`ConstNonNull::new_unchecked + as_ref reads through` | &x = 7 | `7` | `7` | ✅ |

### `layout::type_layout`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`size/align Ptr` |  | `size / align` | `4 / 4` | ✅ |
|`size/align PtrMut` |  | `size / align` | `4 / 4` | ✅ |
|`size/align OwningPtr` |  | `size / align` | `4 / 4` | ✅ |
|`size/align ConstNonNull<u32>` |  | `size / align` | `4 / 4` | ✅ |
|`size/align ThinSlicePtr<u32> (has a debug-only len field)` |  | `size / align` | `8 / 4` | ✅ |
|`Ptr/PtrMut/OwningPtr/ConstNonNull are exactly pointer-sized` | repr(transparent) over NonNull<u8> | `4` | `4` | ✅ |
|`… PtrMut too` |  | `4` | `4` | ✅ |
|`… OwningPtr too` |  | `4` | `4` | ✅ |
|`… ConstNonNull<u32> too` |  | `4` | `4` | ✅ |

### `moving_ptr::assign_to_drops_the_old_value_and_moves_in_the_new_one`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`MovingPtr::assign_to() drops the previous value at dst` |  | `1` | `1` | ✅ |
|`… and does not drop the newly-moved-in value` |  | `0` | `0` | ✅ |
|`… dropping dst now runs the new value's Drop` |  | `1` | `1` | ✅ |

### `moving_ptr::deconstruct_moving_ptr_maybe_uninit_fields`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`deconstruct_moving_ptr!(MaybeUninit) — field_a` |  | `11` | `11` | ✅ |
|`deconstruct_moving_ptr!(MaybeUninit) — field_b` |  | `22` | `22` | ✅ |

### `moving_ptr::deconstruct_moving_ptr_struct_fields`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`deconstruct_moving_ptr!(struct) — field_a` |  | `11` | `11` | ✅ |
|`deconstruct_moving_ptr!(struct) — field_b` |  | `22` | `22` | ✅ |
|`deconstruct_moving_ptr!(struct) — field_c` |  | `33` | `33` | ✅ |

### `moving_ptr::deconstruct_moving_ptr_tuple_fields`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`deconstruct_moving_ptr!(tuple) — .0` |  | `11` | `11` | ✅ |
|`deconstruct_moving_ptr!(tuple) — .1` |  | `22` | `22` | ✅ |
|`deconstruct_moving_ptr!(tuple) — .2` |  | `33` | `33` | ✅ |

### `moving_ptr::drop_runs_exactly_once_if_a_moving_ptr_is_never_consumed`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`MovingPtr::drop() runs the pointee's Drop when never consumed` |  | `1` | `1` | ✅ |

### `moving_ptr::move_as_ptr_and_read`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`move_as_ptr!(x) + MovingPtr::read()` |  | `5` | `5` | ✅ |

### `moving_ptr::owning_ptr_round_trip`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`MovingPtr -> OwningPtr::into() round-trips` |  | `202` | `202` | ✅ |

### `moving_ptr::partial_move_splits_a_value_and_returns_the_rest`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`MovingPtr::partial_move() returns the closure's result` |  | `1` | `1` | ✅ |
|`… and the untouched field is still reachable` |  | `2` | `2` | ✅ |

### `moving_ptr::write_to_does_not_drop_the_destination`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`MovingPtr::write_to(dst)` |  | `17` | `17` | ✅ |

### `owning_ptr::as_ref_and_as_mut_views`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`OwningPtr::as_ref() reads the current value` |  | `1` | `1` | ✅ |
|`OwningPtr::as_mut() mutation is visible afterwards` |  | `2` | `2` | ✅ |

### `owning_ptr::cast_to_moving_ptr_preserves_the_value`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`OwningPtr::cast::<i64>() -> MovingPtr::read()` |  | `314` | `314` | ✅ |

### `owning_ptr::drop_as_runs_the_destructor_in_place`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`OwningPtr::drop_as::<T>() runs Drop exactly once` |  | `1` | `1` | ✅ |

### `owning_ptr::make_gives_a_safe_owning_ptr_for_the_closures_duration`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`OwningPtr::make(77).read::<i32>()` |  | `77` | `77` | ✅ |
|`OwningPtr::make actually invoked the closure` |  | `true` | `true` | ✅ |

### `owning_ptr::read_extracts_the_value_without_double_dropping`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`OwningPtr::read() hasn't dropped the value yet` |  | `0` | `0` | ✅ |
|`… dropping the read-out value runs Drop exactly once` |  | `1` | `1` | ✅ |

### `ptr::as_ptr_matches_source_address`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Ptr::as_ptr() keeps the original address` |  | `140537887` | `140537887` | ✅ |

### `ptr::assert_unique_promotes_a_genuinely_exclusive_ptr_to_ptr_mut`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Ptr::assert_unique() -> PtrMut::deref_mut() sees the value` |  | `55` | `55` | ✅ |

### `ptr::byte_offset_and_byte_add_index_into_an_array`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Ptr::byte_add(4) lands on arr[1]` |  | `20` | `20` | ✅ |
|`Ptr::byte_offset(8) lands on arr[2]` |  | `30` | `30` | ✅ |

### `ptr::from_ref_and_deref_roundtrip`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Ptr::from(&i32) -> deref::<i32>() round-trips` |  | `123` | `123` | ✅ |

### `ptr::to_unaligned_preserves_the_address`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Ptr::to_unaligned() only changes the type parameter` |  | `0x860701c` | `0x860701c` | ✅ |

### `ptr_mut::as_ptr_matches_source_address`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`PtrMut::as_ptr() keeps the original address` |  | `140537887` | `140537887` | ✅ |

### `ptr_mut::as_ref_gives_a_read_only_view`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`PtrMut::as_ref() reads the current value` |  | `7` | `7` | ✅ |

### `ptr_mut::from_mut_and_deref_mut_roundtrip`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`PtrMut::from(&mut i32).deref_mut() mutates the original` |  | `42` | `42` | ✅ |

### `ptr_mut::promote_to_owning_ptr_hands_over_ownership`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`PtrMut::promote() -> OwningPtr::read::<String>()` | owned | `"owned"` | `"owned"` | ✅ |

### `ptr_mut::reborrow_gives_a_shorter_lived_handle_to_the_same_data`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`PtrMut::reborrow() mutation is visible through the original` |  | `9` | `9` | ✅ |

### `thin_slice_ptr::as_slice_unchecked_rebuilds_a_slice`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`ThinSlicePtr::as_slice_unchecked(len) round-trips the slice` |  | `[1, 2, 3, 4, 5]` | `[1, 2, 3, 4, 5]` | ✅ |
|`… a shorter length gives a prefix` |  | `[1, 2, 3]` | `[1, 2, 3]` | ✅ |

### `thin_slice_ptr::from_slice_and_get_unchecked`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`ThinSlicePtr::get_unchecked(0)` |  | `2` | `2` | ✅ |
|`ThinSlicePtr::get_unchecked(1)` |  | `4` | `4` | ✅ |
|`ThinSlicePtr::get_unchecked(3)` |  | `16` | `16` | ✅ |

### `thin_slice_ptr::is_copy_and_clone`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`ThinSlicePtr is Copy` |  | `9` | `9` | ✅ |
|`ThinSlicePtr::clone()` |  | `9` | `9` | ✅ |

### `thin_slice_ptr::unsafe_cell_specialisation_allows_mutation_through_a_shared_slice`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`ThinSlicePtr<UnsafeCell<T>>::as_mut_slice_unchecked mutates through` |  | `20` | `20` | ✅ |
|`ThinSlicePtr<UnsafeCell<T>>::cast() strips the UnsafeCell layer` |  | `1` | `1` | ✅ |

### `unsafe_cell_deref::deref_mut_allows_mutation`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`UnsafeCellDeref::deref_mut() mutation is visible` |  | `5` | `5` | ✅ |

### `unsafe_cell_deref::deref_reads_the_current_value`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`UnsafeCellDeref::deref()` |  | `11` | `11` | ✅ |

### `unsafe_cell_deref::read_copies_out_without_borrowing`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`UnsafeCellDeref::read() (Copy types)` |  | `3` | `3` | ✅ |
|`… the cell is unchanged after read()` |  | `3` | `3` | ✅ |

## Nicht abgedeckt

`MovingPtr::partial_move`/`move_field`/`move_maybe_uninit_field` nur an einem einfachen Zwei-Felder-Beispiel (nicht jede denkbare Verschachtelung), `TryFrom<MovingPtr<Unaligned>> for MovingPtr<Aligned>` (weder Erfolgs- noch Fehlerpfad explizit getestet — die Konversion ist eine simple `is_aligned()`-Prüfung, aber nicht eigens verifiziert), `IsAligned`-Trait direkt (nur indirekt über `Ptr`/`OwningPtr`/`MovingPtr`, da seine Methoden `#[doc(hidden)]` sind).

**Caveat:** nur Emulator. Alignment-Verhalten auf echter Hardware ungeprüft, siehe `../../port.md`.
