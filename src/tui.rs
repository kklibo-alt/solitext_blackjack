use crate::app::{App, GameKey, Screen};
use crate::draw::Draw;
use crate::game_state::GameMode;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

pub struct Ui {
    app: App,
    draw: Draw,
}

impl Ui {
    pub fn new(app: App) -> Self {
        Self {
            app,
            draw: Draw::new(),
        }
    }

    fn read_key_event() -> event::KeyEvent {
        loop {
            if let Ok(Event::Key(key)) = event::read()
                && key.kind == KeyEventKind::Press
            {
                return key;
            }
        }
    }

    fn convert_key(code: KeyCode) -> Option<GameKey> {
        Some(match code {
            KeyCode::Left => GameKey::Left,
            KeyCode::Right => GameKey::Right,
            KeyCode::Up => GameKey::Up,
            KeyCode::Down => GameKey::Down,
            KeyCode::Home => GameKey::Home,
            KeyCode::End => GameKey::End,
            KeyCode::Enter => GameKey::Enter,
            KeyCode::Esc => GameKey::Esc,
            KeyCode::Char(c) => GameKey::Char(c),
            _ => return None,
        })
    }

    fn is_ctrl_c(key: &event::KeyEvent) -> bool {
        key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL)
    }

    fn run_game(&mut self) {
        self.app.update();
        if self.app.screen != Screen::Game {
            return;
        }
        self.draw.display_game_state(&self.app);

        loop {
            let key = Self::read_key_event();
            if Self::is_ctrl_c(&key) {
                self.app.screen = Screen::Quit;
                break;
            }
            if let Some(game_key) = Self::convert_key(key.code) {
                self.app.handle_game_key(game_key);
            } else {
                self.app.update();
            }
            if self.app.screen != Screen::Game {
                break;
            }
            self.draw.display_game_state(&self.app);
        }
    }

    fn run_start_screen(&mut self) {
        self.draw.display_start_screen();
        loop {
            let key = Self::read_key_event();
            if Self::is_ctrl_c(&key) || key.code == KeyCode::Esc {
                self.app.screen = Screen::Quit;
                break;
            }
            match key.code {
                KeyCode::Char('1') => {
                    self.app.new_game(GameMode::SingleDeck);
                    break;
                }
                KeyCode::Char('6') => {
                    self.app.new_game(GameMode::SixDeck);
                    break;
                }
                _ => {}
            }
        }
    }

    fn run_game_menu(&mut self) {
        self.draw.display_game_menu(&self.app);
        loop {
            let key = Self::read_key_event();
            if Self::is_ctrl_c(&key) {
                self.app.screen = Screen::Quit;
                break;
            }
            match key.code {
                KeyCode::Char('1') => {
                    self.app.new_game(GameMode::SingleDeck);
                    break;
                }
                KeyCode::Char('6') => {
                    self.app.new_game(GameMode::SixDeck);
                    break;
                }
                KeyCode::Char('q') => {
                    self.app.screen = Screen::Quit;
                    break;
                }
                KeyCode::Esc => {
                    self.app.screen = Screen::Game;
                    break;
                }
                _ => {}
            }
        }
    }

    fn run_help(&mut self) {
        self.draw.display_help(&self.app);
        Self::read_key_event();
        self.app.screen = Screen::Game;
    }

    fn run_game_over(&mut self) {
        self.draw.display_game_over(&self.app);
        loop {
            let key = Self::read_key_event();
            if Self::is_ctrl_c(&key) {
                self.app.screen = Screen::Quit;
                break;
            }
            match key.code {
                KeyCode::Char('y') => {
                    self.app.new_game(self.app.game_state.game_mode);
                    break;
                }
                KeyCode::Char('n') | KeyCode::Esc => {
                    self.app.screen = Screen::Quit;
                    break;
                }
                _ => {}
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.app.screen {
                Screen::Start => self.run_start_screen(),
                Screen::Game => self.run_game(),
                Screen::GameMenu => self.run_game_menu(),
                Screen::Help => self.run_help(),
                Screen::GameOver => self.run_game_over(),
                Screen::Quit => break,
            }
        }

        self.draw.restore();
        println!("thanks for playing. the dealer will see you next time.");
    }
}
