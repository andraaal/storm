use crate::rules::id::AnyId;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PlayerAction {
    PassPriority,
    PlayCard(AnyId),
    ActivateAbility(AnyId),
}
