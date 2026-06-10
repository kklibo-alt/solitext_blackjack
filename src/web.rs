use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use crate::app::{App, GameKey, Screen};
use crate::draw::{render_game, render_game_menu, render_game_over, render_help, render_start_screen};
use crate::game_state::GameMode;
use ratzilla::event::KeyCode;
use ratzilla::ratatui::Terminal;
use ratzilla::{DomBackend, WebRenderer};

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

pub fn run() -> io::Result<()> {
    let app = Rc::new(RefCell::new(App::new()));

    let backend = DomBackend::new()?;
    let mut terminal = Terminal::new(backend)?;

    let _ = terminal.on_key_event({
        let app = app.clone();
        move |event| {
            let mut app = app.borrow_mut();

            if app.screen == Screen::Quit {
                app.screen = Screen::Start;
            }

            let Some(key) = convert_key(event.code) else {
                if app.screen == Screen::Game {
                    app.update();
                }
                return;
            };

            match app.screen {
                Screen::Start => match key {
                    GameKey::Char('1') => app.new_game(GameMode::SingleDeck),
                    GameKey::Char('6') => app.new_game(GameMode::SixDeck),
                    _ => {}
                },
                Screen::Game => {
                    app.handle_game_key(key);
                }
                Screen::GameMenu => match key {
                    GameKey::Char('1') => app.new_game(GameMode::SingleDeck),
                    GameKey::Char('6') => app.new_game(GameMode::SixDeck),
                    GameKey::Char('q') => app.screen = Screen::Start,
                    GameKey::Esc => app.screen = Screen::Game,
                    _ => {}
                },
                Screen::Help => {
                    app.screen = Screen::Game;
                }
                Screen::GameOver => match key {
                    GameKey::Char('y') => {
                        let mode = app.game_state.game_mode;
                        app.new_game(mode);
                    }
                    GameKey::Char('n') | GameKey::Esc => app.screen = Screen::Start,
                    _ => {}
                },
                Screen::Quit => {}
            }
        }
    });

    terminal.draw_web({
        let app = app.clone();
        move |frame| {
            let app = app.borrow();
            match app.screen {
                Screen::Start | Screen::Quit => render_start_screen(frame),
                Screen::Game => render_game(frame, &app),
                Screen::GameMenu => render_game_menu(frame, &app),
                Screen::Help => render_help(frame, &app),
                Screen::GameOver => render_game_over(frame, &app),
            }
        }
    });

    Ok(())
}
