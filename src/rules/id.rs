use slotmap::new_key_type;

new_key_type! {
    pub(crate)struct ObjectId;
    pub(crate)struct StackId;
    pub(crate)struct TriggeredEffectId;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Timestamp(u64);

impl Timestamp {
    pub(crate) fn new() -> Self {
        Timestamp(0)
    }

    pub(crate) fn increment(&mut self) {
        self.0 += 1;
    }
}
