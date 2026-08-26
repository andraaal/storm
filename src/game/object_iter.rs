use std::borrow::Borrow;

use crate::rules::{
    id::ObjectId,
    object::game_object::GameObject,
    player::PlayerId,
    zone::{BattlefieldInfo, Zone, ZoneId},
};

pub(crate) trait GameObjectIterExt: Iterator {
    fn in_zone(self, zone: ZoneId) -> impl Iterator<Item = Self::Item>
    where
        Self: Sized,
        Self::Item: Borrow<GameObject>,
    {
        self.filter(move |obj| obj.borrow().zone.id() == zone)
    }

    fn in_zones(self, zones: &[ZoneId]) -> impl Iterator<Item = Self::Item>
    where
        Self: Sized,
        Self::Item: Borrow<GameObject>,
    {
        self.filter(move |obj| zones.contains(&obj.borrow().zone.id()))
    }

    fn not_in_zone(self, zone: ZoneId) -> impl Iterator<Item = Self::Item>
    where
        Self: Sized,
        Self::Item: Borrow<GameObject>,
    {
        self.filter(move |obj| obj.borrow().zone.id() != zone)
    }

    fn not_in_zones(self, zones: &[ZoneId]) -> impl Iterator<Item = Self::Item>
    where
        Self: Sized,
        Self::Item: Borrow<GameObject>,
    {
        self.filter(move |obj| !zones.contains(&obj.borrow().zone.id()))
    }

    fn controlled_by(
        self,
        player_id: crate::rules::player::PlayerId,
    ) -> impl Iterator<Item = Self::Item>
    where
        Self: Sized,
        Self::Item: Borrow<GameObject>,
    {
        self.filter(move |obj| match obj.borrow().zone {
            Zone::Battlefield(BattlefieldInfo { controller, .. })
            | Zone::Stack { controller, .. } => controller == player_id,
            _ => false,
        })
    }

    fn owned_by(self, player_id: crate::rules::player::PlayerId) -> impl Iterator<Item = Self::Item>
    where
        Self: Sized,
        Self::Item: Borrow<GameObject>,
    {
        self.filter(move |obj| obj.borrow().owner == player_id)
    }

    fn battlefield_mut<'a>(self) -> impl Iterator<Item = &'a mut BattlefieldInfo>
    where
        Self: Iterator<Item = &'a mut GameObject> + Sized,
    {
        self.filter_map(|obj| match &mut obj.zone {
            Zone::Battlefield(info) => Some(info),
            _ => None,
        })
    }

    fn battlefield<'a>(self) -> impl Iterator<Item = &'a BattlefieldInfo>
    where
        Self: Iterator<Item = &'a GameObject> + Sized,
    {
        self.filter_map(|obj| match &obj.zone {
            Zone::Battlefield(info) => Some(info),
            _ => None,
        })
    }

    fn filter_battlefield(
        self,
        mut f: impl FnMut(&&BattlefieldInfo) -> bool,
    ) -> impl Iterator<Item = Self::Item>
    where
        Self: Sized,
        Self::Item: Borrow<GameObject>,
    {
        self.filter(move |obj| match &obj.borrow().zone {
            Zone::Battlefield(info) => f(&info),
            _ => false,
        })
    }

    fn stack<'a>(self) -> impl Iterator<Item = &'a mut PlayerId>
    where
        Self: Iterator<Item = &'a mut GameObject> + Sized,
    {
        self.filter_map(|obj| match &mut obj.zone {
            Zone::Stack { controller } => Some(controller),
            _ => None,
        })
    }

    fn id(self) -> impl Iterator<Item = ObjectId>
    where
        Self: Sized,
        Self::Item: Borrow<GameObject>,
    {
        self.map(|obj| obj.borrow().id)
    }
}

impl<I> GameObjectIterExt for I where I: Iterator {}
