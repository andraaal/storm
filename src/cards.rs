use crate::cards::behaviour::change_pt::*;
use crate::cards::behaviour::deal_damage::DealDamage;
use crate::cards::selectors::creature::Creature;
use crate::cards::selectors::damagable::Damagable;
use crate::cards::stack_definitions::no_data::NoData;
use crate::cards::stack_definitions::source::SourceData;
use crate::context::Context;
use crate::rules::ability::behaviour::Behaviour;
use crate::rules::ability::behaviour::BehaviourList;
use crate::rules::ability::effect::*;
use crate::rules::id::AnyId;
use crate::rules::object::game_object::GameObject;
use crate::rules::target::damage::Damageable;
use crate::rules::target::selector::Selector;
use crate::rules::target::selector::SelectorList;
use derive_more::From;

pub(crate) mod cards {
    pub(crate) mod anthem_of_champions;
    pub(crate) mod bear_cub;
    pub(crate) mod giant_growth;
    pub(crate) mod lightning_bolt;
}

pub(crate) mod stack_definitions {
    pub(crate) mod no_data;
    pub(crate) mod source;
}

pub(crate) mod selectors {
    pub(crate) mod creature;
    pub(crate) mod damagable;
}

pub(crate) mod behaviour {
    pub(crate) mod change_pt;
    pub(crate) mod deal_damage;
}

macro_rules! effects {
    (
        $(
            $name:ident: $type:ty
        ),* $(,)?
    ) => {
        #[derive(Clone, Copy, From)]
        pub(crate) enum Spells {
            $(
                $name,
            )*
        }

        #[derive(Clone, Copy, From)]
        pub(crate) enum StackAbilities {
            $(
                $name,
            )*
        }

        impl Spells {
            fn create(&self, ctx: &mut Context, source: GameObject) -> Box<dyn StackObj> {
                match self {
                    $(
                        Self::$name => {
                            Box::new(StackObject::<$type, ToGraveyard> {
                                targets: <<$type as StackDefinition>::TargetSelection as SelectorList>::choose(ctx),
                                x: <<$type as StackDefinition>::X as XVal>::choose(ctx),
                                modes: <<$type as StackDefinition>::Modes as Modes>::choose(ctx),
                                data: <$type as StackDefinition>::choose_data(ctx),
                                source,
                                kind: std::marker::PhantomData::<ToGraveyard>,
                            })
                        }
                    )*
                }
            }
        }
    };
}

effects![
    Plus3_3: NoData<(ChangePT<3, 3>,)>,
    Damage3: SourceData<(DealDamage<1, 3>,)>,
];

macro_rules! static_abilities {
    (
        $(
            $name:ident: $type:ty
        ),* $(,)?
    ) => {

        #[derive(Clone, Copy, From)]
        pub(crate) enum StaticAbilities {
            $(
                $name,
            )*
        }

        impl StaticAbilities {
            pub(crate) fn apply(&self, ctx: &mut Context) {
                match self {
                    $(
                        Self::$name => {
                            let targets = <<$type as Behaviour>::TargetSelector as Selector>::choices(&ctx.game);
                            for target in targets {
                                <$type as Behaviour>::apply(ctx, &target, ());
                            }
                        }
                    )*
                };
            }
        }
    };
}

static_abilities!(Plus1_1: ChangePT<1, 1>);
