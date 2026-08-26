use std::borrow::Cow;

use flagset::FlagSet;

use crate::rules::cost::{Cost, ManaCost};
use crate::rules::effect::continuous_effect::{ContinuousEffect, SpellContinuousEffect};
use crate::rules::object::{const_characteristics::ConstCharacteristics, types::ObjectType};
use crate::rules::target::Target;

pub(crate) const GIANT_GROWTH: ConstCharacteristics = ConstCharacteristics {
    name: "Giant Growth",
    casting_cost: Cost {
        mana_cost: ManaCost {
            generic: 0,
            red: 0,
            green: 1,
            blue: 0,
            white: 0,
            black: 0,
            colorless: 0,
        },
    },
    types: &[ObjectType::Instant {
        subtypes: Cow::Borrowed(&[]),
        effects: Cow::Borrowed(&[|game, id| {
            let stmp = game.generate_timestamp();
            let stack_obj = game.get_stack_object(id);
            let target = stack_obj.targets[0];
            game.register_continuous_effect(ContinuousEffect::Static(SpellContinuousEffect {
                timestamp: stmp,
                targets: vec![target],
                source: stack_obj.source,
                effect: Box::new(move |target| {
                    for typ in target.characteristics.types.to_mut().as_mut_slice() {
                        if let ObjectType::Creature {
                            power, toughness, ..
                        } = typ
                        {
                            *power += 3;
                            *toughness += 3;
                            break;
                        }
                    }
                }),
                layer: crate::rules::layer::Layer::ChangePT,
                end: crate::rules::condition::Condition::EndOfTurn,
            }))
        }]),
        target_selectors: Cow::Borrowed(&[|game, target| match target {
            Target::Object(obj_id, _) => {
                let obj = game.objects.get(obj_id).expect("Object not found");
                obj.characteristics
                    .types
                    .iter()
                    .any(|typ| matches!(typ, ObjectType::Creature { .. }))
            }
            _ => false,
        }]),
    }],
    super_types: FlagSet::empty(),
    activated_abilities: &[],
    static_abilities: &[],
    triggered_abilities: &[],
};
