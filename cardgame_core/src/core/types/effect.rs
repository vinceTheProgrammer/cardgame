use crate::core::{state::{card::CardId}, types::event::GameEvent};

#[derive(Debug, Clone)]
pub enum Effect {
    Draw { amount: u32 },
    PlayCard { amount: u32 },
    DiscardCard { amount: u32 },
    SwapCard { card: CardId },
    CancelEvent,
}

#[derive(Clone, Debug)]
pub struct EffectContext {
    pub triggering_event: Option<GameEvent>,
}