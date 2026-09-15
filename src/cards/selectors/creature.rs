use crate::{
    game::Game,
    rules::{id::BattlefieldId, object::types::ObjectType, target::selector::Selector},
};

pub(crate) struct Creature {}

impl Selector for Creature {
    type Selected = BattlefieldId;

    fn choices(game: &Game) -> Vec<Self::Selected> {
        game.objects
            .battlefield
            .iter()
            .filter(|(_, (_, obj))| {
                obj.characteristics
                    .types
                    .iter()
                    .any(|typ| matches!(typ, ObjectType::Creature { .. }))
            })
            .map(|(id, _)| id)
            .collect()
    }
}
