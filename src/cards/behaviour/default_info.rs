use crate::{
    cards::selectors::creature::Creature,
    rules::{ability::behaviour::Behaviour, target::selector::Selector},
};

pub(crate) struct DefaultInfo {}

impl Behaviour for DefaultInfo {
    type TargetSelector = Creature;

    type Data = ();

    fn apply(
        _ctx: &mut crate::context::Context,
        _targets: &<Self::TargetSelector as Selector>::Selected,
        _data: Self::Data,
    ) where
        Self: Sized,
    {
    }
}
