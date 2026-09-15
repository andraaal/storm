use crate::{
    cards::selectors::damagable::Damagable,
    rules::{
        ability::behaviour::Behaviour, game_action::GameAction, id::AnyId,
        target::selector::Selector,
    },
};

pub(crate) struct DealDamage<const D: u32, const T: usize> {}

impl<const D: u32, const T: usize> Behaviour for DealDamage<D, T> {
    type TargetSelector = Damagable;

    type Data = AnyId;

    fn apply(
        ctx: &mut crate::context::Context,
        targets: &<Self::TargetSelector as Selector>::Selected,
        _data: Self::Data,
    ) where
        Self: Sized,
    {
        ctx.execute(vec![GameAction::DealDamage {
            source: _data,
            targets: vec![(D, *targets)],
        }]);
    }
}
