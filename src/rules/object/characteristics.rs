use std::borrow::Cow;

use flagset::FlagSet;

use crate::rules::{
    ability::{
        static_ability::{DynamicAbilityGroup, DynamicEffect, IntrinsicAbility},
    },
    cost::Cost,
    id::Timestamp,
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
    // pub(crate) activated_abilities: Cow<'static, [ActivatedAbility]>,
    pub(crate) static_abilities: Vec<DynamicEffect>,
    // pub(crate) triggered_abilities: Cow<'static, [TriggeredAbility]>,
}

impl Characteristics {
    pub(crate) fn new(card: &'static ConstCharacteristics, time: Timestamp) -> Self {
        let statics = card
            .static_abilities
            .iter()
            .map(|a| DynamicEffect {
                timestamp: time,
                ability: a,
            })
            .collect();

        Characteristics {
            name: card.name.to_owned(),
            casting_cost: card.casting_cost.clone(),
            types: Cow::Borrowed(card.types),
            super_types: card.super_types,
            // activated_abilities: Cow::Borrowed(card.activated_abilities),
            static_abilities: statics,
            // triggered_abilities: Cow::Borrowed(card.triggered_abilities),
        }
    }

    pub(crate) fn has_intrinsic_effect(&self, effect: IntrinsicAbility) -> bool {
        self.static_abilities.iter().any(
            |a| matches!(&a.ability.ability_group, DynamicAbilityGroup::Intrinsic(eff) if *eff == effect),
        )
    }
}
