use thiserror::Error;

use crate::{
    context::{config::Config, controller::Controller},
    game::{Game, objects::Objects},
};

pub(crate) mod config;
pub(crate) mod context_builder;
pub(crate) mod controller;
pub(crate) mod execution {
    pub(crate) mod actions;
    pub(crate) mod casting;
    pub(crate) mod layering;
    pub(crate) mod state_based;
    pub(crate) mod turn;
}

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Deck is too small")]
    TooSmallDeck,
    #[error("No deck provided")]
    NoDeck,
}

pub struct Context {
    pub game: Game,
    pub(crate) controller: Controller,
    config: Config,
}

impl Context {
    pub(crate) fn new(controller: Controller, config: Config, objects: Objects) -> Self {
        Context {
            game: Game::new(objects),
            controller,
            config,
        }
    }

    /// Start the game: performs initial draws and basic checks
    pub fn start(&mut self) -> Result<(), EngineError> {
        todo!()
    }

    /// Pass priority to the other player
    pub fn pass_priority(&mut self) {
        use crate::rules::player::PlayerId;
        self.game.last_non_passed_priority = self.game.priority;
        self.game.priority = match self.game.priority {
            PlayerId::PlayPlayer => PlayerId::DrawPlayer,
            PlayerId::DrawPlayer => PlayerId::PlayPlayer,
        };
    }
}
