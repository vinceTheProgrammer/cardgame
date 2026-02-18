use crate::{core::{state::{game::{GameState}}, types::{command::Command, event::GameEvent}}, engine::dispatch::dispatch_event};

pub(crate) fn execute_command(state: &mut GameState, cmd: Command) {
    match cmd {
        Command::DispatchEvent(ev) => {
            dispatch_event(state, ev);
        }

        Command::DrawCards { player, amount } => {
            dispatch_event(state, GameEvent::CardsDrawn { player, amount });
        }

        Command::PlayCard { player, card } => {
            dispatch_event(state, GameEvent::CardPlayed { player, card });
        }

        Command::EndTurn => {
            dispatch_event(state, GameEvent::TurnEnded);
        }

        _ => { /* same idea */ }
    }
}