use std::rc::Rc;

use crate::{
    cards::selectors::creature::Creature,
    rules::{
        ability::{
            behaviour::Behaviour,
            static_ability::{FixedAbilityGroup, FixedContinuousAbility},
        },
        condition::Condition,
        object::types::ObjectType,
        target::selector::Selector,
    },
};

pub(crate) struct ChangePT<const P: i64, const T: i64> {}

impl<const P: i64, const T: i64> Behaviour for ChangePT<P, T> {
    type TargetSelector = Creature;

    type Data = ();

    fn apply(
        ctx: &mut crate::context::Context,
        targets: &<Self::TargetSelector as Selector>::Selected,
        _data: Self::Data,
    ) where
        Self: Sized,
    {
        let target = *targets;
        ctx.game
            .register_fixed_effect(FixedAbilityGroup::FixedContinuous(vec![
                FixedContinuousAbility {
                    layer: crate::rules::layer::Layer::ChangePT,
                    end: Condition::EndOfTurn,
                    effect: Rc::new(move |ctx| {
                        if let Some((_, obj)) = ctx.game.objects.battlefield.get_mut(target) {
                            for typ in &mut obj.characteristics.types.to_mut().iter_mut() {
                                match typ {
                                    ObjectType::Creature {
                                        power, toughness, ..
                                    } => {
                                        *power += P;
                                        *toughness += T;
                                        return;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }),
                },
            ]));
    }
}
