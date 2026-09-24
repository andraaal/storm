# Storm design guide [AI generated]

Human Note: Actually decent docs, but a little bit outdated

Storm is a Rust Magic: The Gathering rules-engine prototype.  This document
describes the **current, in-progress design**: the zone-owned object model and
typed-stack refactor represented by the active source files.  It is a guide to
the intended boundaries and data flow for future work, not a claim that every
rule or execution path is complete.

Code explicitly labelled as legacy, commented-out implementations, and files
removed by the refactor are deliberately out of scope.

## Project shape

The crate is a library (`src/lib.rs`) with four main areas:

| Area | Responsibility |
| --- | --- |
| `context` | Public game session facade, configuration/building, player input validation, and rules execution orchestration. |
| `game` | Mutable game-state aggregate and physical storage for objects in each zone. |
| `rules` | MTG domain types: zones, IDs, actions, layers, cards' characteristics, targeting, abilities, and turn vocabulary. |
| `cards` | Card definitions and reusable typed effect/target building blocks. |

The crate currently relies on nightly-only feature gates (`checked_type_aliases`
and `min_specialization`).  Its main external building blocks are `slotmap` for
generational object handles, `flagset` for zone/layer/supertype bit sets, and
`thiserror`/`derive_more` for support types.

## Core state model

`Context` is the engine-facing session object. It owns:

* a `Game`, the mutable authoritative game state;
* a `Controller`, which adapts an application-provided `Input` implementation;
* `Config` (currently an empty extension point).

`ContextBuilder` creates a two-player context.  Internal callers supply decks as
`Vec<&'static ConstCharacteristics>` for the `PlayPlayer` and `DrawPlayer`.
It rejects absent or fewer-than-seven-card decks, materializes each entry as a
`GameObject`, puts the decks in their corresponding libraries, and initializes
both players at 20 life.  The builder does not shuffle or draw opening hands;
those belong to game start once implemented.

`Game` tracks the players, `Objects`, registered continuous/replacement
effects, a monotonic timestamp source, the current phase/step and queued
steps, and priority/active-player state.  It is deliberately separate from
`Context`: game data belongs in `Game`, while interaction and rule execution
belong in `Context` extension modules.

## Zones, objects, and identity

`Objects` stores each zone in a separate typed `MyMap` (a `SlotMap` plus an
insertion-order list):

* shared battlefield;
* spell stack and ability stack;
* play/draw player hands, libraries, and graveyards;
* exile.

Battlefield entries carry `BattlefieldInfo` (`tapped`, marked damage,
deathtouch damage marker, controller, owner).  Stack entries carry `StackInfo`
(controller and owner), and exile entries carry owner information.  Ordinary
private-zone ownership is encoded by the map in which a card resides.

Every zone has its own `slotmap` key type (`BattlefieldId`, `PlayHandId`,
`SpellStackId`, etc.).  `AnyId` is the lossless sum type used when an operation
can address a card in any non-ability-object zone.  `ZonedId` lets code read a
`GameObject` through an appropriately typed ID without reimplementing the zone
dispatch.  Because these are generational handles, a removed ID cannot silently
refer to a newly inserted entry.

Moving an object removes it from its current zone map and inserts it into its
destination map, so it obtains a new zone-specific ID.  Movement also assigns a
fresh `Timestamp`; timestamps therefore mark object incarnations across zone
changes.  Movement helpers determine an ordinary destination zone from the
object's owner when moving to hand, library, or graveyard.  Code that preserves
or queries controller/owner must use `Objects::get_controller_and_owner` rather
than infer it from an `AnyId` alone.

`MyMap` maintains an `order` vector when inserting/removing, although its
currently exposed iteration methods use the underlying `SlotMap` iterators.
Do not assume that public zone iteration is library-top or insertion order
unless that contract is made explicit and implemented.

## Card data and characteristics

Cards are static `ConstCharacteristics`: name, `Cost`, base object types and
supertypes, plus static abilities.  `GameObject::new` expands this immutable
definition into mutable `Characteristics` and retains a static pointer to the
definition for resets.

`Characteristics` is the layerable current view: owned name/cost, copy-on-write
type data, supertypes, and timestamped dynamic static effects.  A `GameObject`
also keeps `copy_characteristics`, which is the snapshot at the copy/control
boundary in the layer system.  `reset_characteristics` reconstructs the current
view from the base card before each layer pass.  `is_permanent_spell` identifies
creature, enchantment, artifact, planeswalker, and land types.

`ObjectType` is the current type model.  Creatures contain power/toughness and
creature subtypes; instants contain effect function pointers; the other card
types carry their corresponding subtype lists.  Mana costs live in
`rules::cost`; the separate `mana` module represents individual mana and its
colour, but payment and mana pools are not yet modeled.

Card modules currently provide examples for Bear Cub, Lightning Bolt, Giant
Growth, and Anthem of Champions.  Treat these as prototypes while the typed
effect/stack API is being wired up, rather than as a stable card-registration
interface.

## Actions and execution

`GameAction` is the command/event vocabulary.  It covers damage, zone moves,
draw, tap/untap, discard, and marked-damage removal.  The intended invariant is
stated in `rules::game_action`: game-object movement goes through actions, while
characteristic changes are performed by layering.

`Context::execute` dispatches actions in `context/execution/actions.rs`:

1. a batch is processed action by action;
2. an individual action calls the matching state mutation helper;
3. zone mutations allocate one timestamp per action and use `Objects` movement
   helpers;
4. damage derives lifelink, deathtouch, wither, and infect from the source's
   intrinsic static abilities, then applies damage to each target.

`Damageable` abstracts battlefield objects and players.  Ordinary creature
damage becomes `BattlefieldInfo.marked_damage`; player damage reduces life;
lifelink gains life for the source controller/owner.  Deathtouch records a
marker.  Infect and wither deliberately remain unimplemented.

The comments define the intended atomicity boundary: a supplied action batch
is conceptually simultaneous with no state-based actions or layering between
its members, while nested actions execute immediately.  Replacement and
triggered-effect observation is intended at this boundary, but is not connected
to the active dispatcher yet.  Keep new rule mutations as `GameAction`s so
those hooks can be added centrally.

## Static abilities and layers

Static abilities have two forms:

* `DynamicAbility` is immutable card text with active-zone and applicable-layer
  bit sets.  Its group can be intrinsic, dynamic continuous, or dynamic
  replacement behavior.
* `FixedAbilityGroup` represents an effect created at runtime.  Fixed
  continuous/replacement abilities can own closures and expiry `Condition`s.

`Context::redo_layering` is the recomputation pass.  It resets every object's
characteristics, walks `LAYER_ORDER`, discovers applicable dynamic static
effects, snapshots copy characteristics at `ControlChanging`, and invokes the
continuous effects for each layer.  Effects receive `&mut Context` and their
source `AnyId`; they may therefore inspect and alter eligible objects.

Layer ordering is encoded in `rules::layer::LAYER_ORDER` and includes copy,
control, text/type/color/ability changes, P/T setting and modification, and
P/T switching.  The implementation currently executes dynamic continuous
effects; fixed continuous effects are registered on `Game` but are not yet
applied or expired by this pass.  Dependency ordering, timestamps as layer
tie-breakers, zone activation checks, and replacement-effect execution are also
not yet implemented.  Any new continuous effect should declare its `Layer` and
expiry condition rather than mutate lasting characteristics directly.

## Typed stack direction

The active refactor models stack entries with generic `StackObject<E, R>`:

* `E: StackDefinition` declares compile-time target selection, behavior list,
  X support, modes, per-effect data, and resolution return type.
* `R: ResolutionBehaviour` turns that return value into one of `ReturnKind`:
  move a resolving spell to battlefield, move it to graveyard, or make an
  ability vanish.
* `SpellObject` and `AbilityObject` constrain the valid resolution behavior
  for spells and abilities respectively; zone storage erases concrete types
  behind `Box<dyn Spell>` or `Box<dyn Ability>`.
* `Selection` and `Selector` make a card's target schema type-level.  A selector
  computes legal choices from `Game`; `Selection::choose` asks the controller
  for the required tuple of selected IDs.
* `Behaviour` and `BehaviourList` compose operations that share one typed target
  tuple.  `PTChange` and the creature selector are the reference reusable
  components.

`NestedBorrow`/`ShadowBorrow` exist to resolve a stack object while later
regaining mutable access to its containing `Context`: code can inspect the
stack object first, then consume the guard with `finish()` before mutating the
context.  This is an unsafe, deliberately narrow borrowing escape hatch.  New
uses must maintain the no-aliasing contract: do not retain an inner-object
reference once access to the parent context is recovered.

The typed stack representation is the design direction, but casting, insertion
onto the stack, target revalidation, resolution, and cleanup are not yet
connected end to end.  `context/execution/casting.rs` is presently a stub.

## Input, priority, and turns

Applications implement `Input`; it receives read-only `Game` plus opaque
`Choice<T>` values and returns choices/actions.  `Controller` owns that input,
converts the generic `AnyTarget` choice API to concrete zone/player ID APIs,
and validates cardinality, membership, and uniqueness.  Engine code should ask
the controller rather than accepting unvalidated IDs from callers.

`PlayerId` has two fixed values (`PlayPlayer`, `DrawPlayer`).  `Game` tracks the
active player, player with priority, and last non-passing priority holder.
`pass_player_priority` changes priority and is intended to resolve a stack item
after consecutive passes or advance the turn when the stack is empty.  Stack
resolution remains a placeholder.

`Step`, `Phase`, and `TURN_STEPS` provide the turn vocabulary.  The turn module
contains the intended turn-based actions: draw at draw step, untap at untap,
discard down to seven and clear damage in cleanup.  Combat is explicitly not
implemented.  Step advancement and `Context::start` are incomplete, so this
should be considered scaffolding rather than a runnable rules loop.
