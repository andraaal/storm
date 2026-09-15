use std::borrow::Cow;

use flagset::FlagSet;

use crate::{
    cards::StaticAbilities,
    rules::{
        ability::static_ability::{DynamicAbility, DynamicAbilityGroup, DynamicContinuousAbility},
        cost::{Cost, ManaCost},
        layer::{Layer, layers},
        object::{const_characteristics::ConstCharacteristics, types::ObjectType},
        zone::{ZoneKind, zones},
    },
};

pub(crate) const BEAR_CUB: ConstCharacteristics = ConstCharacteristics {
    name: "Anthem of Champions",
    casting_cost: Cost {
        mana_cost: ManaCost {
            generic: 0,
            red: 0,
            green: 1,
            blue: 0,
            white: 1,
            black: 0,
            colorless: 0,
        },
    },
    types: &[ObjectType::Enchantment {
        subtypes: Cow::Borrowed(&[]),
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
                effect: StaticAbilities::Plus1_1,
            },
        ])),
    }],
    // triggered_abilities: &[],
};
