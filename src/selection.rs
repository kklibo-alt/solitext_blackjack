use crate::game_state::GameState;

/// An action the player can choose during their turn, presented as a row of
/// buttons the cursor moves across.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlayerAction {
    Hit,
    Stand,
    Double,
}

impl PlayerAction {
    pub fn label(self) -> &'static str {
        match self {
            PlayerAction::Hit => "Hit",
            PlayerAction::Stand => "Stand",
            PlayerAction::Double => "Double",
        }
    }

    /// Actions currently available, given the game state (e.g. doubling is
    /// only offered on the first two cards when affordable).
    pub fn available(game_state: &GameState) -> Vec<PlayerAction> {
        let mut actions = vec![PlayerAction::Hit, PlayerAction::Stand];
        if game_state.can_double() {
            actions.push(PlayerAction::Double);
        }
        actions
    }
}

/// Clamp a cursor index to the available action range.
pub fn clamp_cursor(cursor: usize, game_state: &GameState) -> usize {
    let len = PlayerAction::available(game_state).len();
    if len == 0 {
        0
    } else {
        cursor.min(len - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::{GameMode, Phase};

    #[test]
    fn test_double_only_on_two_cards() {
        let mut gs = GameState::init(GameMode::SingleDeck);
        gs.deal_round();
        if gs.phase == Phase::PlayerTurn {
            let actions = PlayerAction::available(&gs);
            assert!(actions.contains(&PlayerAction::Hit));
            assert!(actions.contains(&PlayerAction::Stand));
        }
    }
}
