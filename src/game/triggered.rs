use crate::{
    game::Game,
    rules::{
        effect::triggered_effect::TriggeredEffect, game_action::GameAction, id::TriggeredEffectId,
    },
};

impl Game {
    pub(crate) fn register_triggered_effect(
        &mut self,
        effect: TriggeredEffect,
    ) -> TriggeredEffectId {
        let id = self.triggered_effects.insert(effect);
        self.triggered_effects.get_mut(id).unwrap().id = id;
        id
    }

    pub(crate) fn trigger_effects(&mut self, actions: &[GameAction]) {
        for action in actions {
            for (_, effect) in &self.triggered_effects {
                if let Some(stack_obj) = (effect.trigger)(self, action) {
                    self.stack.push(stack_obj);
                }
            }
        }
    }
}
