use crate::{context::Context, game::Game, rules::target::Target};
use std::range::Range;

pub(crate) trait SelectorList {
    type Selected: Copy;

    fn choose(ctx: &mut Context) -> Self::Selected
    where
        Self: Sized;

    fn choose_cancellable(ctx: &mut Context) -> Option<Self::Selected>
    where
        Self: Sized;
}

macro_rules! impl_selection {
    () => {
        impl SelectorList for () {
            type Selected = ();

            fn choose(_: &mut Context) -> Self::Selected {
                ()
            }

            fn choose_cancellable(_: &mut Context) -> Option<Self::Selected> {
                Some(())
            }
        }
    };
    ($($T:ident),+) => {
        impl<$($T: Selector),+> SelectorList for ($($T,)+) where $($T::Selected: Copy),+ {
            type Selected = ($($T::Selected,)+);

            fn choose(ctx: &mut Context) -> Self::Selected {
                let game = &ctx.game;
                ($(ctx.controller.choose(game,$T::choices(game), Range{start: 1, end: 2}).into_iter().next().unwrap(),)+)
            }

            fn choose_cancellable(ctx: &mut Context) -> Option<Self::Selected> {
                let game = &ctx.game;
                Some(($(ctx.controller.choose_cancellable(game,$T::choices(game), Range{start: 1, end: 2})?.into_iter().next().unwrap(),)+))
            }
        }
    };
}

impl_selection!();
impl_selection!(A);
impl_selection!(A, B);
impl_selection!(A, B, C);
impl_selection!(A, B, C, E);
impl_selection!(A, B, C, E, F);

pub(crate) trait Selector {
    type Selected: Target + Copy;

    fn choices(game: &Game) -> Vec<Self::Selected>
    where
        Self: Sized;
}
