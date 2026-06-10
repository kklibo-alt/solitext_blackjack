use super::Renderer;
use ratatui::style::{Color, Style};

impl Renderer<'_> {
    pub(crate) fn default_bg() -> Color {
        Color::Black
    }

    pub(crate) fn default_fg() -> Color {
        Color::White
    }

    pub(crate) fn set_colors(&mut self, fg: Color, bg: Color) {
        self.style = Style::default().fg(fg).bg(bg);
    }

    pub(crate) fn draw_text(&mut self, col: usize, row: usize, text: &str) {
        let x = col.saturating_sub(1) as u16;
        let y = row.saturating_sub(1) as u16;
        self.buf.set_string(x, y, text, self.style);
    }

    pub(crate) fn draw_box(&mut self, col1: usize, row1: usize, col2: usize, row2: usize) {
        use std::cmp::{max, min};
        for col in min(col1, col2)..=max(col1, col2) {
            for row in min(row1, row2)..=max(row1, row2) {
                self.draw_text(col, row, "█");
            }
        }
    }

    fn centered_box_corners(width: usize, height: usize) -> (usize, usize, usize, usize) {
        const CENTER: (usize, usize) = (26, 8);
        (
            CENTER.0.saturating_sub(width / 2),
            CENTER.1.saturating_sub(height / 2),
            CENTER.0 + width / 2,
            CENTER.1 + height / 2,
        )
    }

    fn draw_centered_box(&mut self, width: usize, height: usize) {
        let (col1, row1, col2, row2) = Self::centered_box_corners(width, height);
        self.draw_box(col1, row1, col2, row2);
    }

    pub(crate) fn draw_text_box(&mut self, lines: &str) {
        let height = lines.split('\n').count();

        const WIDTH: usize = 38;
        self.set_colors(Color::LightGreen, Self::default_bg());
        self.draw_centered_box(WIDTH, height + 2);
        self.set_colors(Color::Gray, Self::default_bg());
        self.draw_centered_box(WIDTH - 2, height);

        self.set_colors(Color::DarkGray, Color::Gray);
        let (col, mut row, _, _) = Self::centered_box_corners(WIDTH - 2, height);

        for line in lines.split('\n') {
            self.draw_text(col, row, line);
            row += 1;
        }
    }
}
