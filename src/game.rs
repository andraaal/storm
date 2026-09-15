use std::ops::{Index, IndexMut};

use crate::game::objects::Objects;
use crate::rules::ability::static_ability::FixedAbilityGroup;
use crate::rules::effect::replacement_effect::ReplacementEffect;
use crate::rules::id::Timestamp;

use crate::rules::player::Player;
use crate::rules::player::PlayerId;
use crate::rules::turn::{Phase, Step};

pub(crate) mod object_iter;
pub(crate) mod objects;
mod stack;

pub(crate) struct Game {
    pub(crate) players: Players,
    pub(crate) objects: Objects,

    pub(crate) continuous_effects: Vec<FixedAbilityGroup>,
    pub(crate) replacement_effects: Vec<ReplacementEffect>,

    pub(crate) current_timestamp: Timestamp,
    pub(crate) current_step: Step,
    pub(crate) current_phase: Phase,
    pub(crate) queued_steps: Vec<Step>,
    pub(crate) priority: PlayerId,
    pub(crate) last_non_passed_priority: PlayerId,
    pub(crate) active_player: PlayerId,
}

impl Game {
    pub(crate) fn new(objects: Objects) -> Self {
        Self {
            players: Players::new(),
            objects,
            continuous_effects: Vec::new(),
            replacement_effects: Vec::new(),
            current_timestamp: Timestamp::new(),
            current_step: Step::Upkeep,
            current_phase: Phase::Beginning,
            queued_steps: Vec::new(),
            priority: PlayerId::PlayPlayer,
            last_non_passed_priority: PlayerId::DrawPlayer,
            active_player: PlayerId::PlayPlayer,
        }
    }

    pub(crate) fn generate_timestamp(&mut self) -> Timestamp {
        let time = self.current_timestamp;
        self.current_timestamp.increment();

        time
    }

    pub(crate) fn register_fixed_effect(&mut self, effect: FixedAbilityGroup) {
        self.continuous_effects.push(effect);
    }

    pub(crate) fn register_replacement_effect(&mut self, effect: ReplacementEffect) {
        self.replacement_effects.push(effect);
    }
}

pub(crate) struct Players {
    pub(crate) play_player: Player,
    pub(crate) draw_player: Player,
}

impl Players {
    pub(crate) fn new() -> Self {
        Self {
            play_player: Player::new(),
            draw_player: Player::new(),
        }
    }

    pub(crate) fn get(&self, player_id: PlayerId) -> &Player {
        match player_id {
            PlayerId::PlayPlayer => &self.play_player,
            PlayerId::DrawPlayer => &self.draw_player,
        }
    }

    pub(crate) fn get_mut(&mut self, player_id: PlayerId) -> &mut Player {
        match player_id {
            PlayerId::PlayPlayer => &mut self.play_player,
            PlayerId::DrawPlayer => &mut self.draw_player,
        }
    }
}

impl Index<PlayerId> for Players {
    type Output = Player;

    fn index(&self, index: PlayerId) -> &Self::Output {
        self.get(index)
    }
}

impl IndexMut<PlayerId> for Players {
    fn index_mut(&mut self, index: PlayerId) -> &mut Self::Output {
        self.get_mut(index)
    }
}
