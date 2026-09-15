// Legacy implementation; for reference only

// use std::borrow::Cow;

// use crate::game::Game;
// use crate::rules::cost::Cost;
// use crate::rules::effect::triggered_effect::Uses;
// use crate::rules::id::AnyId;
// use crate::rules::object::stack_object::StackObject;
// use crate::rules::zone::Zones;

// #[derive(Debug, Clone)]
// pub(crate) struct ActivatedAbility {
//     zone: Zones,
//     pub(crate) cost: Cost,
//     pub(crate) remaining_uses: Uses,
//     pub(crate) target_selectors: Cow<'static, [fn(&mut Game, AnyId) -> Vec<AnyId>]>,
//     // Takes the Game and its source
//     pub(crate) get_effect: fn(&mut Game, AnyId) -> StackObject,
// }
