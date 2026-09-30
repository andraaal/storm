#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Beginning,
    Main,
    Combat,
    Ending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
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
    pub fn phase(&self) -> Phase {
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

pub const TURN_STEPS: [Step; 12] = [
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

pub const COMBAT_STEPS: [Step; 5] = [
    Step::BeginningOfCombat,
    Step::DeclareAttackers,
    Step::DeclareBlockers,
    Step::CombatDamage,
    Step::EndOfCombat,
];

pub const ENDING_STEPS: [Step; 2] = [Step::EndStep, Step::Cleanup];
pub const MAIN_STEPS: [Step; 1] = [Step::MainStep];
pub const BEGINNING_STEPS: [Step; 3] = [Step::Untap, Step::Upkeep, Step::Draw];
