#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cost {
    pub mana_cost: ManaCost,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManaCost {
    pub generic: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub white: u8,
    pub black: u8,
    pub colorless: u8,
}
