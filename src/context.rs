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
    pub(crate) mod turns;
}

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Deck is too small")]
    TooSmallDeck,
    #[error("No deck provided")]
    NoDeck,
    #[error("Deck contains an unsupported card")]
    UnsupportedCard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Winner(crate::rules::player::PlayerId),
}

pub struct Context {
    pub game: Game,
    pub(crate) controller: Controller,
    config: Config,
    result: Option<GameResult>,
}

impl Context {
    pub(crate) fn new(
        controller: Controller,
        config: Config,
        objects: Objects,
        seed: Option<u64>,
    ) -> Self {
        Context {
            game: Game::new(objects, seed),
            controller,
            config,
            result: None,
        }
    }

    /// Start the game: performs initial draws and basic checks
    pub fn start(&mut self) -> Result<(), EngineError> {
        if self.result.is_some() {
            return Ok(());
        }

        self.game.shuffle_libraries();
        self.draw(crate::rules::player::PlayerId::PlayPlayer, 7);
        self.draw(crate::rules::player::PlayerId::DrawPlayer, 7);
        self.request_priority();
        Ok(())
    }

    pub fn result(&self) -> Option<GameResult> {
        self.result
    }
}
