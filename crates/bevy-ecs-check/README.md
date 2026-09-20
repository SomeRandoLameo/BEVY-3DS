# bevy-ecs-check

On-device Check, dass **`bevy_ecs`** — das Herzstück von Bevy, `World`/`Entity`/`Component`/`Query`/`System`/`Schedule`/`Observer` — auf dem Nintendo 3DS vollständig und korrekt funktioniert. Deckt die komplette öffentliche API ab: rohes `World`, `Commands`, Queries + Filter, Change Detection, Resources, Components/Bundles/`#[require(...)]`, Relationships (`ChildOf`/`Children`), Messages (das umbenannte `Events`), Observers, Schedules/`run_if`/`SystemSet`, exotische System-Params.

| | |
|---|---|
| Bevy-Einheit | `bevy_ecs` |
| Version | `0.19.1` |
| Features | `default-features = false`, `features = ["std"]`, + `multi_threaded` (Crate-Default seit 2026-09-20, s. u.) |
| Target | `armv6k-nintendo-3ds` |
| Toleranz | exakte Werte-/Strukturvergleiche (`Debug`/`PartialEq`-Derives); keine Fließkomma-Toleranz nötig — die einzigen `f32`-Werte sind exakt darstellbare Test-Literale |
| Stand | 2026-09-20 · Azahar-Emulator · **80/80 Testfunktionen · 145/145 Checks bestanden** (Default-Build, inkl. Threading) |

Ausführen: `./scripts/test-emulator.sh -p bevy-ecs-check` (GDB) oder `./scripts/send-tests.sh bevy-ecs-check` (interaktiv, echte Hardware/Emulator) — `multi_threaded` ist Crate-Default, läuft also ohne extra Flag mit. Zum Vergleich ohne Threading: `--no-default-features --features std`.

## Warum diese Crate überhaupt testen?

`bevy_ecs` 0.19 ist gegenüber älteren Bevy-Versionen intern deutlich umgebaut: Resources werden nicht mehr separat verwaltet, sondern als Components auf versteckten Entities gespeichert; `Events` wurde zu `Messages` umbenannt; `ChildOf`/`Children` leben jetzt direkt im Kern statt in einer separaten `bevy_hierarchy`-Crate; Observer nutzen `On<E>` statt `Trigger<E>`. Das ist genau die Art von Verhaltensänderung, die man nicht aus der API-Signatur ablesen kann — nur durch echtes Ausführen auf dem Zielsystem. Mehrere der unten dokumentierten Befunde widersprechen der Intuition aus älteren Bevy-Versionen (siehe `port.md`).

## Abgedeckt

**`World` (Rohzugriff)** — `spawn`/`spawn_empty`/`spawn_batch`, `despawn` (inkl. bereits-entfernt-Fall), `get_entity`/`entity`/`entity_mut`, `entity_mut(e).insert/remove`, `clear_entities()` (und dass sie in 0.19 **auch Resources mitlöscht**), `Entities::len()` vs. `count_spawned()` (High-Water-Mark vs. Live-Count).

**`Commands`** — standalone via `CommandQueue`/`Commands::new(&mut queue, &world)` (Deferred-Timing bis `CommandQueue::apply`), Commands innerhalb eines `Schedule`-Laufs (automatisch angewendet), `EntityCommands` (`insert`/`remove`/`despawn`), `spawn_batch`, `insert_resource`/`remove_resource`.

**Queries (Basis)** — `Query<&T>`/`Query<&mut T>`-Iteration, `QueryState::get`/`get_mut` (inkl. Fehlerfall), `Query::contains`, `single()`/`single_mut()` (0/1/2-Treffer-Fälle über `QuerySingleError`).

**Query-Kombinationen** — `iter_combinations::<K>()` (inkl. K > N → leer), `iter_many()` (Reihenfolge folgt der übergebenen Entity-Liste).

**Query-Filter** — `With`/`Without`, `Or<(...)>`, `AnyOf<(...)>`, `Has<T>` (besucht jede Entity, liefert `bool` statt zu filtern).

**Change Detection** — `Added<T>`/`Changed<T>` als Query-Filter über echte `Schedule`-Läufe (nicht per Hand-`iter()`, siehe Befund unten), `Ref<T>` (`is_added`/`is_changed` + Deref), `Res<T>::is_changed()` nach `ResMut`-Zugriff, die **lazy** `Mut<T>`-Änderungsmarkierung (nur `DerefMut` triggert sie, nicht `get_mut()` allein).

**Resources** — `insert_resource`/`init_resource` (`Default`), `remove_resource`, `contains_resource`, `resource`/`get_resource`, `FromWorld` (Custom-Initialisierung mit Zugriff auf vorhandene Resources), `Local<T>` (pro-System-State, nicht global).

**Components/Bundles** — `#[derive(Component)]` (Unit-Struct), `#[derive(Bundle)]` (alle Felder landen als Components), `#[require(T)]` (`Default`-Fall), `#[require(T = expr())]` (Custom-Initializer), Transitivität (`A` braucht `B` braucht `C`), explizite Component überschreibt den `#[require(...)]`-Default.

**`EntityRef`/`EntityMut`/`EntityWorldMut`** — `contains`/`get`/`id`, `get_mut` (Mutation in-place), `insert_if_new` (überschreibt Vorhandenes nicht), `take::<T>()` (entfernt + gibt Eigentum zurück), `World::get_entity_mut()` (inkl. Fehlerfall bei despawnter Entity), `Entity`-Gleichheit/`index()`.

**Relationships (`ChildOf`/`Children`)** — Insert von `ChildOf` befüllt automatisch `Children` des Parents, verschachtelte Hierarchien, `ChildOf::parent()`, `Children` derefs zu `&[Entity]`, Despawn eines Parents despawnt den gesamten Subtree transitiv, Entfernen von `ChildOf` aktualisiert `Children` des alten Parents, `EntityWorldMut::with_children` baut einen ganzen Teilbaum auf einmal.

**Messages** (0.19s Umbenennung von `Events`) — `MessageRegistry::register_message::<M>()`, `MessageWriter`/`MessageReader`, Reader-vor-Writer vs. Reader-nach-Writer in derselben/nächster "Frame", Ablauf nach 2 Buffer-Swaps (Nachricht wird verworfen), `write_batch()`.

**Observers** — `World::add_observer`/`World::trigger` für Custom-`Event`s, `Commands` innerhalb eines Observers (braucht explizites `World::flush()`), Component-Lifecycle-Observer `On<Add, T>`/`On<Insert, T>`/`On<Remove, T>` (inkl. Feuer-Reihenfolge bei Spawn+Despawn).

**Schedules & Conditions** — `.chain()` (erzwingt Reihenfolge) vs. bloßes Tuple (keine Garantie), `.run_if(...)`, benannte `Schedules` (`World::add_schedule`/`run_schedule`), `World::run_system_once()` (via `RunSystemOnce`-Trait), `SystemSet` + `.after(...)`.

**Exotische System-Params** — `ParamSet` (zwei sich eigentlich widersprechende Queries im selben System), `RemovedComponents<T>` (inkl. Despawn zählt als Removal), exklusive Systeme (`&mut World`).

**Layout** — `size_of`/`align_of` von `Entity`, `Option<Entity>` (Niche-Optimierung — kein Overhead ggü. `Entity`), `ChildOf`.

## Ergebnis

Funktion · Eingabe · Erwartet · **tatsächliche Ausgabe** · OK. Reihenfolge = Testausführung (alphabetisch nach Modul/Funktion), transkribiert aus einem echten Azahar-Lauf.

### `change_detection::added_filter_is_true_once_then_clears`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Added<Position>: true the frame right after spawn, then false` |  | `[true, false, false]` | `[true, false, false]` | ✅ |

### `change_detection::changed_filter_fires_on_mutation_and_then_clears`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Changed<Position>: spawn, mutation, and a same-value reassignment each show up once — but an untouched Mut<T> handle doesn't` | DerefMut is the trigger, not data equality | `[true, false, true, false, false, true]` | `[true, false, true, false, false, true]` | ✅ |

### `change_detection::ref_gives_read_access_plus_is_added_is_changed`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Ref<T>::is_added() true on the run right after spawn` |  | `true` | `true` | ✅ |
|`Ref<T>::is_changed() true too (spawn counts)` |  | `true` | `true` | ✅ |
|`Ref<T> derefs to the component` |  | `7` | `7` | ✅ |
|`… is_added() false on a later, untouched run` |  | `true` | `true` | ✅ |
|`… is_changed() false too` |  | `true` | `true` | ✅ |

### `change_detection::resource_is_changed_after_res_mut_access`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Res<T>::is_changed() true right after a ResMut write` |  | `true` | `true` | ✅ |

### `commands::commands_applied_after_a_schedule_run`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`A Commands::spawn() inside a system is visible once schedule.run() returns` |  | `1` | `1` | ✅ |

### `commands::commands_are_deferred_until_the_queue_is_applied`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Commands::spawn() doesn't touch the World yet` | before CommandQueue::apply | `true` | `true` | ✅ |
|`… CommandQueue::apply() makes it real` |  | `true` | `true` | ✅ |
|`… with the spawned data` |  | `Position { x: 1.0, y: 1.0 }` | `Position { x: 1.0, y: 1.0 }` | ✅ |

### `commands::commands_insert_and_remove_resource`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Commands::insert_resource()` |  | `10` | `10` | ✅ |
|`Commands::remove_resource()` |  | `true` | `true` | ✅ |

### `commands::commands_spawn_batch`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Commands::spawn_batch(4).len()` |  | `4` | `4` | ✅ |

### `commands::entity_commands_despawn`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Commands::entity(e).despawn()` |  | `true` | `true` | ✅ |

### `commands::entity_commands_insert_and_remove`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Commands::entity(e).insert()` |  | `true` | `true` | ✅ |
|`Commands::entity(e).remove()` |  | `true` | `true` | ✅ |

### `components_bundles::bundle_derive_spawns_every_field_as_a_component`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Bundle field 1 (Position) landed` |  | `Position(1.0, 2.0)` | `Position(1.0, 2.0)` | ✅ |
|`Bundle field 2 (Velocity) landed` |  | `Velocity(0.5, -0.5)` | `Velocity(0.5, -0.5)` | ✅ |

### `components_bundles::explicitly_provided_component_overrides_the_required_default`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Explicit Team(7) wins over #[require(Team)]'s default` |  | `Team(7)` | `Team(7)` | ✅ |

### `components_bundles::marker_unit_component`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Unit-struct Component derive` |  | `true` | `true` | ✅ |

### `components_bundles::required_component_with_custom_initializer`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`#[require(Health = Health(100))] uses the custom initializer` |  | `Health(100)` | `Health(100)` | ✅ |

### `components_bundles::required_component_with_default_is_auto_inserted`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`#[require(Team)] auto-inserts Team on spawn(Soldier)` |  | `true` | `true` | ✅ |
|`… using Team::default()` |  | `Team(0)` | `Team(0)` | ✅ |

### `components_bundles::transitive_required_components`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`A requires B` |  | `true` | `true` | ✅ |
|`… B (transitively) requires C` |  | `true` | `true` | ✅ |
|`… C got its default` |  | `C(0)` | `C(0)` | ✅ |

### `entity_ref::entity_index_and_equality`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Entity::index() differs between two live entities` |  | `true` | `true` | ✅ |
|`Entity == Entity by value` |  | `true` | `true` | ✅ |
|`… different entities compare unequal` |  | `true` | `true` | ✅ |

### `entity_ref::entity_ref_get_and_contains`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`EntityRef::contains::<T>()` |  | `true` | `true` | ✅ |
|`EntityRef::get::<T>()` |  | `Position(1.0)` | `Position(1.0)` | ✅ |
|`EntityRef::id()` |  | `1v0` | `1v0` | ✅ |

### `entity_ref::entity_world_mut_get_mut`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`EntityWorldMut::get_mut::<T>() mutates in place` |  | `9` | `9` | ✅ |

### `entity_ref::entity_world_mut_insert_if_new_does_not_overwrite`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`EntityWorldMut::insert_if_new() keeps the existing value` | already had Position(1.0) | `Position(1.0)` | `Position(1.0)` | ✅ |
|`… but does insert when the component is absent` |  | `Position(3.0)` | `Position(3.0)` | ✅ |

### `entity_ref::entity_world_mut_take_removes_and_returns_owned`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`EntityWorldMut::take::<T>() returns the owned value` |  | `Some(Position(5.0))` | `Some(Position(5.0))` | ✅ |
|`… and removes it from the entity` |  | `true` | `true` | ✅ |

### `entity_ref::get_entity_mut_allows_mutation_via_world`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`World::get_entity_mut(e) -> mutation round-trips` |  | `42` | `42` | ✅ |
|`World::get_entity_mut() on a despawned entity is Err` |  | `true` | `true` | ✅ |

### `layout::type_layout`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`size/align Entity` |  | `size / align` | `8 / 8` | ✅ |
|`size/align Option<Entity> (niche-optimised, same size as Entity)` |  | `size / align` | `8 / 8` | ✅ |
|`size/align ChildOf` |  | `size / align` | `8 / 8` | ✅ |
|`Option<Entity> has no size overhead over Entity (niche optimisation)` |  | `8` | `8` | ✅ |

### `messages::message_writer_write_batch`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`MessageWriter::write_batch() delivers all of them in order` |  | `[1, 2, 3]` | `[1, 2, 3]` | ✅ |

### `messages::messages_older_than_two_updates_are_dropped`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`A message not read within 2 buffer-swaps is dropped` | 3 frames after the write | `0` | `0` | ✅ |

### `messages::reader_ordered_before_the_writer_sees_it_next_frame_instead`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`MessageReader ordered before the writer sees nothing yet` |  | `[]` | `[]` | ✅ |
|`… but it's there on the next frame` | message survives one buffer swap | `[99]` | `[99]` | ✅ |

### `messages::reader_sees_a_message_written_earlier_the_same_frame`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`MessageReader sees a message written earlier the same frame` |  | `[1]` | `[1]` | ✅ |

### `observers::add_insert_remove_fire_in_the_documented_order`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`A single spawn+despawn cycle: Add, then Insert, then Remove` |  | `["add", "insert", "remove"]` | `["add", "insert", "remove"]` | ✅ |

### `observers::custom_event_reaches_its_observer`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`World::trigger() runs the matching observer synchronously` |  | `["hello"]` | `["hello"]` | ✅ |

### `observers::observer_can_query_and_use_commands`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`An observer's Commands::spawn() lands in the World after World::flush()` |  | `1` | `1` | ✅ |

### `observers::on_add_fires_exactly_once_per_insertion_not_per_later_mutation`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`On<Add, A> fires when A is first inserted (spawn)` |  | `["add"]` | `["add"]` | ✅ |
|`… and again on a fresh insert after removal` |  | `["add", "add"]` | `["add", "add"]` | ✅ |

### `observers::on_insert_fires_on_every_insert_including_overwrites`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`On<Insert, T> fires on the initial spawn and on overwriting inserts` |  | `["insert", "insert"]` | `["insert", "insert"]` | ✅ |

### `observers::on_remove_fires_before_the_component_is_gone`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`On<Remove, A> fires when A is removed` |  | `["remove"]` | `["remove"]` | ✅ |
|`… and despawn() counts as a removal` |  | `["remove", "remove"]` | `["remove", "remove"]` | ✅ |

### `query_basic::get_and_get_mut_by_entity_id`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`QueryState::get(a)` |  | `Position { x: 1.0, y: 1.0 }` | `Position { x: 1.0, y: 1.0 }` | ✅ |
|`QueryState::get_mut(b).x = 20` |  | `20` | `20` | ✅ |

### `query_basic::get_on_a_non_matching_entity_errs`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`QueryState::get() on an entity missing the component is Err` |  | `true` | `true` | ✅ |

### `query_basic::immutable_iteration_sees_every_matching_entity`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Query<&Position>::iter().map(..).sum()` | 3 entities, x=1,2,3 | `6` | `6` | ✅ |

### `query_basic::mutable_iteration_persists_across_two_schedule_runs`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Query<&mut Position> mutation persists across runs` | vel=1.5, 2 runs | `3` | `3` | ✅ |

### `query_basic::query_contains_checks_without_borrowing_data`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Query::contains() true when the component is present` |  | `true` | `true` | ✅ |
|`… false when it's not` |  | `true` | `true` | ✅ |

### `query_basic::single_succeeds_with_exactly_one_match_and_errs_otherwise`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`QueryState::single() on zero matches is Err` |  | `true` | `true` | ✅ |
|`QueryState::single() with exactly one match` |  | `Position { x: 9.0, y: 9.0 }` | `Position { x: 9.0, y: 9.0 }` | ✅ |
|`QueryState::single() with two matches is Err` |  | `true` | `true` | ✅ |

### `query_combinations::iter_combinations_k_larger_than_n_is_empty`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`QueryState::iter_combinations::<3>() on 1 entity is empty` | K > N | `0` | `0` | ✅ |

### `query_combinations::iter_combinations_visits_every_unordered_pair`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`QueryState::iter_combinations::<2>() count on 3 entities` | C(3,2) | `3` | `3` | ✅ |
|`… pairwise sums` | (1,2)=3, (1,3)=4, (2,3)=5 | `[3.0, 4.0, 5.0]` | `[3.0, 4.0, 5.0]` | ✅ |

### `query_combinations::iter_many_visits_only_the_given_entities_in_order`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`QueryState::iter_many([b, a]) — order follows the given list` |  | `[20.0, 10.0]` | `[20.0, 10.0]` | ✅ |

### `query_filters::any_of_returns_options_for_each_side`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Query<AnyOf<(&Va, &Vb)>> sees all 3, with the missing side as None` |  | `[(None, Some(2)), (Some(1), None), (Some(3), Some(4))]` | `[(None, Some(2)), (Some(1), None), (Some(3), Some(4))]` | ✅ |

### `query_filters::has_reports_presence_as_a_bool_without_borrowing_the_component`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Query<(Entity, Has<A>)> — entities reporting has=true` | 1 of 2 has A | `1` | `1` | ✅ |
|`… every entity is still visited (Has isn't a filter), relative to baseline` |  | `3` | `3` | ✅ |

### `query_filters::or_filter_matches_either_side`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Query<Entity, Or<(With<A>, With<B>)>>::iter().count()` | A-only, B-only, C-only spawned | `2` | `2` | ✅ |

### `query_filters::query_filtered_reused_after_spawning_more`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`query_filtered count before more spawns` |  | `1` | `1` | ✅ |
|`… re-querying the same World sees the new entity too` |  | `2` | `2` | ✅ |

### `query_filters::with_and_without`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Query<Entity, With<A>>::iter().count()` | 2 of 3 have A | `2` | `2` | ✅ |
|`Query<Entity, Without<A>>::iter().count() (relative to baseline)` | 1 of 3 lacks A | `2` | `2` | ✅ |
|`Query<Entity, (With<A>, With<B>)>::iter().count()` | 1 has both | `1` | `1` | ✅ |

### `relationships::child_of_auto_populates_children_on_the_parent`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Inserting ChildOf(root) auto-adds root's Children` | 2 children | `[2v0, 3v0]` | `[2v0, 3v0]` | ✅ |
|`… nested: child1's Children` |  | `[4v0]` | `[4v0]` | ✅ |
|`ChildOf::parent()` |  | `1v0` | `1v0` | ✅ |

### `relationships::children_component_is_iterable`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Children derefs to a plain &[Entity] slice` |  | `[2v0, 3v0]` | `[2v0, 3v0]` | ✅ |

### `relationships::despawning_a_parent_despawns_the_whole_subtree`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Despawning root also despawns child1` |  | `true` | `true` | ✅ |
|`… and grandchild, transitively` |  | `true` | `true` | ✅ |
|`… root itself is gone too` |  | `true` | `true` | ✅ |

### `relationships::removing_child_of_updates_the_parents_children`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Removing ChildOf updates the old parent's Children` | child2 removed | `[2v0]` | `[2v0]` | ✅ |
|`… and the removed child has no Children component of its own to worry about (it's a leaf)` |  | `true` | `true` | ✅ |

### `relationships::with_children_builder_spawns_a_whole_subtree_at_once`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`EntityWorldMut::with_children builds the whole tree` |  | `[2v0, 4v0]` | `[2v0, 4v0]` | ✅ |
|`… nested with_children too` |  | `[3v0]` | `[3v0]` | ✅ |

### `resources::from_world_runs_custom_initialization`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`FromWorld::from_world() can read prior resources` | Score(21) -> Doubled | `42` | `42` | ✅ |

### `resources::init_resource_uses_default`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`World::init_resource::<T>() uses Default` |  | `0` | `0` | ✅ |

### `resources::insert_get_and_contains`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`World::contains_resource before insert` |  | `true` | `true` | ✅ |
|`… after insert` |  | `true` | `true` | ✅ |
|`World::resource::<T>()` |  | `5` | `5` | ✅ |
|`World::get_resource::<T>() is Some` |  | `Some(5)` | `Some(5)` | ✅ |

### `resources::local_state_is_per_system_not_global`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Local<u32> is independent per system across 3 runs` | counter_a +1 each run, counter_b +10 each run | `[1, 2, 3, 10, 20, 30]` | `[1, 2, 3, 10, 20, 30]` | ✅ |

### `resources::remove_resource`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`World::remove_resource::<T>() returns it` |  | `Some(1)` | `Some(1)` | ✅ |
|`… and it's gone` |  | `true` | `true` | ✅ |

### `resources::res_mut_mutates_in_place`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`ResMut<T> mutation persists across 5 runs` |  | `5` | `5` | ✅ |

### `schedule_and_conditions::chain_enforces_order_tuple_alone_does_not`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`.chain() runs systems in the given order` |  | `["a", "b"]` | `["a", "b"]` | ✅ |

### `schedule_and_conditions::multiple_named_schedules_on_one_world`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`World::run_schedule() picks the right named Schedule each time` |  | `["setup", "tick", "tick"]` | `["setup", "tick", "tick"]` | ✅ |

### `schedule_and_conditions::run_if_gates_a_system`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`.run_if(false) skips the system` |  | `[]` | `[]` | ✅ |
|`.run_if(true) lets it run` |  | `["ran"]` | `["ran"]` | ✅ |

### `schedule_and_conditions::run_system_once_runs_a_system_without_a_schedule`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`World::run_system_once() runs it exactly once` | 3 -> doubled | `6` | `6` | ✅ |

### `schedule_and_conditions::system_set_after_orders_a_whole_group`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`late runs after both Phase::Early systems` |  | `"late"` | `"late"` | ✅ |
|`… and both Early systems ran (order between them is unspecified)` |  | `3` | `3` | ✅ |

### `system_params::exclusive_system_gets_unrestricted_world_access`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`An exclusive system (&mut World) can spawn and immediately query in the same call` |  | `1` | `1` | ✅ |

### `system_params::param_set_lets_one_system_hold_two_conflicting_queries`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`ParamSet lets p0 (mutable, all) and p1 (read-only, filtered) coexist` | 2 entities bumped by p0 | `2` | `2` | ✅ |

### `system_params::removed_components_also_fires_on_despawn`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`Despawning an entity with Marker also shows up in RemovedComponents<Marker>` |  | `[2v0]` | `[2v0]` | ✅ |

### `system_params::removed_components_reports_entities_that_lost_a_component`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`RemovedComponents<T> sees nothing before any removal` |  | `0` | `0` | ✅ |
|`… reports the entity the frame after Marker was removed` |  | `[2v0]` | `[2v0]` | ✅ |

### `world::clear_entities_also_wipes_resources_now`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`World::clear_entities() -> no Position entities left` | 2 spawned | `0` | `0` | ✅ |
|`… and resources are gone too (they're entity-backed)` | not the pre-0.19 behaviour | `true` | `true` | ✅ |

### `world::despawn_a_missing_entity_returns_false`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`World::despawn() on an already-despawned entity returns false` |  | `true` | `true` | ✅ |

### `world::despawn_removes_the_entity`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`World::despawn(entity) returns true for a live entity` |  | `true` | `true` | ✅ |
|`… the entity is now gone (get_entity errs)` |  | `true` | `true` | ✅ |
|`… a different entity is untouched` |  | `true` | `true` | ✅ |

### `world::entities_len_is_a_high_water_mark_not_a_live_count`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`World::new() already has >= 1 live entity (resource-backing)` | bootstrap's DefaultQueryFilters resource | `true` | `true` | ✅ |
|`… and len() (slots ever allocated) is >= count_spawned() (live count)` | bootstrap frees some scratch slots along the way | `true` | `true` | ✅ |
|`count_spawned() grows by exactly 3 after 3 spawns` | unlike len(), it's a precise live count | `4` | `4` | ✅ |
|`… and len() has kept pace (still >= count_spawned())` |  | `true` | `true` | ✅ |
|`… count_spawned() shrinks by 1 on despawn` |  | `3` | `3` | ✅ |
|`… but len() (slots ever allocated) does not shrink` | despawning frees the slot for reuse, it doesn't remove it | `4` | `4` | ✅ |

### `world::entity_mut_inserts_and_removes_components`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`EntityWorldMut::insert() adds a component` |  | `true` | `true` | ✅ |
|`EntityWorldMut::remove() removes it again` |  | `true` | `true` | ✅ |

### `world::get_entity_batch_partial_failure`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`World::get_entity(live) is Ok` |  | `true` | `true` | ✅ |
|`World::get_entity(despawned) is Err` |  | `true` | `true` | ✅ |

### `world::spawn_and_get_entity`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`World::entity(e).get::<T>()` | spawn(Position{1,2}) | `Position { x: 1.0, y: 2.0 }` | `Position { x: 1.0, y: 2.0 }` | ✅ |
|`World::get_entity(e) is Ok for a live entity` |  | `true` | `true` | ✅ |

### `world::spawn_batch_creates_every_entity`

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`World::spawn_batch(5 positions).count()` |  | `5` | `5` | ✅ |
|`… Query count matches` |  | `5` | `5` | ✅ |
|`… last entity's Position` |  | `Position { x: 4.0, y: 0.0 }` | `Position { x: 4.0, y: 0.0 }` | ✅ |

## Threading (Crate-Default seit 2026-09-20)

Läuft standardmäßig mit — kein extra Flag mehr nötig (bis 2026-09-20 war es ein Opt-in `--features multi_threaded`; siehe `git log` für die alte Abgrenzung). Zieht `bevy_tasks`' `async-executor`/`concurrent-queue`/`async-channel` und schaltet `bevy_ecs`'s `default_executor()` von `SingleThreadedExecutor` auf `MultiThreadedExecutor` um. Das matcht normales Bevy: die `bevy`-Umbrella-Crate hat `multi_threaded` selbst standardmäßig an (nur die einzelnen Sub-Crates `bevy_ecs`/`bevy_tasks` *allein* defaulten auf single-threaded) — jetzt auch hier, sowohl in `dove` selbst (`Cargo.toml`, `App::new()` + `TaskPoolPlugin`) als auch in dieser Check-Crate und in `crates/all-checks`. Beantwortet port.md §B1/§B2 (der bisher als "harter Block" eingestufte Risikobereich): **echte OS-Threads über `pthread-3ds` funktionieren auf `armv6k-nintendo-3ds`**, ohne dass an `pthread-3ds` selbst etwas gepatcht werden musste. Zum Vergleich ohne Threading: `--no-default-features --features std`.

`crates/bevy-ecs-check/src/checks/threading.rs`, `./scripts/test-emulator.sh -p bevy-ecs-check`. **5/5 Testfunktionen · 10/10 Checks bestanden** (Teil der 80/80 Default-Gesamtzahl oben).

| Funktion | Eingabe | Erwartet | Ausgabe | OK |
|---|---|---|---|---|
|`std::thread::spawn() + join() returns the closure's value` | 1 + 1 | `2` | `2` | ✅ |
|`… and the closure actually ran (not skipped/no-op'd)` |  | `true` | `true` | ✅ |
|`ComputeTaskPool::get_or_init() yields a pool with >= 1 real thread` | num_threads(2) requested somewhere in this binary | `true` | `true` | ✅ |
|`5 schedule.run()s through the MultiThreadedExecutor: CounterA` | 2 non-conflicting systems/run | `5` | `5` | ✅ |
|`… CounterB` |  | `5` | `5` | ✅ |
|`Query::par_iter() across 200 entities on a real TaskPool` | sum(1..=200) | `20100` | `20100` | ✅ |
|`30 rounds of Query::par_iter_mut() across 50 entities: every counter == 30` | no deadlock, no lost update | `true` | `true` | ✅ |

Abgedeckt: `std::thread::spawn`/`join` (der rohe `pthread-3ds`-Unterbau), `ComputeTaskPool::get_or_init` mit einer für die 3DS-Kernzahl expliziten `TaskPoolBuilder::num_threads(2)` (`std::thread::available_parallelism()` ist auf diesem Target praktisch nutzlos — fällt auf 1 zurück, siehe unten), `Schedule::default()`'s automatische `MultiThreadedExecutor`-Wahl mit zwei nicht-konfligierenden Systemen über mehrere `run()`-Aufrufe, `Query::par_iter()`/`par_iter_mut()` (die echte Parallel-Query-Iteration, über `TaskPool::scope` batched) über 200 bzw. 50 Entities, 30 Wiederholungsrunden als Deadlock-/Race-Stichprobe (jeder Counter muss exakt 30 sein — ein verlorenes Update oder ein Deadlock wäre hier sichtbar).

- **`ComputeTaskPool` ist ein prozessweiter `OnceLock`.** Wer zuerst `get_or_init()` aufruft, gewinnt für den Rest des Prozesses — deshalb prüft `compute_task_pool_initializes_with_a_real_thread_pool` nur `thread_num() >= 1`, nicht exakt `== 2`: welcher Test zuerst läuft, ist von der Testreihenfolge des Runners abhängig, nicht garantiert.
- **`std::thread::available_parallelism()` ist auf `armv6k-nintendo-3ds` faktisch ungestützt** — `bevy_tasks::available_parallelism()` fängt den Fehler ab und fällt auf `1` zurück (kein Panic, aber auch keine automatische 2-Kern-Erkennung). Für echte Parallelität also immer explizit `TaskPoolBuilder::num_threads(...)` setzen, wie port.md es schon vermutet hatte ("Kein automatisches num_cpus-Sizing").
- **`Query::par_iter()`/`par_iter_mut()` ruft intern `ComputeTaskPool::get()` auf — nicht `get_or_init()`.** Ohne eine vorherige `ComputeTaskPool::get_or_init()`-Initialisierung irgendwo im Prozess **panickt** das. Alle Tests hier rufen deshalb zuerst `ensure_compute_task_pool()`.
- **Nicht abgedeckt / weiterhin offen:** echte Parallelität/Contention auf **Hardware** (der Emulator emuliert nicht notwendigerweise reale ARM11-Zweikern-Nebenläufigkeit inkl. Cache-Kohärenz-Timing — siehe port.md §B1 zum 64-Bit-Atomic-Fallback-Lock), `AsyncComputeTaskPool`/`IoTaskPool` (nur `ComputeTaskPool` getestet), explizite `TaskPool::scope`-Nutzung außerhalb von `bevy_ecs`s eigenen Aufrufern, System-Konflikt-Erkennung unter dem parallelen Executor mit tatsächlich konfligierenden Systemen (nur der Erfolgsfall mit zwei disjunkten Resources getestet — die Konflikt-Erkennung selbst ist Scheduling-Logik, die für beide Executor-Varianten identisch ist und schon über die Kompilierung/`Schedule::initialize` abgesichert wird), Lang-/Dauerlauf unter Last (§B7).

## Nicht abgedeckt

`bevy_ecs`'s Reflection-Integration (`bevy_reflect`-Feature), `System`-Piping (`.pipe(...)`), `EntityHashMap`/`EntityHashSet` direkt (nur indirekt über `Vec<Entity>`-Vergleiche genutzt), Serialisierung (`bevy_ecs` hat dafür ohnehin kein eigenes Feature ohne `bevy_reflect`), Observer-Propagation über `EntityEvent`/Bubbling (nur direktes `World::trigger` getestet), `SubApp`s (die leben in `bevy_app`, nicht `bevy_ecs`).

**Caveat:** nur Emulator (Azahar). Echte Hardware ungeprüft, siehe `../../port.md`.
