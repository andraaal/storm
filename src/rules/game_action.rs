use std::range::Range;

use crate::rules::{
    id::{AnyId, BattlefieldId},
    player::PlayerId,
    target::damage::DamageableTarget,
    zone::{BattlefieldInfo, ExileInfo},
};

/// A GameAction represents an event that replacement and triggered effects can observe.
/// It may immediately change Game state or register additional GameActions.
/// A GameAction is the only way to change game objects (save for their characteristics which can only be modified by layering)
pub(crate) enum GameAction {
    DealDamage {
        source: AnyId,
        targets: Vec<(u32, DamageableTarget)>,
    },
    NoOp,
    MoveToBattlefield {
        objects: Vec<(AnyId, BattlefieldInfo)>,
    },
    MoveToExile {
        objects: Vec<(AnyId, ExileInfo)>,
    },
    MoveToLibrary {
        objects: Vec<AnyId>,
    },
    MoveToHand {
        objects: Vec<AnyId>,
    },
    MoveToGraveyard {
        objects: Vec<AnyId>,
    },
    DrawCards {
        player: PlayerId,
        amount: u32,
    },
    Tap {
        objects: Vec<BattlefieldId>,
    },
    Untap {
        objects: Vec<BattlefieldId>,
    },
    Discard {
        player: PlayerId,
        amount: Range<usize>,
    },
    RemoveDamage {
        player: PlayerId,
    },
}
