use crate::context::Context;
use crate::rules::game_action::GameAction;
use crate::rules::player::PlayerId;
use crate::rules::turn::{Step, TURN_STEPS};

impl Context {
    pub(crate) fn pass_player_priority(&mut self) {
        self.game.priority = match self.game.priority {
            PlayerId::PlayPlayer => PlayerId::DrawPlayer,
            PlayerId::DrawPlayer => PlayerId::PlayPlayer,
        };
        if self.game.last_non_passed_priority == self.game.priority {
            if !(self.game.objects.spell_stack.is_empty()
                && self.game.objects.ability_stack.is_empty())
            {
                // Resolve top of stack
            } else {
                self.advance_step();
            }
        }
    }

    fn advance_step(&mut self) {
        // Until end of ...-step effects expire (previous step)

        if self.game.queued_steps.is_empty() {
            self.game.queued_steps.extend_from_slice(&TURN_STEPS);
        }
        let next_step = self.game.queued_steps.pop().unwrap();
        self.game.current_step = next_step;
        self.game.current_phase = next_step.phase();

        // Until ...-step effects expire (upcoming step)
        // At the beginning of ...-step tiggers are added to pending
        // Do turn-based actions
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
                    objects: self.game.objects.battlefield.keys().collect::<Vec<_>>(),
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
                        amount: (hand_size..hand_size + 1).into(),
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
                todo!("Combat not yet implemented");
            }
            Step::EndStep | Step::MainStep | Step::Upkeep => {}
        }
    }
}
