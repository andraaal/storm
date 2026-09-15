use crate::{
    context::{
        Context, EngineError,
        config::Config,
        controller::{Controller, Input},
    },
    game::objects::Objects,
    rules::{
        id::Timestamp,
        object::{const_characteristics::ConstCharacteristics, game_object::GameObject},
    },
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

    pub fn with_config(&mut self, config: Config) {
        self.config = Some(config);
    }

    pub fn with_play_deck(&mut self, deck: Vec<&'static ConstCharacteristics>) {
        self.play_deck = deck;
    }

    pub fn with_draw_deck(&mut self, deck: Vec<&'static ConstCharacteristics>) {
        self.draw_deck = deck;
    }

    pub fn build(self) -> Result<Context, EngineError> {
        if self.play_deck.is_empty() || self.draw_deck.is_empty() {
            return Err(EngineError::NoDeck);
        }

        if self.play_deck.len() < 7 || self.draw_deck.len() < 7 {
            return Err(EngineError::TooSmallDeck);
        }

        let config = self.config.unwrap_or_default();
        let controller = Controller::new(self.controller);
        let objects = Objects::new(
            self.play_deck
                .into_iter()
                .map(|r| GameObject::new(r, Timestamp::new()))
                .collect(),
            self.draw_deck
                .into_iter()
                .map(|r| GameObject::new(r, Timestamp::new()))
                .collect(),
        );
        let ctx = Context::new(controller, config, objects);

        Ok(ctx)
    }
}
