use std::borrow::Cow;

use flagset::FlagSet;

use crate::{
    cards::Spells,
    rules::{
        cost::{Cost, ManaCost},
        object::{const_characteristics::ConstCharacteristics, types::ObjectType},
    },
};

pub(crate) const LIGHTNING_BOLT: ConstCharacteristics = ConstCharacteristics {
    name: "Lightning Bolt",
    casting_cost: Cost {
        mana_cost: ManaCost {
            generic: 0,
            red: 1,
            green: 0,
            blue: 0,
            white: 0,
            black: 0,
            colorless: 0,
        },
    },
    types: &[ObjectType::Instant {
        subtypes: Cow::Borrowed(&[]),
        effect: Spells::Damage3,
    }],
    super_types: FlagSet::empty(),
    // activated_abilities: &[],
    static_abilities: &[],
    // triggered_abilities: &[],
};
