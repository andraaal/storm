use std::marker::PhantomData;

use crate::rules::{
    ability::{
        behaviour::{BehaviourList, EmptyData},
        effect::{StackDefinition, WithoutModes, WithoutX},
    },
};

pub(crate) struct NoData<B: BehaviourList> {
    _marker: PhantomData<B>,
}

impl<B> StackDefinition for NoData<B>
where
    B: BehaviourList,
    <B as BehaviourList>::Data: EmptyData,
{
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
        let ctx = guard.finish();
        B::apply(
            ctx,
            targets,
            <<B as BehaviourList>::Data as Default>::default(),
        );
    }

    fn choose_data(_: &mut crate::context::Context) -> <Self::Behaviours as BehaviourList>::Data {
        <Self::Behaviours as BehaviourList>::Data::default()
    }
}
