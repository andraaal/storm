use std::ops::AddAssign;

use crate::{
    context::Context,
    rules::{game_action::GameAction, object::types::ObjectType},
};

#[derive(PartialEq)]
pub(crate) enum StateBasedActionResult {
    NoneExecuted,
    SomeExecuted,
}

impl AddAssign for StateBasedActionResult {
    fn add_assign(&mut self, rhs: Self) {
        if rhs == StateBasedActionResult::SomeExecuted {
            *self = StateBasedActionResult::SomeExecuted;
        }
    }
}

impl Context {
    pub(crate) fn exec_state_based(&mut self) -> StateBasedActionResult {
        let mut res = StateBasedActionResult::NoneExecuted;
        res += player0life(self);
        res += creature_dies(self);

        res
    }
}

fn player0life(ctx: &mut Context) -> StateBasedActionResult {
    if ctx.game.players.play_player.life <= 0 {
        panic!("Play Player lost the game, due to having <= 0 life");
    }
    if ctx.game.players.draw_player.life <= 0 {
        panic!("Draw Player lost the game, due to having <= life");
    }
    StateBasedActionResult::NoneExecuted
}

fn creature_dies(ctx: &mut Context) -> StateBasedActionResult {
    let mut to_graveyard = vec![];
    for (id, (info, obj)) in ctx.game.objects.battlefield.iter() {
        let Some(toughness) = obj.characteristics.types.iter().find_map(|typ| match typ {
            ObjectType::Creature { toughness, .. } => Some(*toughness),
            _ => None,
        }) else {
            continue;
        };

        if toughness <= info.marked_damage as i64
            || (info.damaged_by_deathtouch && info.marked_damage > 0)
        {
            to_graveyard.push(id.into());
        }
    }
    if !to_graveyard.is_empty() {
        ctx.execute(vec![GameAction::MoveToGraveyard {
            objects: to_graveyard,
        }]);
        StateBasedActionResult::SomeExecuted
    } else {
        StateBasedActionResult::NoneExecuted
    }
}
