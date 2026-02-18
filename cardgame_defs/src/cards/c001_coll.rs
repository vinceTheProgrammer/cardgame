use cardgame_core::core::{state::card::{CardColor, CardDef}, types::{event::EventMatcher, trigger::{Timing, TriggerDef}}};

use crate::CardRegistration;

pub fn def() -> CardDef {
    CardDef {
        name: "Coll",
        description: "Draw 1.",
        creator: "Collision",
        color: CardColor::Blue,
        nodes: 3,
        triggers: &[TriggerDef {
            timing: Timing::OnPlay,
            matcher: EventMatcher::
        }],
    }
}

inventory::submit! {
    CardRegistration(def)
}
