use derive_more::From;

use crate::{
    context::Context,
    rules::{
        id::BattlefieldId,
        player::PlayerId,
        target::{AnyTarget, Target},
    },
};

#[derive(From, Clone, Copy)]
pub(crate) enum DamageableTarget {
    Battlefield(BattlefieldId),
    Player(PlayerId),
}

impl Into<AnyTarget> for DamageableTarget {
    fn into(self) -> AnyTarget {
        match self {
            DamageableTarget::Battlefield(id) => id.into(),
            DamageableTarget::Player(id) => id.into(),
        }
    }
}

impl TryFrom<AnyTarget> for DamageableTarget {
    fn try_from(value: AnyTarget) -> Result<Self, Self::Error> {
        match value {
            AnyTarget::Battlefield(id) => Ok(id.into()),
            AnyTarget::PlayerTarget(id) => Ok(id.into()),
            _ => Err(()),
        }
    }

    type Error = ();
}

impl Target for DamageableTarget {
    fn choose(
        cont: &mut crate::context::controller::Controller,
        game: &crate::game::Game,
        choices: Vec<Self>,
        range: std::range::Range<usize>,
    ) -> Vec<Self>
    where
        Self: Sized,
    {
        cont.choose_any(game, choices.into_iter().map(|i| i.into()).collect(), range)
            .into_iter()
            .map(|i| i.try_into().unwrap())
            .collect()
    }

    fn choose_cancellable(
        cont: &mut crate::context::controller::Controller,
        game: &crate::game::Game,
        choices: Vec<Self>,
        range: std::range::Range<usize>,
    ) -> Option<Vec<Self>>
    where
        Self: Sized,
    {
        cont.choose_any_cancellable(game, choices.into_iter().map(|i| i.into()).collect(), range)
            .map(|o| o.into_iter().map(|i| i.try_into().unwrap()).collect())
    }
}

pub(crate) trait Damageable: Into<DamageableTarget> + Target {
    fn apply_damage(
        &self,
        ctx: &mut Context,
        amount: u32,
        wither: bool,
        infect: bool,
        deathtouch: bool,
    );
}

impl Damageable for DamageableTarget {
    fn apply_damage(
        &self,
        ctx: &mut Context,
        amount: u32,
        wither: bool,
        infect: bool,
        deathtouch: bool,
    ) {
        match self {
            DamageableTarget::Battlefield(id) => {
                id.apply_damage(ctx, amount, wither, infect, deathtouch)
            }
            DamageableTarget::Player(id) => {
                id.apply_damage(ctx, amount, wither, infect, deathtouch)
            }
        }
    }
}

impl Damageable for BattlefieldId {
    fn apply_damage(&self, ctx: &mut Context, amount: u32, wither: bool, infect: bool, dt: bool) {
        if let Some((info, _)) = ctx.game.objects.battlefield.get_mut(*self) {
            if dt {
                info.damaged_by_deathtouch = true;
            }
            if wither || infect {
                todo!("Wither and Infect not yet implemented");
            } else {
                info.marked_damage += amount;
            }
        } else {
            println!("Tried accessing invalid/expired object (Damagable)");
        }
    }
}

impl Damageable for PlayerId {
    fn apply_damage(&self, ctx: &mut Context, amount: u32, _wither: bool, infect: bool, _dt: bool) {
        if infect {
            todo!("-1/-1 counters not yet implemented!");
        } else {
            ctx.game.players.get_mut(*self).life -= amount as i32;
        }
    }
}
