use crate::core::{state::{game::GameState, utils::{draw_card, end_turn, move_card, start_turn, sync_nodes}, zones::Zone}, types::event::GameEvent};

pub(crate) fn apply_event(state: &mut GameState, ev: &GameEvent) {
    match ev {
        GameEvent::CardMoved { card, from, to } => {
            move_card(state, *card, *from, *to);
        }

        GameEvent::TurnStarted { player } => {
            start_turn(state, *player);
        }

        GameEvent::TurnEnded => {
            end_turn(state);
        }

        GameEvent::CardsDrawn { player, amount } => {
            for _ in 0..*amount {
                if let Some(card) = draw_card(state, *player) {
                    move_card(state, card, Zone::Deck(*player), Zone::Hand(*player))
                }
            }
        }

        GameEvent::CardPlayed { player, card } => {
            move_card(state, *card, Zone::Hand(*player), Zone::Field(*player));
            sync_nodes(state);
        }

        GameEvent::CardDiscarded { .. } => {}
    }
}
