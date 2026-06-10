use crate::cards::{Card, Rank};
use crate::game_logic;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Hand(pub Vec<Card>);

impl Hand {
    /// Best (highest non-busting if possible) blackjack total.
    pub fn total(&self) -> u8 {
        let mut sum: u8 = 0;
        let mut aces = 0u8;
        for card in &self.0 {
            sum += card.rank.blackjack_value();
            if card.rank == Rank::Ace {
                aces += 1;
            }
        }
        while sum > 21 && aces > 0 {
            sum -= 10;
            aces -= 1;
        }
        sum
    }

    /// True when an ace is still counted as 11 (a "soft" total).
    pub fn is_soft(&self) -> bool {
        let mut sum: u8 = 0;
        let mut aces = 0u8;
        for card in &self.0 {
            sum += card.rank.blackjack_value();
            if card.rank == Rank::Ace {
                aces += 1;
            }
        }
        while sum > 21 && aces > 0 {
            sum -= 10;
            aces -= 1;
        }
        aces > 0
    }

    pub fn is_bust(&self) -> bool {
        self.total() > 21
    }

    /// A two-card 21.
    pub fn is_blackjack(&self) -> bool {
        self.0.len() == 2 && self.total() == 21
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum GameMode {
    /// One deck, dealer stands on all 17s.
    #[default]
    SingleDeck,
    /// Six-deck shoe, dealer hits soft 17.
    SixDeck,
}

impl GameMode {
    pub fn decks(self) -> usize {
        match self {
            GameMode::SingleDeck => 1,
            GameMode::SixDeck => 6,
        }
    }

    /// Whether the dealer hits a soft 17 in this rule set.
    pub fn dealer_hits_soft_17(self) -> bool {
        matches!(self, GameMode::SixDeck)
    }

    pub fn label(self) -> &'static str {
        match self {
            GameMode::SingleDeck => "Single Deck",
            GameMode::SixDeck => "Six Deck Shoe",
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum Phase {
    /// Choosing a wager before the cards are dealt.
    #[default]
    Betting,
    /// The player is deciding to hit, stand, or double.
    PlayerTurn,
    /// The hand is settled; waiting to deal the next round.
    RoundOver,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Outcome {
    PlayerBlackjack,
    PlayerWin,
    DealerBust,
    Push,
    DealerWin,
    PlayerBust,
}

impl Outcome {
    pub fn message(self) -> &'static str {
        match self {
            Outcome::PlayerBlackjack => "Blackjack! You win 3:2",
            Outcome::PlayerWin => "You win!",
            Outcome::DealerBust => "Dealer busts. You win!",
            Outcome::Push => "Push. Bet returned.",
            Outcome::DealerWin => "Dealer wins.",
            Outcome::PlayerBust => "Bust! You lose.",
        }
    }

    pub fn player_won(self) -> bool {
        matches!(
            self,
            Outcome::PlayerBlackjack | Outcome::PlayerWin | Outcome::DealerBust
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameState {
    pub game_mode: GameMode,
    pub shoe: Vec<Card>,
    pub dealer: Hand,
    pub player: Hand,
    pub bankroll: u32,
    pub bet: u32,
    pub phase: Phase,
    pub outcome: Option<Outcome>,
    /// Hide the dealer's second card while the player is acting.
    pub hole_hidden: bool,
}

impl GameState {
    pub const STARTING_BANKROLL: u32 = 100;
    pub const MIN_BET: u32 = 5;
    pub const BET_STEP: u32 = 5;
    /// Reshuffle the shoe once it drops below this many cards.
    const RESHUFFLE_AT: usize = 15;

    pub fn init(mode: GameMode) -> Self {
        Self {
            game_mode: mode,
            shoe: Card::shuffled_shoe(mode.decks()),
            dealer: Hand::default(),
            player: Hand::default(),
            bankroll: Self::STARTING_BANKROLL,
            bet: Self::MIN_BET.max(10),
            phase: Phase::Betting,
            outcome: None,
            hole_hidden: false,
        }
    }

    /// Highest bet the player can currently afford.
    pub fn max_bet(&self) -> u32 {
        self.bankroll.max(Self::MIN_BET)
    }

    pub fn bet_up(&mut self) {
        let limit = self.max_bet();
        self.bet = (self.bet + Self::BET_STEP).min(limit);
    }

    pub fn bet_down(&mut self) {
        self.bet = self.bet.saturating_sub(Self::BET_STEP).max(Self::MIN_BET);
    }

    fn draw_card(&mut self) -> Card {
        if self.shoe.is_empty() {
            self.shoe = Card::shuffled_shoe(self.game_mode.decks());
        }
        self.shoe.pop().expect("freshly filled shoe is non-empty")
    }

    fn reshuffle_if_low(&mut self) {
        if self.shoe.len() < Self::RESHUFFLE_AT {
            self.shoe = Card::shuffled_shoe(self.game_mode.decks());
        }
    }

    /// Deduct the wager and deal two cards each. Resolves immediately on a
    /// natural blackjack.
    pub fn deal_round(&mut self) {
        if self.phase != Phase::Betting || self.bankroll < Self::MIN_BET {
            return;
        }
        self.reshuffle_if_low();
        self.bet = self.bet.clamp(Self::MIN_BET, self.bankroll);
        self.bankroll -= self.bet;

        self.player = Hand::default();
        self.dealer = Hand::default();
        for _ in 0..2 {
            let c = self.draw_card();
            self.player.0.push(c);
            let c = self.draw_card();
            self.dealer.0.push(c);
        }
        self.hole_hidden = true;
        self.phase = Phase::PlayerTurn;
        self.outcome = None;

        if self.player.is_blackjack() || self.dealer.is_blackjack() {
            self.finish_with_dealer();
        }
    }

    /// Whether doubling down is currently allowed.
    pub fn can_double(&self) -> bool {
        self.phase == Phase::PlayerTurn
            && self.player.0.len() == 2
            && self.bankroll >= self.bet
    }

    pub fn player_hit(&mut self) {
        if self.phase != Phase::PlayerTurn {
            return;
        }
        let c = self.draw_card();
        self.player.0.push(c);
        if self.player.is_bust() {
            self.hole_hidden = false;
            self.resolve(Outcome::PlayerBust);
        }
    }

    pub fn player_stand(&mut self) {
        if self.phase != Phase::PlayerTurn {
            return;
        }
        self.finish_with_dealer();
    }

    pub fn player_double(&mut self) {
        if !self.can_double() {
            return;
        }
        self.bankroll -= self.bet;
        self.bet *= 2;
        let c = self.draw_card();
        self.player.0.push(c);
        if self.player.is_bust() {
            self.hole_hidden = false;
            self.resolve(Outcome::PlayerBust);
        } else {
            self.finish_with_dealer();
        }
    }

    /// Reveal the hole card, let the dealer play out, and settle the hand.
    fn finish_with_dealer(&mut self) {
        self.hole_hidden = false;

        if !self.player.is_blackjack() && !self.dealer.is_blackjack() {
            while game_logic::dealer_should_hit(&self.dealer, self.game_mode) {
                let c = self.draw_card();
                self.dealer.0.push(c);
            }
        }

        let outcome = game_logic::determine_outcome(&self.player, &self.dealer);
        self.resolve(outcome);
    }

    fn resolve(&mut self, outcome: Outcome) {
        let payout = match outcome {
            Outcome::PlayerBlackjack => self.bet + (self.bet * 3) / 2,
            Outcome::PlayerWin | Outcome::DealerBust => self.bet * 2,
            Outcome::Push => self.bet,
            Outcome::DealerWin | Outcome::PlayerBust => 0,
        };
        self.bankroll += payout;
        self.outcome = Some(outcome);
        self.phase = Phase::RoundOver;
    }

    /// Return to the betting phase for the next hand.
    pub fn next_round(&mut self) {
        self.player = Hand::default();
        self.dealer = Hand::default();
        self.outcome = None;
        self.hole_hidden = false;
        self.phase = Phase::Betting;
        self.bet = self.bet.clamp(Self::MIN_BET, self.max_bet());
    }

    pub fn is_broke(&self) -> bool {
        self.phase == Phase::Betting && self.bankroll < Self::MIN_BET
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::{Rank::*, Suit::*};

    fn hand(cards: &[(crate::cards::Suit, Rank)]) -> Hand {
        Hand(cards.iter().map(|&(s, r)| Card::new(s, r)).collect())
    }

    #[test]
    fn test_hard_total() {
        assert_eq!(hand(&[(Spades, R10), (Hearts, R7)]).total(), 17);
    }

    #[test]
    fn test_soft_total_and_demotion() {
        let h = hand(&[(Spades, Ace), (Hearts, R6)]);
        assert_eq!(h.total(), 17);
        assert!(h.is_soft());

        let h = hand(&[(Spades, Ace), (Hearts, R6), (Clubs, R10)]);
        assert_eq!(h.total(), 17);
        assert!(!h.is_soft());
    }

    #[test]
    fn test_blackjack_detection() {
        assert!(hand(&[(Spades, Ace), (Hearts, King)]).is_blackjack());
        assert!(!hand(&[(Spades, R7), (Hearts, R7), (Clubs, R7)]).is_blackjack());
    }

    #[test]
    fn test_bust() {
        assert!(hand(&[(Spades, King), (Hearts, Queen), (Clubs, R5)]).is_bust());
    }

    #[test]
    fn test_init_deals_full_shoe() {
        assert_eq!(GameState::init(GameMode::SingleDeck).shoe.len(), 52);
        assert_eq!(GameState::init(GameMode::SixDeck).shoe.len(), 312);
    }

    #[test]
    fn test_deal_round_deducts_bet() {
        let mut gs = GameState::init(GameMode::SingleDeck);
        let bet = gs.bet;
        let start = gs.bankroll;
        gs.deal_round();
        assert_eq!(gs.player.0.len(), 2);
        assert_eq!(gs.dealer.0.len(), 2);
        // Either still playing (bet deducted) or already resolved on a natural.
        assert!(gs.bankroll <= start - bet || gs.phase == Phase::RoundOver);
    }
}
