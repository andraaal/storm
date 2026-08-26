use std::{collections::HashSet, range::Range};

use crate::{
    game::Game,
    mana::ManaColor,
    rules::{
        id::{ObjectId, StackId},
        player::PlayerId,
        zone::Zone,
    },
};

pub trait Input {
    fn choose_objects(
        &mut self,
        game: &Game,
        choices: &[ObjectId],
        from: Zone,
        range: Range<usize>,
    ) -> Vec<ObjectId>;
    fn choose_player(&mut self, game: &Game) -> PlayerId;
    fn choose_color(&mut self, game: &Game) -> ManaColor;
    fn choose_number(&mut self, game: &Game, range: Range<usize>) -> usize;
    fn choose_stack_objects(
        &mut self,
        game: &Game,
        choices: &[StackId],
        amount: usize,
    ) -> Vec<StackId>;
    fn take_action(&mut self, active_player: PlayerId);
}

pub struct Controller {
    pub(crate) input: Box<dyn Input>,
}

impl Controller {
    pub(crate) fn new(input: Box<dyn Input>) -> Self {
        Self { input }
    }

    pub(crate) fn choose_objects(
        &mut self,
        game: &Game,
        choices: Vec<ObjectId>,
        from: Zone,
        number_of_choices: Range<usize>,
    ) -> Vec<ObjectId> {
        if number_of_choices.is_empty() {
            panic!("Invalid requested number of choices: range is empty");
        }

        let input = self
            .input
            .choose_objects(game, &choices, from, number_of_choices);

        if !number_of_choices.contains(&input.len()) {
            panic!("Invalid number of objects chosen");
        }
        if input.iter().any(|id| !choices.contains(id)) {
            panic!("Invalid object chosen");
        }
        // Check that all targets are unique
        if input.len() != input.iter().collect::<std::collections::HashSet<_>>().len() {
            panic!("Duplicate objects chosen");
        }
        input
    }

    #[expect(unused_variables)]
    pub(crate) fn choose_objects_cancellable(
        &mut self,
        game: &Game,
        choices: Vec<ObjectId>,
        from: Zone,
        number_of_choices: Range<usize>,
    ) -> Option<Vec<ObjectId>> {
        todo!()
    }

    pub(crate) fn choose_player(&mut self, game: &Game) -> PlayerId {
        let player = self.input.choose_player(game);
        player
    }

    #[expect(unused_variables)]
    pub(crate) fn choose_player_cancellable(&mut self, game: &Game) -> Option<PlayerId> {
        todo!()
    }

    pub(crate) fn choose_color(&mut self, game: &Game) -> ManaColor {
        self.input.choose_color(game)
    }

    #[expect(unused_variables)]
    pub(crate) fn choose_color_cancellable(&mut self, game: &Game) -> Option<ManaColor> {
        todo!()
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

    #[expect(unused_variables)]
    pub(crate) fn choose_number_cancellable(&mut self, game: &Game) -> Option<usize> {
        todo!()
    }

    pub(crate) fn choose_stack_objects(
        &mut self,
        game: &Game,
        targets: Vec<StackId>,
        amount: usize,
    ) -> Vec<StackId> {
        let input = self.input.choose_stack_objects(game, &targets, amount);

        if input.len() != amount {
            panic!("Invalid number of stack objects chosen");
        }

        if input.iter().any(|id| !targets.contains(id)) {
            panic!("Invalid stack object chosen");
        }

        if input.len() != input.iter().collect::<HashSet<_>>().len() {
            panic!("Duplicate stack objects chosen");
        }

        input
    }

    #[expect(unused_variables)]
    pub(crate) fn choose_stack_objects_cancellable(
        &mut self,
        game: &Game,
        targets: Vec<StackId>,
        amount: usize,
    ) -> Option<Vec<StackId>> {
        todo!()
    }

    pub(crate) fn take_action(&mut self, active_player: PlayerId) {
        self.input.take_action(active_player);
    }
}
