use crate::{
    game::Game,
    rules::{player::PlayerId, turn::TURN_STEPS},
};

impl Game {
    pub(crate) fn pass_player_priority(&mut self) {
        self.priority = match self.priority {
            PlayerId::PlayPlayer => PlayerId::DrawPlayer,
            PlayerId::DrawPlayer => PlayerId::PlayPlayer,
        };
        if self.last_non_passed_priority == self.priority {
            if !self.stack.is_empty() {
                self.resolve_top_of_stack();
            } else {
                self.advance_step();
            }
        }
    }

    fn advance_step(&mut self) {
        if self.queued_steps.is_empty() {
            self.queued_steps.extend_from_slice(&TURN_STEPS);
        }
        let next_step = self.queued_steps.pop().unwrap();
        self.current_step = next_step;
        self.current_phase = next_step.phase();
    }
}
