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
        ContextBuilder, FOREST, Game, Input, MOUNTAIN, PlayerId,
        context::controller::Choice as InternalChoice,
        mana::ManaColor,
        rules::{player_action::PlayerAction, target::AnyTarget},
    };

    struct PlayTwoLands {
        next_card: usize,
    }

    impl Input for PlayTwoLands {
        fn choose_any_cancellable(
            &mut self,
            _game: &Game,
            choices: &[InternalChoice<AnyTarget>],
            _range: Range<usize>,
        ) -> Option<Vec<InternalChoice<AnyTarget>>> {
            Some(vec![choices[0].clone()])
        }

        fn choose_any(
            &mut self,
            _game: &Game,
            choices: &[InternalChoice<AnyTarget>],
            _range: Range<usize>,
        ) -> Vec<InternalChoice<AnyTarget>> {
            vec![choices[0].clone()]
        }

        fn choose_player(
            &mut self,
            _game: &Game,
            choices: &[InternalChoice<PlayerId>],
            _range: Range<usize>,
        ) -> Vec<InternalChoice<PlayerId>> {
            vec![choices[0].clone()]
        }

        fn choose_player_cancellable(
            &mut self,
            _game: &Game,
            choices: &[InternalChoice<PlayerId>],
            _range: Range<usize>,
        ) -> Option<Vec<InternalChoice<PlayerId>>> {
            Some(vec![choices[0].clone()])
        }

        fn choose_color(&mut self, _game: &Game) -> ManaColor {
            ManaColor::Green
        }

        fn choose_color_cancellable(&mut self, _game: &Game) -> Option<ManaColor> {
            Some(ManaColor::Green)
        }

        fn choose_number_cancellable(
            &mut self,
            _game: &Game,
            range: Range<usize>,
        ) -> Option<usize> {
            Some(range.start)
        }

        fn choose_number(&mut self, _game: &Game, range: Range<usize>) -> usize {
            range.start
        }

        fn take_action(
            &mut self,
            game: &Game,
            choices: &[InternalChoice<PlayerAction>],
        ) -> InternalChoice<PlayerAction> {
            let wanted = match self.next_card {
                0 => "Forest",
                1 => "Mountain",
                _ => "",
            };

            if let Some(choice) = choices.iter().find(|choice| {
                let PlayerAction::PlayCard(id) = choice.id() else {
                    return false;
                };
                game.objects
                    .get_any(*id)
                    .is_some_and(|object| object.characteristics.name == wanted)
            }) {
                self.next_card += 1;
                return choice.clone();
            }

            choices
                .iter()
                .find(|choice| matches!(choice.id(), PlayerAction::PassPriority))
                .unwrap()
                .clone()
        }
    }

    #[test]
    fn starts_with_seven_cards_and_plays_two_lands() {
        let deck = vec![
            &FOREST, &MOUNTAIN, &FOREST, &MOUNTAIN, &FOREST, &MOUNTAIN, &FOREST,
        ];
        let mut context = ContextBuilder::new(Box::new(PlayTwoLands { next_card: 0 }))
            .with_play_deck(deck.clone())
            .with_draw_deck(deck)
            .build()
            .unwrap();

        context.start().unwrap();

        assert_eq!(context.game.objects.play_hand.len(), 5);
        assert_eq!(context.game.objects.draw_hand.len(), 7);
        assert_eq!(context.game.objects.battlefield.len(), 2);
        assert_eq!(
            context
                .game
                .objects
                .battlefield
                .values()
                .filter(|(_, card)| {
                    card.characteristics.name == "Forest" || card.characteristics.name == "Mountain"
                })
                .count(),
            2
        );
    }
}
