#[derive(Debug, Clone)]
pub struct PlayerState {
    pub role: PlayerRole,

    pub node_count: i32
}

impl PlayerState {
    pub(crate) fn new(role: PlayerRole) -> Self {
        Self { role, node_count: 0}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlayerRole {
    Spectator,
    Player1,
    Player2,
    Admin,
}

impl PlayerRole {
    pub fn opponent(self) -> Option<PlayerRole> {
        match self {
            PlayerRole::Player1 => Some(PlayerRole::Player2),
            PlayerRole::Player2 => Some(PlayerRole::Player1),
            _ => None,
        }
    }

    pub fn index(self) -> usize {
        self as usize - 1
    }
}