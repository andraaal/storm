use crate::{
    context::Context,
    nested_borrow::ShadowBorrow,
    rules::{id::AnyId, object::types::ObjectType, zone::StackInfo},
};

impl Context {
    pub(crate) fn cast_spell(&mut self, id: AnyId) {
        let (controller, owner) = self.game.objects.get_controller_and_owner(id);
        let owner = owner.expect("A card being cast must have an owner");
        let controller = controller.unwrap_or(owner);
        let obj = self
            .game
            .objects
            .get_any(id)
            .expect("Card being cast no longer exists")
            .clone();
        let mut obj = obj;
        let cost = obj.characteristics.casting_cost.clone();
        obj.timestamp = self.game.generate_timestamp();
        let info = StackInfo { controller, owner };

        let spell = match obj.characteristics.types[0] {
            ObjectType::Land { .. } => panic!("Can't cast lands"),
            ObjectType::Creature { effect, .. }
            | ObjectType::Artifact { effect, .. }
            | ObjectType::Planeswalker { effect, .. }
            | ObjectType::Enchantment { effect, .. } => effect.create_cancellable(self, obj),
            ObjectType::Sorcery { effect, .. } | ObjectType::Instant { effect, .. } => {
                effect.create_cancellable(self, obj)
            }
        };

        if let Some(spell) = spell {
            if cost.pay(self, id) == Err(()) {
                return;
            }
            id.remove(&mut self.game.objects);
            self.game.objects.insert_spell_stack((info, spell));
        }
    }

    pub(crate) fn activate_ability(&mut self, id: AnyId) {
        let maybe_obj = self.game.objects.get_any(id);
        if let Some(obj) = maybe_obj {
            let ability = obj.characteristics.activated_abilities[0].clone();
            if ability.cost.pay(self, id) == Ok(()) {
                (ability.effect)(self);
            }
        }
    }

    pub(crate) fn play_card(&mut self, id: AnyId) {
        self.validate_cast_source(id);
        let obj = self.game.objects.get_any(id).expect("Card not found");

        if let Some(effect) = obj.characteristics.types.iter().find_map(|typ| match typ {
            ObjectType::Land { effect, .. } => Some(effect.clone()),
            _ => None,
        }) {
            let source = id.remove(&mut self.game.objects);
            let boxed = effect.create(self, source);
            self.game.objects.resolving_id = Some(id);
            let (sb, _) = ShadowBorrow::<'_, Context>::new(self);
            boxed.execute(sb);
            self.game.objects.resolving_id = None;
        } else {
            self.cast_spell(id);
        }

        // Casting or playing a card starts a fresh priority round.  The
        // player who acted keeps priority, as required by the rules.
        self.game.last_non_passed_priority = self.game.priority;
    }

    fn validate_cast_source(&self, id: AnyId) {
        let is_hand = matches!(id, AnyId::PlayHand(_) | AnyId::DrawHand(_));
        assert!(
            is_hand,
            "Casting or playing a card is currently only supported from a hand"
        );

        let (_, owner) = self.game.objects.get_controller_and_owner(id);
        assert_eq!(
            owner,
            Some(self.game.priority),
            "Only the player with priority can cast or play a card"
        );
    }
}
