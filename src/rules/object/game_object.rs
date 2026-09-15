use crate::rules::{
    id::Timestamp,
    object::{
        characteristics::Characteristics, const_characteristics::ConstCharacteristics,
        types::ObjectType,
    },
};

#[derive(Clone)]
pub(crate) struct GameObject {
    pub(crate) characteristics: Characteristics,
    pub(crate) copy_characteristics: Characteristics,
    pub(crate) timestamp: Timestamp,

    card: &'static ConstCharacteristics,
}

impl GameObject {
    pub(crate) fn new(card: &'static ConstCharacteristics, stamp: Timestamp) -> Self {
        GameObject {
            characteristics: Characteristics::new(card, stamp),
            copy_characteristics: Characteristics::new(card, stamp),
            card: card,
            timestamp: stamp,
        }
    }

    pub(crate) fn is_permanent_spell(&self) -> bool {
        let result = self.characteristics.types.iter().any(|t| {
            matches!(
                t,
                ObjectType::Creature { .. }
                    | ObjectType::Enchantment { .. }
                    | ObjectType::Artifact { .. }
                    | ObjectType::Planeswalker { .. }
                    | ObjectType::Land { .. }
            )
        });
        result
    }

    pub(crate) fn reset_characteristics(&mut self) {
        self.characteristics = Characteristics::new(self.card, self.timestamp);
        // Is overwritten by layering anyways
        // self.copy_characteristics = Characteristics::new(self.card, self.timestamp, self.id);
    }
}
