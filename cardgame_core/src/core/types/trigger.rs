use crate::core::{state::{card::CardId, player::PlayerRole}, types::{effect::Effect, event::{EventMatcher, GameEvent, PendingEvent}}};

#[derive(Debug, Clone)]
pub enum TriggerDef {
    Instant { matcher: EventMatcher, effect: Effect },
    Trigger { matcher: EventMatcher, effect: Effect },
    Recurring { effect: Effect },
    OnPlay { effect: Effect}
}
#[derive(Debug, Clone)]
pub struct ResolvedTrigger {
    pub controller: PlayerRole,
    pub source_card: CardId,
    pub effect: Effect,
    pub ability_index: u32, // for deterministic sorting if needed
}

#[derive(Debug, Clone, PartialEq)]
pub enum Timing {
    /// for "instant" effects
    Before,
    /// for "trigger" effects
    After,
    Recurring,
    OnPlay,
}

pub trait TriggerEventRef {
    fn event(&self) -> &GameEvent;
}

impl TriggerEventRef for GameEvent {
    fn event(&self) -> &GameEvent {
        self
    }
}

impl TriggerEventRef for PendingEvent {
    fn event(&self) -> &GameEvent {
        &self.event
    }
}

impl TriggerEventRef for &PendingEvent {
    fn event(&self) -> &GameEvent {
        &self.event
    }
}

impl TriggerEventRef for &mut PendingEvent {
    fn event(&self) -> &GameEvent {
        &self.event
    }
}