use crate::{
    context::Context,
    rules::{
        ability::static_ability::DynamicAbilityGroup,
        layer::{LAYER_ORDER, Layer},
    },
};

impl Context {
    pub(crate) fn redo_layering(&mut self) {
        // First reset the layering info on all objects
        for obj in self.game.objects_mut() {
            obj.reset_characteristics();
        }

        // Actually begin layering
        let mut saved_effects = Vec::new();

        for layer in LAYER_ORDER {
            for obj in self.game.objects_mut() {
                saved_effects.extend(
                    obj.characteristics
                        .static_abilities
                        .iter()
                        .filter(|&s| {
                            s.ability.layers.contains(layer)
                                && s.ability.layers.bits() & ((layer as u16) - 1) == 0
                        })
                        .cloned(),
                );

                // Snapshot copyable effects (Control Changing Layer is directly after the copyable effects)
                if layer == Layer::ControlChanging {
                    obj.copy_characteristics = obj.characteristics.clone();
                }
            }

            for static_effect in &saved_effects {
                let DynamicAbilityGroup::DynamicContinuous(effects) =
                    &static_effect.ability.ability_group
                else {
                    continue;
                };

                for effect in effects.iter() {
                    if effect.layer == layer {
                        // apply effect
                        (effect.effect)(self, static_effect.source);
                    }
                }
            }
        }
    }
}
