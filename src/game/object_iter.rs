use std::borrow::Borrow;

use crate::rules::{
    object::game_object::GameObject,
    player::PlayerId,
    zone::{BattlefieldInfo, StackInfo},
};

pub(crate) trait GameObjectIterExt: Iterator {
    fn controlled_by<T>(
        self,
        player_id: crate::rules::player::PlayerId,
    ) -> impl Iterator<Item = Self::Item>
    where
        Self: Sized,
        Self::Item: Borrow<T>,
        T: Controlled,
    {
        self.filter(move |obj| {
            let borrowed: &T = obj.borrow();
            borrowed.get_controller() == player_id
        })
    }

    fn battlefield_mut<'a>(self) -> impl Iterator<Item = &'a mut BattlefieldInfo>
    where
        Self: Iterator<Item = &'a mut (BattlefieldInfo, GameObject)> + Sized,
    {
        self.map(|obj| &mut obj.0)
    }
}

impl<I> GameObjectIterExt for I where I: Iterator {}

pub(crate) trait Controlled {
    fn get_controller(&self) -> PlayerId;
    fn get_controller_mut(&mut self) -> &mut PlayerId;
}

impl Controlled for BattlefieldInfo {
    fn get_controller(&self) -> PlayerId {
        self.controller
    }

    fn get_controller_mut(&mut self) -> &mut PlayerId {
        &mut self.controller
    }
}

impl Controlled for StackInfo {
    fn get_controller(&self) -> PlayerId {
        self.controller
    }

    fn get_controller_mut(&mut self) -> &mut PlayerId {
        &mut self.controller
    }
}

impl<C> Controlled for &mut (C, GameObject)
where
    C: Controlled,
{
    fn get_controller(&self) -> PlayerId {
        self.0.get_controller()
    }
    fn get_controller_mut(&mut self) -> &mut PlayerId {
        self.0.get_controller_mut()
    }
}
