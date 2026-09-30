use crate::rules::cost::ManaAmount;

pub struct Player {
    pub life: i32,
    pub mana: ManaAmount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerId {
    PlayPlayer,
    DrawPlayer,
}

impl Player {
    pub(crate) fn new() -> Self {
        Player {
            life: 20,
            mana: ManaAmount::default(),
        }
    }
}
