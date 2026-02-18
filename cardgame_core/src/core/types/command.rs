use crate::core::{state::{card::CardId, player::PlayerRole, zones::Zone}, types::event::GameEvent};

pub(crate) enum Command {
    DispatchEvent(GameEvent),

    MoveCard { card: CardId, from: Zone, to: Zone },
    PlayCard { player: PlayerRole, card: CardId },
    DrawCards { player: PlayerRole, amount: u32 },
    DiscardCard { card: CardId },
    EndTurn,
}