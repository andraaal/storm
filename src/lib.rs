#![allow(dead_code, private_interfaces)]
#![feature(checked_type_aliases)]
#![feature(min_specialization)]

mod cards;
mod context;
mod game;
pub(crate) mod mana;
pub(crate) mod nested_borrow;
mod rules;

pub use cards::cards::basic_lands::{FOREST, MOUNTAIN, PLAINS};
pub use cards::cards::{
    anthem_of_champions::ANTHEM_OF_CHAMPIONS, bear_cub::BEAR_CUB, giant_growth::GIANT_GROWTH,
    lightning_bolt::LIGHTNING_BOLT,
};
pub use context::context_builder::ContextBuilder;
pub use context::controller::{Choice, Input};
pub use context::{Context, EngineError, GameResult};
pub use game::Game;
pub use game::objects::Objects;
pub use rules::id::{
    AbilityStackId, AnyId, BattlefieldId, DrawGraveyardId, DrawHandId, DrawLibraryId, ExileId,
    PlayGraveyardId, PlayHandId, PlayLibraryId, SpellStackId,
};
pub use rules::object::const_characteristics::ConstCharacteristics;
pub use rules::player::PlayerId;

#[cfg(test)]
mod tests {
    use std::range::Range;

    use crate::{
        BEAR_CUB, Choice, ContextBuilder, EngineError, FOREST, GIANT_GROWTH, Game, LIGHTNING_BOLT,
        MOUNTAIN, PLAINS, PlayerId,
        context::controller::Input,
        mana::ManaColor,
        rules::{player_action::PlayerAction, target::AnyTarget},
    };

    struct PassInput;

    impl Input for PassInput {
        fn choose_any_cancellable(
            &mut self,
            _game: &Game,
            _choices: &[Choice<AnyTarget>],
            _range: Range<usize>,
        ) -> Option<Vec<Choice<AnyTarget>>> {
            None
        }

        fn choose_any(
            &mut self,
            _game: &Game,
            choices: &[Choice<AnyTarget>],
            _range: Range<usize>,
        ) -> Vec<Choice<AnyTarget>> {
            choices.first().cloned().into_iter().collect()
        }

        fn choose_player(
            &mut self,
            _game: &Game,
            choices: &[Choice<PlayerId>],
            _range: Range<usize>,
        ) -> Vec<Choice<PlayerId>> {
            choices.first().cloned().into_iter().collect()
        }

        fn choose_player_cancellable(
            &mut self,
            _game: &Game,
            _choices: &[Choice<PlayerId>],
            _range: Range<usize>,
        ) -> Option<Vec<Choice<PlayerId>>> {
            None
        }

        fn choose_color(&mut self, _game: &Game) -> ManaColor {
            ManaColor::Green
        }

        fn choose_color_cancellable(&mut self, _game: &Game) -> Option<ManaColor> {
            None
        }

        fn choose_number_cancellable(
            &mut self,
            _game: &Game,
            _range: Range<usize>,
        ) -> Option<usize> {
            None
        }

        fn choose_number(&mut self, _game: &Game, range: Range<usize>) -> usize {
            range.start
        }

        fn take_action(
            &mut self,
            _game: &Game,
            choices: &[Choice<PlayerAction>],
        ) -> Choice<PlayerAction> {
            choices
                .last()
                .cloned()
                .expect("priority always has an action")
        }
    }

    #[test]
    fn public_contract_builds_and_starts_a_game() {
        let deck = vec![
            &FOREST,
            &BEAR_CUB,
            &GIANT_GROWTH,
            &LIGHTNING_BOLT,
            &PLAINS,
            &MOUNTAIN,
            &FOREST,
        ];
        let mut context = ContextBuilder::new(Box::new(PassInput))
            .with_play_deck(deck.clone())
            .with_draw_deck(deck)
            .build()
            .expect("valid alpha decks");

        context.start().expect("startup succeeds");

        assert_eq!(context.game.players.play_player.life, 20);
        assert_eq!(context.game.players.draw_player.life, 20);
        assert_eq!(context.game.objects.play_hand.len(), 7);
        assert_eq!(context.game.objects.draw_hand.len(), 7);
    }

    #[test]
    fn builder_rejects_missing_and_small_decks() {
        let missing = ContextBuilder::new(Box::new(PassInput)).build();
        assert!(matches!(missing, Err(EngineError::NoDeck)));

        let small = ContextBuilder::new(Box::new(PassInput))
            .with_play_deck(vec![&FOREST; 6])
            .with_draw_deck(vec![&FOREST; 7])
            .build();
        assert!(matches!(small, Err(EngineError::TooSmallDeck)));
    }
}
