pub(crate) struct Mana {
    pub(crate) color: ManaColor,
    pub(crate) amount: u32,
}

pub(crate) enum ManaColor {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}

pub(crate) struct ManaCost {
    pub(crate) generic: u32,
    pub(crate) colored: Vec<ManaColor>,
}
