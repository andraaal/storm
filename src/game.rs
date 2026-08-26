use std::borrow::Borrow;
use std::iter::{Copied, FilterMap, Rev};
use std::ops::{Index, IndexMut};
use std::slice::Iter;

use slotmap::SlotMap;

use crate::rules::effect::continuous_effect::ContinuousEffect;
use crate::rules::effect::{
    replacement_effect::ReplacementEffect, triggered_effect::TriggeredEffect,
};
use crate::rules::id::{ObjectId, Timestamp, TriggeredEffectId};
use crate::rules::object::const_characteristics::ConstCharacteristics;
use crate::rules::object::game_object::GameObject;
use crate::rules::object::stack_object::{StackObject, StackObjectKind};
use crate::rules::player::Player;
use crate::rules::player::PlayerId;
use crate::rules::turn::{Phase, Step};
use crate::rules::zone::ZoneId;

pub(crate) mod object_iter;
mod stack;
mod triggered;
mod turn;

pub(crate) struct Game {
    pub(crate) players: Players,
    pub(crate) objects: SlotMap<ObjectId, GameObject>,
    pub(crate) stack: Vec<StackObject>,
    pub(crate) battlefield: Vec<ObjectId>,

    pub(crate) continuous_effects: Vec<ContinuousEffect>,
    pub(crate) replacement_effects: Vec<ReplacementEffect>,
    pub(crate) triggered_effects: SlotMap<TriggeredEffectId, TriggeredEffect>,

    pub(crate) current_timestamp: Timestamp,
    pub(crate) pending_triggers: Vec<StackObject>,
    pub(crate) current_step: Step,
    pub(crate) current_phase: Phase,
    pub(crate) queued_steps: Vec<Step>,
    pub(crate) priority: PlayerId,
    pub(crate) last_non_passed_priority: PlayerId,
    pub(crate) active_player: PlayerId,
}

impl Game {
    pub(crate) fn register_object(
        &mut self,
        characteristics: &'static ConstCharacteristics,
        owner: PlayerId,
    ) -> ObjectId {
        let timestamp = self.generate_timestamp();
        let id = self
            .objects
            .insert_with_key(|id| GameObject::new(characteristics, owner, id, timestamp));
        id
    }

    pub(crate) fn generate_timestamp(&mut self) -> Timestamp {
        let time = self.current_timestamp;
        self.current_timestamp.increment();

        time
    }

    pub(crate) fn register_continuous_effect(&mut self, effect: ContinuousEffect) {
        self.continuous_effects.push(effect);
    }

    pub(crate) fn register_replacement_effect(&mut self, effect: ReplacementEffect) {
        self.replacement_effects.push(effect);
    }

    pub(crate) fn objects(&self) -> impl Iterator<Item = &GameObject> {
        self.objects.values()
    }

    pub(crate) fn objects_mut(&mut self) -> impl Iterator<Item = &mut GameObject> {
        self.objects.values_mut()
    }

    pub(crate) fn objects_from_ids_mut<'a, I, B>(
        &mut self,
        ids: I,
    ) -> impl Iterator<Item = &mut GameObject>
    where
        I: IntoIterator<Item = B>,
        B: Borrow<ObjectId>,
    {
        let mut ids = ids.into_iter().map(|id| *id.borrow()).collect::<Vec<_>>();
        self.objects.values_mut().filter_map(move |obj| {
            let pos = ids.iter().position(|&id| id == obj.id)?;
            Some(obj)
        })
    }

    pub(crate) fn get_valid_object(
        &self,
        object_id: ObjectId,
        timestamp: Timestamp,
    ) -> Option<&GameObject> {
        if let Some(obj) = self.objects.get(object_id) {
            if obj.timestamp == timestamp {
                return Some(obj);
            }
        }
        None
    }

    pub(crate) fn get_valid_object_mut(
        &mut self,
        object_id: ObjectId,
        timestamp: Timestamp,
    ) -> Option<&mut GameObject> {
        if let Some(obj) = self.objects.get_mut(object_id) {
            if obj.timestamp == timestamp {
                return Some(obj);
            }
        }
        None
    }

    pub(crate) fn objects_from_ids<'a, 'b, I>(&self, ids: I) -> impl Iterator<Item = &GameObject>
    where
        I: IntoIterator<Item = ObjectId>,
    {
        ids.into_iter().map(move |id| match self.objects.get(id) {
            Some(obj) => obj,
            None => panic!("Object with id {id:?} does not exist"),
        })
    }

    pub(crate) fn get_objects_by_zone(&self, zone: ZoneId) -> impl Iterator<Item = &GameObject> {
        enum Ids<'a> {
            Slice(Copied<Iter<'a, ObjectId>>),
            Lib(Copied<Rev<Iter<'a, ObjectId>>>),
            Stack(
                FilterMap<std::slice::Iter<'a, StackObject>, fn(&StackObject) -> Option<ObjectId>>,
            ),
        }

        impl Iterator for Ids<'_> {
            type Item = ObjectId;

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    Ids::Slice(iter) => iter.next(),
                    Ids::Stack(iter) => iter.next(),
                    Ids::Lib(iter) => iter.next(),
                }
            }
        }

        #[inline]
        fn id(obj: &StackObject) -> Option<ObjectId> {
            match obj.kind {
                StackObjectKind::Spell(object_id) => Some(object_id),
                _ => None,
            }
        }

        let ids = match zone {
            ZoneId::Battlefield => Ids::Slice(self.battlefield.iter().copied()),
            ZoneId::Hand(player_id) => Ids::Slice(self.players[player_id].hand.iter().copied()),
            ZoneId::Library(player_id) => {
                // Library is iterated in reverse order because the top of the library is at the end of the vector for easier access
                Ids::Lib(self.players[player_id].library.iter().rev().copied())
            }
            ZoneId::Graveyard(player_id) => {
                Ids::Slice(self.players[player_id].graveyard.iter().copied())
            }
            ZoneId::Exile(player_id) => Ids::Slice(self.players[player_id].exile.iter().copied()),
            ZoneId::Stack => Ids::Stack(self.stack.iter().filter_map(id)),
        };
        self.objects_from_ids(ids)
    }

    // FIXME: Improve this implementation with custom iterator that makes use of the indices

    pub(crate) fn get_objects_by_zone_mut(
        &mut self,
        zone: ZoneId,
    ) -> impl Iterator<Item = &mut GameObject> {
        self.objects
            .values_mut()
            .filter(move |obj| obj.zone.id() == zone)
    }
}

pub(crate) struct Players {
    pub(crate) play_player: Player,
    pub(crate) draw_player: Player,
}

impl Players {
    pub(crate) fn get(&self, player_id: PlayerId) -> &Player {
        match player_id {
            PlayerId::PlayPlayer => &self.play_player,
            PlayerId::DrawPlayer => &self.draw_player,
        }
    }

    pub(crate) fn get_mut(&mut self, player_id: PlayerId) -> &mut Player {
        match player_id {
            PlayerId::PlayPlayer => &mut self.play_player,
            PlayerId::DrawPlayer => &mut self.draw_player,
        }
    }
}

impl Index<PlayerId> for Players {
    type Output = Player;

    fn index(&self, index: PlayerId) -> &Self::Output {
        self.get(index)
    }
}

impl IndexMut<PlayerId> for Players {
    fn index_mut(&mut self, index: PlayerId) -> &mut Self::Output {
        self.get_mut(index)
    }
}
