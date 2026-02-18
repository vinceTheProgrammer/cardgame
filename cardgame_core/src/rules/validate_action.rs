use crate::core::{state::{card::CardId, game::GameState, player::PlayerRole, zones::Zone}, types::action::Action};

#[derive(Debug, Clone)]
pub enum ActionError {
    NotYourTurn,
    CardNotFound(CardId),
    CardNotInZone { card: CardId, expected: Zone },
    InvalidAction,
}

pub(crate) fn validate_action(
    state: &GameState,
    player: PlayerRole,
    action: &Action,
) -> Result<(), ActionError> {
    if state.turn.active_player != player {
        return Err(ActionError::NotYourTurn);
    }

    match action {
        Action::PlayCard { card } => {
            let card_state = state
                .entities
                .cards
                .get(card)
                .ok_or(ActionError::CardNotFound(*card))?;

            // must be in your hand
            if card_state.zone != Zone::Hand(player) {
                return Err(ActionError::CardNotInZone {
                    card: *card,
                    expected: Zone::Hand(player),
                });
            }

            // must be controlled/owned by you
            if card_state.owner != player {
                return Err(ActionError::InvalidAction);
            }

            Ok(())
        }
    }
}
