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
        let (controller, owner) = ctx
            .game
            .objects
            .get_controller_and_owner(ctx.game.objects.resolving_id.unwrap());
        let owner = owner.unwrap();
        let controller = controller.unwrap_or(owner);

        BattlefieldInfo {
            tapped: false,
            marked_damage: 0,
            damaged_by_deathtouch: false,
            controller: controller,
            owner: owner,
        }
    }

    fn choose_data(_: &mut crate::context::Context) -> <Self::Behaviours as BehaviourList>::Data {
        <Self::Behaviours as BehaviourList>::Data::default()
    }
}
