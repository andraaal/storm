use crate::rules::{
    ability::{
        behaviour::BehaviourList,
        effect::{StackDefinition, WithoutModes, WithoutX},
    },
    zone::BattlefieldInfo,
};

pub(crate) struct Permanent {}

impl StackDefinition for Permanent {
    type Behaviours = ();

    type TargetSelection = ();

    type X = WithoutX;

    type Modes = WithoutModes;

    type Return = BattlefieldInfo;

    fn execute<R: crate::rules::ability::effect::ResolutionBehaviour<Return = Self::Return>>(
        mut guard: crate::nested_borrow::NestedBorrow<
            '_,
            '_,
            crate::rules::ability::effect::StackObject<Self, R>,
            crate::context::Context,
        >,
    ) -> Self::Return
    where
        Self: Sized,
    {
        BattlefieldInfo {
            tapped: false,
            marked_damage: 0,
            damaged_by_deathtouch: false,
            controller: guard.object().stack_info.controller,
            owner: guard.object().stack_info.owner,
        }
    }

    fn choose_data(_: &mut crate::context::Context) -> <Self::Behaviours as BehaviourList>::Data {
        <Self::Behaviours as BehaviourList>::Data::default()
    }
}
