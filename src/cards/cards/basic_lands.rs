use std::borrow::Cow;

use flagset::FlagSet;

use crate::{
    cards::PermanentEffects,
    rules::{
        cost::{Cost, ManaCost},
        object::{
            const_characteristics::ConstCharacteristics,
            types::{LandType, ObjectType},
        },
    },
};

const LAND_COST: Cost = Cost {
    mana_cost: ManaCost {
        generic: 0,
        red: 0,
        green: 0,
        blue: 0,
        white: 0,
        black: 0,
        colorless: 0,
    },
};

pub const PLAINS: ConstCharacteristics = ConstCharacteristics {
    name: "Plains",
    casting_cost: LAND_COST,
    types: &[ObjectType::Land {
        subtypes: Cow::Borrowed(&[LandType::Plains]),
        effect: PermanentEffects::Trivial,
    }],
    super_types: FlagSet::empty(),
    static_abilities: &[],
};

pub const FOREST: ConstCharacteristics = ConstCharacteristics {
    name: "Forest",
    casting_cost: LAND_COST,
    types: &[ObjectType::Land {
        subtypes: Cow::Borrowed(&[LandType::Forest]),
        effect: PermanentEffects::Trivial,
    }],
    super_types: FlagSet::empty(),
    static_abilities: &[],
};

pub const MOUNTAIN: ConstCharacteristics = ConstCharacteristics {
    name: "Mountain",
    casting_cost: LAND_COST,
    types: &[ObjectType::Land {
        subtypes: Cow::Borrowed(&[LandType::Mountain]),
        effect: PermanentEffects::Trivial,
    }],
    super_types: FlagSet::empty(),
    static_abilities: &[],
};
