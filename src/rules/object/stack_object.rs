use std::borrow::Cow;
use std::rc::Rc;

use crate::game::Game;
use crate::rules::id::{ObjectId, StackId, TriggeredEffectId};
use crate::rules::{cost::Cost, target::Target};

#[derive(Debug, Clone)]
pub(crate) struct StackObject {
    pub(crate) id: StackId,
    pub(crate) kind: StackObjectKind,
    pub(crate) x: Option<i32>,
    pub(crate) modes: Vec<u8>,
    pub(crate) paid_cost: Cost,
    pub(crate) targets: Vec<Target>,
    pub(crate) target_validators: Cow<'static, [fn(&Game, Target) -> bool]>,
    pub(crate) source: ObjectId,
}

#[derive(Clone)]
pub(crate) enum StackObjectKind {
    Spell(ObjectId),
    TriggeredAbility(TriggeredEffectId, Rc<dyn Fn(&mut Game, StackId)>),
    ActivatedAbility(Rc<dyn Fn(&mut Game, StackId)>),
}

impl std::fmt::Debug for StackObjectKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackObjectKind::Spell(_) => f.write_str("Spell"),
            StackObjectKind::TriggeredAbility(_, _) => f.write_str("TriggeredAbility"),
            StackObjectKind::ActivatedAbility(_) => f.write_str("ActivatedAbility"),
        }
    }
}
