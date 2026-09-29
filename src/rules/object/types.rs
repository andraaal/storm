use flagset::flags;
use std::borrow::Cow;

use crate::cards::{PermanentEffects, Spells};

#[derive(Clone)]
pub enum ObjectType {
    Creature {
        power: i64,
        toughness: i64,
        subtypes: Cow<'static, [CreatureType]>,
        effect: PermanentEffects,
    },
    Sorcery {
        subtypes: Cow<'static, [SpellType]>,
        effect: Spells,
    },
    Instant {
        subtypes: Cow<'static, [SpellType]>,
        effect: Spells,
    },
    Enchantment {
        subtypes: Cow<'static, [EnchantmentType]>,
        effect: PermanentEffects,
    },
    Artifact {
        subtypes: Cow<'static, [ArtifactType]>,
        effect: PermanentEffects,
    },
    Planeswalker {
        subtypes: Cow<'static, [PlaneswalkerType]>,
        effect: PermanentEffects,
    },
    Land {
        subtypes: Cow<'static, [LandType]>,
        effect: PermanentEffects,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreatureType {
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
pub enum PlaneswalkerType {
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
pub enum ArtifactType {
    Equipment,
    Vehicle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnchantmentType {
    Aura,
    Rune,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LandType {
    Cave,
    Forest,
    Island,
    Mountain,
    Plains,
    Swamp,
    Desert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellType {
    Arcane,
    Trap,
}

flags! {
    pub enum Supertype: u8 {
        Basic,
        Legendary,
        Snow,
    }
}

impl LandType {
    pub fn is_basic(&self) -> bool {
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
