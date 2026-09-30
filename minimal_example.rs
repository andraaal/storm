use storm_engine::AnyTarget;
use storm_engine::Choice;
use storm_engine::ContextBuilder;
use storm_engine::FOREST;
use storm_engine::Game;
use storm_engine::Input;
use storm_engine::MOUNTAIN;
use storm_engine::ManaColor;
use storm_engine::PlayerAction;
use storm_engine::PlayerId;

// The Input object that controls player actions
struct MyInput {}
impl Input for MyInput {
    fn choose_any_cancellable(
        &mut self,
        game: &Game, // The Game objects records the current game state. You can extract every available information about the game from here.
        choices: &[Choice<AnyTarget>], // The choices that are available to you
        range: std::range::Range<usize>, // The amount of choices you need to submit (currently there only exist cards where you submit one choice)
    ) -> Option<Vec<Choice<AnyTarget>>> {
        None
    }

    fn choose_any(
        &mut self,
        game: &Game,
        choices: &[Choice<AnyTarget>],
        range: std::range::Range<usize>,
    ) -> Vec<Choice<AnyTarget>> {
        choices
            .into_iter()
            .take(range.start)
            .map(|e| e.clone())
            .collect()
    }

    fn choose_player(
        &mut self,
        game: &Game,
        choices: &[Choice<PlayerId>],
        range: std::range::Range<usize>,
    ) -> Vec<Choice<PlayerId>> {
        vec![choices[0].clone()]
    }

    fn choose_player_cancellable(
        &mut self,
        game: &Game,
        choices: &[Choice<PlayerId>],
        range: std::range::Range<usize>,
    ) -> Option<Vec<Choice<PlayerId>>> {
        None
    }

    fn choose_color(&mut self, game: &Game) -> ManaColor {
        ManaColor::Black
    }

    fn choose_color_cancellable(&mut self, game: &Game) -> Option<ManaColor> {
        None
    }

    fn choose_number_cancellable(
        &mut self,
        game: &Game,
        range: std::range::Range<usize>,
    ) -> Option<usize> {
        None
    }

    fn choose_number(&mut self, game: &Game, range: std::range::Range<usize>) -> usize {
        range.start
    }

    fn take_action(
        &mut self,
        game: &Game,
        choices: &[Choice<PlayerAction>],
    ) -> Choice<PlayerAction> {
        // Last option is always passing
        let last = choices.iter().last().unwrap().clone();
        println!("Turn of {:?}", game.active_player);
        println!("{:?} has priority", game.priority);
        println!("Action: {:?}", last);
        println!(
            "Step: {:?} in phase {:?}",
            game.current_step, game.current_phase
        );
        println!("");
        last
    }
}

fn main() {
    // Create deck with 7 cards
    let deck = vec![
        &FOREST, &MOUNTAIN, &FOREST, &MOUNTAIN, &FOREST, &MOUNTAIN, &FOREST, &FOREST,
    ];

    // Create a new game
    let mut context = ContextBuilder::new(Box::new(MyInput {}))
        .with_play_deck(deck.clone())
        .with_draw_deck(deck)
        .build()
        .unwrap();

    // Print game result
    // The expecte result is that both players pass every step and phase for an entire turn + Upkeep and then the PlayPlayer loses, since they would need to draw a card, but their library is empty
    println!("Result: {:?}", context.start().unwrap());

    // Try increasing the card count to see the players play through multiple turns without losing.
}
