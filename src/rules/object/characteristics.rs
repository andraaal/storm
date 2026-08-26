use std::borrow::Cow;

use flagset::FlagSet;

use crate::rules::{
    ability::{
        activated_ability::ActivatedAbility,
        static_ability::{DynamicAbilityGroup, IntrinsicAbility, StaticEffect},
        triggered_ability::TriggeredAbility,
    },
    cost::Cost,
    id::{ObjectId, Timestamp},
    object::{
        const_characteristics::ConstCharacteristics,
        types::{ObjectType, Supertype},
    },
};

#[derive(Clone)]
pub(crate) struct Characteristics {
    pub(crate) name: String,
    pub(crate) casting_cost: Cost,
    pub(crate) types: Cow<'static, [ObjectType]>,
    pub(crate) super_types: FlagSet<Supertype>,
    pub(crate) activated_abilities: Cow<'static, [ActivatedAbility]>,
    pub(crate) static_abilities: Vec<StaticEffect>,
    pub(crate) triggered_abilities: Cow<'static, [TriggeredAbility]>,
}

impl Characteristics {
    pub(crate) fn new(card: &'static ConstCharacteristics, time: Timestamp, id: ObjectId) -> Self {
        let statics = card
            .static_abilities
            .iter()
            .map(|a| StaticEffect {
                timestamp: time,
                source: id,
                ability: a,
            })
            .collect();

        Characteristics {
            name: card.name.to_owned(),
            casting_cost: card.casting_cost.clone(),
            types: Cow::Borrowed(card.types),
            super_types: card.super_types,
            activated_abilities: Cow::Borrowed(card.activated_abilities),
            static_abilities: statics,
            triggered_abilities: Cow::Borrowed(card.triggered_abilities),
        }
    }

    pub(crate) fn has_intrinsic_effect(&self, effect: IntrinsicAbility) -> bool {
        self.static_abilities.iter().any(
            |a| matches!(&a.ability.ability_group, DynamicAbilityGroup::Intrinsic(eff) if *eff == effect),
        )
    }
}
