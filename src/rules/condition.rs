use std::rc::Rc;

#[derive(Clone)]
pub(crate) enum Condition {
    Never,
    EndOfTurn,
    Chained(Box<Condition>, Box<Condition>),
    Custom(Rc<dyn Fn(&mut crate::game::Game) -> bool>),
}

impl std::fmt::Debug for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::EndOfTurn => write!(f, "EndOfTurn"),
            Condition::Chained(one, two) => write!(f, "Chained: {:?}, {:?}", one, two),
            Condition::Custom(_) => write!(f, "Custom"),
            Condition::Never => write!(f, "Never"),
        }
    }
}
