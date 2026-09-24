use std::borrow::Cow;

use flagset::FlagSet;

use crate::cards::Spells;
use crate::rules::cost::{Cost, ManaCost};
use crate::rules::object::{const_characteristics::ConstCharacteristics, types::ObjectType};

pub(crate) const GIANT_GROWTH: ConstCharacteristics = ConstCharacteristics {
    name: "Giant Growth",
    casting_cost: Cost {
        mana_cost: ManaCost {
            generic: 0,
            red: 0,
            green: 1,
            blue: 0,
            white: 0,
            black: 0,
            colorless: 0,
        },
    },
    types: &[ObjectType::Instant {
        subtypes: Cow::Borrowed(&[]),
        effect: Spells::Plus3_3,
    }],
    super_types: FlagSet::empty(),
    // activated_abilities: &[],
    static_abilities: &[],
    // triggered_abilities: &[],
};
