use crate::cards::behaviour::change_pt::*;
use crate::cards::behaviour::deal_damage::DealDamage;
use crate::cards::stack_definitions::no_data::NoData;
use crate::cards::stack_definitions::simple_permanent::Permanent;
use crate::cards::stack_definitions::source::SourceData;
use crate::context::Context;
use crate::rules::ability::behaviour::Behaviour;
use crate::rules::ability::effect::*;
use crate::rules::id::AnyId;
use crate::rules::object::game_object::GameObject;
use crate::rules::target::selector::Selector;
use crate::rules::target::selector::SelectorList;
use derive_more::From;

pub(crate) mod cards {
    pub(crate) mod anthem_of_champions;
    pub(crate) mod basic_lands;
    pub(crate) mod bear_cub;
    pub(crate) mod giant_growth;
    pub(crate) mod lightning_bolt;
}

pub(crate) mod stack_definitions {
    pub(crate) mod no_data;
    pub(crate) mod simple_permanent;
    pub(crate) mod source;
}

pub(crate) mod selectors {
    pub(crate) mod creature;
    pub(crate) mod damagable;
}

pub(crate) mod behaviour {
    pub(crate) mod change_pt;
    pub(crate) mod deal_damage;
    pub(crate) mod default_info;
}

macro_rules! stack_effects {
    (
        $(
            $name:ident: $type:ty
        ),* $(,)?
    ) => {
        #[derive(Clone, Copy, From)]
        pub enum Spells {
            $(
                $name,
            )*
        }

        #[derive(Clone, Copy, From)]
        pub enum StackAbilities {
            $(
                $name,
            )*
        }

        impl Spells {
            pub fn create(&self, ctx: &mut Context, source: GameObject) -> Box<dyn Spell> {
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

            pub fn create_cancellable(&self, ctx: &mut Context, source: GameObject) -> Option<Box<dyn Spell>> {
                Some(match self {
                    $(
                        Self::$name => {
                            Box::new(StackObject::<$type, ToGraveyard> {
                                targets: <<$type as StackDefinition>::TargetSelection as SelectorList>::choose_cancellable(ctx)?,
                                x: <<$type as StackDefinition>::X as XVal>::choose(ctx),
                                modes: <<$type as StackDefinition>::Modes as Modes>::choose(ctx),
                                data: <$type as StackDefinition>::choose_data(ctx),
                                source,
                                kind: std::marker::PhantomData::<ToGraveyard>,
                            })
                        }
                    )*
                })
            }
        }

        impl StackAbilities {
            fn create(&self, ctx: &mut Context, source: AnyId) -> Box<dyn Ability> {
                match self {
                    $(
                        Self::$name => {
                            Box::new(StackObject::<$type, Vanish> {
                                targets: <<$type as StackDefinition>::TargetSelection as SelectorList>::choose(ctx),
                                x: <<$type as StackDefinition>::X as XVal>::choose(ctx),
                                modes: <<$type as StackDefinition>::Modes as Modes>::choose(ctx),
                                data: <$type as StackDefinition>::choose_data(ctx),
                                source,
                                kind: std::marker::PhantomData::<Vanish>,
                            })
                        }
                    )*
                }
            }
        }
    };
}

macro_rules! effects {
    (
        $(
            $name:ident: $type:ty
        ),* $(,)?
    ) => {

        #[derive(Clone, Copy, From)]
        pub enum Effects {
            $(
                $name,
            )*
        }

        impl Effects {
            pub fn apply(&self, ctx: &mut Context) {
                match self {
                    $(
                        Self::$name => {
                            let targets = <<$type as Behaviour>::TargetSelector as Selector>::choices(&ctx.game);
                            for target in targets {
                                let _: () = <$type as Behaviour>::apply(ctx, &target, ());
                            }
                        }
                    )*
                };
            }
        }
    };
}

macro_rules! enter_abilities {
    (
        $(
            $name:ident: $type:ty
        ),* $(,)?
    ) => {

        #[derive(Clone, Copy, From)]
        pub enum EnterAbilities {
            $(
                $name,
            )*
        }

        impl EnterAbilities {
            pub fn create(&self, ctx: &mut Context) -> BattlefieldInfo {
                match self {
                    $(
                        Self::$name => {
                            let targets = <<$type as Behaviour>::TargetSelector as Selector>::choices(&ctx.game);
                            for target in targets {
                                let _: () = <$type as Behaviour>::apply(ctx, &target, ());
                            }
                        }
                    )*
                };
            }
        }
    };
}

macro_rules! permanent_effects {
    (
        $(
            $name:ident: $type:ty
        ),* $(,)?
    ) => {
        #[derive(Clone, Copy, From)]
        pub enum PermanentEffects {
            $(
                $name,
            )*
        }

        impl PermanentEffects {
            pub fn create(&self, ctx: &mut Context, source: GameObject) -> Box<dyn Spell> {
                match self {
                    $(
                        Self::$name => {
                            Box::new(StackObject::<$type, ToBattlefield> {
                                targets: <<$type as StackDefinition>::TargetSelection as SelectorList>::choose(ctx),
                                x: <<$type as StackDefinition>::X as XVal>::choose(ctx),
                                modes: <<$type as StackDefinition>::Modes as Modes>::choose(ctx),
                                data: <$type as StackDefinition>::choose_data(ctx),
                                source,
                                kind: std::marker::PhantomData::<ToBattlefield>,
                            })
                        }
                    )*
                }
            }

            pub fn create_cancellable(&self, ctx: &mut Context, source: GameObject) -> Option<Box<dyn Spell>> {
                Some(match self {
                    $(
                        Self::$name => {
                            Box::new(StackObject::<$type, ToBattlefield> {
                                targets: <<$type as StackDefinition>::TargetSelection as SelectorList>::choose_cancellable(ctx)?,
                                x: <<$type as StackDefinition>::X as XVal>::choose(ctx),
                                modes: <<$type as StackDefinition>::Modes as Modes>::choose(ctx),
                                data: <$type as StackDefinition>::choose_data(ctx),
                                source,
                                kind: std::marker::PhantomData::<ToBattlefield>,
                            })
                        }
                    )*
                })
            }
        }
    };
}

// Most Permanents just use trivial; Can do things like this permanent enters tapped, ie modify BattlefieldInfo before the object enters
permanent_effects![Trivial: Permanent];

// Entries with a single Behaviour to be used in static/activated abilities
effects![Plus1_1: ChangePT<1, 1>];

// Entries that produce an effect that can be used as a ability on the stack or an instant/sorcery spell
stack_effects![
    Plus3_3: NoData<(ChangePT<3, 3>,)>,
    Damage3: SourceData<(DealDamage<1, 3>,)>,
];
