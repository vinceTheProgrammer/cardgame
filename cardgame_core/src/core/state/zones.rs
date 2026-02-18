use crate::core::state::{card::CardId, player::PlayerRole};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Zone {
    Deck(PlayerRole),
    Hand(PlayerRole),
    Field(PlayerRole),
    DiscardPile(PlayerRole),
}

#[derive(Debug, Clone)]
pub struct Zones {
    pub deck: [Vec<CardId>; 2],
    pub hand: [Vec<CardId>; 2],
    pub field: [Vec<CardId>; 2],
    pub discard_pile: [Vec<CardId>; 2],
}

impl Zones {
    pub(crate) fn new() -> Self {
        Self {
            deck: [Vec::new(), Vec::new()],
            hand: [Vec::new(), Vec::new()],
            field: [Vec::new(), Vec::new()],
            discard_pile: [Vec::new(), Vec::new()],
        }
    }

    pub(crate) fn get_zone_mut(&mut self, zone: Zone) -> &mut Vec<CardId> {
        match zone {
            Zone::Deck(p) => &mut self.deck[p.index()],
            Zone::Hand(p) => &mut self.hand[p.index()],
            Zone::Field(p) => &mut self.field[p.index()],
            Zone::DiscardPile(p) => &mut self.discard_pile[p.index()],
        }
    }

    pub(crate) fn get_zone(&self, zone: Zone) -> &Vec<CardId> {
        match zone {
            Zone::Deck(p) => &self.deck[p.index()],
            Zone::Hand(p) => &self.hand[p.index()],
            Zone::Field(p) => &self.field[p.index()],
            Zone::DiscardPile(p) => &self.discard_pile[p.index()],
        }
    }
}

pub(crate) fn remove_from_zone(zones: &mut Zones, zone: Zone, card: CardId) -> bool {
    let vec = zones.get_zone_mut(zone);

    if let Some(idx) = vec.iter().position(|c| *c == card) {
        vec.swap_remove(idx);
        true
    } else {
        false
    }
}

pub(crate) fn push_to_zone(zones: &mut Zones, zone: Zone, card: CardId) {
    let vec = zones.get_zone_mut(zone);
    vec.push(card);
}