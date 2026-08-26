use crate::{
    game::Game,
    rules::{
        id::{ObjectId, Timestamp},
        object::game_object::GameObject,
        player::PlayerId,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Target {
    StackObject(ObjectId, Timestamp),
    Object(ObjectId, Timestamp),
    Player(PlayerId),
}

impl Target {
    pub(crate) fn validate(&self, game: &Game) -> bool {
        match self {
            Target::StackObject(obj_id, timestamp) => {
                if let Some(obj) = game.objects.get(*obj_id) {
                    obj.timestamp == *timestamp
                } else {
                    false
                }
            }
            Target::Object(obj_id, timestamp) => {
                if let Some(obj) = game.objects.get(*obj_id) {
                    obj.timestamp == *timestamp
                } else {
                    false
                }
            }
            Target::Player(_) => true,
        }
    }

    pub(crate) fn as_object_valid<'a>(&self, game: &'a mut Game) -> Option<&'a mut GameObject> {
        match self {
            Target::Object(obj_id, timestamp) => {
                if let Some(obj) = game.objects.get_mut(*obj_id) {
                    if obj.timestamp == *timestamp {
                        return Some(obj);
                    }
                }
                None
            }
            _ => None,
        }
    }
}
