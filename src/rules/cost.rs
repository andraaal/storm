use crate::{AnyId, Context};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cost {
    pub mana_cost: ManaAmount,
    pub tapping: bool,
}

impl Cost {
    pub(crate) fn pay(&self, ctx: &mut Context, id: AnyId) -> Result<(), ()> {
        if self.tapping {
            let bid = id.try_into();
            if let Ok(battlefield_id) = bid {
                let obj = ctx.game.objects.battlefield.get(battlefield_id);
                if let Some((info, _)) = obj {
                    if info.tapped {
                        return Err(());
                    }
                }
            } else {
                return Err(());
            }
        }

        let mana = &self.mana_cost;
        let player = ctx.game.priority;

        let pool = &ctx.game.players[player].mana;

        // Colored mana must be paid with the corresponding color.
        if pool.red < mana.red
            || pool.green < mana.green
            || pool.blue < mana.blue
            || pool.white < mana.white
            || pool.black < mana.black
            || pool.colorless < mana.colorless
        {
            return Err(());
        }

        // Calculate how much mana remains after paying colored costs.
        let available_generic =
            pool.red - mana.red + pool.green - mana.green + pool.blue - mana.blue + pool.white
                - mana.white
                + pool.black
                - mana.black
                + pool.colorless
                - mana.colorless;

        if available_generic < mana.generic {
            return Err(());
        }

        let pool = &mut ctx.game.players[player].mana;

        pool.red -= mana.red;
        pool.green -= mana.green;
        pool.blue -= mana.blue;
        pool.white -= mana.white;
        pool.black -= mana.black;
        pool.colorless -= mana.colorless;

        // Pay generic mana from the remaining pool. The exact choice of
        // mana is irrelevant, so consume colorless first, then the colors.
        let mut generic = mana.generic;

        let payment = generic.min(pool.colorless);
        pool.colorless -= payment;
        generic -= payment;

        let payment = generic.min(pool.red);
        pool.red -= payment;
        generic -= payment;

        let payment = generic.min(pool.green);
        pool.green -= payment;
        generic -= payment;

        let payment = generic.min(pool.blue);
        pool.blue -= payment;
        generic -= payment;

        let payment = generic.min(pool.white);
        pool.white -= payment;
        generic -= payment;

        let payment = generic.min(pool.black);
        pool.black -= payment;

        if self.tapping {
            let bid = id.try_into();
            if let Ok(battlefield_id) = bid {
                let obj = ctx.game.objects.battlefield.get_mut(battlefield_id);
                if let Some((info, _)) = obj {
                    info.tapped = true;
                }
            } else {
                return Err(());
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ManaAmount {
    pub generic: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub white: u8,
    pub black: u8,
    pub colorless: u8,
}

impl ManaAmount {
    pub(crate) const fn def() -> Self {
        ManaAmount {
            generic: 0,
            red: 0,
            green: 0,
            blue: 0,
            white: 0,
            black: 0,
            colorless: 0,
        }
    }
}
