pub(crate) mod ability {
    pub(crate) mod activated_ability;
    pub(crate) mod behaviour;
    pub(crate) mod effect;
    pub(crate) mod static_ability;
    pub(crate) mod triggered_ability;
}

pub(crate) mod effect {
    pub(crate) mod replacement_effect;
    pub(crate) mod triggered_effect;
}

pub(crate) mod object {
    pub(crate) mod characteristics;
    pub(crate) mod const_characteristics;
    pub(crate) mod copy_characteristics;
    pub(crate) mod game_object;
    pub(crate) mod types;
}

pub(crate) mod condition;
pub(crate) mod cost;
pub(crate) mod game_action;
pub(crate) mod id;
pub(crate) mod layer;
pub(crate) mod player;
pub(crate) mod target;
pub(crate) mod turn;
pub(crate) mod zone;
