use crate::{
    context::Context,
    rules::target::selector::{Selector, SelectorList},
};

pub(crate) trait Behaviour
where
    <Self::TargetSelector as Selector>::Selected: Copy,
{
    type TargetSelector: Selector;
    type Data;

    fn apply(
        ctx: &mut Context,
        targets: &<Self::TargetSelector as Selector>::Selected,
        data: Self::Data,
    ) where
        Self: Sized;
}

pub(crate) trait BehaviourList {
    type Targets: SelectorList;
    type Data;

    fn apply(
        ctx: &mut Context,
        targets: <Self::Targets as SelectorList>::Selected,
        data: Self::Data,
    ) where
        Self: Sized;
}

impl BehaviourList for () {
    type Targets = ();
    type Data = ();

    fn apply(_: &mut Context, _: <Self::Targets as SelectorList>::Selected, _: Self::Data)
    where
        Self: Sized,
    {
    }
}

impl<B: Behaviour> BehaviourList for (B,) {
    type Targets = (B::TargetSelector,);
    type Data = (B::Data,);

    fn apply(
        ctx: &mut Context,
        targets: <Self::Targets as SelectorList>::Selected,
        data: Self::Data,
    ) where
        Self: Sized,
    {
        B::apply(ctx, &targets.0, data.0)
    }
}

impl<B1: Behaviour, B2: Behaviour> BehaviourList for (B1, B2) {
    type Targets = (B1::TargetSelector, B2::TargetSelector);
    type Data = (B1::Data, B2::Data);

    fn apply(
        ctx: &mut Context,
        targets: <Self::Targets as SelectorList>::Selected,
        data: Self::Data,
    ) where
        Self: Sized,
    {
        B1::apply(ctx, &targets.0, data.0);
        B2::apply(ctx, &targets.1, data.1);
    }
}

impl<B1: Behaviour, B2: Behaviour, B3: Behaviour> BehaviourList for (B1, B2, B3) {
    type Targets = (B1::TargetSelector, B2::TargetSelector, B3::TargetSelector);
    type Data = (B1::Data, B2::Data, B3::Data);

    fn apply(
        ctx: &mut Context,
        targets: <Self::Targets as SelectorList>::Selected,
        data: Self::Data,
    ) where
        Self: Sized,
    {
        B1::apply(ctx, &targets.0, data.0);
        B2::apply(ctx, &targets.1, data.1);
        B3::apply(ctx, &targets.2, data.2);
    }
}

pub(crate) trait EmptyData: Default {}
impl EmptyData for () {}
impl EmptyData for ((),) {}
impl EmptyData for ((), ()) {}
impl EmptyData for ((), (), ()) {}
impl EmptyData for ((), (), (), ()) {}
impl EmptyData for ((), (), (), (), ()) {}
