use std::iter::once;

use crate::{
    game::Game,
    rules::{
        object::types::ObjectType,
        player::PlayerId,
        target::{damage::DamageableTarget, selector::Selector},
    },
};

pub(crate) struct Damagable {}

impl Selector for Damagable {
    type Selected = DamageableTarget;

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
            .map(|(id, _)| id.into())
            .chain(once(PlayerId::PlayPlayer.into()))
            .chain(once(PlayerId::DrawPlayer.into()))
            .collect()
    }
}
