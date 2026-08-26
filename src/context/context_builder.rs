use crate::{
    context::{Context, EngineError, config::Config, controller::Input},
    rules::object::const_characteristics::ConstCharacteristics,
};

pub struct ContextBuilder {
    controller: Box<dyn Input>,
    config: Option<Config>,
    play_deck: Vec<&'static ConstCharacteristics>,
    draw_deck: Vec<&'static ConstCharacteristics>,
}

impl ContextBuilder {
    pub fn new(controller: Box<dyn Input>) -> Self {
        ContextBuilder {
            controller,
            config: None,
            play_deck: Vec::new(),
            draw_deck: Vec::new(),
        }
    }

    pub fn with_config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    pub fn build(self) -> Result<Context, EngineError> {
        let config = self.config.unwrap_or_default();
        let ctx = Context::new(self.controller, config);

        if self.play_deck.is_empty() || self.draw_deck.is_empty() {
            return Err(EngineError::NoDeck);
        }

        if self.play_deck.len() < 7 || self.draw_deck.len() < 7 {
            return Err(EngineError::TooSmallDeck);
        }

        // Add the decks to the players

        Ok(ctx)
    }
}
