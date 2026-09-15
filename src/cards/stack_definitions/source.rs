use std::marker::PhantomData;

use crate::rules::{
    ability::{
        behaviour::BehaviourList,
        effect::{StackDefinition, WithoutModes, WithoutX},
    },
    id::AnyId,
};

pub(crate) struct SourceData<B: BehaviourList<Data = (AnyId,)>> {
    _marker: PhantomData<B>,
}

impl<B: BehaviourList<Data = (AnyId,)>> StackDefinition for SourceData<B> {
    type Behaviours = B;

    type TargetSelection = B::Targets;

    type X = WithoutX;

    type Modes = WithoutModes;

    type Return = ();

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
        let targets = guard.object().targets;
        let data = guard.object().data;
        let ctx = guard.finish();
        B::apply(ctx, targets, data);
    }

    fn choose_data(
        _ctx: &mut crate::context::Context,
    ) -> <Self::Behaviours as BehaviourList>::Data {
        // Get currently cast AnyId from context
        todo!()
    }
}
