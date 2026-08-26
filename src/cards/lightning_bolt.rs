use std::borrow::Cow;

use flagset::FlagSet;

use crate::rules::{
    cost::{Cost, ManaCost},
    game_action::GameAction,
    object::{const_characteristics::ConstCharacteristics, types::ObjectType},
    target::Target,
};

pub(crate) const LIGHTNING_BOLT: ConstCharacteristics = ConstCharacteristics {
    name: "Lightning Bolt",
    casting_cost: Cost {
        mana_cost: ManaCost {
            generic: 0,
            red: 1,
            green: 0,
            blue: 0,
            white: 0,
            black: 0,
            colorless: 0,
        },
    },
    types: &[ObjectType::Instant {
        subtypes: Cow::Borrowed(&[]),
        effects: Cow::Borrowed(&[|game, id| {
            let stack_obj = game.get_stack_object(id);
            let target = stack_obj.targets[0];
            game.execute_actions(vec![GameAction::DealDamage {
                source: stack_obj.source,
                targets: vec![(3, target)],
            }])
        }]),
        target_selectors: Cow::Borrowed(&[|game, target| match target {
            Target::Object(obj_id, _) => {
                let obj = game.objects.get(obj_id).expect("Object not found");
                obj.characteristics.types.iter().any(|typ| {
                    matches!(typ, ObjectType::Creature { .. })
                        || matches!(typ, ObjectType::Planeswalker { .. })
                })
            }
            Target::Player(_) => true,
            _ => false,
        }]),
    }],
    super_types: FlagSet::empty(),
    activated_abilities: &[],
    static_abilities: &[],
    triggered_abilities: &[],
};
