# Storm Alpha v1 Specification

**Status:** Proposed; derived from `ALPHA-V1-DESIGN.md` and the useful,
non-legacy portions of `DESIGN.md`.

This document is the normative alpha specification. Existing implementation
details that contradict it are not requirements. `DESIGN.md` is an
architectural reference and may describe unfinished or outdated behavior.

## 1. Scope

Storm alpha v1 is a two-player, local, synchronous Rust rules engine for a
small hand-authored card set. It must support complete, testable games using
the cards and rules in this document. It does not attempt to implement the
Magic: The Gathering Comprehensive Rules in full.

The alpha is a library. It does not require a CLI, UI, network transport,
persistence, replay, semver compatibility, release tags, CI, or an imported
card database.

## 2. Architectural Constraints

The following architecture from `DESIGN.md` remains the intended direction:

- `Context` owns the game session, application input adapter, and rule
  execution orchestration.
- `Game` is the authoritative mutable game-state aggregate.
- `rules` contains domain types such as zones, IDs, costs, targets, layers,
  actions, and turn vocabulary.
- `cards` contains hand-authored card definitions and reusable effect/selector
  components.
- Objects are stored by zone using generational IDs. Moving an object to a
  different zone gives it a new zone-specific ID and timestamp.
- `GameAction` remains the central vocabulary for state mutations.
- Characteristic changes are represented through the existing layering model,
  not by bypassing it with unrelated direct mutations.
- The typed stack model remains the preferred representation for spells and
  their effects, provided it is wired end to end.
- The callback-based `Input` integration seam is retained and narrowed to the
  alpha interaction surface.

Legacy, commented-out, or unreachable implementations are not part of the
alpha contract.

## 3. Public API

The crate root must expose:

- `Game`
- `Context`
- `ContextBuilder`
- `Input`
- Relevant player, card, object, zone, stack, and target IDs
- The supported card definitions
- Expected error types
- `GameResult`

Implementation modules remain private unless they are deliberately part of
the consumer-facing API.

`Game` fields are public for inspection. The normal integration path exposes
the game through `&Game`; gameplay mutation is owned by the engine. Direct
arbitrary mutation of public fields is not a supported way to play a game.

The exact names and signatures may change during alpha implementation. The
minimum usable flow is:

1. Construct two decks and an optional shuffle seed.
2. Build a context with an application-provided `Input` implementation.
3. Start the context.
4. Inspect `&Game`.
5. Respond to each requested legal decision.
6. Continue until `GameResult` is terminal.

## 4. Input and Progression

The engine must use a synchronous callback model:

- It requests one decision at a time.
- It never starts a background thread or blocks waiting for an external
  controller.
- It automatically performs mandatory rules work between decisions.
- It must not request input after the game reaches a terminal result.

At each priority point, the engine must expose only currently legal actions.
The minimum priority actions are:

- Pass priority.
- Play a legal land.
- Cast a legal spell.

The engine must not offer unsupported choices such as mulligans. Expected
invalid game actions return typed errors. Impossible internal invariant
violations may panic.

## 5. Construction and Startup

`ContextBuilder` must reject:

- A missing deck.
- A deck smaller than seven cards.
- Unsupported card definitions.

Building a context creates the game object and initializes both players to 20
life. Starting the context then:

1. Shuffles each library using the optional seed.
2. Draws seven cards for each player.
3. Initializes the active player and turn state.
4. Runs mandatory startup work.
5. Requests the first legal priority decision.

The seed is stored in `Game` and controls library shuffling only. Given the
same decks, seed, and player decisions, setup and all card outcomes must be
reproducible. Card effects themselves contain no randomness.

The alpha uses a seven-card minimum for fixture decks. It does not enforce a
format, deck size beyond the minimum, or four-copy limit.

## 6. Card Catalog

The supported catalog is exactly:

| Card | Definition |
|---|---|
| Bear Cub | Creature — Bear, `{1}{G}`, 2/2, no abilities |
| Lightning Bolt | Instant, `{R}`, deal 3 damage to one damageable target |
| Giant Growth | Instant, `{G}`, one creature gets +3/+3 until end of turn |
| Anthem of Champions | Enchantment, `{G}{W}`, creatures its controller controls get +1/+1 |
| Plains | Basic Land; its mana ability produces `{W}` |
| Forest | Basic Land; its mana ability produces `{G}` |
| Mountain | Basic Land; its mana ability produces `{R}` |

The existing Bear Cub and Anthem constant/name wiring must be corrected.
Anthem is a continuous static effect, not a trigger and not a targeted effect.
There are no ETB or other triggered abilities in alpha.

## 7. Zones, Ownership, and Control

The engine must support the zones needed by the catalog and rules:

- Library
- Hand
- Battlefield
- Stack
- Graveyard

Cards retain their owner and remain under their owner’s control for alpha.
The model may represent owner and controller separately where it already does
so, but no control-changing effect is required.

Moving an object between zones must preserve the engine’s generational-ID and
timestamp invariants. Code must not infer ownership from an opaque ID when the
zone/object model provides an ownership lookup.

## 8. Mana and Casting

- Basic lands enter the battlefield untapped.
- A player may play at most one land per turn.
- Tapping a basic land produces one mana of its color.
- Mana costs are typed and are validated when a spell is cast.
- Casting consumes the required mana.
- Mana pools empty at the relevant phase/step boundaries.
- No floating mana across phases, alternate costs, additional costs, or `X`
  costs are required.

## 9. Turns and Priority

The active player receives priority first after each mandatory step. Priority
then alternates between the two players.

Two consecutive passes have these effects:

- If the stack is nonempty, resolve its top object.
- If the stack is empty, advance to the next turn step.

Mandatory operations are automatic and must not require user input. This
includes the alpha-supported untap, draw, cleanup, mana-pool clearing, and
state-based-action processing.

The first player skips the draw during their first draw step. The normal
one-land-per-turn restriction applies.

## 10. Stack, Targets, and Resolution

Supported instants and other spells use the stack:

1. The player chooses a legal spell and pays its cost.
2. Targets are selected during casting.
3. The spell is placed on the stack.
4. Players receive priority.
5. After the required passes, the top object resolves.
6. Targets are revalidated at resolution.
7. An effect with an illegal or no-longer-legal target does nothing.
8. A resolved instant or sorcery moves to its owner’s graveyard.

The alpha has no multi-target effects. Target selectors must return only
currently legal targets.

## 11. Combat

Combat is explicit and consists of:

1. Declare attackers.
2. Declare blockers.
3. Resolve ordinary simultaneous combat damage.
4. Process state-based actions and resulting life loss.

The engine must reject illegal attacker and blocker declarations. It must not
infer attacks or blocks from available cards.

The following are excluded:

- First strike
- Double strike
- Trample
- Vigilance
- Banding
- Combat-triggered abilities

## 12. Layering and State-Based Actions

Anthem’s continuous +1/+1 effect must be applied through the existing
layering mechanism and only while the enchantment is in its applicable zone.
The effect applies to creatures controlled by Anthem’s controller.

State-based actions must run repeatedly until the state is stable:

- Before priority.
- After priority actions where required.
- After spell resolution.
- After combat damage.

At minimum:

- A creature with lethal marked damage moves to its owner’s graveyard.
- A creature with zero or less toughness moves to its owner’s graveyard.
- A player at zero or less life loses.
- Drawing from an empty library causes that player to lose.

## 13. Game Results

The engine must expose a terminal `GameResult` when a player loses. The
opponent is the winner for this two-player alpha.

Once `GameResult` is terminal:

- No further input is requested.
- The engine does not advance the game.
- Inspection of the final `Game` state remains possible.

Concessions and multiplayer result handling are out of scope.

## 14. Explicit Exclusions

The following are not alpha requirements:

- Mulligans
- Triggered abilities
- Replacement effects
- Control-changing effects
- Alternate, additional, or `X` costs
- Comprehensive layer dependency handling beyond Anthem
- Planeswalkers
- First strike, double strike, trample, vigilance, or banding
- Multiplayer
- Concessions
- Networking
- Persistence
- Replay
- Imported card data
- Broad completion of unrelated legacy mechanics

If an excluded path is reached despite the input surface, it must be rejected
explicitly rather than silently treated as successful.

## 15. Acceptance Tests

The alpha is complete only when Rust unit and integration tests cover:

- Context construction and validation.
- Seeded shuffle and seven-card opening hands.
- Game startup and first priority.
- Land play and one-land-per-turn enforcement.
- Basic-land mana production.
- Mana payment and insufficient-mana rejection.
- Priority passing and turn advancement.
- Bear Cub casting and battlefield placement.
- Lightning Bolt targeting, damage, resolution, and graveyard movement.
- Giant Growth targeting and end-of-turn expiry.
- Anthem’s continuous effect and controller filtering.
- Target revalidation after a target changes zones.
- Explicit attacker and blocker choices.
- Simultaneous combat damage.
- Repeated state-based actions.
- Lethal creature cleanup.
- Player life loss.
- Empty-library loss.
- Terminal `GameResult` and absence of post-game input.
- Invalid action handling.

At least one documented happy-path example or executable/manual smoke test
must demonstrate deck construction, startup, priority decisions, and a
complete representative action sequence.
