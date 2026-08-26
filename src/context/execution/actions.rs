use crate::game::object_iter::GameObjectIterExt;
use crate::rules::ability::static_ability::IntrinsicAbility;
use crate::rules::id::ObjectId;
use crate::rules::object::stack_object::StackObjectKind;
use crate::rules::player::PlayerId;
use crate::rules::zone::{BattlefieldInfo, ZoneId};
use crate::{
    context::Context,
    rules::{game_action::GameAction, target::Target, zone::Zone},
};

impl Context {
    /// Executes a list of GameActions, in the context of the game they happen simultaneously. Thus no state-based actions and no layering happens in between.
    /// Every GameAction itself is atomic, unless it issues further GameActions. These will be checked for replacement and triggered effects and then immediately executed.
    pub(crate) fn execute(&mut self, actions: Vec<GameAction>) {
        self.execute_multiple(actions);
    }

    fn execute_multiple(&mut self, actions: Vec<GameAction>) {
        for action in actions {
            self.execute_single(action);
        }
    }

    fn execute_single(&mut self, action: GameAction) {
        match action {
            GameAction::NoOp => {}
            GameAction::DealDamage { source, targets } => {
                self.damage(targets, source);
            }
            GameAction::MoveToZone { objects, to, from } => {
                self.move_to_zone(objects, to, from);
            }
            GameAction::DrawCards { player, amount } => {
                self.draw(player, amount);
            }
            GameAction::Tap { objects } => {
                self.tap_untap(objects, true);
            }
            GameAction::Untap { objects } => {
                self.tap_untap(objects, false);
            }
            GameAction::Discard { player, amount } => {
                self.discard(player, amount);
            }
            GameAction::RemoveDamage { player } => {
                self.remove_damage(player);
            }
        }
    }

    fn discard(
        &mut self,
        player: crate::rules::player::PlayerId,
        amount: std::range::Range<usize>,
    ) {
        let chosen = self.controller.choose_objects(
            &self.game,
            &self.game.players.get(player).hand,
            Zone::Hand { player },
            amount,
        );
        let action = GameAction::MoveToZone {
            objects: chosen,
            to: Zone::Graveyard { player },
            from: Zone::Hand { player },
        };
        self.execute_single(action);
    }

    fn remove_damage(&mut self, player: PlayerId) {
        self.game
            .get_objects_by_zone_mut(ZoneId::Battlefield)
            .controlled_by(player)
            .battlefield_mut()
            .for_each(|b| b.marked_damage = 0);
    }

    fn tap_untap(&mut self, objects: Vec<ObjectId>, tap: bool) {
        self.game
            .objects_from_ids_mut(objects)
            .battlefield_mut()
            .for_each(|b| b.tapped = tap);
    }

    fn draw(&mut self, player: crate::rules::player::PlayerId, amount: u32) {
        let top_cards: Vec<ObjectId> = self
            .game
            .get_objects_by_zone(ZoneId::Library(player))
            .take(amount as usize)
            .id()
            .collect();

        if top_cards.len() < amount as usize {
            panic!(
                "Player {:?} lost, because they tried to draw {} cards, but only had {} cards in their library.",
                player,
                amount,
                top_cards.len()
            );
        }

        let action = GameAction::MoveToZone {
            objects: top_cards,
            to: Zone::Hand { player },
            from: Zone::Library { player },
        };

        self.execute(vec![action]);
    }

    fn move_to_zone(&mut self, objects: Vec<ObjectId>, to: Zone, from: Zone) {
        let next_timestamp = self.game.generate_timestamp();
        for id in objects {
            if let Some(obj) = self.game.objects.get_mut(id) {
                // Remove the object from the index of all objects in its previous zone
                if obj.zone.kind() != from.kind() {
                    panic!(
                        "Object {:?} claimed it was in zone {:?}, but the action said it was in zone {:?} (move to zone)",
                        id,
                        obj.zone.kind(),
                        from.kind()
                    );
                }
                match obj.zone {
                    Zone::Battlefield(_) => {
                        if let Some(pos) = self.game.battlefield.iter().position(|&x| x == id) {
                            self.game.battlefield.swap_remove(pos);
                        } else {
                            panic!(
                                "Object {:?} claimed it was on the battlefield, but it wasn't found in the index (move to zone)",
                                id
                            );
                        }
                    }
                    Zone::Hand { player } => {
                        if let Err(()) = self.game.players.get_mut(player).remove_from_hand(id) {
                            panic!(
                                "Object {:?} claimed it was in the hand, but it wasn't found in the index (move to zone)",
                                id
                            );
                        }
                    }
                    Zone::Graveyard { player } => {
                        if let Err(()) = self.game.players.get_mut(player).remove_from_graveyard(id)
                        {
                            panic!(
                                "Object {:?} claimed it was in the graveyard, but it wasn't found in the index (move to zone)",
                                id
                            );
                        }
                    }
                    Zone::Exile { player } => {
                        if let Err(()) = self.game.players.get_mut(player).remove_from_exile(id) {
                            panic!(
                                "Object {:?} claimed it was in exile, but it wasn't found in the index (move to zone)",
                                id
                            );
                        }
                    }
                    Zone::Library { player } => {
                        if let Err(()) = self.game.players.get_mut(player).remove_from_library(id) {
                            panic!(
                                "Object {:?} claimed it was in the library, but it wasn't found in the index (move to zone)",
                                id
                            );
                        }
                    }
                    Zone::Stack { .. } => {
                        if let Some(pos) = self.game.stack.iter().position(
                        |x| matches!(x.kind, StackObjectKind::Spell (object_id) if object_id == id),
                    ) {
                        self.game.stack.remove(pos);
                    } else {
                        panic!(
                            "Object {:?} claimed it was on the stack, but it wasn't found in the index (move to zone)",
                            id
                        );
                    }
                    }
                }

                // Update the object itself
                obj.zone = to;
                obj.timestamp = next_timestamp;

                // Add the object to the index of all objects in its new zone
                match &to {
                    Zone::Battlefield { .. } => {
                        self.game.battlefield.push(id);
                    }
                    Zone::Hand { player } => {
                        self.game.players.get_mut(*player).add_to_hand(id);
                    }
                    Zone::Graveyard { player } => {
                        self.game.players.get_mut(*player).add_to_graveyard(id);
                    }
                    Zone::Exile { player } => {
                        self.game.players.get_mut(*player).add_to_exile(id);
                    }
                    Zone::Library { player } => {
                        self.game.players.get_mut(*player).add_to_library(id);
                    }
                    Zone::Stack { .. } => {
                        panic!(
                            "Please do not use the MoveToZone GameAction to move objects to the stack. Cast the spell them instead. (move to zone)"
                        )
                    }
                }
            }
        }
    }

    fn damage(&mut self, targets: Vec<(u32, Target)>, source: ObjectId) {
        for (amount, target) in targets {
            let mut deathtouch = false;
            let mut wither = false;
            let mut infect = false;
            if let Some(source_obj) = self.game.objects.get(source) {
                if source_obj
                    .characteristics
                    .has_intrinsic_effect(IntrinsicAbility::Lifelink)
                {
                    let player = source_obj.get_controller_or_owner();
                    self.game.players.get_mut(player).life += amount as i32;
                }
                if source_obj
                    .characteristics
                    .has_intrinsic_effect(IntrinsicAbility::Deathtouch)
                {
                    deathtouch = true;
                }
                if source_obj
                    .characteristics
                    .has_intrinsic_effect(IntrinsicAbility::Wither)
                {
                    wither = true;
                }
                if source_obj
                    .characteristics
                    .has_intrinsic_effect(IntrinsicAbility::Infect)
                {
                    infect = true;
                }
            } else {
                panic!("Don't execute game actions for objects that don't exist (damage)");
            }

            match target {
                Target::Player(id) => {
                    if infect {
                        todo!("Poison counters not yet implemented");
                    } else {
                        self.game.players.get_mut(id).life -= amount as i32;
                    }
                }
                Target::Object(id, timestamp) => {
                    if let Some(obj) = self.game.get_valid_object_mut(id, timestamp) {
                        if let Zone::Battlefield(BattlefieldInfo { marked_damage, .. }) =
                            &mut obj.zone
                        {
                            if deathtouch {
                                obj.damaged_by_deathtouch = true;
                            }
                            if wither || infect {
                                todo!("Wither and Infect not yet implemented");
                            } else {
                                *marked_damage += amount;
                            }
                        } else {
                            panic!(
                                "What the hell are you thinking!? You can't damage anything that's not on the field."
                            )
                        }
                    }
                }
                Target::StackObject(..) => {
                    panic!("Bruh, even worse. Trying to damage a spell on the stack??")
                }
            }
        }
    }
}
