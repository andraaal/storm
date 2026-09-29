pub struct Player {
    pub life: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerId {
    PlayPlayer,
    DrawPlayer,
}

impl Player {
    pub(crate) fn new() -> Self {
        Player { life: 20 }
    }
}
