use crate::{core::{state::{game::GameState, utils::sync_nodes}, types::stack::StackItem}, rules::{exec_command::execute_command, expand_action::expand_action, expand_effect::expand_effect}};

pub(crate) fn resolve(state: &mut GameState) {
    while let Some(item) = state.stack.pop() {
        resolve_stack_item(state, item);
        sync_nodes(state);
    }
}

pub(crate) fn resolve_stack_item(state: &mut GameState, item: StackItem) {
    match item {
        StackItem::PlayerAction { player, action } => {
            let cmds = expand_action(state, player, action);
            for cmd in cmds {
                execute_command(state, cmd);
            }
        }

        StackItem::CardEffect { controller, source, effect, ctx } => {
            let cmds = expand_effect(state, controller, source, effect, ctx);
            for cmd in cmds {
                execute_command(state, cmd);
            }
        }
    }
}

// And you need to decide whether stack resolves:

// LIFO (Vec::pop)

// FIFO (VecDeque)