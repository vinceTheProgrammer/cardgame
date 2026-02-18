use crate::core::state::{card::{CardDefId, CardId, CardState, FaceDirection}, game::GameState, player::PlayerRole, zones::{Zone, push_to_zone, remove_from_zone}};

pub(crate) fn find_card_zone(state: &GameState, card: CardId) -> Option<Zone> {
    for &p in &[PlayerRole::Player1, PlayerRole::Player2] {
        if state.zones.deck[p.index()].contains(&card) {
            return Some(Zone::Deck(p));
        }
        if state.zones.hand[p.index()].contains(&card) {
            return Some(Zone::Hand(p));
        }
        if state.zones.field[p.index()].contains(&card) {
            return Some(Zone::Field(p));
        }
        if state.zones.discard_pile[p.index()].contains(&card) {
            return Some(Zone::DiscardPile(p));
        }
    }
    None
}

pub(crate) fn sync_nodes(state: &mut GameState) {
    sync_player_nodes(state, PlayerRole::Player1);
    sync_player_nodes(state, PlayerRole::Player2);
}

pub(crate) fn sync_player_nodes(state: &mut GameState, player: PlayerRole) {
    let field = &state.zones.field[player.index()];

    let node_count = field.iter().map(|card_id| state.entities.cards.get(card_id)).map(|card| {
        match card {
            Some(card_state) => card_state.nodes,
            None => 0
        }
    }).sum();

    state.players[player.index()].node_count = node_count;
}

pub(crate) fn spawn_card(
    state: &mut GameState,
    def_id: CardDefId,
    owner: PlayerRole,
    zone: Zone,
) -> CardId {
    let card_id = CardId(state.next_card_id);
    state.next_card_id += 1;

    let def = &state.card_def(def_id);

    let card_state = CardState {
        id: card_id,
        def: def_id,
        owner,
        zone,
        nodes: def.nodes,
        tokens: 0,
        face_dir: FaceDirection::Down
    };

    state.entities.cards.insert(card_id, card_state);

    card_id
}

pub(crate) fn draw_card(state: &mut GameState, player: PlayerRole) -> Option<CardId> {
    let zones = state.zones.clone();
    let deck = zones.get_zone(Zone::Deck(player));

    let Some(card) = deck.last() else {
        return None;
    };

    Some(*card)
}

pub(crate) fn start_turn(state: &mut GameState, player: PlayerRole) {
    state.turn.active_player = player;
    state.turn.turn_number += 1;

    draw_card(state, player);
}

pub(crate) fn end_turn(state: &mut GameState) {
    let player = state.turn.active_player;

    let next_player = if player == PlayerRole::Player1 {
        PlayerRole::Player2
    } else {
        PlayerRole::Player1
    };

    start_turn(state, next_player);
}

pub(crate) fn move_card(state: &mut GameState, card: CardId, from: Zone, to: Zone) {
    let removed = remove_from_zone(&mut state.zones, from, card);
    if !removed {
        // If this happens, your state is inconsistent.
        // In a real engine you'd return Result. // TODO
        return;
    }

    push_to_zone(&mut state.zones, to, card);

    if let Some(cs) = state.entities.cards.get_mut(&card) {
        cs.zone = to;
    }
}

pub(crate) fn set_card_face_dir(state: &mut GameState, card_id: CardId, face_dir: FaceDirection) {
    if let Some(card) = state.entities.cards.get_mut(&card_id) {
        card.face_dir = face_dir;
    }
}

pub(crate) fn get_card_face_dir(state: &mut GameState, card_id: CardId) -> FaceDirection {
    state.entities.cards.get(&card_id).unwrap().face_dir.clone() // unwrap because uhh card should always be defined ig.... check this later. I'm tired rn
}

pub(crate) fn discard_card(state: &mut GameState, card: CardId) -> bool {
    let Some(cs) = state.entities.cards.get(&card) else {
        return false;
    };

    let controller = match cs.zone {
        Zone::Deck(player_id) => player_id,
        Zone::Hand(player_id) => player_id,
        Zone::Field(player_id) => player_id,
        _ => return false
    };

    true
}