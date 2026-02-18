use std::collections::HashMap;

use crate::core::state::card::{CardId, CardState};

#[derive(Debug, Clone)]
pub struct EntityStore {
    entities: Vec<Option<CardState>>,
    pub cards: HashMap<CardId, CardState>,
}

impl EntityStore {
    pub(crate) fn new() -> Self {
        Self { entities: Vec::new(), cards: HashMap::new() }
    }

    pub(crate) fn insert(&mut self, entity: CardState) {
        let id = entity.id.0 as usize;

        if id >= self.entities.len() {
            self.entities.resize_with(id + 1, || None);
        }

        self.entities[id] = Some(entity);
    }

    pub(crate) fn get(&self, id: CardId) -> Option<&CardState> {
        self.entities.get(id.0 as usize)?.as_ref()
    }

    pub(crate) fn get_mut(&mut self, id: CardId) -> Option<&mut CardState> {
        self.entities.get_mut(id.0 as usize)?.as_mut()
    }

    pub(crate) fn remove(&mut self, id: CardId) -> Option<CardState> {
        let slot = self.entities.get_mut(id.0 as usize)?;
        slot.take()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &CardState> {
        self.entities.iter().filter_map(|x| x.as_ref())
    }
}