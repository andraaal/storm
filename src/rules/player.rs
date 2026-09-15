pub(crate) struct Player {
    pub(crate) life: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PlayerId {
    PlayPlayer,
    DrawPlayer,
}

impl Player {
    pub(crate) fn new() -> Self {
        Player { life: 20 }
    }
}
