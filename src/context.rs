use thiserror::Error;

use crate::{
    context::{config::Config, controller::Input},
    game::Game,
};

pub(crate) mod config;
pub(crate) mod context_builder;
pub(crate) mod controller;
pub(crate) mod execution {
    pub(crate) mod actions;
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
    controller: Box<dyn Input>,
    config: Config,
}

impl Context {
    pub(crate) fn new(controller: Box<dyn Input>, config: Config) -> Self {
        Context {
            game: Game::new(),
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
