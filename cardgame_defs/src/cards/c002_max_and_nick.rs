use cardgame_core::{CardDef, Colour, Effect, Nodes};
use crate::CardRegistration;

pub fn def() -> CardDef {
    CardDef {
        name: "Draw two".into(),
        cost: Nodes(3),
        colour: Colour::Blue,
        on_play: vec![Effect::Draw { amount: 1 }],
        triggers: vec![],
    }
}

inventory::submit! {
    CardRegistration(def)
}
