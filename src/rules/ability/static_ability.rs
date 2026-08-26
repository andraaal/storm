use std::borrow::Cow;
use std::rc::Rc;

use crate::context::Context;
use crate::rules::condition::Condition;
use crate::rules::game_action::GameAction;
use crate::rules::id::{ObjectId, Timestamp};
use crate::rules::layer::{Layer, Layers};
use crate::rules::target::Target;
use crate::rules::zone::Zones;

#[derive(Clone)]
pub(crate) struct StaticAbility {
    pub(crate) active_zones: Zones,
    pub(crate) layers: Layers,
    pub(crate) ability_group: DynamicAbilityGroup,
}

#[derive(Clone)]
pub(crate) struct StaticEffect {
    pub(crate) source: ObjectId,
    pub(crate) timestamp: Timestamp,
    pub(crate) ability: &'static StaticAbility,
}

#[derive(Clone)]
pub(crate) enum DynamicAbilityGroup {
    // EffectGroups need to be of only a single type
    Intrinsic(IntrinsicAbility),
    DynamicContinuous(Cow<'static, [DynamicContinuousAbility]>),
    DynamicReplacement(Cow<'static, [DynamicReplacementAbility]>),
}

#[derive(Clone)]
pub(crate) enum StaticAbilityGroup {
    FixedContinuous(Cow<'static, [FixedContinuousAbility]>),
    FixedReplacement(Cow<'static, [FixedReplacementAbility]>),
}

#[derive(Clone)]
pub(crate) struct DynamicContinuousAbility {
    pub(crate) layer: Layer,
    pub(crate) is_cd: bool,
    pub(crate) effect: fn(ctx: &mut Context, source: ObjectId),
}

#[derive(Clone)]
pub(crate) struct FixedContinuousAbility {
    pub(crate) layer: Layer,
    pub(crate) targets: Vec<Target>,
    pub(crate) end: Condition,
    pub(crate) effect: Rc<dyn Fn(&mut Context)>,
}

#[derive(Clone)]
pub(crate) struct DynamicReplacementAbility {
    pub(crate) layer: Layer,
    pub(crate) check: fn(ctx: &mut Context, action: &GameAction) -> bool,
    pub(crate) apply: fn(ctx: &mut Context, action: GameAction) -> Vec<GameAction>,
}

#[derive(Clone)]
pub(crate) struct FixedReplacementAbility {
    pub(crate) layer: Layer,
    pub(crate) targets: Vec<Target>,
    pub(crate) end: Condition,
    // Only the fixed variant created by spells has closures
    pub(crate) check: Rc<dyn Fn(&mut Context, &GameAction) -> bool>,
    pub(crate) apply: Rc<dyn Fn(&mut Context, &GameAction) -> Vec<GameAction>>,
}

#[derive(Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub(crate) enum IntrinsicAbility {
    Vigilance,
    Deathtouch,
    Wither,
    Infect,
    Lifelink,
}
