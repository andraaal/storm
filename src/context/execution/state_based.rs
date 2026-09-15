use crate::context::Context;

pub(crate) enum StateBasedActionResult {
    NoneExecuted,
    SomeExecuted,
}

impl Context {
    /// Checks for and executes all state-based actions, returning true if any were executed
    pub(crate) fn exec_state_based(&mut self) -> StateBasedActionResult {
        StateBasedActionResult::NoneExecuted
    }
}
