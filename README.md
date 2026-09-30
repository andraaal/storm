# storm
A MTG rules engine written in Rust

Currently under active development.

## Setup
1. Install Rust: [Rust](https://rust-lang.org/tools/install/)
2. Go into your project folder an run `cargo init`
3. Then go into `Cargo.toml` and add this entry under `[dependencies]`: `storm-engine = "0.1"`
4. Copy the code from `minimal_example.rs` into `src/main.rs`
5. Build and execute with `cargo run`

### Explanation
First there is a struct `MyInput`, which implements the trait `Input`. This serves as the source of input for the engine. Then (at the end of the example file) you create a sample deck from card constants provided by the library.

In the context builder you put everything together to obtain a runnable game. Calling `.start()` starts the game and blocks execution until it is finished. It returns the result of the game. In this minimal example the players don't do anything and just draw cards until the starting player loses, because they run out of cards.

## Implemented cards
Currently available card definitions: `FOREST`, `MOUNTAIN`, `PLAINS`,
`BEAR_CUB`, `GIANT_GROWTH`, `LIGHTNING_BOLT`, and
`ANTHEM_OF_CHAMPIONS`.

## Missing features
Sadly combat with creatures is not implemented yet
