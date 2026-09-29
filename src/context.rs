use thiserror::Error;

use crate::{
    context::controller::Controller,
    game::{Game, objects::Objects},
};

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Winner(crate::rules::player::PlayerId),
}

pub struct Context {
    pub game: Game,
    pub(crate) controller: Controller,
    result: Option<GameResult>,
}

impl Context {
    pub(crate) fn new(controller: Controller, objects: Objects) -> Self {
        Context {
            game: Game::new(objects),
            controller,
            result: None,
        }
    }

    /// Start the game: performs initial draws and basic checks
    pub fn start(&mut self) -> Result<GameResult, EngineError> {
        if let Some(res) = self.result {
            return Ok(res);
        }

        self.game.shuffle_libraries();
        self.draw(crate::rules::player::PlayerId::PlayPlayer, 7);
        self.draw(crate::rules::player::PlayerId::DrawPlayer, 7);
        self.game_loop();
        Ok(self.result.unwrap())
    }
}
