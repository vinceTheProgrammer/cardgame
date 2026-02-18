use cardgame_core::core::{state::card::{CardColor, CardDef}, types::{event::EventMatcher, trigger::{Timing, TriggerDef}}};

use crate::CardRegistration;

pub fn def() -> CardDef {
    CardDef {
        id: 2,
        name: "Max and Nick",
        description: "Play a Red card.",
        creator: "Cheese Biscuit",
        color: CardColor::Blue,
        nodes: -1,
        triggers: &[TriggerDef::OnPlay {
            PlayCardFromHand {
                player: EffectPlayer::Owner,
                restriction: Some(CardRestriction::Color::Red),
            },
        }],
    }
}

inventory::submit! {
    CardRegistration(def)
}
