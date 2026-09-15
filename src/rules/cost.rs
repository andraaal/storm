#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Cost {
    pub(crate) mana_cost: ManaCost,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ManaCost {
    pub(crate) generic: u8,
    pub(crate) red: u8,
    pub(crate) green: u8,
    pub(crate) blue: u8,
    pub(crate) white: u8,
    pub(crate) black: u8,
    pub(crate) colorless: u8,
}
