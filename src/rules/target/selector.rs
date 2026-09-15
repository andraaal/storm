use crate::{context::Context, game::Game, rules::target::Target};
use std::{array, range::Range};

pub(crate) trait SelectorList {
    type Selected: Copy;

    fn choose(ctx: &mut Context) -> Self::Selected
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

        }
    };
    ($($T:ident),+) => {
        impl<$($T: Selector),+> SelectorList for ($($T,)+) where $($T::Selected: Copy),+ {
            type Selected = ($($T::Selected,)+);

            fn choose(ctx: &mut Context) -> Self::Selected {
                let game = &ctx.game;
                ($(ctx.controller.choose(game,$T::choices(game), Range{start: 1, end: 2}).into_iter().next().unwrap(),)+)
            }

        }
    };
}

impl<const N: usize, T: Selector> SelectorList for [T; N] {
    type Selected = [T::Selected; N];

    fn choose(ctx: &mut Context) -> Self::Selected
    where
        Self: Sized,
    {
        let game = &ctx.game;
        array::from_fn(|_| {
            ctx.controller
                .choose(game, T::choices(game), Range { start: 1, end: 2 })
                .into_iter()
                .next()
                .unwrap()
        })
    }
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
