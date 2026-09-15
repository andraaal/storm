use std::borrow::Cow;

use flagset::FlagSet;

use crate::{
    rules::cost::{Cost, ManaCost},
    rules::object::{
        const_characteristics::ConstCharacteristics,
        types::{CreatureType, ObjectType},
    },
};

pub(crate) const BEAR_CUB: ConstCharacteristics = ConstCharacteristics {
    name: "Bear Cub",
    casting_cost: Cost {
        mana_cost: ManaCost {
            generic: 1,
            red: 0,
            green: 1,
            blue: 0,
            white: 0,
            black: 0,
            colorless: 0,
        },
    },
    types: &[ObjectType::Creature {
        power: 2,
        toughness: 2,
        subtypes: Cow::Borrowed(&[CreatureType::Bear]),
    }],
    super_types: FlagSet::empty(),
    // activated_abilities: &[],
    static_abilities: &[],
    // triggered_abilities: &[],
};
