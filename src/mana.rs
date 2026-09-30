pub struct Mana {
    pub color: ManaColor,
    pub amount: u32,
}

pub enum ManaColor {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}

pub struct ManaCost {
    pub generic: u32,
    pub colored: Vec<ManaColor>,
}
