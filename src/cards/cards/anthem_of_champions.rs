use std::borrow::Cow;

use flagset::FlagSet;

use crate::{
    cards::{Effects, PermanentEffects},
    rules::{
        ability::static_ability::{DynamicAbility, DynamicAbilityGroup, DynamicContinuousAbility},
        cost::{Cost, ManaAmount},
        layer::{Layer, layers},
        object::{const_characteristics::ConstCharacteristics, types::ObjectType},
        zone::{ZoneKind, zones},
    },
};

pub const ANTHEM_OF_CHAMPIONS: ConstCharacteristics = ConstCharacteristics {
    name: "Anthem of Champions",
    casting_cost: Cost {
        mana_cost: ManaAmount {
            generic: 0,
            red: 0,
            green: 1,
            blue: 0,
            white: 1,
            black: 0,
            colorless: 0,
        },
        tapping: false,
    },
    types: &[ObjectType::Enchantment {
        subtypes: Cow::Borrowed(&[]),
        effect: PermanentEffects::Trivial,
    }],
    super_types: FlagSet::empty(),
    // activated_abilities: &[],
    static_abilities: &[DynamicAbility {
        active_zones: zones(&[ZoneKind::Battlefield]),
        layers: layers(&[Layer::ChangePT]),
        ability_group: DynamicAbilityGroup::DynamicContinuous(Cow::Borrowed(&[
            DynamicContinuousAbility {
                layer: Layer::ChangePT,
                is_cd: false,
                effect: Effects::Plus1_1,
            },
        ])),
    }],
    activated_abilities: &[],
};
