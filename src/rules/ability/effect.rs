use std::borrow::{Borrow, BorrowMut};

use crate::{context::Context, rules::ability::effect::private::Sealed};

pub trait StackObjectInfoTrait {
    fn targets(&self) -> &[crate::rules::target::Target];
    fn x(&self) -> Option<u32>;
    fn modes(&self) -> Option<&[usize]>;
    fn modes_mut(&mut self) -> Option<&mut [usize]>;
}

pub trait Resolveable {
    unsafe fn execute(&self, ctx: *mut Context);
}

pub trait StackObj: StackObjectInfoTrait + Resolveable {}

impl<E: Effect<Z, R>, Z: ZoneIdTrait, R: ResolutionBehaviour> StackObj for StackObject<E, Z, R> {}

impl<E: Effect<Z, R>, Z: ZoneIdTrait, R: ResolutionBehaviour> Resolveable for StackObject<E, Z, R> {
    unsafe fn execute(&self, c: *mut Context) {
        // SAFETY: The caller has to guarantee safety
        let guard = unsafe { ResolutionGuard::new(self, c) };
        R::execute(guard);
    }
}

pub struct StackObject<E: Effect<Z, R>, Z: ZoneIdTrait, R: ResolutionBehaviour> {
    pub(crate) targets: E::Targets,
    pub(crate) x: E::X,
    pub(crate) modes: E::Modes,
    pub(crate) data: E::Data,

    pub(crate) source: ObjectId<Z>,
    // pub(crate) kind: std::marker::PhantomData<R>,
}

/// The goal of this struct is be able to pass both a reference to context and a reference to a specific object within the context to another function. To make this safe, the reference to the context is only available after the reference to the inner object is no longer accessible.
pub struct ResolutionGuard<'a, E: Effect<Z, R>, Z: ZoneIdTrait, R: ResolutionBehaviour> {
    object: &'a StackObject<E, Z, R>,
    ctx: *mut Context,
    _marker: std::marker::PhantomData<&'a mut Context>,
}

impl<'a, E: Effect<Z, R>, Z: ZoneIdTrait, R: ResolutionBehaviour> ResolutionGuard<'a, E, Z, R> {
    /// Safety requirement: The context pointer must be valid, exclusive (except for the borrow object) and life for at least 'a, usually achieved by borrowing the object from the context.
    pub(crate) unsafe fn new(object: &'a StackObject<E, Z, R>, ctx: *mut Context) -> Self {
        Self {
            object,
            ctx,
            _marker: std::marker::PhantomData,
        }
    }

    pub(crate) fn object(&self) -> &StackObject<E, Z, R> {
        self.object
    }

    pub(crate) fn finish(self) -> &'a mut Context {
        // SAFETY: The constructor requires that the context pointer is valid, exclusive and lives for exactly 'a, so this is safe.
        // Additionally the object reference is no longer accessible after this function, so the context can be safely accessed again.
        unsafe { &mut *self.ctx }
    }
}

pub trait ResolutionBehaviour: Sealed {
    fn execute<E: Effect<Z, Self>, Z: ZoneIdTrait>(guard: ResolutionGuard<'_, E, Z, Self>)
    where
        Self: Sized;
}

struct Permanent;
impl Sealed for Permanent {}
impl ResolutionBehaviour for Permanent {
    fn execute<E, Z>(guard: ResolutionGuard<'_, E, Z, Self>)
    where
        E: Effect<Z, Permanent>,
        Z: ZoneIdTrait,
    {
        E::execute(guard);
        // Move to battlefield afterwards
    }
}
struct Spell;
impl Sealed for Spell {}
impl ResolutionBehaviour for Spell {
    fn execute<E, Z>(guard: ResolutionGuard<'_, E, Z, Self>)
    where
        E: Effect<Z, Spell>,
        Z: ZoneIdTrait,
    {
        E::execute(guard);
        // Move to graveyard afterwards
    }
}
struct TriggeredAbility;
impl Sealed for TriggeredAbility {}
impl ResolutionBehaviour for TriggeredAbility {
    fn execute<E, Z>(guard: ResolutionGuard<'_, E, Z, TriggeredAbility>)
    where
        E: Effect<Z, TriggeredAbility>,
        Z: ZoneIdTrait,
    {
        E::execute(guard);
    }
}
struct ActivatedAbility;
impl Sealed for ActivatedAbility {}
impl ResolutionBehaviour for ActivatedAbility {
    fn execute<E, Z>(guard: ResolutionGuard<'_, E, Z, ActivatedAbility>)
    where
        E: Effect<Z, ActivatedAbility>,
        Z: ZoneIdTrait,
    {
        E::execute(guard);
    }
}

pub struct ObjectId<Z: ZoneIdTrait> {
    pub(crate) id: crate::rules::id::ObjectId,
    pub(crate) zone: Z,
}

pub trait ZoneIdTrait: Sealed {}

struct AnyZone;
impl Sealed for AnyZone {}
impl ZoneIdTrait for AnyZone {}
struct Battlefield;
impl Sealed for Battlefield {}
impl ZoneIdTrait for Battlefield {}
struct Stack;
impl Sealed for Stack {}
impl ZoneIdTrait for Stack {}
struct Graveyard<const PLAYER_ID: usize>;
impl<const PLAYER_ID: usize> Sealed for Graveyard<PLAYER_ID> {}
impl<const PLAYER_ID: usize> ZoneIdTrait for Graveyard<PLAYER_ID> {}

impl<E: Effect<Z, R>, Z: ZoneIdTrait, R: ResolutionBehaviour> StackObjectInfoTrait
    for StackObject<E, Z, R>
{
    fn targets(&self) -> &[crate::rules::target::Target] {
        self.targets.borrow()
    }

    fn x(&self) -> Option<u32> {
        self.x.get()
    }

    fn modes(&self) -> Option<&[usize]> {
        self.modes.get_modes()
    }

    fn modes_mut(&mut self) -> Option<&mut [usize]> {
        self.modes.get_modes_mut()
    }
}

pub trait Effect<Z: ZoneIdTrait, R: ResolutionBehaviour> {
    type Targets: BorrowMut<[crate::rules::target::Target]>;
    type X: XVal;
    type Modes: Modes;
    type Data;

    // How do we solve this borrowing issue? We could of course solve this with some sort of generic id for stack objects, I find that a bit unappealing, because we already have access to the stack object and then we throw it away to just access it again with the id.
    fn execute(guard: ResolutionGuard<'_, Self, Z, R>)
    where
        Self: Sized;
}

mod private {
    pub(super) trait Sealed {}
}

pub(crate) struct WithX(u32);
pub(crate) struct WithoutX;

trait XVal: private::Sealed {
    fn get(&self) -> Option<u32>;
    fn get_mut(&mut self) -> Option<&mut u32>;
}
impl private::Sealed for WithX {}
impl private::Sealed for WithoutX {}
impl XVal for WithX {
    fn get(&self) -> Option<u32> {
        Some(self.0)
    }
    fn get_mut(&mut self) -> Option<&mut u32> {
        Some(&mut self.0)
    }
}
impl XVal for WithoutX {
    fn get(&self) -> Option<u32> {
        None
    }
    fn get_mut(&mut self) -> Option<&mut u32> {
        None
    }
}

pub(crate) struct WithoutModes;
pub(crate) struct WithModes<const CHOICE_RANGE: usize, const CHOICE_AMOUNT: usize> {
    modes: [usize; CHOICE_AMOUNT],
}

pub(crate) trait Modes: private::Sealed {
    fn get_modes(&self) -> Option<&[usize]>;
    fn get_modes_mut(&mut self) -> Option<&mut [usize]>;
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

impl private::Sealed for WithoutModes {}
impl<'a, const CHOICE_RANGE: usize, const CHOICE_AMOUNT: usize> private::Sealed
    for WithModes<CHOICE_RANGE, CHOICE_AMOUNT>
{
}
impl Modes for WithoutModes {
    fn get_modes(&self) -> Option<&[usize]> {
        None
    }

    fn get_modes_mut(&mut self) -> Option<&mut [usize]> {
        None
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
}
