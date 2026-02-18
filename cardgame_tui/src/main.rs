use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use cardgame_core::core::state::card::{CardDef, CardDefId, CardId};
use cardgame_core::core::state::game::{GameConfig, GameState, InitialTurn};
use cardgame_core::core::state::player::PlayerRole;
use cardgame_core::core::types::action::Action;
use cardgame_core::core::types::effect::Effect;
use cardgame_core::core::types::event::EventMatcher;
use cardgame_core::core::types::trigger::{Timing, TriggerDef};
use cardgame_core::engine::engine::submit_action;
use cardgame_core::rules::observe::GameView;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use image::{DynamicImage};
use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::widgets::{Clear, Wrap};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use ratatui_image::{Image, Resize};
use ratatui_image::picker::Picker;

use image::io::Reader as ImageReader;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FocusPanel {
    Hand,
    FieldP1,
    FieldP2,
}

struct App {
    game: GameState<'static>,
    role: PlayerRole,

    focus: FocusPanel,

    hand_index: usize,
    field_p1_index: usize,
    field_p2_index: usize,

    status: String,
    last_tick: Instant,

    pub show_card_popup: bool,

    pub image_cache: HashMap<CardDefId, DynamicImage>,
}

impl App {
    fn new(game: GameState<'static>, role: PlayerRole) -> Self {
        Self {
            game,
            role,
            focus: FocusPanel::Hand,
            hand_index: 0,
            field_p1_index: 0,
            field_p2_index: 0,
            status: "Ready".to_string(),
            last_tick: Instant::now(),
            show_card_popup: false,
            image_cache: HashMap::new(),
        }
    }

    fn view(&self) -> GameView<'_> {
        GameView::new(&self.game, self.role)
    }

    fn clamp_indices(&mut self) {
        let hand_len = {
            let view = self.view();
            view.my_hand().len()
        };
    
        if hand_len == 0 {
            self.hand_index = 0;
        } else if self.hand_index >= hand_len {
            self.hand_index = hand_len - 1;
        }
    
        let f1_len = {
            let view = self.view();
            view.field(PlayerRole::Player1).len()
        };
    
        if f1_len == 0 {
            self.field_p1_index = 0;
        } else if self.field_p1_index >= f1_len {
            self.field_p1_index = f1_len - 1;
        }
    
        let f2_len = {
            let view = self.view();
            view.field(PlayerRole::Player2).len()
        };
    
        if f2_len == 0 {
            self.field_p2_index = 0;
        } else if self.field_p2_index >= f2_len {
            self.field_p2_index = f2_len - 1;
        }
    }    

    fn selected_card(&self) -> Option<CardId> {
        let view = self.view();

        match self.focus {
            FocusPanel::Hand => view.my_hand().get(self.hand_index).copied(),
            FocusPanel::FieldP1 => view.field(PlayerRole::Player1).get(self.field_p1_index).copied(),
            FocusPanel::FieldP2 => view.field(PlayerRole::Player2).get(self.field_p2_index).copied(),
        }
    }

    fn move_selection_up(&mut self) {
        match self.focus {
            FocusPanel::Hand => {
                if self.hand_index > 0 {
                    self.hand_index -= 1;
                }
            }
            FocusPanel::FieldP1 => {
                if self.field_p1_index > 0 {
                    self.field_p1_index -= 1;
                }
            }
            FocusPanel::FieldP2 => {
                if self.field_p2_index > 0 {
                    self.field_p2_index -= 1;
                }
            }
        }
    }

    fn move_selection_down(&mut self) {
        let view = self.view();

        match self.focus {
            FocusPanel::Hand => {
                let len = view.my_hand().len();
                if len > 0 && self.hand_index + 1 < len {
                    self.hand_index += 1;
                }
            }
            FocusPanel::FieldP1 => {
                let len = view.field(PlayerRole::Player1).len();
                if len > 0 && self.field_p1_index + 1 < len {
                    self.field_p1_index += 1;
                }
            }
            FocusPanel::FieldP2 => {
                let len = view.field(PlayerRole::Player2).len();
                if len > 0 && self.field_p2_index + 1 < len {
                    self.field_p2_index += 1;
                }
            }
        }
    }

    fn focus_next(&mut self) {
        self.focus = match self.focus {
            FocusPanel::Hand => FocusPanel::FieldP1,
            FocusPanel::FieldP1 => FocusPanel::FieldP2,
            FocusPanel::FieldP2 => FocusPanel::Hand,
        };
    }

    fn focus_prev(&mut self) {
        self.focus = match self.focus {
            FocusPanel::Hand => FocusPanel::FieldP2,
            FocusPanel::FieldP1 => FocusPanel::Hand,
            FocusPanel::FieldP2 => FocusPanel::FieldP1,
        };
    }

    fn play_selected_hand_card(&mut self) {
        if self.focus != FocusPanel::Hand {
            self.status = "Not in hand panel (TAB to switch)".to_string();
            return;
        }

        let view = self.view();
        let Some(card_id) = view.my_hand().get(self.hand_index).copied() else {
            self.status = "No card selected".to_string();
            return;
        };

        let result = submit_action(
            &mut self.game,
            self.role,
            Action::PlayCard { card: card_id },
        );

        match result {
            Ok(_) => {
                self.status = format!("Played card {:?}", card_id);
            }
            Err(e) => {
                self.status = format!("Action error: {:?}", e);
            }
        }

        self.clamp_indices();
    }

    fn get_card_image(&mut self, card_id: CardDefId) -> Option<&DynamicImage> {
        if !self.image_cache.contains_key(&card_id) {
            let mut path = PathBuf::from("card_images");
            path.push(format!("{}.png", card_id.0));

            let img = ImageReader::open(path).ok()?.decode().ok()?;
            self.image_cache.insert(card_id, img);
        }

        self.image_cache.get(&card_id)
    }

    pub fn draw_card_popup(&mut self, f: &mut Frame, card_id: CardId, picker: &mut Picker) {
        let area = centered_rect(60, 80, f.size()); // % of screen

        // Clear behind popup
        f.render_widget(Clear, area);

        let outer = Block::default()
            .title("Card View (press c to close)")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));

        f.render_widget(outer, area);

        let inner = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // name
                Constraint::Min(10),    // image
                Constraint::Length(8),  // description/stats
            ])
            .split(inner);

        // Pull def
        let (name, desc, nodes, def) = if let Some(card) = self.game.entities.cards.get(&card_id) {
            let def = self.game.card_def(card.def);
            (def.name, def.description, def.nodes, card.def)
        } else {
            ("<missing card>", "", 0, CardDefId(0))
        };

        // Name banner
        let name_widget = Paragraph::new(name)
            .alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::ALL));

        f.render_widget(name_widget, chunks[0]);

        // Image area
        let image_block = Block::default().borders(Borders::ALL).title("Artwork");
        f.render_widget(image_block.clone(), chunks[1]);

        let image_inner = Rect {
            x: chunks[1].x + 1,
            y: chunks[1].y + 1,
            width: chunks[1].width.saturating_sub(2),
            height: chunks[1].height.saturating_sub(2),
        };

        if let Some(img) = self.get_card_image(def) {
            let proto = picker
                .new_protocol(img.clone(), image_inner, Resize::Fit)
                .ok();
        
            if let Some(proto) = proto {
                f.render_widget(Image::new(&*proto), image_inner);
            }
        }
         else {
            let fallback = Paragraph::new("(missing image)")
                .alignment(Alignment::Center)
                .block(Block::default());

            f.render_widget(fallback, image_inner);
        }

        // Description + stats
        let info_text = format!(
            "{}\n\nNodes: {}",
            desc,
            nodes
        );

        let info_widget = Paragraph::new(info_text)
            .wrap(Wrap { trim: false })
            .block(Block::default().borders(Borders::ALL).title("Info"));

        f.render_widget(info_widget, chunks[2]);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    let vertical = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1]);

    vertical[1]
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    static ALL_CARDS: &[CardDef] = &[
        CardDef {
            name: "Coll",
            description: "Draw 1.",
            nodes: 3,
            triggers: &[],
        },
        CardDef {
            name: "Max and Nick",
            description: "Play a Red card.",
            nodes: -1,
            triggers: &[],
        },
        CardDef {
            name: "Kurotama",
            description: "Discard a card from your field.",
            nodes: 3,
            triggers: &[],
        },
    ];

    let p1_deck = vec![0, 1, 2, 1, 0, 1, 2, 0, 1, 1, 1, 2, 0, 2, 2, 0, 0, 0, 2].iter().map(|id| CardDefId(*id)).collect();
    let p2_deck = vec![1, 0, 2, 1, 2, 0, 1, 0, 1, 2, 1, 1, 2, 2, 0, 0, 0, 0, 1].iter().map(|id| CardDefId(*id)).collect();

    let config = GameConfig {
        seed: 2,
        card_defs: ALL_CARDS,
        p1_deck,
        p2_deck,
        initial_turn: InitialTurn::CoinFlip
    };

    let game = GameState::new(config);

    let mut app = App::new(game, PlayerRole::Player1);

    let tick_rate = Duration::from_millis(100);

    let res = run_app(&mut terminal, &mut app, tick_rate);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    tick_rate: Duration,
) -> Result<(), io::Error> {
    loop {
        terminal.draw(|f| ui(f, app)).unwrap();

        let timeout = tick_rate
            .checked_sub(app.last_tick.elapsed())
            .unwrap_or(Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),

                        KeyCode::Tab => app.focus_next(),
                        KeyCode::BackTab => app.focus_prev(),

                        KeyCode::Up | KeyCode::Char('k') => app.move_selection_up(),
                        KeyCode::Down | KeyCode::Char('j') => app.move_selection_down(),

                        KeyCode::Enter => app.play_selected_hand_card(),

                        KeyCode::Char('r') => {
                            app.status = "Refreshed".to_string();
                            app.clamp_indices();
                        }

                        KeyCode::Char('c') => {
                            app.show_card_popup = !app.show_card_popup;
                        }

                        _ => {}
                    }
                }
            }
        }

        if app.last_tick.elapsed() >= tick_rate {
            app.last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let mut picker = Picker::from_termios().unwrap();

    let view = app.view();

    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // header
            Constraint::Min(10),    // main
            Constraint::Length(7),  // hand
            Constraint::Length(5),  // inspector
            Constraint::Length(3),  // status
        ])
        .split(f.size());

    draw_header(f, app, &view, outer[0]);
    draw_main_fields(f, app, &view, outer[1]);
    draw_hand(f, app, &view, outer[2]);
    draw_inspector(f, app, outer[3]);
    draw_status(f, app, outer[4]);

    if app.show_card_popup {
        if let Some(card_id) = app.selected_card() {
            app.draw_card_popup(f, card_id, &mut picker);
        }
    }
    
}

fn draw_header(f: &mut ratatui::Frame, app: &App, view: &GameView<'_>, area: Rect) {
    let turn = view.turn();

    let title = format!(
        "Cardgame | Role: {:?} | Active player: {:?}",
        app.role, turn.active_player
    );

    let block = Block::default().title(title).borders(Borders::ALL);

    f.render_widget(block, area);
}

fn draw_main_fields(f: &mut ratatui::Frame, app: &App, view: &GameView<'_>, area: Rect) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    draw_player_panel(f, app, view, PlayerRole::Player1, cols[0]);
    draw_player_panel(f, app, view, PlayerRole::Player2, cols[1]);
}

fn draw_player_panel(
    f: &mut ratatui::Frame,
    app: &App,
    view: &GameView<'_>,
    player: PlayerRole,
    area: Rect,
) {
    let field = view.field(player);

    let is_focused = match (app.focus, player) {
        (FocusPanel::FieldP1, PlayerRole::Player1) => true,
        (FocusPanel::FieldP2, PlayerRole::Player2) => true,
        _ => false,
    };

    let title = format!(
        "{:?} | Nodes: {} | Deck: {} | Discard: {} | Hand: {}",
        player,
        view.player_state(player).node_count,
        view.deck_count(player),
        view.discard_pile_count(player),
        view.hand_count(player),
    );

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(if is_focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    let items: Vec<ListItem> = field
        .iter()
        .enumerate()
        .map(|(i, id)| {
            let mut line = format!("{:?}", id);

            if let Some(card) = app.game.entities.cards.get(id) {
                let def = app.game.card_def(card.def);
                line = format!("{} ({}, nodes {}, tokens {})", def.name, def.description, card.nodes, card.tokens);
            }

            let mut style = Style::default();
            if is_focused {
                let selected_index = if player == PlayerRole::Player1 {
                    app.field_p1_index
                } else {
                    app.field_p2_index
                };

                if i == selected_index {
                    style = style.add_modifier(Modifier::REVERSED);
                }
            }

            ListItem::new(line).style(style)
        })
        .collect();

    let list = List::new(items).block(block);

    f.render_widget(list, area);
}

fn draw_hand(f: &mut ratatui::Frame, app: &App, view: &GameView<'_>, area: Rect) {
    let hand = view.my_hand();

    let is_focused = app.focus == FocusPanel::Hand;

    let block = Block::default()
        .title("My Hand (ENTER to Play)")
        .borders(Borders::ALL)
        .border_style(if is_focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    let items: Vec<ListItem> = hand
        .iter()
        .enumerate()
        .map(|(i, id)| {
            let mut line = format!("{:?}", id);

            if let Some(card) = app.game.entities.cards.get(id) {
                let def = app.game.card_def(card.def);
                line = format!("{} - {} (nodes {})", def.name, def.description, def.nodes);
            }

            let mut style = Style::default();
            if is_focused && i == app.hand_index {
                style = style.add_modifier(Modifier::REVERSED);
            }

            ListItem::new(line).style(style)
        })
        .collect();

    let list = List::new(items).block(block);

    f.render_widget(list, area);
}

fn draw_inspector(f: &mut ratatui::Frame, app: &App, area: Rect) {
    let block = Block::default().title("Inspector").borders(Borders::ALL);

    let mut lines: Vec<Line> = vec![];

    if let Some(card_id) = app.selected_card() {
        lines.push(Line::from(vec![
            Span::styled("CardId: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(format!("{:?}", card_id)),
        ]));

        if let Some(card) = app.game.entities.cards.get(&card_id) {
            let def = app.game.card_def(card.def);

            lines.push(Line::from(vec![
                Span::styled("Name: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(def.name),
            ]));

            lines.push(Line::from(vec![
                Span::styled("Nodes: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(def.nodes.to_string()),
                Span::styled(" (actual: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(card.nodes.to_string()),
                Span::styled(")", Style::default().add_modifier(Modifier::BOLD)),
            ]));

            lines.push(Line::from(vec![
                Span::styled("Tokens: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(card.tokens.to_string()),
            ]));

            lines.push(Line::from(vec![
                Span::styled("Description: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(def.description),
            ]));

            lines.push(Line::from(vec![
                Span::styled("Trigger count: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(def.triggers.len().to_string()),
            ]));
        } else {
            lines.push(Line::from("Card not found in entity store."));
        }
    } else {
        lines.push(Line::from("No card selected."));
    }

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

fn draw_status(f: &mut ratatui::Frame, app: &App, area: Rect) {
    let help = "TAB switch panel | ↑↓ move | ENTER play (hand) | q quit";
    let text = format!("{} | {}", app.status, help);

    let block = Block::default().title("Status").borders(Borders::ALL);
    let p = Paragraph::new(text).block(block);

    f.render_widget(p, area);
}
