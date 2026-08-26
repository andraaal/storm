use std::borrow::Cow;

use flagset::flags;

use crate::{
    context::Context,
    rules::{id::StackId, target::Target},
};

#[derive(Debug, Clone)]
pub(crate) enum ObjectType {
    Creature {
        power: i64,
        toughness: i64,
        subtypes: Cow<'static, [CreatureType]>,
    },
    Sorcery {
        subtypes: Cow<'static, [SpellType]>,
        effects: Cow<'static, [fn(&mut Context, StackId)]>,
        target_selectors: Cow<'static, [fn(&Context, Target) -> bool]>,
    },
    Instant {
        subtypes: Cow<'static, [SpellType]>,
        effects: Cow<'static, [fn(&mut Context, StackId)]>,
        target_selectors: Cow<'static, [fn(&Context, Target) -> bool]>,
    },
    Enchantment {
        subtypes: Cow<'static, [EnchantmentType]>,
    },
    Artifact {
        subtypes: Cow<'static, [ArtifactType]>,
    },
    Planeswalker {
        subtypes: Cow<'static, [PlaneswalkerType]>,
    },
    Land {
        subtypes: Cow<'static, [LandType]>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CreatureType {
    Elf,
    Goblin,
    Zombie,
    Vampire,
    Angel,
    Demon,
    Dragon,
    Wizard,
    Soldier,
    Knight,
    Cleric,
    Rogue,
    Shaman,
    Druid,
    Elemental,
    Bear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PlaneswalkerType {
    Ajani,
    Chandra,
    Garruk,
    Jace,
    Liliana,
    Nissa,
    Sorin,
    Teferi,
    Vraska,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ArtifactType {
    Equipment,
    Vehicle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EnchantmentType {
    Aura,
    Rune,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LandType {
    Cave,
    Forest,
    Island,
    Mountain,
    Plains,
    Swamp,
    Desert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SpellType {
    Arcane,
    Trap,
}

flags! {
    pub(crate)enum Supertype: u8 {
        Basic,
        Legendary,
        Snow,
    }
}

impl LandType {
    pub(crate) fn is_basic(&self) -> bool {
        match self {
            LandType::Cave => false,
            LandType::Forest => true,
            LandType::Island => true,
            LandType::Mountain => true,
            LandType::Plains => true,
            LandType::Swamp => true,
            LandType::Desert => false,
        }
    }
}
