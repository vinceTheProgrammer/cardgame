use crate::core::{state::{card::CardId, player::PlayerRole}, types::{action::Action, effect::{Effect, EffectContext}}};

#[derive(Debug)]
pub enum StackItem {
    PlayerAction {
        player: PlayerRole,
        action: Action,
    },

    CardEffect {
        controller: PlayerRole,
        source: CardId,
        effect: Effect,
        ctx: EffectContext,
    },
}