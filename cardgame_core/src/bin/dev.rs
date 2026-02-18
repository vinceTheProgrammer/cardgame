
use cardgame_core::{core::{state::{card::{CardDef, CardDefId}, game::{GameConfig, GameState, InitialTurn}, player::PlayerRole}, types::{action::Action, effect::Effect, event::EventMatcher, trigger::{Timing, TriggerDef}}}, engine::engine::submit_action, rules::observe::GameView};

fn main() {
    static ALL_CARDS: &[CardDef] = &[
        // example card defs go here
        CardDef {
            name: "Test card 1",
            description: "test card 1.",
            nodes: 2,
            triggers: &[
                TriggerDef {
                    timing: Timing::After,
                    matcher: EventMatcher::CardMoved,
                    effect: Effect::DiscardCard { amount: 1 }
                }
            ],
        },
        CardDef {
            name: "Test card 2",
            description: "test card 2.",
            nodes: 1,
            triggers: &[],
        },
        CardDef {
            name: "Test card 3",
            description: "test card 3.",
            nodes: 5,
            triggers: &[],
        },
    ];

    let p1_deck = vec![0, 1, 2, 1, 0, 1, 2, 0].iter().map(|id| CardDefId(*id)).collect();
    let p2_deck = vec![1, 0, 2, 1, 2, 0, 1, 0].iter().map(|id| CardDefId(*id)).collect();

    let config = GameConfig {
        seed: 0,
        card_defs: ALL_CARDS,
        p1_deck,
        p2_deck,
        initial_turn: InitialTurn::CoinFlip
    };

    let mut game = GameState::new(config);

    print_game(&game);

    let p1_view = GameView::new(&game, PlayerRole::Player1);

    let valid_card_id = p1_view.my_hand()[0];

    if let Some(card) = game.entities.cards.get(&valid_card_id) {
        let def_id = card.def;
        let def = game.card_def(def_id);
    }

    submit_action(&mut game, PlayerRole::Player1, Action::PlayCard { card: valid_card_id }).unwrap();

    print_game(&game);
}


fn print_game(game: &GameState) {
    let view = GameView::new(&game, PlayerRole::Admin);

    println!("field 1: {:?}\n", view.field(PlayerRole::Player1));
    println!("field 2: {:?}\n", view.field(PlayerRole::Player2));

    println!("hand 1: {:?}\n", view.debug_hand(PlayerRole::Player1));
    println!("deck 1: {:?}\n", view.debug_deck(PlayerRole::Player1));
    println!("discard_pile 1: {:?}\n", view.debug_discard_pile(PlayerRole::Player1));

    println!("hand 2: {:?}\n", view.debug_hand(PlayerRole::Player1));
    println!("deck 2: {:?}\n", view.debug_deck(PlayerRole::Player1));
    println!("discard_pile 2: {:?}\n", view.debug_discard_pile(PlayerRole::Player1));

    println!("turn: {:?}\n-------------------", view.turn());
}