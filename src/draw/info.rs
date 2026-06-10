use super::Renderer;
use crate::app::App;
use crate::game_state::GameState;
use ratatui::style::Color;
use ratatui::Frame;

fn render_base_game(r: &mut Renderer, game_state: &GameState) {
    r.display_header();
    r.display_table(game_state);
    r.display_status(game_state);
}

pub(crate) fn render_start_screen(frame: &mut Frame) {
    let buf = frame.buffer_mut();
    let mut r = Renderer::new(buf, 0, false);
    r.clear();

    r.set_colors(Color::LightYellow, Renderer::default_bg());
    r.draw_text(15, 1, "Blacktext   ♠ ♥ ♦ ♣");

    let lines = r#"1: New Game (Single Deck)
6: New Game (Six Deck Shoe)
Esc: Quit"#;
    r.draw_text_box(lines);

    r.set_colors(Renderer::default_fg(), Renderer::default_bg());
}

pub(crate) fn render_game_menu(frame: &mut Frame, app: &App) {
    let buf = frame.buffer_mut();
    let mut r = Renderer::new(buf, app.cursor, app.debug_mode);
    r.clear();

    render_base_game(&mut r, &app.game_state);

    let lines = r#"1: New Game (Single Deck)
6: New Game (Six Deck Shoe)
q: Quit
Esc: Return to game"#;
    r.draw_text_box(lines);

    r.set_colors(Renderer::default_fg(), Renderer::default_bg());
}

pub(crate) fn render_help(frame: &mut Frame, app: &App) {
    let buf = frame.buffer_mut();
    let mut r = Renderer::new(buf, app.cursor, app.debug_mode);
    r.clear();

    render_base_game(&mut r, &app.game_state);

    let lines = r#"How to play:

 Reach 21 without going over.
 Beat the dealer's hand to win.
 Blackjack pays 3:2.

 ←/→: Bet / choose action
 Enter: Deal / hit-stand / next
 Esc: Menu   ?: Help"#;
    r.draw_text_box(lines);

    r.set_colors(Renderer::default_fg(), Renderer::default_bg());
}

pub(crate) fn render_game_over(frame: &mut Frame, app: &App) {
    let buf = frame.buffer_mut();
    let mut r = Renderer::new(buf, 0, app.debug_mode);
    r.clear();

    render_base_game(&mut r, &app.game_state);

    let lines = r#"Out of chips!

The house always wins.

y: Play again
n: Quit"#;
    r.draw_text_box(lines);

    r.set_colors(Renderer::default_fg(), Renderer::default_bg());
}

#[cfg(feature = "native")]
impl super::Draw {
    pub fn display_start_screen(&mut self) {
        self.terminal
            .draw(|frame| render_start_screen(frame))
            .unwrap();
    }

    pub fn display_game_menu(&mut self, app: &App) {
        self.terminal
            .draw(|frame| render_game_menu(frame, app))
            .unwrap();
    }

    pub fn display_help(&mut self, app: &App) {
        self.terminal.draw(|frame| render_help(frame, app)).unwrap();
    }

    pub fn display_game_over(&mut self, app: &App) {
        self.terminal
            .draw(|frame| render_game_over(frame, app))
            .unwrap();
    }
}
