use crate::{core::{state::game::GameState, types::{event::{GameEvent, PendingEvent}, trigger::Timing}}, engine::triggers::collect_and_resolve_triggers, rules::apply_event::apply_event};

pub(crate) fn dispatch_event(state: &mut GameState, ev: GameEvent) {
    let mut pending = PendingEvent { event: ev, canceled: false };

    // BEFORE window: modifiers/replacements/cancel
    collect_and_resolve_triggers(state, Timing::Before, &mut pending);

    if pending.canceled {
        return; // TODO maybe add cancellation of the event to the event log?
    }

    // Apply the event to state (this is where mutation happens)
    apply_event(state, &pending.event);

    // record for replay/debug
    state.log.push(pending.event.clone());

    // AFTER window: reactions
    collect_and_resolve_triggers(state, Timing::After, pending.event);
}