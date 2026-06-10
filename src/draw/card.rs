use super::Renderer;
use crate::cards::Card;
use crate::game_state::Hand;
use ratatui::style::Color;

impl Renderer<'_> {
    pub(super) const CARD_STEP: usize = 5;

    /// Draw a single face-up card cell, e.g. `A♠`, on a light background.
    fn display_card(&mut self, card: Card, col: usize, row: usize) {
        if card.suit.is_red() {
            self.set_colors(Color::Red, Color::Gray);
        } else {
            self.set_colors(Color::Black, Color::Gray);
        }
        self.draw_text(col, row, format!(" {:<3}", card.to_string()).as_str());
    }

    /// Draw a face-down card cell.
    fn display_card_back(&mut self, col: usize, row: usize) {
        self.set_colors(Color::LightGreen, Color::DarkGray);
        self.draw_text(col, row, " ▒▒ ");
    }

    /// Lay a hand out left-to-right. When `hide_hole` is set, the second card
    /// (the dealer's hole card) is drawn face down.
    pub(super) fn display_hand(&mut self, hand: &Hand, hide_hole: bool, init_col: usize, row: usize) {
        for (index, card) in hand.0.iter().enumerate() {
            let col = init_col + index * Self::CARD_STEP;
            if hide_hole && index == 1 {
                self.display_card_back(col, row);
            } else {
                self.display_card(*card, col, row);
            }
        }
    }
}
