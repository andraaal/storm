use std::hash::Hash;

use derive_more::From;
use slotmap::new_key_type;

use crate::{
    game::objects::Objects,
    rules::{object::game_object::GameObject, zone::ZoneKind},
};

new_key_type! {
    pub(crate) struct SpellStackId;
    pub(crate) struct AbilityStackId;
    pub(crate) struct BattlefieldId;
    pub(crate) struct PlayGraveyardId;
    pub(crate) struct DrawGraveyardId;
    pub(crate) struct ExileId;
    pub(crate) struct PlayHandId;
    pub(crate) struct DrawHandId;
    pub(crate) struct PlayLibraryId;
    pub(crate) struct DrawLibraryId;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From)]
pub(crate) enum AnyId {
    Stack(SpellStackId),
    Battlefield(BattlefieldId),
    PlayGraveyard(PlayGraveyardId),
    DrawGraveyard(DrawGraveyardId),
    Exile(ExileId),
    PlayHand(PlayHandId),
    DrawHand(DrawHandId),
    PlayLibrary(PlayLibraryId),
    DrawLibrary(DrawLibraryId),
}

impl AnyId {
    pub(crate) fn kind(&self) -> ZoneKind {
        match self {
            AnyId::Stack(_) => ZoneKind::Stack,
            AnyId::Battlefield(_) => ZoneKind::Battlefield,
            AnyId::PlayGraveyard(_) | AnyId::DrawGraveyard(_) => ZoneKind::Graveyard,
            AnyId::Exile(_) => ZoneKind::Exile,
            AnyId::PlayHand(_) | AnyId::DrawHand(_) => ZoneKind::Hand,
            AnyId::PlayLibrary(_) | AnyId::DrawLibrary(_) => ZoneKind::Library,
        }
    }
}

pub(crate) trait ZonedId: Into<AnyId> + Clone + Copy + PartialEq + Eq + Hash {
    fn get<'a>(&self, objects: &'a Objects) -> Option<&'a GameObject>;
    fn get_mut<'a>(&self, objects: &'a mut Objects) -> Option<&'a mut GameObject>;
}

impl ZonedId for SpellStackId {
    fn get<'a>(&self, objects: &'a Objects) -> Option<&'a GameObject> {
        objects.spell_stack.get(*self).map(|b| b.1.get_source())
    }

    fn get_mut<'a>(&self, objects: &'a mut Objects) -> Option<&'a mut GameObject> {
        objects
            .spell_stack
            .get_mut(*self)
            .map(|b| b.1.get_source_mut())
    }
}

impl ZonedId for BattlefieldId {
    fn get<'a>(&self, objects: &'a Objects) -> Option<&'a GameObject> {
        objects.battlefield.get(*self).map(|b| &b.1)
    }

    fn get_mut<'a>(&self, objects: &'a mut Objects) -> Option<&'a mut GameObject> {
        objects.battlefield.get_mut(*self).map(|b| &mut b.1)
    }
}

impl ZonedId for PlayGraveyardId {
    fn get<'a>(&self, objects: &'a Objects) -> Option<&'a GameObject> {
        objects.play_graveyard.get(*self)
    }

    fn get_mut<'a>(&self, objects: &'a mut Objects) -> Option<&'a mut GameObject> {
        objects.play_graveyard.get_mut(*self)
    }
}

impl ZonedId for DrawGraveyardId {
    fn get<'a>(&self, objects: &'a Objects) -> Option<&'a GameObject> {
        objects.draw_graveyard.get(*self)
    }

    fn get_mut<'a>(&self, objects: &'a mut Objects) -> Option<&'a mut GameObject> {
        objects.draw_graveyard.get_mut(*self)
    }
}

impl ZonedId for ExileId {
    fn get<'a>(&self, objects: &'a Objects) -> Option<&'a GameObject> {
        objects.exile.get(*self).map(|b| &b.1)
    }

    fn get_mut<'a>(&self, objects: &'a mut Objects) -> Option<&'a mut GameObject> {
        objects.exile.get_mut(*self).map(|b| &mut b.1)
    }
}

impl ZonedId for PlayHandId {
    fn get<'a>(&self, objects: &'a Objects) -> Option<&'a GameObject> {
        objects.play_hand.get(*self)
    }

    fn get_mut<'a>(&self, objects: &'a mut Objects) -> Option<&'a mut GameObject> {
        objects.play_hand.get_mut(*self)
    }
}

impl ZonedId for DrawHandId {
    fn get<'a>(&self, objects: &'a Objects) -> Option<&'a GameObject> {
        objects.draw_hand.get(*self)
    }

    fn get_mut<'a>(&self, objects: &'a mut Objects) -> Option<&'a mut GameObject> {
        objects.draw_hand.get_mut(*self)
    }
}

impl ZonedId for PlayLibraryId {
    fn get<'a>(&self, objects: &'a Objects) -> Option<&'a GameObject> {
        objects.play_library.get(*self)
    }

    fn get_mut<'a>(&self, objects: &'a mut Objects) -> Option<&'a mut GameObject> {
        objects.play_library.get_mut(*self)
    }
}

impl ZonedId for DrawLibraryId {
    fn get<'a>(&self, objects: &'a Objects) -> Option<&'a GameObject> {
        objects.draw_library.get(*self)
    }

    fn get_mut<'a>(&self, objects: &'a mut Objects) -> Option<&'a mut GameObject> {
        objects.draw_library.get_mut(*self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Timestamp(u64);

impl Timestamp {
    pub(crate) fn new() -> Self {
        Timestamp(0)
    }

    pub(crate) fn increment(&mut self) {
        self.0 += 1;
    }
}
