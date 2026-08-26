use std::borrow::Cow;

use flagset::FlagSet;

use crate::rules::{
    ability::static_ability::{DynamicAbilityGroup, DynamicContinuousAbility, StaticAbility},
    cost::{Cost, ManaCost},
    layer::{Layer, layers},
    object::{const_characteristics::ConstCharacteristics, types::ObjectType},
    zone::{ZoneId, ZoneKind, zones},
};

pub(crate) const BEAR_CUB: ConstCharacteristics = ConstCharacteristics {
    name: "Anthem of Champions",
    casting_cost: Cost {
        mana_cost: ManaCost {
            generic: 0,
            red: 0,
            green: 1,
            blue: 0,
            white: 1,
            black: 0,
            colorless: 0,
        },
    },
    types: &[ObjectType::Enchantment {
        subtypes: Cow::Borrowed(&[]),
    }],
    super_types: FlagSet::empty(),
    activated_abilities: &[],
    static_abilities: &[StaticAbility {
        active_zones: zones(&[ZoneKind::Battlefield]),
        layers: layers(&[Layer::ChangePT]),
        ability_group: DynamicAbilityGroup::DynamicContinuous(Cow::Borrowed(&[
            DynamicContinuousAbility {
                layer: Layer::ChangePT,
                is_cd: false,
                effect: |ctx, source| {
                    let obj = ctx
                        .game
                        .objects
                        .get(source)
                        .expect("Object not found (Anthem of Champions)");
                    let controller = obj.get_controller_or_owner();
                    for obj in ctx.game.get_objects_by_zone_mut(ZoneId::Battlefield) {
                        if obj.get_controller_or_owner() == controller {
                            for typ in obj.characteristics.types.to_mut().as_mut_slice() {
                                if let ObjectType::Creature {
                                    power, toughness, ..
                                } = typ
                                {
                                    *power += 1;
                                    *toughness += 1;
                                    break;
                                }
                            }
                        }
                    }
                },
            },
        ])),
    }],
    triggered_abilities: &[],
};
