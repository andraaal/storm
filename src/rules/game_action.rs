use std::range::Range;

use crate::rules::{id::ObjectId, player::PlayerId, target::Target, zone::Zone};

/// A GameAction represents an event that replacement and triggered effects can observe.
/// It may immediately change Game state or register additional GameActions.
/// A GameAction is the only way to change game objects (save for their characteristics which can only be modified by layering)
pub(crate) enum GameAction {
    DealDamage {
        source: ObjectId,
        targets: Vec<(u32, Target)>,
    },
    NoOp,
    MoveToZone {
        objects: Vec<ObjectId>,
        to: Zone,
        from: Zone,
    },
    DrawCards {
        player: PlayerId,
        amount: u32,
    },
    Tap {
        objects: Vec<ObjectId>,
    },
    Untap {
        objects: Vec<ObjectId>,
    },
    Discard {
        player: PlayerId,
        amount: Range<usize>,
    },
    RemoveDamage {
        player: PlayerId,
    },
}
