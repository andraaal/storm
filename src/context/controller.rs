use std::{fmt::Debug, range::Range};

use crate::{
    game::Game,
    mana::ManaColor,
    rules::{
        id::{
            AbilityStackId, BattlefieldId, DrawGraveyardId, DrawHandId, DrawLibraryId, ExileId,
            PlayGraveyardId, PlayHandId, PlayLibraryId, SpellStackId,
        },
        player::PlayerId,
        player_action::PlayerAction,
        target::{AnyTarget, Target},
    },
};

#[derive(PartialEq, Clone)]
pub struct Choice<T: PartialEq + Clone> {
    option: T,
}

impl<T: PartialEq + Clone> Choice<T> {
    pub fn id(&self) -> &T {
        &self.option
    }
}

fn cast<T: PartialEq + Into<U> + Copy, U: PartialEq + Clone>(
    choices: &[Choice<T>],
) -> Vec<Choice<U>> {
    choices
        .iter()
        .map(|c| Choice {
            option: (*c.id()).into(),
        })
        .collect()
}

fn try_cast<T: PartialEq + TryInto<U> + Copy, U: PartialEq + Clone>(
    choices: Vec<Choice<T>>,
) -> Vec<Choice<U>>
where
    <T as TryInto<U>>::Error: Debug,
{
    choices
        .iter()
        .map(|c| Choice {
            option: (*c.id())
                .try_into()
                .expect("User returned invalid type choice"),
        })
        .collect()
}

pub trait Input {
    fn choose_battlefield_cancellable(
        &mut self,
        game: &Game,
        choices: &[Choice<BattlefieldId>],
        range: Range<usize>,
    ) -> Option<Vec<Choice<BattlefieldId>>> {
        Some(try_cast(self.choose_any_cancellable(
            game,
            &cast(choices),
            range,
        )?))
    }

    fn choose_battlefield(
        &mut self,
        game: &Game,
        choices: &[Choice<BattlefieldId>],
        range: Range<usize>,
    ) -> Vec<Choice<BattlefieldId>> {
        try_cast(self.choose_any(game, &cast(choices), range))
    }

    fn choose_exile_cancellable(
        &mut self,
        game: &Game,
        choices: &[Choice<ExileId>],
        range: Range<usize>,
    ) -> Option<Vec<Choice<ExileId>>> {
        Some(try_cast(self.choose_any_cancellable(
            game,
            &cast(choices),
            range,
        )?))
    }

    fn choose_exile(
        &mut self,
        game: &Game,
        choices: &[Choice<ExileId>],
        range: Range<usize>,
    ) -> Vec<Choice<ExileId>> {
        try_cast(self.choose_any(game, &cast(choices), range))
    }

    fn choose_stack_spell_cancellable(
        &mut self,
        game: &Game,
        choices: &[Choice<SpellStackId>],
        range: Range<usize>,
    ) -> Option<Vec<Choice<SpellStackId>>> {
        Some(try_cast(self.choose_any_cancellable(
            game,
            &cast(choices),
            range,
        )?))
    }

    fn choose_stack_spell(
        &mut self,
        game: &Game,
        choices: &[Choice<SpellStackId>],
        range: Range<usize>,
    ) -> Vec<Choice<SpellStackId>> {
        try_cast(self.choose_any(game, &cast(choices), range))
    }

    fn choose_play_hand_cancellable(
        &mut self,
        game: &Game,
        choices: &[Choice<PlayHandId>],
        range: Range<usize>,
    ) -> Option<Vec<Choice<PlayHandId>>> {
        Some(try_cast(self.choose_any_cancellable(
            game,
            &cast(choices),
            range,
        )?))
    }

    fn choose_play_hand(
        &mut self,
        game: &Game,
        choices: &[Choice<PlayHandId>],
        range: Range<usize>,
    ) -> Vec<Choice<PlayHandId>> {
        try_cast(self.choose_any(game, &cast(choices), range))
    }

    fn choose_draw_hand_cancellable(
        &mut self,
        game: &Game,
        choices: &[Choice<DrawHandId>],
        range: Range<usize>,
    ) -> Option<Vec<Choice<DrawHandId>>> {
        Some(try_cast(self.choose_any_cancellable(
            game,
            &cast(choices),
            range,
        )?))
    }

    fn choose_draw_hand(
        &mut self,
        game: &Game,
        choices: &[Choice<DrawHandId>],
        range: Range<usize>,
    ) -> Vec<Choice<DrawHandId>> {
        try_cast(self.choose_any(game, &cast(choices), range))
    }

    fn choose_play_graveyard_cancellable(
        &mut self,
        game: &Game,
        choices: &[Choice<PlayGraveyardId>],
        range: Range<usize>,
    ) -> Option<Vec<Choice<PlayGraveyardId>>> {
        Some(try_cast(self.choose_any_cancellable(
            game,
            &cast(choices),
            range,
        )?))
    }

    fn choose_play_graveyard(
        &mut self,
        game: &Game,
        choices: &[Choice<PlayGraveyardId>],
        range: Range<usize>,
    ) -> Vec<Choice<PlayGraveyardId>> {
        try_cast(self.choose_any(game, &cast(choices), range))
    }

    fn choose_draw_graveyard_cancellable(
        &mut self,
        game: &Game,
        choices: &[Choice<DrawGraveyardId>],
        range: Range<usize>,
    ) -> Option<Vec<Choice<DrawGraveyardId>>> {
        Some(try_cast(self.choose_any_cancellable(
            game,
            &cast(choices),
            range,
        )?))
    }

    fn choose_draw_graveyard(
        &mut self,
        game: &Game,
        choices: &[Choice<DrawGraveyardId>],
        range: Range<usize>,
    ) -> Vec<Choice<DrawGraveyardId>> {
        try_cast(self.choose_any(game, &cast(choices), range))
    }

    fn choose_play_library_cancellable(
        &mut self,
        game: &Game,
        choices: &[Choice<PlayLibraryId>],
        range: Range<usize>,
    ) -> Option<Vec<Choice<PlayLibraryId>>> {
        Some(try_cast(self.choose_any_cancellable(
            game,
            &cast(choices),
            range,
        )?))
    }

    fn choose_play_library(
        &mut self,
        game: &Game,
        choices: &[Choice<PlayLibraryId>],
        range: Range<usize>,
    ) -> Vec<Choice<PlayLibraryId>> {
        try_cast(self.choose_any(game, &cast(choices), range))
    }

    fn choose_draw_library_cancellable(
        &mut self,
        game: &Game,
        choices: &[Choice<DrawLibraryId>],
        range: Range<usize>,
    ) -> Option<Vec<Choice<DrawLibraryId>>> {
        Some(try_cast(self.choose_any_cancellable(
            game,
            &cast(choices),
            range,
        )?))
    }

    fn choose_draw_library(
        &mut self,
        game: &Game,
        choices: &[Choice<DrawLibraryId>],
        range: Range<usize>,
    ) -> Vec<Choice<DrawLibraryId>> {
        try_cast(self.choose_any(game, &cast(choices), range))
    }

    fn choose_spell_stack_cancellable(
        &mut self,
        game: &Game,
        choices: &[Choice<SpellStackId>],
        range: Range<usize>,
    ) -> Option<Vec<Choice<SpellStackId>>> {
        Some(try_cast(self.choose_any_cancellable(
            game,
            &cast(choices),
            range,
        )?))
    }

    fn choose_spell_stack(
        &mut self,
        game: &Game,
        choices: &[Choice<SpellStackId>],
        range: Range<usize>,
    ) -> Vec<Choice<SpellStackId>> {
        try_cast(self.choose_any(game, &cast(choices), range))
    }

    fn choose_ability_stack_cancellable(
        &mut self,
        game: &Game,
        choices: &[Choice<AbilityStackId>],
        range: Range<usize>,
    ) -> Option<Vec<Choice<AbilityStackId>>> {
        Some(try_cast(self.choose_any_cancellable(
            game,
            &cast(choices),
            range,
        )?))
    }

    fn choose_ability_stack(
        &mut self,
        game: &Game,
        choices: &[Choice<AbilityStackId>],
        range: Range<usize>,
    ) -> Vec<Choice<AbilityStackId>> {
        try_cast(self.choose_any(game, &cast(choices), range))
    }

    fn choose_any_cancellable(
        &mut self,
        game: &Game,
        choices: &[Choice<AnyTarget>],
        range: Range<usize>,
    ) -> Option<Vec<Choice<AnyTarget>>>;

    fn choose_any(
        &mut self,
        game: &Game,
        choices: &[Choice<AnyTarget>],
        range: Range<usize>,
    ) -> Vec<Choice<AnyTarget>>;
    fn choose_player(
        &mut self,
        game: &Game,
        choices: &[Choice<PlayerId>],
        range: Range<usize>,
    ) -> Vec<Choice<PlayerId>>;
    fn choose_player_cancellable(
        &mut self,
        game: &Game,
        choices: &[Choice<PlayerId>],
        range: Range<usize>,
    ) -> Option<Vec<Choice<PlayerId>>>;
    fn choose_color(&mut self, game: &Game) -> ManaColor;
    fn choose_color_cancellable(&mut self, game: &Game) -> Option<ManaColor>;
    fn choose_number_cancellable(&mut self, game: &Game, range: Range<usize>) -> Option<usize>;
    fn choose_number(&mut self, game: &Game, range: Range<usize>) -> usize;
    fn take_action(
        &mut self,
        game: &Game,
        choices: &[Choice<PlayerAction>],
    ) -> Choice<PlayerAction>;
}

pub struct Controller {
    pub(crate) input: Box<dyn Input>,
}

fn validate<T: PartialEq + Clone>(
    choices: Vec<Choice<T>>,
    range: Range<usize>,
    input: Vec<Choice<T>>,
) -> Vec<T> {
    if range.is_empty() {
        panic!("Invalid requested number of choices: range is empty");
    }

    if !range.contains(&input.len()) {
        panic!("Invalid number of objects chosen");
    }
    if input.iter().any(|choice| !choices.contains(choice)) {
        panic!("Invalid object chosen");
    }
    // Check that all targets are unique
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            if input[i] == input[j] {
                panic!("Duplicate objects chosen (or duplicates in target selection)");
            }
        }
    }
    input.into_iter().map(|Choice { option }| option).collect()
}

macro_rules! impl_choose {
    (
        $ty:ty,
        $choose:ident,
        $choose_cancellable:ident
    ) => {
        pub(crate) fn $choose(
            &mut self,
            game: &Game,
            choices: Vec<$ty>,
            range: Range<usize>,
        ) -> Vec<$ty> {
            let choices: Vec<_> = choices
                .into_iter()
                .map(|option| Choice { option })
                .collect();

            let input = self.input.$choose(game, &choices, range);
            validate(choices, range, input)
        }

        pub(crate) fn $choose_cancellable(
            &mut self,
            game: &Game,
            choices: Vec<$ty>,
            range: Range<usize>,
        ) -> Option<Vec<$ty>> {
            let choices: Vec<_> = choices
                .into_iter()
                .map(|option| Choice { option })
                .collect();

            let input = self.input.$choose_cancellable(game, &choices, range)?;

            Some(validate(choices, range, input))
        }
    };
}

impl Controller {
    pub(crate) fn new(input: Box<dyn Input>) -> Self {
        Self { input }
    }

    pub(crate) fn choose<T: Target>(
        &mut self,
        game: &Game,
        choices: Vec<T>,
        range: Range<usize>,
    ) -> Vec<T> {
        T::choose(self, game, choices, range)
    }

    pub(crate) fn choose_cancellable<T: Target>(
        &mut self,
        game: &Game,
        choices: Vec<T>,
        range: Range<usize>,
    ) -> Option<Vec<T>> {
        T::choose_cancellable(self, game, choices, range)
    }

    impl_choose!(AnyTarget, choose_any, choose_any_cancellable);
    impl_choose!(ExileId, choose_exile, choose_exile_cancellable);
    impl_choose!(DrawHandId, choose_draw_hand, choose_draw_hand_cancellable);
    impl_choose!(PlayHandId, choose_play_hand, choose_play_hand_cancellable);
    impl_choose!(PlayerId, choose_player, choose_player_cancellable);

    impl_choose!(
        BattlefieldId,
        choose_battlefield,
        choose_battlefield_cancellable
    );

    impl_choose!(
        DrawLibraryId,
        choose_draw_library,
        choose_draw_library_cancellable
    );

    impl_choose!(
        PlayLibraryId,
        choose_play_library,
        choose_play_library_cancellable
    );

    impl_choose!(
        DrawGraveyardId,
        choose_draw_graveyard,
        choose_draw_graveyard_cancellable
    );

    impl_choose!(
        PlayGraveyardId,
        choose_play_graveyard,
        choose_play_graveyard_cancellable
    );

    impl_choose!(
        AbilityStackId,
        choose_ability_stack,
        choose_ability_stack_cancellable
    );

    impl_choose!(
        SpellStackId,
        choose_spell_stack,
        choose_spell_stack_cancellable
    );

    pub(crate) fn choose_color(&mut self, game: &Game) -> ManaColor {
        self.input.choose_color(game)
    }

    pub(crate) fn choose_color_cancellable(&mut self, game: &Game) -> Option<ManaColor> {
        self.input.choose_color_cancellable(game)
    }

    pub(crate) fn choose_number(&mut self, game: &Game, range: Range<usize>) -> usize {
        if range.is_empty() {
            panic!("Invalid requested number choice range: range is empty");
        }

        let number = self.input.choose_number(game, range);

        if !range.contains(&number) {
            panic!("Invalid number chosen");
        }

        number
    }

    pub(crate) fn choose_number_cancellable(
        &mut self,
        game: &Game,
        range: Range<usize>,
    ) -> Option<usize> {
        if range.is_empty() {
            panic!("Invalid requested number choice range: range is empty");
        }

        let number = self.input.choose_number_cancellable(game, range)?;

        if !range.contains(&number) {
            panic!("Invalid number chosen");
        }
        Some(number)
    }

    pub(crate) fn choose_action(
        &mut self,
        game: &Game,
        choices: Vec<PlayerAction>,
    ) -> PlayerAction {
        let choices = choices
            .into_iter()
            .map(|i| Choice { option: i })
            .collect::<Vec<_>>();
        let input = self.input.take_action(game, &choices);

        validate(choices, Range { start: 1, end: 2 }, vec![input])[0].clone()
    }
}
