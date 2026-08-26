use crate::rules::id::ObjectId;

pub(crate) struct Player {
    pub(crate) life: i32,
    pub(crate) hand: Vec<ObjectId>,
    pub(crate) library: Vec<ObjectId>,
    pub(crate) graveyard: Vec<ObjectId>,
    pub(crate) exile: Vec<ObjectId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PlayerId {
    PlayPlayer,
    DrawPlayer,
}

impl Player {
    pub(crate) fn new() -> Self {
        Player {
            life: 20,
            hand: Vec::new(),
            library: Vec::new(),
            graveyard: Vec::new(),
            exile: Vec::new(),
        }
    }

    pub(crate) fn add_to_hand(&mut self, card: ObjectId) {
        self.hand.push(card);
    }

    pub(crate) fn remove_from_hand(&mut self, card: ObjectId) -> Result<(), ()> {
        if let Some(pos) = self.hand.iter().position(|&x| x == card) {
            self.hand.remove(pos);
            Ok(())
        } else {
            Err(())
        }
    }

    pub(crate) fn add_to_library(&mut self, card: ObjectId) {
        self.library.push(card);
    }

    pub(crate) fn remove_from_library(&mut self, card: ObjectId) -> Result<(), ()> {
        if let Some(pos) = self.library.iter().position(|&x| x == card) {
            self.library.remove(pos);
            Ok(())
        } else {
            Err(())
        }
    }

    pub(crate) fn add_to_graveyard(&mut self, card: ObjectId) {
        self.graveyard.push(card);
    }

    pub(crate) fn remove_from_graveyard(&mut self, card: ObjectId) -> Result<(), ()> {
        if let Some(pos) = self.graveyard.iter().position(|&x| x == card) {
            self.graveyard.remove(pos);
            Ok(())
        } else {
            Err(())
        }
    }

    pub(crate) fn add_to_exile(&mut self, card: ObjectId) {
        self.exile.push(card);
    }

    pub(crate) fn remove_from_exile(&mut self, card: ObjectId) -> Result<(), ()> {
        if let Some(pos) = self.exile.iter().position(|&x| x == card) {
            self.exile.remove(pos);
            Ok(())
        } else {
            Err(())
        }
    }
}
