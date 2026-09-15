use std::range::Range;

use crate::{
    context::Context,
    nested_borrow::{NestedBorrow, ShadowBorrow},
    rules::{
        ability::behaviour::BehaviourList, id::AnyId, object::game_object::GameObject,
        target::selector::SelectorList, zone::BattlefieldInfo,
    },
};

pub trait StackObj {
    fn x(&self) -> Option<u32>;
    fn modes(&self) -> Option<&[usize]>;
    fn modes_mut(&mut self) -> Option<&mut [usize]>;
    fn execute<B>(&mut self, guard: ShadowBorrow<'_, Context>) -> ReturnKind
    where
        Self: Sized;
}

pub struct StackObject<E: StackDefinition, R: ResolutionBehaviour<Return = E::Return>> {
    pub(crate) targets: <E::TargetSelection as SelectorList>::Selected,
    pub(crate) x: E::X,
    pub(crate) modes: E::Modes,
    pub(crate) data: <E::Behaviours as BehaviourList>::Data,
    pub(crate) source: R::StackSource,

    pub(crate) kind: std::marker::PhantomData<R>,
}

pub trait ResolutionBehaviour {
    type Return: Return;
    type StackSource;
    fn execute<E: StackDefinition<Return = Self::Return>>(
        guard: NestedBorrow<'_, '_, StackObject<E, Self>, Context>,
    ) -> ReturnKind
    where
        Self: Sized;
}

pub(crate) struct ToBattlefield;
impl ResolutionBehaviour for ToBattlefield {
    type Return = BattlefieldInfo;
    type StackSource = GameObject;
    fn execute<E: StackDefinition<Return = Self::Return>>(
        guard: NestedBorrow<'_, '_, StackObject<E, Self>, Context>,
    ) -> ReturnKind {
        let battlefield_info = E::execute(guard);
        ReturnKind::ToBattlefield(battlefield_info)
    }
}
pub(crate) struct ToGraveyard;
impl ResolutionBehaviour for ToGraveyard {
    type Return = ();
    type StackSource = GameObject;
    fn execute<E: StackDefinition<Return = Self::Return>>(
        guard: NestedBorrow<'_, '_, StackObject<E, Self>, Context>,
    ) -> ReturnKind {
        E::execute(guard);
        // Move to graveyard afterwards
        ReturnKind::ToGraveyard
    }
}
pub(crate) struct Vanish;
impl ResolutionBehaviour for Vanish {
    type Return = ();
    type StackSource = AnyId;
    fn execute<E: StackDefinition<Return = Self::Return>>(
        guard: NestedBorrow<'_, '_, StackObject<E, Self>, Context>,
    ) -> ReturnKind {
        E::execute(guard);
        ReturnKind::Vanish
    }
}

impl<E: StackDefinition, R: ResolutionBehaviour<Return = E::Return>> StackObj
    for StackObject<E, R>
{
    fn x(&self) -> Option<u32> {
        self.x.get()
    }

    fn modes(&self) -> Option<&[usize]> {
        self.modes.get_modes()
    }

    fn modes_mut(&mut self) -> Option<&mut [usize]> {
        self.modes.get_modes_mut()
    }

    fn execute<B>(&mut self, guard: ShadowBorrow<'_, Context>) -> ReturnKind
    where
        Self: Sized,
    {
        unsafe { guard.call_unsafe(self, R::execute) }
    }
}

pub trait StackDefinition: Sized {
    type TargetSelection: SelectorList;
    type Behaviours: BehaviourList<Targets = Self::TargetSelection>;
    type X: XVal;
    type Modes: Modes;
    type Return: Return;

    fn execute<R: ResolutionBehaviour<Return = Self::Return>>(
        guard: NestedBorrow<'_, '_, StackObject<Self, R>, Context>,
    ) -> Self::Return
    where
        Self: Sized;

    fn choose_data(ctx: &mut Context) -> <Self::Behaviours as BehaviourList>::Data;
}

trait Return {}
impl Return for () {}
impl Return for BattlefieldInfo {}

pub(crate) enum ReturnKind {
    Vanish,
    ToGraveyard,
    ToBattlefield(BattlefieldInfo),
}

// ================

pub(crate) trait SpellResolutionBehaviour: ResolutionBehaviour {}

impl SpellResolutionBehaviour for ToBattlefield {}
impl SpellResolutionBehaviour for ToGraveyard {}

pub(crate) type SpellObject<E: StackDefinition<Return = R::Return>, R: SpellResolutionBehaviour> =
    StackObject<E, R>;

pub(crate) type AbilityObject<E: StackDefinition<Return = ()>> = StackObject<E, Vanish>;

pub(crate) trait Spell: StackObj {
    fn get_source(&self) -> &GameObject;
    fn get_source_mut(&mut self) -> &mut GameObject;
    fn take_source(self: Box<Self>) -> GameObject;
}
pub(crate) trait Ability: StackObj {
    fn get_source(&self) -> AnyId;
}

impl<E: StackDefinition, R: SpellResolutionBehaviour<Return = E::Return, StackSource = GameObject>>
    Spell for SpellObject<E, R>
{
    fn get_source(&self) -> &GameObject {
        &self.source
    }
    fn get_source_mut(&mut self) -> &mut GameObject {
        &mut self.source
    }
    fn take_source(self: Box<Self>) -> GameObject {
        self.source
    }
}

impl<E: StackDefinition<Return = ()>> Ability for AbilityObject<E> {
    fn get_source(&self) -> AnyId {
        self.source
    }
}

// ================

pub(crate) struct WithX(u32);
pub(crate) struct WithoutX;

pub(crate) trait XVal {
    fn get(&self) -> Option<u32>;
    fn get_mut(&mut self) -> Option<&mut u32>;

    fn choose(ctx: &mut Context) -> Self
    where
        Self: Sized;
}
impl XVal for WithX {
    fn get(&self) -> Option<u32> {
        Some(self.0)
    }
    fn get_mut(&mut self) -> Option<&mut u32> {
        Some(&mut self.0)
    }
    fn choose(ctx: &mut Context) -> Self
    where
        Self: Sized,
    {
        WithX(ctx.controller.choose_number(
            &ctx.game,
            Range {
                start: 0,
                end: usize::MAX,
            },
        ) as u32)
    }
}
impl XVal for WithoutX {
    fn get(&self) -> Option<u32> {
        None
    }
    fn get_mut(&mut self) -> Option<&mut u32> {
        None
    }
    fn choose(_: &mut Context) -> Self
    where
        Self: Sized,
    {
        Self {}
    }
}

pub(crate) struct WithoutModes;
pub(crate) struct WithModes<const CHOICE_RANGE: usize, const CHOICE_AMOUNT: usize> {
    modes: [usize; CHOICE_AMOUNT],
}

pub(crate) trait Modes {
    fn get_modes(&self) -> Option<&[usize]>;
    fn get_modes_mut(&mut self) -> Option<&mut [usize]>;

    fn choose(ctx: &mut Context) -> Self
    where
        Self: Sized;
}
impl<'a, const CHOICE_RANGE: usize, const CHOICE_AMOUNT: usize>
    WithModes<CHOICE_RANGE, CHOICE_AMOUNT>
{
    pub(crate) const fn new(modes: [usize; CHOICE_AMOUNT]) -> Self {
        let mut i = 0;
        while i < CHOICE_AMOUNT {
            assert!(modes[i] < CHOICE_RANGE);
            i += 1;
        }
        Self { modes }
    }
}

impl Modes for WithoutModes {
    fn get_modes(&self) -> Option<&[usize]> {
        None
    }

    fn get_modes_mut(&mut self) -> Option<&mut [usize]> {
        None
    }

    fn choose(_: &mut Context) -> Self {
        Self {}
    }
}

impl<'a, const CHOICE_RANGE: usize, const CHOICE_AMOUNT: usize> Modes
    for WithModes<CHOICE_RANGE, CHOICE_AMOUNT>
{
    fn get_modes(&self) -> Option<&[usize]> {
        Some(&self.modes[..])
    }

    fn get_modes_mut(&mut self) -> Option<&mut [usize]> {
        Some(&mut self.modes[..])
    }

    fn choose(_ctx: &mut Context) -> Self {
        todo!()
    }
}
