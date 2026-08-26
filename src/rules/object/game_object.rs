use crate::rules::{
    id::{ObjectId, Timestamp},
    object::{
        characteristics::Characteristics, const_characteristics::ConstCharacteristics,
        types::ObjectType,
    },
    player::PlayerId,
    zone::{BattlefieldInfo, Zone},
};

pub(crate) struct GameObject {
    pub(crate) id: ObjectId,
    pub(crate) characteristics: Characteristics,
    pub(crate) copy_characteristics: Characteristics,
    pub(crate) zone: Zone,
    pub(crate) marked_damage: u32,
    pub(crate) damaged_by_deathtouch: bool,
    pub(crate) owner: PlayerId,
    pub(crate) timestamp: Timestamp,

    card: &'static ConstCharacteristics,
}

impl GameObject {
    pub(crate) fn new(
        card: &'static ConstCharacteristics,
        owner: PlayerId,
        id: ObjectId,
        stamp: Timestamp,
    ) -> Self {
        GameObject {
            id: id,
            characteristics: Characteristics::new(card, stamp, id),
            copy_characteristics: Characteristics::new(card, stamp, id),
            card: card,
            zone: Zone::Library { player: owner },
            marked_damage: 0,
            damaged_by_deathtouch: false,
            owner: owner,
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

    pub(crate) fn get_controller_or_owner(&self) -> PlayerId {
        match self.zone {
            Zone::Battlefield(BattlefieldInfo { controller, .. })
            | Zone::Stack { controller, .. } => controller,
            _ => self.owner,
        }
    }

    pub(crate) fn reset_characteristics(&mut self) {
        self.characteristics = Characteristics::new(self.card, self.timestamp, self.id);
        // Is overwritten by layering anyways
        // self.copy_characteristics = Characteristics::new(self.card, self.timestamp, self.id);
    }
}
