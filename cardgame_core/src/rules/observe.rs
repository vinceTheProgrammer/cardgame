use crate::core::{state::{card::CardId, game::{GameState, TurnState}, player::{PlayerRole, PlayerState}}, types::event::GameEvent};

pub struct GameView<'a> {
    state: &'a GameState<'a>,
    role: PlayerRole,
}

impl<'a> GameView<'a> {
    pub fn new(state: &'a GameState<'a>, role: PlayerRole) -> Self {
        Self { state, role }
    }

    pub fn player_state(&self, player: PlayerRole) -> &PlayerState {
        &self.state.players[player.index()]
    }

    pub fn my_hand(&self) -> &[CardId] {
        match self.role {
            PlayerRole::Player1 => &self.state.zones.hand[0],
            PlayerRole::Player2 => &self.state.zones.hand[1],
            _ => &[],
        }
    }

    pub fn hand_count(&self, player: PlayerRole) -> usize {
        match player {
            PlayerRole::Player1 => self.state.zones.hand[0].len(),
            PlayerRole::Player2 => self.state.zones.hand[1].len(),
            _ => 0,
        }
    }

    pub fn deck_count(&self, player: PlayerRole) -> usize {
        match player {
            PlayerRole::Player1 => self.state.zones.deck[0].len(),
            PlayerRole::Player2 => self.state.zones.deck[1].len(),
            _ => 0,
        }
    }

    pub fn discard_pile_count(&self, player: PlayerRole) -> usize {
        match player {
            PlayerRole::Player1 => self.state.zones.discard_pile[0].len(),
            PlayerRole::Player2 => self.state.zones.discard_pile[1].len(),
            _ => 0,
        }
    }

    pub fn field(&self, player: PlayerRole) -> &[CardId] {
        if player.index() > 1 { return &[]; }
        &self.state.zones.field[player.index()]
    }

    pub fn discard_pile(&self, player: PlayerRole) -> &[CardId] {
        if player.index() > 1 { return &[]; }
        &self.state.zones.discard_pile[player.index()]
    }

    pub fn turn(&self) -> &TurnState {
        &self.state.turn
    }

    pub fn log(&self) -> &[GameEvent] {
        match self.role {
            PlayerRole::Admin => &self.state.log,
            _ => &[],
        }
    }

    pub fn debug_hand(&self, player: PlayerRole) -> &[CardId] {
        if player.index() > 1 { return &[]; }
        match self.role {
            PlayerRole::Admin => &self.state.zones.hand[player.index()],
            _ => &[],
        }
    }

    pub fn debug_deck(&self, player: PlayerRole) -> &[CardId] {
        if player.index() > 1 { return &[]; }
        match self.role {
            PlayerRole::Admin => &self.state.zones.deck[player.index()],
            _ => &[],
        }
    }

    pub fn debug_discard_pile(&self, player: PlayerRole) -> &[CardId] {
        if player.index() > 1 { return &[]; }
        match self.role {
            PlayerRole::Admin => &self.state.zones.discard_pile[player.index()],
            _ => &[],
        }
    }
}
