use flagset::FlagSet;

use crate::rules::player::PlayerId;

#[derive(Debug, Clone, Copy)]
pub(crate) struct BattlefieldInfo {
    pub(crate) tapped: bool,
    pub(crate) marked_damage: u32,
    pub(crate) damaged_by_deathtouch: bool,
    pub(crate) controller: PlayerId,
    pub(crate) owner: PlayerId,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ExileInfo {
    pub(crate) owner: PlayerId,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct StackInfo {
    pub(crate) controller: PlayerId,
    pub(crate) owner: PlayerId,
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
