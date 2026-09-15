use crate::{context::Context, rules::target::Target};

pub(crate) struct Zero {}

pub(crate) struct One<T: Target> {
    first: T,
}

pub(crate) struct Two<T: Target, T2: Target> {
    first: T,
    second: T2,
}

pub(crate) struct Three<T: Target, T2: Target, T3: Target> {
    first: T,
    second: T2,
    third: T3,
}

pub(crate) struct Four<T: Target, T2: Target, T3: Target, T4: Target> {
    first: T,
    second: T2,
    third: T3,
    fourth: T4,
}

pub(crate) struct Five<T: Target, T2: Target, T3: Target, T4: Target, T5: Target> {
    first: T,
    second: T2,
    third: T3,
    fourth: T4,
    fifth: T5,
}

pub(crate) trait TargetList {
    fn choose(ctx: &mut Context) -> Self
    where
        Self: Sized;
}

impl TargetList for Zero {
    fn choose(_: &mut Context) -> Self
    where
        Self: Sized,
    {
        Zero {}
    }
}
