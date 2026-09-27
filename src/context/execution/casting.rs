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
        obj.timestamp = self.game.generate_timestamp();
        let info = StackInfo { controller, owner };

        let spell = match obj.characteristics.types[0] {
            ObjectType::Land { .. } => panic!("Can't cast lands"),
            ObjectType::Creature { effect, .. }
            | ObjectType::Artifact { effect, .. }
            | ObjectType::Planeswalker { effect, .. }
            | ObjectType::Enchantment { effect, .. } => effect.create(self, obj),
            ObjectType::Sorcery { effect, .. } | ObjectType::Instant { effect, .. } => {
                effect.create(self, obj)
            }
        };

        id.remove(&mut self.game.objects);
        self.game.objects.spell_stack.insert((info, spell));
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
            let (sb, _) = ShadowBorrow::<'_, Context>::new(self);
            boxed.execute(sb);
        } else {
            self.cast_spell(id);
        }
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
