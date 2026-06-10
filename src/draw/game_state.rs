use super::Renderer;
use crate::app::App;
use crate::game_state::{GameState, Hand, Phase};
use crate::selection::PlayerAction;
use ratatui::style::Color;
use ratatui::Frame;

const TITLE_ROW: usize = 1;
const DEALER_LABEL_ROW: usize = 3;
const DEALER_CARD_ROW: usize = 4;
const PLAYER_LABEL_ROW: usize = 7;
const PLAYER_CARD_ROW: usize = 8;
const STATUS_ROW: usize = 11;
const ACTION_ROW: usize = 13;
const MESSAGE_ROW: usize = 15;
const HAND_INIT_COL: usize = 2;

pub(crate) fn render_game(frame: &mut Frame, app: &App) {
    let buf = frame.buffer_mut();
    let mut r = Renderer::new(buf, app.cursor, app.debug_mode);
    r.clear();

    r.display_header();
    r.display_table(&app.game_state);
    r.display_status(&app.game_state);
    r.display_controls(&app.game_state);

    let message_fg = match app.game_state.outcome {
        Some(outcome) if outcome.player_won() => Color::LightGreen,
        Some(_) => Color::LightRed,
        None => Color::LightYellow,
    };
    r.set_colors(message_fg, Renderer::default_bg());
    r.draw_text(HAND_INIT_COL, MESSAGE_ROW, &app.message);

    if r.debug_mode {
        r.set_colors(Color::DarkGray, Renderer::default_bg());
        r.draw_text(
            HAND_INIT_COL,
            MESSAGE_ROW + 1,
            &format!("debug: {} cards left in shoe", app.game_state.shoe.len()),
        );
    }

    r.set_colors(Renderer::default_fg(), Renderer::default_bg());
}

#[cfg(feature = "native")]
impl super::Draw {
    pub fn display_game_state(&mut self, app: &App) {
        self.terminal
            .draw(|frame| {
                render_game(frame, app);
            })
            .unwrap();
    }
}

impl Renderer<'_> {
    pub(super) fn display_header(&mut self) {
        self.set_colors(Color::LightYellow, Self::default_bg());
        self.draw_text(1, TITLE_ROW, "Blacktext  ♠ ♥ ♦ ♣");
        self.set_colors(Color::DarkGray, Self::default_bg());
        self.draw_text(32, TITLE_ROW, "?: Help  Esc: Menu");
    }

    /// Dealer and player hands with their running totals.
    pub(super) fn display_table(&mut self, game_state: &GameState) {
        let hidden = game_state.hole_hidden;

        self.set_colors(Color::Gray, Self::default_bg());
        let dealer_total = if hidden {
            // Only the upcard is known to the player.
            let up = Hand(game_state.dealer.0.iter().take(1).copied().collect());
            if game_state.dealer.0.is_empty() {
                "Dealer".to_string()
            } else {
                format!("Dealer: {} + ?", up.total())
            }
        } else if game_state.dealer.0.is_empty() {
            "Dealer".to_string()
        } else {
            format!("Dealer: {}", game_state.dealer.total())
        };
        self.draw_text(HAND_INIT_COL, DEALER_LABEL_ROW, &dealer_total);
        self.display_hand(&game_state.dealer, hidden, HAND_INIT_COL, DEALER_CARD_ROW);

        self.set_colors(Color::Gray, Self::default_bg());
        let player_total = if game_state.player.0.is_empty() {
            "You".to_string()
        } else {
            format!("You: {}", game_state.player.total())
        };
        self.draw_text(HAND_INIT_COL, PLAYER_LABEL_ROW, &player_total);
        self.display_hand(&game_state.player, false, HAND_INIT_COL, PLAYER_CARD_ROW);
    }

    /// Bankroll, current wager, and table rules.
    pub(super) fn display_status(&mut self, game_state: &GameState) {
        self.set_colors(Color::LightGreen, Self::default_bg());
        self.draw_text(
            HAND_INIT_COL,
            STATUS_ROW,
            &format!("Chips: {}", game_state.bankroll),
        );
        self.set_colors(Color::LightCyan, Self::default_bg());
        self.draw_text(18, STATUS_ROW, &format!("Bet: {}", game_state.bet));
        self.set_colors(Color::DarkGray, Self::default_bg());
        self.draw_text(34, STATUS_ROW, game_state.game_mode.label());
    }

    /// Phase-specific controls: bet selector, action buttons, or next prompt.
    pub(super) fn display_controls(&mut self, game_state: &GameState) {
        match game_state.phase {
            Phase::Betting => {
                self.set_colors(Color::White, Color::Blue);
                self.draw_text(
                    HAND_INIT_COL,
                    ACTION_ROW,
                    &format!("  ◂ Bet {} ▸     Deal  ", game_state.bet),
                );
            }
            Phase::PlayerTurn => {
                let actions = PlayerAction::available(game_state);
                let mut col = HAND_INIT_COL;
                for (index, action) in actions.iter().enumerate() {
                    let selected = index == self.cursor;
                    if selected {
                        self.set_colors(Color::Black, Color::LightYellow);
                    } else {
                        self.set_colors(Color::White, Color::Blue);
                    }
                    let label = format!(" {} ", action.label());
                    self.draw_text(col, ACTION_ROW, &label);
                    col += label.chars().count() + 2;
                }
            }
            Phase::RoundOver => {
                self.set_colors(Color::Black, Color::LightYellow);
                self.draw_text(HAND_INIT_COL, ACTION_ROW, "  Next Round  ");
            }
        }
    }
}
