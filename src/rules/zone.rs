use flagset::FlagSet;

use crate::rules::player::PlayerId;

/// This enum represents the different zones + zone-specific data, that can only exist on objects that are in that zone. For example only objects on the battlefield can be tapped.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Zone {
    Battlefield(BattlefieldInfo),
    Hand { player: PlayerId },
    Library { player: PlayerId },
    Graveyard { player: PlayerId },
    Exile { player: PlayerId },
    Stack { controller: PlayerId },
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BattlefieldInfo {
    pub(crate) tapped: bool,
    pub(crate) marked_damage: u32,
    pub(crate) controller: PlayerId,
}

/// Identifies a specific zone, while ZoneKind specifies a type of zone. For Stack and Battlefield there is no difference since only one of those exists, but for the other zones it also contains the player that this zone belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ZoneId {
    Battlefield,
    Hand(PlayerId),
    Library(PlayerId),
    Graveyard(PlayerId),
    Exile(PlayerId),
    Stack,
}

impl ZoneId {
    pub(crate) fn kind(&self) -> ZoneKind {
        match self {
            ZoneId::Battlefield => ZoneKind::Battlefield,
            ZoneId::Hand(_) => ZoneKind::Hand,
            ZoneId::Library(_) => ZoneKind::Library,
            ZoneId::Graveyard(_) => ZoneKind::Graveyard,
            ZoneId::Exile(_) => ZoneKind::Exile,
            ZoneId::Stack => ZoneKind::Stack,
        }
    }
}

flagset::flags! {
    /// Specifies a type of zone, whereas ZoneId specifies a specific zone. For Stack and Battlefield there is no difference since only one of those exists.
    pub(crate) enum ZoneKind: u8 {
        Battlefield,
        Hand,
        Library,
        Graveyard,
        Exile,
        Stack,
    }
}

impl Zone {
    pub(crate) fn kind(&self) -> ZoneKind {
        match self {
            Zone::Battlefield { .. } => ZoneKind::Battlefield,
            Zone::Hand { .. } => ZoneKind::Hand,
            Zone::Library { .. } => ZoneKind::Library,
            Zone::Graveyard { .. } => ZoneKind::Graveyard,
            Zone::Exile { .. } => ZoneKind::Exile,
            Zone::Stack { .. } => ZoneKind::Stack,
        }
    }

    pub(crate) fn id(&self) -> ZoneId {
        match self {
            Zone::Battlefield { .. } => ZoneId::Battlefield,
            Zone::Hand { player } => ZoneId::Hand(*player),
            Zone::Library { player } => ZoneId::Library(*player),
            Zone::Graveyard { player } => ZoneId::Graveyard(*player),
            Zone::Exile { player } => ZoneId::Exile(*player),
            Zone::Stack { .. } => ZoneId::Stack,
        }
    }
}

pub(crate) type Zones = FlagSet<ZoneKind>;

pub const fn zones(zones: &[ZoneKind]) -> Zones {
    let mut bits: u8 = 0;
    let mut i = 0;
    while i < zones.len() {
        bits |= zones[i] as u8;
        i += 1;
    }

    // SAFETY: The discriminant of every ZoneKind is by definition a valid bit in the Zones bitset.
    unsafe { Zones::new_unchecked(bits) }
}
