use slotmap::{
    Key, SlotMap,
    basic::{Iter, IterMut, Keys, Values, ValuesMut},
};

use crate::rules::{
    ability::effect::{Ability, Spell},
    id::*,
    object::game_object::GameObject,
    player::PlayerId,
    zone::{BattlefieldInfo, ExileInfo, StackInfo},
};

pub(crate) struct Objects {
    pub(crate) battlefield: MyMap<BattlefieldId, (BattlefieldInfo, GameObject)>,
    pub(crate) spell_stack: MyMap<SpellStackId, (StackInfo, Box<dyn Spell>)>,
    pub(crate) ability_stack: MyMap<AbilityStackId, (StackInfo, Box<dyn Ability>)>,
    pub(crate) play_hand: MyMap<PlayHandId, GameObject>,
    pub(crate) draw_hand: MyMap<DrawHandId, GameObject>,
    pub(crate) exile: MyMap<ExileId, (ExileInfo, GameObject)>,
    pub(crate) play_library: MyMap<PlayLibraryId, GameObject>,
    pub(crate) draw_library: MyMap<DrawLibraryId, GameObject>,
    pub(crate) play_graveyard: MyMap<PlayGraveyardId, GameObject>,
    pub(crate) draw_graveyard: MyMap<DrawGraveyardId, GameObject>,
    pub(crate) resolving_spell: Option<AnyId>,
}

impl Objects {
    pub(crate) fn new(play_deck: Vec<GameObject>, draw_deck: Vec<GameObject>) -> Self {
        let mut play = MyMap::new();
        let mut draw = MyMap::new();

        for card in play_deck {
            play.insert(card);
        }

        for card in draw_deck {
            draw.insert(card);
        }

        Objects {
            battlefield: MyMap::new(),
            spell_stack: MyMap::new(),
            ability_stack: MyMap::new(),
            exile: MyMap::new(),
            play_hand: MyMap::new(),
            draw_hand: MyMap::new(),
            play_library: play,
            draw_library: draw,
            play_graveyard: MyMap::new(),
            draw_graveyard: MyMap::new(),
            resolving_spell: None,
        }
    }

    pub(crate) fn get_any(&self, id: AnyId) -> Option<&GameObject> {
        match id {
            AnyId::Battlefield(id) => self.battlefield.get(id).map(|f| &f.1),
            AnyId::Stack(id) => self.spell_stack.get(id).map(|f| f.1.get_source()),
            AnyId::Exile(id) => self.exile.get(id).map(|f| &f.1),
            AnyId::PlayGraveyard(id) => self.play_graveyard.get(id),
            AnyId::DrawGraveyard(id) => self.draw_graveyard.get(id),
            AnyId::PlayHand(id) => self.play_hand.get(id),
            AnyId::DrawHand(id) => self.draw_hand.get(id),
            AnyId::PlayLibrary(id) => self.play_library.get(id),
            AnyId::DrawLibrary(id) => self.draw_library.get(id),
        }
    }

    pub(crate) fn get_any_mut(&mut self, id: AnyId) -> Option<&mut GameObject> {
        match id {
            AnyId::Battlefield(id) => self.battlefield.get_mut(id).map(|f| &mut f.1),
            AnyId::Stack(id) => self.spell_stack.get_mut(id).map(|f| f.1.get_source_mut()),
            AnyId::Exile(id) => self.exile.get_mut(id).map(|f| &mut f.1),
            AnyId::PlayGraveyard(id) => self.play_graveyard.get_mut(id),
            AnyId::DrawGraveyard(id) => self.draw_graveyard.get_mut(id),
            AnyId::PlayHand(id) => self.play_hand.get_mut(id),
            AnyId::DrawHand(id) => self.draw_hand.get_mut(id),
            AnyId::PlayLibrary(id) => self.play_library.get_mut(id),
            AnyId::DrawLibrary(id) => self.draw_library.get_mut(id),
        }
    }

    pub(crate) fn values_mut(&mut self) -> impl Iterator<Item = &mut GameObject> {
        self.battlefield
            .values_mut()
            .map(|v| &mut v.1)
            .chain(self.spell_stack.values_mut().map(|v| v.1.get_source_mut()))
            .chain(self.draw_graveyard.values_mut())
            .chain(self.play_graveyard.values_mut())
            .chain(self.draw_hand.values_mut())
            .chain(self.play_hand.values_mut())
            .chain(self.draw_library.values_mut())
            .chain(self.play_library.values_mut())
            .chain(self.exile.values_mut().map(|v| &mut v.1))
    }

    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = (AnyId, &mut GameObject)> {
        self.battlefield
            .iter_mut()
            .map(|(id, v)| (id.into(), &mut v.1))
            .chain(
                self.spell_stack
                    .iter_mut()
                    .map(|(id, v)| (id.into(), v.1.get_source_mut())),
            )
            .chain(self.draw_graveyard.iter_mut().map(|(id, v)| (id.into(), v)))
            .chain(self.play_graveyard.iter_mut().map(|(id, v)| (id.into(), v)))
            .chain(self.draw_hand.iter_mut().map(|(id, v)| (id.into(), v)))
            .chain(self.play_hand.iter_mut().map(|(id, v)| (id.into(), v)))
            .chain(self.draw_library.iter_mut().map(|(id, v)| (id.into(), v)))
            .chain(self.play_library.iter_mut().map(|(id, v)| (id.into(), v)))
            .chain(self.exile.iter_mut().map(|(id, v)| (id.into(), &mut v.1)))
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = &GameObject> {
        self.battlefield
            .values()
            .map(|v| &v.1)
            .chain(self.spell_stack.values().map(|v| v.1.get_source()))
            .chain(self.draw_graveyard.values())
            .chain(self.play_graveyard.values())
            .chain(self.draw_hand.values())
            .chain(self.play_hand.values())
            .chain(self.draw_library.values())
            .chain(self.play_library.values())
            .chain(self.exile.values().map(|v| &v.1))
    }

    pub(crate) fn get<I: ZonedId>(&self, id: I) -> Option<&GameObject> {
        id.get(&self)
    }

    pub(crate) fn get_mut<I: ZonedId>(&mut self, id: I) -> Option<&mut GameObject> {
        id.get_mut(self)
    }

    pub(crate) fn get_controller_and_owner(
        &self,
        id: AnyId,
    ) -> (Option<PlayerId>, Option<PlayerId>) {
        match id {
            AnyId::Battlefield(id) => {
                let info = self.battlefield.get(id);
                (info.map(|i| i.0.controller), info.map(|i| i.0.owner))
            }
            AnyId::Stack(id) => {
                let info = self.spell_stack.get(id);
                (info.map(|i| i.0.controller), info.map(|i| i.0.owner))
            }
            AnyId::Exile(id) => (None, self.exile.get(id).map(|v| v.0.owner)),
            AnyId::DrawGraveyard(_) | AnyId::DrawHand(_) | AnyId::DrawLibrary(_) => {
                (None, Some(PlayerId::DrawPlayer))
            }
            AnyId::PlayGraveyard(_) | AnyId::PlayHand(_) | AnyId::PlayLibrary(_) => {
                (None, Some(PlayerId::PlayPlayer))
            }
        }
    }

    pub(crate) fn move_to_battlefield(
        &mut self,
        id: AnyId,
        dest: BattlefieldInfo,
        stmp: Timestamp,
    ) -> BattlefieldId {
        let mut obj = id.remove(self);
        obj.timestamp = stmp;
        self.battlefield.insert((dest, obj))
    }

    pub(crate) fn move_to_hand(&mut self, id: AnyId, stmp: Timestamp) -> Option<AnyId> {
        let (_, owner) = self.get_controller_and_owner(id);
        let mut obj = id.remove(self);
        obj.timestamp = stmp;

        match owner {
            Some(PlayerId::DrawPlayer) => Some(self.draw_hand.insert(obj).into()),
            Some(PlayerId::PlayPlayer) => Some(self.play_hand.insert(obj).into()),
            None => {
                println!("Tried to move non-existent object");
                None
            }
        }
    }

    pub(crate) fn move_to_exile(&mut self, id: AnyId, dest: ExileInfo, stmp: Timestamp) -> ExileId {
        let mut obj = id.remove(self);
        obj.timestamp = stmp;
        self.exile.insert((dest, obj))
    }

    pub(crate) fn move_to_library(&mut self, id: AnyId, stmp: Timestamp) -> Option<AnyId> {
        let (_, owner) = self.get_controller_and_owner(id);
        let mut obj = id.remove(self);
        obj.timestamp = stmp;

        match owner {
            Some(PlayerId::DrawPlayer) => Some(self.draw_library.insert(obj).into()),
            Some(PlayerId::PlayPlayer) => Some(self.play_library.insert(obj).into()),
            None => {
                println!("Tried to move non-existent object");
                None
            }
        }
    }

    pub(crate) fn move_to_graveyard(&mut self, id: AnyId, stmp: Timestamp) -> Option<AnyId> {
        let (_, owner) = self.get_controller_and_owner(id);
        let mut obj = id.remove(self);
        obj.timestamp = stmp;

        match owner {
            Some(PlayerId::DrawPlayer) => Some(self.draw_graveyard.insert(obj).into()),
            Some(PlayerId::PlayPlayer) => Some(self.play_graveyard.insert(obj).into()),
            None => {
                println!("Tried to move non-existent object");
                None
            }
        }
    }
}

pub(crate) struct MyMap<K: Key, V> {
    map: SlotMap<K, V>,
    order: Vec<K>,
}

impl<K: Key, V> MyMap<K, V> {
    fn new() -> Self {
        MyMap {
            map: SlotMap::with_key(),
            order: Vec::new(),
        }
    }

    pub(crate) default fn insert(&mut self, value: V) -> K {
        let key = self.map.insert(value);
        self.order.push(key);
        key
    }

    default fn remove(&mut self, key: K) -> Option<V> {
        if let Some(pos) = self.order.iter().position(|&k| k == key) {
            self.order.remove(pos);
        }
        self.map.remove(key)
    }

    pub(crate) fn get(&self, key: K) -> Option<&V> {
        self.map.get(key)
    }

    pub(crate) fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.map.get_mut(key)
    }

    pub(crate) fn values(&self) -> Values<'_, K, V> {
        self.map.values()
    }

    pub(crate) fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        self.map.values_mut()
    }

    pub(crate) fn keys(&self) -> Keys<'_, K, V> {
        self.map.keys()
    }

    pub(crate) fn iter(&self) -> Iter<'_, K, V> {
        self.map.iter()
    }

    pub(crate) fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        self.map.iter_mut()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.map.len()
    }
}

impl AnyId {
    pub(crate) fn remove(self, objects: &mut Objects) -> GameObject {
        match self {
            AnyId::Battlefield(id) => {
                objects
                    .battlefield
                    .remove(id)
                    .expect("Battlefield object not found")
                    .1
            }
            AnyId::Stack(id) => objects
                .spell_stack
                .remove(id)
                .expect("Spell Stack Object not found")
                .1
                .take_source(),
            AnyId::PlayGraveyard(id) => objects
                .play_graveyard
                .remove(id)
                .expect("Graveyard play object not found"),

            AnyId::DrawGraveyard(id) => objects
                .draw_graveyard
                .remove(id)
                .expect("Graveyard draw object not found"),
            AnyId::Exile(id) => objects.exile.remove(id).expect("Exile object not found").1,

            AnyId::PlayHand(id) => objects
                .play_hand
                .remove(id)
                .expect("Hand play object not found"),
            AnyId::DrawHand(id) => objects
                .draw_hand
                .remove(id)
                .expect("Hand draw object not found"),
            AnyId::PlayLibrary(id) => objects
                .play_library
                .remove(id)
                .expect("Library play object not found"),
            AnyId::DrawLibrary(id) => objects
                .draw_library
                .remove(id)
                .expect("Library draw object not found"),
        }
    }
}
