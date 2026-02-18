use crate::{core::{state::{game::GameState, player::PlayerRole}, types::{action::Action, command::Command}}};

pub(crate) fn expand_action(_state: &GameState, player: PlayerRole, action: Action) -> Vec<Command> {
    match action {
        Action::PlayCard { card } => {
            let mut cmds = vec![];

            cmds.push(Command::PlayCard {
                player,
                card,
            });

            cmds
        }
    }
}
