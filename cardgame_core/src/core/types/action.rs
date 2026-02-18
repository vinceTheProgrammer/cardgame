use crate::core::state::card::CardId;

#[derive(Debug, Clone)]
pub enum Action {
    PlayCard {
        card: CardId,
    }
}