use std::rc::Rc;

use crate::{
    game::Game, rules::condition::Condition, rules::game_action::GameAction, rules::id::AnyId,
};

#[derive(Clone)]
pub(crate) struct ReplacementEffect {
    source: AnyId,
    check: Rc<dyn Fn(&mut Game, &GameAction)>,
    apply: Rc<dyn Fn(&mut Game, &mut GameAction)>,
    end: Condition,
}

impl std::fmt::Debug for ReplacementEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReplacementEffect")
            .field("source", &self.source)
            .field("end", &self.end)
            .finish()
    }
}
