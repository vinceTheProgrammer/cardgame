use cardgame_core::core::{state::card::{CardColor, CardDef}, types::{event::EventMatcher, trigger::{Timing, TriggerDef}}};

use crate::CardRegistration;

pub fn def() -> CardDef {
    CardDef {
        name: "Coll",
        description: "Draw 1.",
        creator: "Collision",
        color: CardColor::Blue,
        nodes: 3,
        triggers: &[TriggerDef::OnPlay {
            effect: CardEffect::DrawCard {
                amount: 1
            }
        }],
    }
}

inventory::submit! {
    CardRegistration(def)
}
