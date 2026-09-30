#![allow(dead_code, private_interfaces)]
#![feature(checked_type_aliases)]
#![feature(min_specialization)]

mod cards;
mod context;
mod game;
pub(crate) mod mana;
pub(crate) mod nested_borrow;
mod rules;

pub use cards::cards::basic_lands::{FOREST, MOUNTAIN, PLAINS};
pub use cards::cards::{
    anthem_of_champions::ANTHEM_OF_CHAMPIONS, bear_cub::BEAR_CUB, giant_growth::GIANT_GROWTH,
    lightning_bolt::LIGHTNING_BOLT,
};
pub use context::context_builder::ContextBuilder;
pub use context::controller::{Choice, Input};
pub use context::{Context, EngineError, GameResult};
pub use game::Game;
pub use game::objects::Objects;
pub use rules::id::{
    AbilityStackId, AnyId, BattlefieldId, DrawGraveyardId, DrawHandId, DrawLibraryId, ExileId,
    PlayGraveyardId, PlayHandId, PlayLibraryId, SpellStackId,
};
pub use rules::object::const_characteristics::ConstCharacteristics;
pub use rules::player::PlayerId;
