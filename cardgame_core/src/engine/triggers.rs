use crate::{core::{state::{game::GameState, zones::Zone}, types::{effect::EffectContext, event::{GameEvent, matcher_matches}, stack::StackItem, trigger::{ResolvedTrigger, Timing, TriggerDef, TriggerEventRef}}}, engine::resolve::resolve_stack_item};

pub(crate) fn collect_matching_triggers(
    state: &GameState,
    timing: Timing,
    ev: &GameEvent,
) -> Vec<ResolvedTrigger> {
    let mut out = Vec::new();

    for card in state.entities.iter() {
        let def = &state.card_def(card.def);

        for (idx, trig) in def.triggers.iter().enumerate() {
            match trig {
                TriggerDef::Instant { matcher, effect } => {
                    if Timing::Before != timing {
                        continue;
                    }

                    if card.zone == Zone::Hand(card.owner) && matcher_matches(*matcher, ev.event()) {
                        out.push(ResolvedTrigger {
                            controller: card.owner,
                            source_card: card.id,
                            effect: effect.clone(), 
                            ability_index: idx as u32,
                        });
                    }
                }, 
                TriggerDef::Trigger { matcher, effect } => {
                    if Timing::After != timing {
                        continue;
                    }

                    if card.zone == Zone::Field(card.owner) && matcher_matches(*matcher, ev.event()) {
                        out.push(ResolvedTrigger {
                            controller: card.owner,
                            source_card: card.id,
                            effect: effect.clone(), 
                            ability_index: idx as u32,
                        });
                    }
                }, 
                _ => continue,
            };
        }
    }

    // deterministic ordering rule:
    // controller priority (active player first), then card id, then ability index // TODO review if this priority order is good
    let active = state.turn.active_player;

    out.sort_by_key(|t| {
        let prio = if t.controller == active { 0 } else { 1 };
        (prio, t.controller, t.source_card.0, t.ability_index)
    });

    out
}

pub(crate) fn collect_and_resolve_triggers(
    state: &mut GameState,
    timing: Timing,
    ev: impl TriggerEventRef,
) {
    let triggers = collect_matching_triggers(state, timing, ev.event());

    for trig in triggers {
        state.stack.push(StackItem::CardEffect {
            controller: trig.controller,
            source: trig.source_card,
            effect: trig.effect,
            ctx: EffectContext {
                triggering_event: Some(ev.event().clone()),
            },
        });
    }

    // resolve immediately so BEFORE triggers fully apply
    while let Some(item) = state.stack.pop() {
        resolve_stack_item(state, item);
    }
}