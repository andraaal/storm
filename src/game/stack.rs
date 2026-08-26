use crate::{
    game::Game,
    rules::{
        game_action::GameAction,
        id::StackId,
        object::{
            stack_object::{StackObject, StackObjectKind},
            types::ObjectType,
        },
        zone::Zone,
    },
};

/// Legacy implementation. Only here for reference. The new implementation is in context/stack.rs

impl Game {
    /// Resolves the top object of the stack. Does these steps:
    /// 1. Checks if all targets are still valid. (Maybe already make them None here)
    /// 2. Check intervening ifs for triggered abilities.
    pub(crate) fn resolve_top_of_stack(&mut self) {
        let stack_obj = self
            .stack
            .last()
            .expect("Mustn't call resolve_top_of_stack when stack is empty");
        let kind = stack_obj.kind.clone();
        let stack_id = stack_obj.id;

        if !self.validate_targets(stack_obj) {
            todo!();
            // Put the spell into the graveyard if all targets became invalid.
            return;
        }

        match kind {
            StackObjectKind::Spell(obj_id) => {
                let obj = self
                    .objects
                    .get(obj_id)
                    .expect("Stack object refers to non-existing object");

                // If the destination after resolving isn't the same for all the types it has, that is a bug in the code that creates them. We are free to assume they are the same here.
                match obj.characteristics.types.first() {
                    Some(ObjectType::Instant { effects, .. })
                    | Some(ObjectType::Sorcery { effects, .. }) => {
                        // Clone the effects to avoid borrowing issues when executing them.
                        let copy = effects.clone();
                        for effect in copy.iter() {
                            effect(self, stack_id);
                        }
                        self.execute_actions(vec![GameAction::MoveToZone {
                            object: obj_id,
                            to_zone: Zone::Graveyard,
                        }]);
                    }
                    Some(ObjectType::Artifact { .. })
                    | Some(ObjectType::Creature { .. })
                    | Some(ObjectType::Enchantment { .. })
                    | Some(ObjectType::Planeswalker { .. }) => {
                        self.execute_actions(vec![GameAction::MoveToZone {
                            object: obj_id,
                            to_zone: Zone::Battlefield,
                        }]);
                    }
                    _ => {
                        panic!(
                            "Tried to resolve illegal object on the stack: {:?}. It is either a land or has no type at all",
                            obj.id
                        );
                    }
                }
            }
            StackObjectKind::TriggeredAbility(id, resolver) => {
                let triggered_ability = self
                    .triggered_effects
                    .get(id)
                    .expect("Stack object refers to non-existing triggered ability");
                resolver(self, stack_id);
            }
            StackObjectKind::ActivatedAbility(func) => {
                func(self, stack_id);
            }
        }
    }

    fn validate_targets(&self, stack_obj: &StackObject) -> bool {
        if stack_obj.targets.is_empty() {
            return true;
        }
        if stack_obj.targets.len() != stack_obj.target_validators.len() {
            eprintln!("Target validation error: mismatch between targets and validators");
            return false;
        }
        for (target, validator) in stack_obj
            .targets
            .iter()
            .zip(stack_obj.target_validators.iter())
        {
            if validator(self, *target) {
                return true;
            }
        }

        false
    }

    pub(crate) fn get_stack_object(&self, id: StackId) -> &StackObject {
        self.stack
            .iter()
            .find(|obj| obj.id == id)
            .expect("Stack object with given id not found")
    }

    pub(crate) fn get_stack_object_mut(&mut self, id: StackId) -> &mut StackObject {
        self.stack
            .iter_mut()
            .find(|obj| obj.id == id)
            .expect("Stack object with given id not found")
    }
}
