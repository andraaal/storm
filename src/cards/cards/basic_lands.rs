use std::borrow::Cow;

use flagset::FlagSet;

use crate::{
    cards::PermanentEffects,
    rules::{
        ability::activated_ability::ActivatedAbility,
        cost::{Cost, ManaAmount},
        object::{
            const_characteristics::ConstCharacteristics,
            types::{LandType, ObjectType},
        },
    },
};

const LAND_COST: Cost = Cost {
    mana_cost: ManaAmount {
        generic: 0,
        red: 0,
        green: 0,
        blue: 0,
        white: 0,
        black: 0,
        colorless: 0,
    },
    tapping: false,
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
    activated_abilities: &[ActivatedAbility {
        cost: Cost {
            mana_cost: ManaAmount::def(),
            tapping: true,
        },
        effect: |ctx| {
            let priority = ctx.game.priority;
            ctx.game.players[priority].mana.white += 1;
        },
    }],
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
    activated_abilities: &[ActivatedAbility {
        cost: Cost {
            mana_cost: ManaAmount::def(),
            tapping: true,
        },
        effect: |ctx| {
            let priority = ctx.game.priority;
            ctx.game.players[priority].mana.green += 1;
        },
    }],
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
    activated_abilities: &[ActivatedAbility {
        cost: Cost {
            mana_cost: ManaAmount::def(),
            tapping: true,
        },
        effect: |ctx| {
            let priority = ctx.game.priority;
            ctx.game.players[priority].mana.red += 1;
        },
    }],
};
