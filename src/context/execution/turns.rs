use crate::context::Context;
use crate::rules::ability::static_ability::FixedAbilityGroup;
use crate::rules::condition::Condition;
use crate::rules::game_action::GameAction;
use crate::rules::player::PlayerId;
use crate::rules::player_action::PlayerAction;
use crate::rules::turn::{Step, TURN_STEPS};

impl Context {
    pub(crate) fn game_loop(&mut self) {
        while self.result.is_none() {
            let action = self
                .controller
                .choose_action(&self.game, self.possible_actions());

            match action {
                PlayerAction::PassPriority => self.pass_player_priority(),
                PlayerAction::PlayCard(id) => self.play_card(id),
                PlayerAction::ActivateAbility(id) => self.activate_ability(id),
            }
            self.redo_layering();
            self.exec_state_based();
            self.redo_layering();
        }
    }

    fn possible_actions(&self) -> Vec<PlayerAction> {
        let mut ids = match self.game.priority {
            PlayerId::DrawPlayer => self
                .game
                .objects
                .draw_hand
                .keys()
                .map(|k| k.into())
                .collect::<Vec<_>>(),
            PlayerId::PlayPlayer => self
                .game
                .objects
                .play_hand
                .keys()
                .map(|k| k.into())
                .collect::<Vec<_>>(),
        };

        for (id, obj) in self.game.objects.iter() {
            if !obj.characteristics.activated_abilities.is_empty() {
                ids.push(id);
            }
        }

        let mut actions = Vec::with_capacity(ids.len() + 1);
        for id in ids {
            actions.push(PlayerAction::ActivateAbility(id));
        }
        actions.push(PlayerAction::PassPriority);
        actions
    }

    pub(crate) fn pass_player_priority(&mut self) {
        let next_priority = match self.game.priority {
            PlayerId::PlayPlayer => PlayerId::DrawPlayer,
            PlayerId::DrawPlayer => PlayerId::PlayPlayer,
        };
        if next_priority == self.game.last_non_passed_priority {
            if self.stack_is_empty() {
                self.advance_step();
            } else {
                self.game.priority = self.game.active_player;
                self.game.last_non_passed_priority = self.game.active_player;
            }
        } else {
            self.game.priority = next_priority;
        }
    }

    fn advance_step(&mut self) {
        // Until end of ...-step effects expire (previous step)
        loop {
            if self.game.queued_steps.is_empty() {
                self.game.queued_steps.extend(TURN_STEPS);
            }
            let next_step = self.game.queued_steps.pop_front().unwrap();
            self.game.current_step = next_step;
            self.game.current_phase = next_step.phase();

            // Until ...-step effects expire (upcoming step)
            // At the beginning of ...-step triggers are added to pending
            let mut end = self.game.continuous_effects.len();
            let mut i = 0;
            while i < end {
                let effect = &self.game.continuous_effects[i];
                match effect {
                    FixedAbilityGroup::FixedContinuous(items) => {
                        if matches!(items[0].end, Condition::EndOfTurn) {
                            self.game.continuous_effects.swap_remove(i);
                            end -= 1;
                        }
                    }
                    FixedAbilityGroup::FixedReplacement(_) => i += 1,
                }
            }
            self.exec_turn_based_actions();

            // Untap and cleanup have no priority window.  Cleanup is the
            // boundary between turns, so the other player becomes active
            // before the next turn's untap step.
            if next_step == Step::Cleanup {
                self.game.active_player = match self.game.active_player {
                    PlayerId::PlayPlayer => PlayerId::DrawPlayer,
                    PlayerId::DrawPlayer => PlayerId::PlayPlayer,
                };
            }
            if next_step == Step::Untap || next_step == Step::Cleanup {
                continue;
            }

            self.game.priority = self.game.active_player;
            self.game.last_non_passed_priority = self.game.active_player;
            break;
        }
    }

    fn stack_is_empty(&self) -> bool {
        self.game.objects.spell_stack.is_empty() && self.game.objects.ability_stack.is_empty()
    }

    fn exec_turn_based_actions(&mut self) {
        let active_player = self.game.active_player;
        match self.game.current_step {
            Step::Draw => {
                let draw_action = GameAction::DrawCards {
                    player: active_player,
                    amount: 1,
                };
                self.execute(vec![draw_action]);
            }
            Step::Untap => {
                let untap_action = GameAction::Untap {
                    objects: self
                        .game
                        .objects
                        .battlefield
                        .iter()
                        .filter_map(|(id, (info, _))| {
                            if info.controller == active_player {
                                Some(id)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>(),
                };
                self.execute(vec![untap_action]);
            }
            Step::Cleanup => {
                let hand_size = match active_player {
                    PlayerId::DrawPlayer => self.game.objects.draw_hand.len(),
                    PlayerId::PlayPlayer => self.game.objects.play_hand.len(),
                };
                if hand_size > 7 {
                    let action = GameAction::Discard {
                        player: active_player,
                        amount: (hand_size - 7..hand_size - 6).into(),
                    };
                    self.execute(vec![action]);
                }

                let remove_damage_action = GameAction::RemoveDamage {
                    player: active_player,
                };
                self.execute(vec![remove_damage_action]);
            }
            Step::BeginningOfCombat
            | Step::CombatDamage
            | Step::DeclareAttackers
            | Step::DeclareBlockers
            | Step::EndOfCombat => {
                // todo!("Combat not yet implemented");
            }
            Step::EndStep | Step::MainStep | Step::Upkeep => {}
        }
    }
}
