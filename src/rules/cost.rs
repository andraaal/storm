#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Cost {
    pub(crate) mana_cost: ManaCost,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ManaCost {
    pub(crate) generic: u32,
    pub(crate) red: u32,
    pub(crate) green: u32,
    pub(crate) blue: u32,
    pub(crate) white: u32,
    pub(crate) black: u32,
    pub(crate) colorless: u32,
}
