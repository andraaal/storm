# storm
A MTG rules engine written in Rust

Currently under active development.

## Setup
Create an `Input` implementation to provide player decisions, then build a
context with two deck with at least 7 cards each. The example assumes `MyInput` implements the Input
trait:

```rust
use storm::{ContextBuilder, FOREST};

// In the input trait you have to implement callbacks that provide desicions for the game
struct MyInput;

fn main() -> Result<(), storm::EngineError> {
    let deck = vec![
        &FOREST, &FOREST, &FOREST, &FOREST,
        &FOREST, &FOREST, &FOREST,
    ];

    let mut game = ContextBuilder::new(Box::new(MyInput))
        .with_play_deck(deck.clone())
        .with_draw_deck(deck)
        .build()?;

    let result = game.start()?;
    println!("Game result: {result:?}");
    Ok(())
}
```

`game.start()` shuffles both libraries, draws the opening hands, and runs
the game loop using decisions from `Input`.

## Implemented cards
Currently available card definitions: `FOREST`, `MOUNTAIN`, `PLAINS`,
`BEAR_CUB`, `GIANT_GROWTH`, `LIGHTNING_BOLT`, and
`ANTHEM_OF_CHAMPIONS`.

## Missing features
Sadly combat with creatures is not implemented yet
