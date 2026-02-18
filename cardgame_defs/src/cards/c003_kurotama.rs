use cardgame_core::core::{state::card::{CardColor, CardDef}, types::{event::EventMatcher, trigger::{Timing, TriggerDef}}};

use crate::CardRegistration;

pub fn def() -> CardDef {
    CardDef {
        id: 3,
        name: "Kurotama",
        description: "Discard a card from your field.",
        creator: "Darkboy",
        color: CardColor::Blue,
        nodes: 3,
        triggers: &[TriggerDef::OnPlay {
            effect: CardEffect::DiscardCardFromField {
                player: EffectPlayer::Owner,
                amount: 1,
                restriction: None,
                selection: SelectionRule::OwnerChooses,
            }
        }],
    }
}

inventory::submit! {
    CardRegistration(def)
}
