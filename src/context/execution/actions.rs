use crate::game::object_iter::GameObjectIterExt;
use crate::rules::ability::static_ability::IntrinsicAbility;
use crate::rules::id::{AnyId, BattlefieldId};
use crate::rules::object::game_object::GameObject;
use crate::rules::player::PlayerId;
use crate::rules::target::damage::{Damageable, DamageableTarget};
use crate::rules::zone::{BattlefieldInfo, ExileInfo};
use crate::{context::Context, rules::game_action::GameAction};

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
            GameAction::MoveToBattlefield { objects } => {
                self.move_to_battlefield(objects);
            }
            GameAction::MoveToExile { objects } => {
                self.move_to_exile(objects);
            }
            GameAction::MoveToHand { objects } => {
                self.move_to_hand(objects);
            }
            GameAction::MoveToLibrary { objects } => {
                self.move_to_library(objects);
            }
            GameAction::MoveToGraveyard { objects } => {
                self.move_to_graveyard(objects);
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

    fn discard(&mut self, player: PlayerId, amount: std::range::Range<usize>) {
        let action;
        match player {
            PlayerId::DrawPlayer => {
                let keys = self.game.objects.draw_hand.keys();
                let ids = keys.collect::<Vec<_>>();
                let chosen = self.controller.choose_draw_hand(&self.game, ids, amount);
                action = GameAction::MoveToGraveyard {
                    objects: chosen.into_iter().map(|i| i.into()).collect(),
                };
            }
            PlayerId::PlayPlayer => {
                let keys = self.game.objects.play_hand.keys();
                let ids = keys.collect::<Vec<_>>();
                let chosen = self.controller.choose_play_hand(&self.game, ids, amount);
                action = GameAction::MoveToGraveyard {
                    objects: chosen.into_iter().map(|i| i.into()).collect(),
                };
            }
        }
        self.execute_single(action);
    }

    fn remove_damage(&mut self, player: PlayerId) {
        let objs = self
            .game
            .objects
            .battlefield
            .values_mut()
            .controlled_by::<&mut (BattlefieldInfo, GameObject)>(player);
        objs.battlefield_mut().for_each(|b| b.marked_damage = 0);
    }

    fn tap_untap(&mut self, ids: Vec<BattlefieldId>, tap: bool) {
        for id in ids {
            let btf = self.game.objects.battlefield.get_mut(id);
            match btf {
                Some((info, _)) => {
                    info.tapped = tap;
                }
                None => {
                    println!("Invalid/Expired Object (tap/untap)");
                }
            }
        }
    }

    fn draw(&mut self, player: PlayerId, amount: u32) {
        let amount = amount as usize;
        let action = match player {
            PlayerId::DrawPlayer => {
                let top = self
                    .game
                    .objects
                    .draw_hand
                    .iter()
                    .take(amount)
                    .map(|i| i.0.into())
                    .collect::<Vec<_>>();
                if top.len() < amount {
                    println!(
                        "Draw player lost; they drew {} cards, while having {amount} cards",
                        top.len()
                    )
                }
                GameAction::MoveToHand { objects: top }
            }
            PlayerId::PlayPlayer => {
                let top = self
                    .game
                    .objects
                    .play_hand
                    .iter()
                    .take(amount)
                    .map(|i| i.0.into())
                    .collect::<Vec<_>>();
                if top.len() < amount {
                    println!(
                        "Play player lost; they drew {} cards, while having {amount} cards",
                        top.len()
                    )
                }
                GameAction::MoveToHand { objects: top }
            }
        };
        self.execute(vec![action]);
    }

    fn move_to_battlefield(&mut self, objects: Vec<(AnyId, BattlefieldInfo)>) {
        let next_timestamp = self.game.generate_timestamp();

        for (id, info) in objects {
            self.game
                .objects
                .move_to_battlefield(id, info, next_timestamp);
        }
    }

    fn move_to_exile(&mut self, objects: Vec<(AnyId, ExileInfo)>) {
        let next_timestamp = self.game.generate_timestamp();

        for (id, info) in objects {
            self.game.objects.move_to_exile(id, info, next_timestamp);
        }
    }

    fn move_to_hand(&mut self, objects: Vec<AnyId>) {
        let next_timestamp = self.game.generate_timestamp();

        for id in objects {
            self.game.objects.move_to_hand(id, next_timestamp);
        }
    }

    fn move_to_graveyard(&mut self, objects: Vec<AnyId>) {
        let next_timestamp = self.game.generate_timestamp();

        for id in objects {
            self.game.objects.move_to_graveyard(id, next_timestamp);
        }
    }

    fn move_to_library(&mut self, objects: Vec<AnyId>) {
        let next_timestamp = self.game.generate_timestamp();

        for id in objects {
            self.game.objects.move_to_library(id, next_timestamp);
        }
    }

    fn damage(&mut self, targets: Vec<(u32, DamageableTarget)>, source_id: AnyId) {
        let mut lifelink: Option<PlayerId> = None;
        let mut deathtouch = false;
        let mut wither = false;
        let mut infect = false;
        if let Some(source) = self.game.objects.get_any(source_id) {
            let characs = &source.characteristics;
            if characs.has_intrinsic_effect(IntrinsicAbility::Lifelink) {
                let cao = self.game.objects.get_controller_and_owner(source_id);
                lifelink = cao.0.or(cao.1);
            }
            if characs.has_intrinsic_effect(IntrinsicAbility::Deathtouch) {
                deathtouch = true;
            }
            if characs.has_intrinsic_effect(IntrinsicAbility::Wither) {
                wither = true;
            }
            if characs.has_intrinsic_effect(IntrinsicAbility::Infect) {
                infect = true;
            }
        } else {
            println!("Invalid/Expired object (damage)");
        }
        for (amount, target) in targets {
            if let Some(id) = lifelink {
                self.game.players.get_mut(id).life += amount as i32;
            }

            target.apply_damage(self, amount, wither, infect, deathtouch);
        }
    }
}
