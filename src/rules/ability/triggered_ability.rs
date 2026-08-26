use crate::game::Game;
use crate::rules::effect::triggered_effect::TriggeredEffect;
use crate::rules::id::ObjectId;
use crate::rules::zone::Zones;

#[derive(Debug, Clone)]
pub(crate) struct TriggeredAbility {
    zone: Zones,
    get_effect: fn(&mut Game, &ObjectId) -> TriggeredEffect,
}
