use crate::{core::{state::{game::GameState, player::PlayerRole}, types::{action::Action, command::Command, stack::StackItem}}, engine::resolve::resolve, rules::{exec_command::execute_command, validate_action::{ActionError, validate_action}}};

pub fn submit_action(state: &mut GameState, player: PlayerRole, action: Action) -> Result<(), ActionError> {
    validate_action(state, player, &action)?;

    state.stack.push(StackItem::PlayerAction {
        player,
        action,
    });

    resolve(state);

    execute_command(state, Command::EndTurn);

    Ok(())
}