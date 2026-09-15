use std::range::Range;

use derive_more::{From, TryInto};

use crate::{
    context::controller::Controller,
    game::Game,
    rules::{id::*, player::PlayerId},
};

pub(crate) mod damage;
pub(crate) mod selector;

pub(crate) trait Target {
    fn choose(
        cont: &mut Controller,
        game: &Game,
        choices: Vec<Self>,
        range: Range<usize>,
    ) -> Vec<Self>
    where
        Self: Sized;

    fn choose_cancellable(
        cont: &mut Controller,
        game: &Game,
        choices: Vec<Self>,
        range: Range<usize>,
    ) -> Option<Vec<Self>>
    where
        Self: Sized;
}

#[derive(From, TryInto, Clone, Copy, Debug, PartialEq)]
pub(crate) enum AnyTarget {
    Battlefield(BattlefieldId),
    Exile(ExileId),
    SpellStack(SpellStackId),
    HandDraw(DrawHandId),
    HandPlay(PlayHandId),
    LibraryDraw(DrawLibraryId),
    LibraryPlay(PlayLibraryId),
    GraveyardDraw(DrawGraveyardId),
    GraveyardPlay(PlayGraveyardId),
    AbilityStack(AbilityStackId),
    PlayerTarget(PlayerId),
}

macro_rules! impl_target {
    ($ty:ty, $choose:ident, $choose_cancellable:ident) => {
        impl Target for $ty {
            fn choose(
                cont: &mut Controller,
                game: &Game,
                choices: Vec<Self>,
                range: Range<usize>,
            ) -> Vec<Self>
            where
                Self: Sized,
            {
                cont.$choose(game, choices, range)
            }

            fn choose_cancellable(
                cont: &mut Controller,
                game: &Game,
                choices: Vec<Self>,
                range: Range<usize>,
            ) -> Option<Vec<Self>>
            where
                Self: Sized,
            {
                cont.$choose_cancellable(game, choices, range)
            }
        }
    };
}

impl_target!(
    BattlefieldId,
    choose_battlefield,
    choose_battlefield_cancellable
);

impl_target!(AnyTarget, choose_any, choose_any_cancellable);
impl_target!(ExileId, choose_exile, choose_exile_cancellable);

impl_target!(
    SpellStackId,
    choose_spell_stack,
    choose_spell_stack_cancellable
);

impl_target!(DrawHandId, choose_draw_hand, choose_draw_hand_cancellable);

impl_target!(PlayHandId, choose_play_hand, choose_play_hand_cancellable);

impl_target!(
    DrawLibraryId,
    choose_draw_library,
    choose_draw_library_cancellable
);

impl_target!(
    PlayLibraryId,
    choose_play_library,
    choose_play_library_cancellable
);

impl_target!(
    DrawGraveyardId,
    choose_draw_graveyard,
    choose_draw_graveyard_cancellable
);

impl_target!(
    PlayGraveyardId,
    choose_play_graveyard,
    choose_play_graveyard_cancellable
);

impl_target!(
    AbilityStackId,
    choose_ability_stack,
    choose_ability_stack_cancellable
);

impl_target!(PlayerId, choose_player, choose_player_cancellable);
