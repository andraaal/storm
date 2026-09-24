use crate::rules::id::AnyId;

#[derive(Clone, PartialEq, Eq)]
pub enum PlayerAction {
    PassPriority,
    PlayCard(AnyId),
}
