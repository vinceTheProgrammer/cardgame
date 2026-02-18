use cardgame_core::core::{state::card::{CardColor, CardDef}, types::{event::EventMatcher, trigger::{Timing, TriggerDef}}};

use crate::CardRegistration;

pub fn def() -> CardDef {
    CardDef {
        id: 1,
        name: "Coll",
        description: "Draw 1.",
        creator: "Collision",
        color: CardColor::Blue,
        nodes: 3,
        triggers: &[TriggerDef::OnPlay {
            effect: CardEffect::DrawCard {
                player: EffectPlayer::Owner,
                amount: 1
            }
        }],
    }
}

inventory::submit! {
    CardRegistration(def)
}
