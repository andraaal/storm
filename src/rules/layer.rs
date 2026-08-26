use flagset::{FlagSet, flags};

flags! {
    #[derive(PartialOrd)]
    pub(crate)enum Layer: u16 {
        CopyableValues,
        FaceDownValues,
        ControlChanging,
        TextChanging ,
        TypeChanging ,
        ColorChanging,
        AbilityChanging,
        SetBasePT,
        SetPT ,
        ChangePT ,
        ExchangePT,
    }
}

pub(crate) type Layers = FlagSet<Layer>;

pub const fn layers(layers: &[Layer]) -> Layers {
    let mut bits: u16 = 0;
    let mut i = 0;
    while i < layers.len() {
        bits |= layers[i] as u16;
        i += 1;
    }

    // SAFETY: The discriminant of every Layer is by definition a valid bit in the Layers bitset.
    unsafe { Layers::new_unchecked(bits) }
}

pub const LAYER_ORDER: [Layer; 11] = [
    Layer::CopyableValues,
    Layer::FaceDownValues,
    Layer::ControlChanging,
    Layer::TextChanging,
    Layer::TypeChanging,
    Layer::ColorChanging,
    Layer::AbilityChanging,
    Layer::SetBasePT,
    Layer::SetPT,
    Layer::ChangePT,
    Layer::ExchangePT,
];
