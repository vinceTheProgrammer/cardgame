use crate::{color::Color, core::{state::{player::PlayerRole, zones::Zone}, types::trigger::TriggerDef}};

#[derive(Debug, Clone)]
pub struct CardState {
    pub id: CardId,
    pub def: CardDefId,
    pub owner: PlayerRole,
    pub zone: Zone,
    pub nodes: i32,
    pub tokens: u32,
    pub face_dir: FaceDirection
}

#[derive(Debug, Clone)]
pub enum FaceDirection {
    Up,
    Down
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CardId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CardDefId(pub usize);

#[derive(Debug)]
pub struct CardDef {
    pub name: &'static str,
    pub description: &'static str,
    pub creator: &'static str,
    pub color: CardColor,
    pub nodes: i32,
    pub triggers: &'static [TriggerDef],
}

#[derive(Debug)]
pub enum CardColor {
    Red,
    Green,
    Blue,
    Yellow
}

impl CardColor {
    pub fn to_color(self) -> Color {
        match self {
            CardColor::Red => Color::RED,
            CardColor::Green => Color::GREEN,
            CardColor::Blue => Color::BLUE,
            CardColor::Yellow => Color::YELLOW,
        }
    }
}