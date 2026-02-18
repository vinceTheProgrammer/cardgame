use crate::{core::{state::{card::{CardDef, CardDefId}, entities::EntityStore, player::{PlayerRole, PlayerState}, utils::spawn_card, zones::{Zone, Zones}}, types::{event::GameEvent, rng::DeterministicRng, stack::StackItem}}, engine::dispatch::dispatch_event};

#[derive(Debug)]
pub struct GameState<'a> {
    pub turn: TurnState,
    pub players: [PlayerState; 2],

    pub card_defs: &'a [CardDef],


    pub zones: Zones,
    pub entities: EntityStore, // all cards/units by CardId

    pub next_card_id: usize,

    pub rng: DeterministicRng,

    pub stack: Vec<StackItem>, // LIFO
    pub log: Vec<GameEvent>,   // for replay/debug
}

pub enum InitialTurn {
    Player1,
    Player2,
    CoinFlip,
}

pub struct GameConfig<'a> {
    pub seed: u64,
    pub card_defs: &'a [CardDef],
    pub p1_deck: Vec<CardDefId>,
    pub p2_deck: Vec<CardDefId>,
    pub initial_turn: InitialTurn,
}
 
impl<'a> GameState<'a> {
    pub fn new(config: GameConfig<'a>) -> Self {

        let mut player_role = PlayerRole::Player1;
        let mut rng = DeterministicRng::new(config.seed);

        match config.initial_turn {
            InitialTurn::Player2 => player_role = PlayerRole::Player2,
            InitialTurn::CoinFlip => {
                let heads = rng.coin_flip();
                if !heads {player_role = PlayerRole::Player2};
            },
            _ => { player_role = PlayerRole::Player1}
        }

        let mut state = Self { 
            turn: TurnState::new(player_role), 
            players: [PlayerState::new(PlayerRole::Player1), PlayerState::new(PlayerRole::Player2)], 
            zones: Zones::new(), 
            entities: EntityStore::new(),
            next_card_id: 0,
            rng, 
            stack: Vec::new(), 
            log: Vec::new(),
            card_defs: config.card_defs,  
        };
    
        // create players
        state.players = [
            PlayerState::new(PlayerRole::Player1),
            PlayerState::new(PlayerRole::Player2),
        ];
    
        // spawn deck cards
        for def in config.p1_deck {
            let id = spawn_card(&mut state, def, PlayerRole::Player1, Zone::Deck(PlayerRole::Player1));
            state.zones.get_zone_mut(Zone::Deck(PlayerRole::Player1)).push(id);
        }
    
        for def in config.p2_deck {
            let id = spawn_card(&mut state, def, PlayerRole::Player2, Zone::Deck(PlayerRole::Player2));
            state.zones.get_zone_mut(Zone::Deck(PlayerRole::Player2)).push(id);
        }
    
        // shuffle decks deterministically
        state.rng.shuffle(&mut state.zones.get_zone_mut(Zone::Deck(PlayerRole::Player1)));
        state.rng.shuffle(&mut state.zones.get_zone_mut(Zone::Deck(PlayerRole::Player2)));
    
        // draw starting hands
        let amount_to_draw = 5;
        dispatch_event(&mut state, GameEvent::CardsDrawn { player: PlayerRole::Player1, amount: amount_to_draw });
        dispatch_event(&mut state, GameEvent::CardsDrawn { player: PlayerRole::Player2, amount: amount_to_draw });
    
        // choose starting player deterministically
        let start_player = if state.rng.next_u32() % 2 == 0 {
            PlayerRole::Player1
        } else {
            PlayerRole::Player2
        };
    
        state.turn = TurnState::new(start_player);
    
        state
    }

    pub fn card_def(&self, card_def: CardDefId) -> &CardDef {
        &self.card_defs[card_def.0]
    }
}

#[derive(Debug, Clone)]
pub struct TurnState {
    pub active_player: PlayerRole,
    pub turn_number: u32,

    // useful for rules:
    pub actions_remaining: u32,
    pub has_ended: bool,
}

impl TurnState {
    pub(crate) fn new(starting: PlayerRole) -> Self {
        Self {
            active_player: starting,
            turn_number: 1,
            actions_remaining: 1,
            has_ended: false,
        }
    }
}