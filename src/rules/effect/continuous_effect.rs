use crate::{
    game::Game,
    rules::id::{ObjectId, Timestamp},
    rules::object::game_object::GameObject,
    rules::{condition::Condition, layer::Layer, target::Target},
};

pub(crate) struct PermanentContinuousEffect {
    pub(crate) timestamp: Timestamp,
    pub(crate) source: ObjectId,
    pub(crate) effect: Box<dyn Fn(&mut Game, ObjectId)>,
    pub(crate) layer: Layer,
}

pub(crate) struct SpellContinuousEffect {
    pub(crate) timestamp: Timestamp,
    pub(crate) source: ObjectId,
    pub(crate) targets: Vec<Target>,
    pub(crate) effect: Box<dyn Fn(&mut GameObject)>,
    pub(crate) layer: Layer,
    pub(crate) end: Condition,
}

#[derive(Debug)]
pub(crate) enum ContinuousEffect {
    Dynamic(PermanentContinuousEffect),
    Static(SpellContinuousEffect),
}

impl std::fmt::Debug for PermanentContinuousEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContinuousEffect")
            .field("timestamp", &self.timestamp)
            .field("layer", &self.layer)
            .field("source", &self.source)
            .finish()
    }
}

impl std::fmt::Debug for SpellContinuousEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StaticContinuousEffect")
            .field("timestamp", &self.timestamp)
            .field("layer", &self.layer)
            .field("source", &self.source)
            .field("targets", &self.targets)
            .field("end", &self.end)
            .finish()
    }
}
