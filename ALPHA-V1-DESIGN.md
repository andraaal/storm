# Alpha v1 Design Summary

> **Status:** Proposed design for review. Implementation should not begin until this summary is explicitly confirmed.

## Goal

Ship a repository-usable alpha v1 of the Rust MTG rules engine with a small, coherent rules subset. The goal is functional gameplay for a representative set of basic cards, not broad or complete Comprehensive Rules coverage.

## Game and Integration Model

- Two-player local games.
- Rust library only; no CLI, UI, networking, multiplayer, persistence, or replay.
- Retain the existing user-implemented callback-based `Input` integration seam.
- Use a synchronous, one-choice-at-a-time interaction model.
- The engine automatically performs mandatory rules work between user decisions.
- The engine calls the input trait at every decision point, even when passing is the only legal choice.
- No blocking controller loop, background thread, or implicit user decision.

`ContextBuilder::build()` constructs the initial game object. `Context::start()` performs startup processing:

1. Seeded library shuffling.
2. Seven-card opening hands.
3. First-player initialization.
4. Automatic mandatory setup until the first priority request.

## Public API Boundary

Export the following at the crate root:

- `Game`
- `Context`
- `ContextBuilder`
- `Input`
- Relevant player, card, object, zone, and stack IDs.
- Supported card constants.
- Error and terminal-result types.

Internal implementation modules remain private. Existing APIs may be renamed or restructured freely because backwards compatibility is not a requirement, but the callback concept and direct `Game` inspection should remain.

`Game` should expose public fields for inspection. Normal consumers receive it through `&Game`; gameplay mutation remains owned by the engine rather than being a supported direct-mutation workflow.

## Determinism and Randomness

- Accept an optional seed.
- Store the seed in `Game`.
- Use the seed only for library shuffling during setup.
- Card effects contain no randomness.
- Use a small established RNG dependency.
- No persistence or replay is required for alpha.

## Supported Cards

The alpha card suite consists of:

| Card | Characteristics and effect |
|---|---|
| Bear Cub | `{1}{G}` creature, 2/2 |
| Lightning Bolt | `{R}` instant; deals 3 damage to one damageable target |
| Giant Growth | `{G}` instant; one creature gets +3/+3 until end of turn |
| Anthem of Champions | `{G}{W}` enchantment; creatures you control get +1/+1 continuously |
| Plains | Basic land; taps for one white mana |
| Forest | Basic land; taps for one green mana |
| Mountain | Basic land; taps for one red mana |

Existing card constant/name wiring errors must be repaired. No Shock, ETB creature, or triggered abilities are included.

## Game Setup and Deck Contract

- Two players.
- 20 starting life.
- At least seven cards per deck.
- Compact fixture decks are allowed.
- Only supported cards may be used.
- No format rules or four-copy limit.
- Libraries are shuffled using the optional seed.
- Each player draws seven cards.
- The active player starts.
- The first player skips their first draw.
- Each player may play at most one land per turn.
- Lands enter untapped.

## Mana and Casting

- Basic lands are the only mana abilities.
- Tapping a basic land produces one mana of its color.
- Supported typed mana costs are validated and paid when casting.
- Mana pools empty at the relevant phase/step boundaries.
- Exclude floating mana across phases, alternate/additional costs, and `X` costs.

## Turn, Priority, and Stack

- The active player receives priority first.
- Priority alternates between players.
- Every priority decision is explicit.
- Legal choices should include only actions currently available, such as:
  - Pass priority.
  - Play a land, if the player has not played one this turn and a legal land is available.
  - Cast a spell, if it is legal and payable.
- Two consecutive passes:
  - Resolve the top stack object when the stack is nonempty.
  - Advance to the next step when the stack is empty.
- Mandatory work, such as untapping, drawing, cleanup, and state-based actions, happens automatically between decision points.

## Spell Resolution and Targets

- Instants and other supported spells use the stack.
- Targets are chosen while casting.
- Targets are revalidated on resolution.
- A spell with an illegal or no-longer-legal target has no effect.
- Resolved instants and sorceries move to their owner’s graveyard.
- Cards remain under their owner’s control; no control-changing effects are included.

## Combat

Combat requires explicit player decisions:

1. Declare attackers.
2. Declare blockers.
3. Assign and resolve ordinary simultaneous combat damage.
4. Process state-based actions and life loss.

Excluded combat mechanics:

- First strike.
- Double strike.
- Trample.
- Vigilance.
- Banding.
- Combat-triggered abilities.

## State-Based Actions and Game End

- Run state-based actions repeatedly until the game state is stable.
- Run them before and after priority and after spell resolution.
- Lethal-damage and zero-toughness creatures go to their owners’ graveyards.
- A player at 0 or less life loses.
- Drawing from an empty library causes a player to lose.
- The opponent wins.
- Expose a terminal `GameResult`.
- Once the result is terminal, the engine requests no further input.

## Errors and Unsupported Features

- Expected invalid gameplay actions should return typed errors.
- Impossible internal invariant violations may panic.
- Do not offer unsupported choices such as mulligans.
- If an unsupported path is nevertheless reached, reject it explicitly rather than silently skipping it.
- Invalid caller input should not be treated as a successful action.

## Explicitly Out of Scope

- Mulligans.
- Triggered abilities.
- Replacement effects.
- Alternate, additional, or `X` costs.
- Continuous-effect dependency/layering beyond what Anthem requires.
- Planeswalkers.
- First strike and double strike.
- Trample.
- Vigilance.
- Banding.
- Multiplayer.
- Concessions.
- Networking.
- Persistence and replay.
- Imported card databases.
- Complete Comprehensive Rules coverage.
- Broad unrelated refactors or completion of unsupported legacy mechanics.

## Acceptance Evidence

Add Rust unit and integration tests using deterministic seeds and compact fixture decks. The acceptance suite should cover:

- Game construction and startup.
- Seeded shuffle and opening hands.
- Land play and mana payment.
- Priority and turn progression.
- Bear Cub.
- Lightning Bolt.
- Giant Growth.
- Anthem of Champions and its continuous power/toughness effect.
- Target legality and target revalidation.
- Combat with attackers, blockers, and simultaneous damage.
- State-based actions.
- Player life loss.
- Empty-library loss.
- Terminal game results.
- Invalid action handling.

Also provide one documented happy-path example or executable/manual smoke test showing deck construction through a complete game-action sequence. Useful doc comments are expected, but semver, release tags, CI, and a polished quickstart are not alpha requirements.
