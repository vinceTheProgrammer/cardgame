use crate::core::state::{card::CardId, player::PlayerRole, zones::Zone};

macro_rules! define_events {
    (
        $(
            $name:ident $( { $($field:ident : $fty:ty),* $(,)? } )?
        ),* $(,)?
    ) => {
        // ============================
        // GameEvent (real event data)
        // ============================
        #[derive(Debug, Clone)]
        pub enum GameEvent {
            $(
                $name $( { $($field : $fty),* } )?
            ),*
        }

        // ============================
        // EventMatcher (pattern event)
        // ============================
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum EventMatcher {
            Any,
            $(
                $name
            ),*
        }

        // ============================
        // matcher_matches()
        // ============================
        pub(crate) fn matcher_matches(matcher: EventMatcher, ev: &GameEvent) -> bool {
            match (matcher, ev) {
                (EventMatcher::Any, _) => true,

                $(
                    (EventMatcher::$name, GameEvent::$name { .. }) => true,
                )*

                _ => false,
            }
        }
    };
}

define_events! {
    CardPlayed { player: PlayerRole, card: CardId },
    CardMoved { card: CardId, from: Zone, to: Zone },
    CardDiscarded { card: CardId },
    CardsDrawn { player: PlayerRole, amount: u32 },
    TurnEnded,
    TurnStarted { player: PlayerRole },
}

pub struct PendingEvent {
    pub event: GameEvent,
    pub canceled: bool,
}