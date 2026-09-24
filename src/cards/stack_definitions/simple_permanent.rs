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
        guard: crate::nested_borrow::NestedBorrow<
            '_,
            '_,
            crate::rules::ability::effect::StackObject<Self, R>,
            crate::context::Context,
        >,
    ) -> Self::Return
    where
        Self: Sized,
    {
        let ctx = guard.finish();
        let (owner, controller) = ctx
            .game
            .objects
            .get_controller_and_owner(ctx.game.objects.resolving_spell.unwrap());

        BattlefieldInfo {
            tapped: false,
            marked_damage: 0,
            damaged_by_deathtouch: false,
            controller: owner.unwrap(),
            owner: controller.unwrap_or(owner.unwrap()),
        }
    }

    fn choose_data(_: &mut crate::context::Context) -> <Self::Behaviours as BehaviourList>::Data {
        <Self::Behaviours as BehaviourList>::Data::default()
    }
}
