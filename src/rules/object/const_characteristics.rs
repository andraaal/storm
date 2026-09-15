use flagset::FlagSet;

use crate::{
    rules::ability::{
        // activated_ability::ActivatedAbility,
        static_ability::DynamicAbility,
        // triggered_ability::TriggeredAbility,
    },
    rules::cost::Cost,
    rules::object::types::{ObjectType, Supertype},
};

#[derive(Clone)]
pub(crate) struct ConstCharacteristics {
    pub(crate) name: &'static str,
    pub(crate) casting_cost: Cost,
    pub(crate) types: &'static [ObjectType],
    pub(crate) super_types: FlagSet<Supertype>,
    // pub(crate) activated_abilities: &'static [ActivatedAbility],
    pub(crate) static_abilities: &'static [DynamicAbility],
    // pub(crate) triggered_abilities: &'static [TriggeredAbility],
}
