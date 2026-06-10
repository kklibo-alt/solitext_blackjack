use crate::game_state::{GameMode, GameState, Phase};
use crate::selection::{self, PlayerAction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Start,
    Game,
    GameMenu,
    Help,
    GameOver,
    Quit,
}

pub enum GameKey {
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    Enter,
    Esc,
    Char(char),
}

pub struct App {
    pub screen: Screen,
    pub game_state: GameState,
    /// Highlighted action during the player's turn.
    pub cursor: usize,
    pub message: String,
    pub debug_mode: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Start,
            game_state: GameState::init(GameMode::SingleDeck),
            cursor: 0,
            message: String::new(),
            debug_mode: false,
        }
    }

    pub fn new_game(&mut self, mode: GameMode) {
        self.game_state = GameState::init(mode);
        self.cursor = 0;
        self.message.clear();
        self.screen = Screen::Game;
        self.update();
    }

    /// The action the cursor currently points at, if any.
    fn current_action(&self) -> Option<PlayerAction> {
        PlayerAction::available(&self.game_state)
            .get(self.cursor)
            .copied()
    }

    fn activate_action(&mut self) {
        match self.current_action() {
            Some(PlayerAction::Hit) => self.game_state.player_hit(),
            Some(PlayerAction::Stand) => self.game_state.player_stand(),
            Some(PlayerAction::Double) => self.game_state.player_double(),
            None => {}
        }
        self.cursor = 0;
    }

    fn betting_key(&mut self, key: &GameKey) {
        match key {
            GameKey::Left | GameKey::Down => self.game_state.bet_down(),
            GameKey::Right | GameKey::Up => self.game_state.bet_up(),
            GameKey::Home => self.game_state.bet = GameState::MIN_BET,
            GameKey::End => self.game_state.bet = self.game_state.max_bet(),
            GameKey::Enter | GameKey::Char(' ') => {
                self.game_state.deal_round();
                self.cursor = 0;
            }
            _ => {}
        }
    }

    fn player_turn_key(&mut self, key: &GameKey) {
        let len = PlayerAction::available(&self.game_state).len();
        match key {
            GameKey::Left => self.cursor = self.cursor.saturating_sub(1),
            GameKey::Right => self.cursor = (self.cursor + 1).min(len.saturating_sub(1)),
            GameKey::Enter | GameKey::Char(' ') => self.activate_action(),
            // Convenience hotkeys.
            GameKey::Char('s') => self.game_state.player_stand(),
            _ => {}
        }
    }

    fn round_over_key(&mut self, key: &GameKey) {
        if let GameKey::Enter | GameKey::Char(' ') = key {
            self.game_state.next_round();
            self.cursor = 0;
        }
    }

    /// Handle a key on the Game screen, then run housekeeping.
    pub fn handle_game_key(&mut self, key: GameKey) {
        match key {
            GameKey::Char('?') => {
                self.screen = Screen::Help;
                return;
            }
            GameKey::Char('d') => {
                self.debug_mode = !self.debug_mode;
                self.update();
                return;
            }
            GameKey::Esc => {
                self.screen = Screen::GameMenu;
                return;
            }
            _ => {}
        }

        match self.game_state.phase {
            Phase::Betting => self.betting_key(&key),
            Phase::PlayerTurn => self.player_turn_key(&key),
            Phase::RoundOver => self.round_over_key(&key),
        }
        self.update();
    }

    /// Game-state housekeeping run after each action.
    pub fn update(&mut self) {
        self.cursor = selection::clamp_cursor(self.cursor, &self.game_state);
        self.set_message();

        if self.game_state.is_broke() {
            self.screen = Screen::GameOver;
        }
    }

    fn set_message(&mut self) {
        self.message = match self.game_state.phase {
            Phase::Betting => "←/→: Bet   Enter: Deal".to_string(),
            Phase::PlayerTurn => "←/→: Choose   Enter: Confirm".to_string(),
            Phase::RoundOver => match self.game_state.outcome {
                Some(outcome) => format!("{}   Enter: Next", outcome.message()),
                None => "Enter: Next".to_string(),
            },
        };
    }
}
