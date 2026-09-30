use flagset::FlagSet;

use crate::rules::{
    ability::{activated_ability::ActivatedAbility, static_ability::DynamicAbility},
    cost::Cost,
    object::types::{ObjectType, Supertype},
};

#[derive(Clone)]
pub struct ConstCharacteristics {
    pub name: &'static str,
    pub casting_cost: Cost,
    pub types: &'static [ObjectType],
    pub super_types: FlagSet<Supertype>,
    pub(crate) activated_abilities: &'static [ActivatedAbility],
    pub(crate) static_abilities: &'static [DynamicAbility],
    // pub(crate) triggered_abilities: &'static [TriggeredAbility],
}
