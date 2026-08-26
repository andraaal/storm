#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Phase {
    Beginning,
    Main,
    Combat,
    Ending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Step {
    Untap,
    Upkeep,
    Draw,
    MainStep,
    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    EndOfCombat,
    EndStep,
    Cleanup,
}

impl Step {
    pub(crate) fn phase(&self) -> Phase {
        match self {
            Step::Untap => Phase::Beginning,
            Step::Upkeep => Phase::Beginning,
            Step::Draw => Phase::Beginning,
            Step::MainStep => Phase::Main,
            Step::BeginningOfCombat => Phase::Combat,
            Step::DeclareAttackers => Phase::Combat,
            Step::DeclareBlockers => Phase::Combat,
            Step::CombatDamage => Phase::Combat,
            Step::EndOfCombat => Phase::Combat,
            Step::EndStep => Phase::Ending,
            Step::Cleanup => Phase::Ending,
        }
    }
}

pub(crate) const TURN_STEPS: [Step; 12] = [
    Step::Untap,
    Step::Upkeep,
    Step::Draw,
    Step::MainStep,
    Step::BeginningOfCombat,
    Step::DeclareAttackers,
    Step::DeclareBlockers,
    Step::CombatDamage,
    Step::EndOfCombat,
    Step::MainStep,
    Step::EndStep,
    Step::Cleanup,
];

pub(crate) const COMBAT_STEPS: [Step; 5] = [
    Step::BeginningOfCombat,
    Step::DeclareAttackers,
    Step::DeclareBlockers,
    Step::CombatDamage,
    Step::EndOfCombat,
];

pub(crate) const ENDING_STEPS: [Step; 2] = [Step::EndStep, Step::Cleanup];
pub(crate) const MAIN_STEPS: [Step; 1] = [Step::MainStep];
pub(crate) const BEGINNING_STEPS: [Step; 3] = [Step::Untap, Step::Upkeep, Step::Draw];
