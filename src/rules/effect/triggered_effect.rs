// Legacy implementation; for reference only

// use std::{borrow::Cow, rc::Rc};

// use crate::{
//     game::Game,
//     rules::{condition::Condition, game_action::GameAction, id::AnyId, target::AnyTarget},
// };

// #[derive(Clone)]
// pub(crate) struct TriggeredEffect {
//     pub(crate) source: AnyId,
//     pub(crate) trigger: Rc<dyn Fn(&Game, &GameAction) -> Option<StackObject>>,
//     pub(crate) end: Condition,
//     pub(crate) remaining_uses: Uses,
//     pub(crate) intervening_if: Option<Rc<dyn Fn(&Game) -> bool>>,
//     pub(crate) target_selectors: Cow<'static, [fn(&Game, AnyTarget) -> bool]>,
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub(crate) enum Uses {
//     Unlimited,
//     Limited(u32),
//     LimitedPerTurn(u32),
// }

// impl std::fmt::Debug for TriggeredEffect {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("TriggeredEffect")
//             .field("source", &self.source)
//             .field("end", &self.end)
//             .finish()
//     }
// }

// impl TriggeredEffect {
//     pub(crate) fn new(
//         source: AnyId,
//         trigger: Rc<dyn Fn(&Game, &GameAction) -> Option<StackObject>>,
//         end: Condition,
//         remaining_uses: Uses,
//         intervening_if: Option<Rc<dyn Fn(&Game) -> bool>>,
//         target_selectors: Cow<'static, [fn(&Game, AnyTarget) -> bool]>,
//     ) -> Self {
//         TriggeredEffect {
//             source,
//             trigger,
//             end,
//             remaining_uses,
//             intervening_if,
//             target_selectors,
//         }
//     }
// }

// impl Uses {
//     pub(crate) fn decrement(&mut self) {
//         match self {
//             Uses::Unlimited => {}
//             Uses::Limited(n) => {
//                 if *n > 0 {
//                     *n -= 1;
//                 }
//             }
//             Uses::LimitedPerTurn(n) => {
//                 if *n > 0 {
//                     *n -= 1;
//                 }
//             }
//         }
//     }

//     pub(crate) fn is_exhausted(&self) -> bool {
//         match self {
//             Uses::Unlimited => false,
//             Uses::Limited(n) => *n == 0,
//             Uses::LimitedPerTurn(n) => *n == 0,
//         }
//     }
// }
