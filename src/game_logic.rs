use crate::game_state::{GameMode, Hand, Outcome};

/// Standard dealer policy: draw to 17, then stand. In rule sets that hit a
/// soft 17 the dealer takes one more card on a soft total of exactly 17.
pub fn dealer_should_hit(dealer: &Hand, mode: GameMode) -> bool {
    let total = dealer.total();
    if total < 17 {
        return true;
    }
    if total == 17 && dealer.is_soft() && mode.dealer_hits_soft_17() {
        return true;
    }
    false
}

/// Settle a completed hand. Assumes the dealer has already finished drawing.
pub fn determine_outcome(player: &Hand, dealer: &Hand) -> Outcome {
    let player_bj = player.is_blackjack();
    let dealer_bj = dealer.is_blackjack();

    if player_bj && dealer_bj {
        return Outcome::Push;
    }
    if player_bj {
        return Outcome::PlayerBlackjack;
    }
    if dealer_bj {
        return Outcome::DealerWin;
    }
    if player.is_bust() {
        return Outcome::PlayerBust;
    }
    if dealer.is_bust() {
        return Outcome::DealerBust;
    }

    match player.total().cmp(&dealer.total()) {
        std::cmp::Ordering::Greater => Outcome::PlayerWin,
        std::cmp::Ordering::Less => Outcome::DealerWin,
        std::cmp::Ordering::Equal => Outcome::Push,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::{Card, Rank::*, Suit::*};

    fn hand(cards: &[(crate::cards::Suit, crate::cards::Rank)]) -> Hand {
        Hand(cards.iter().map(|&(s, r)| Card::new(s, r)).collect())
    }

    #[test]
    fn test_dealer_stands_on_hard_17() {
        let d = hand(&[(Spades, R10), (Hearts, R7)]);
        assert!(!dealer_should_hit(&d, GameMode::SingleDeck));
        assert!(!dealer_should_hit(&d, GameMode::SixDeck));
    }

    #[test]
    fn test_dealer_hits_low() {
        let d = hand(&[(Spades, R10), (Hearts, R6)]);
        assert!(dealer_should_hit(&d, GameMode::SingleDeck));
    }

    #[test]
    fn test_soft_17_rule_varies_by_mode() {
        let d = hand(&[(Spades, Ace), (Hearts, R6)]);
        assert!(!dealer_should_hit(&d, GameMode::SingleDeck));
        assert!(dealer_should_hit(&d, GameMode::SixDeck));
    }

    #[test]
    fn test_outcomes() {
        let player = hand(&[(Spades, R10), (Hearts, R9)]);
        let dealer = hand(&[(Clubs, R10), (Diamonds, R7)]);
        assert_eq!(determine_outcome(&player, &dealer), Outcome::PlayerWin);

        let dealer = hand(&[(Clubs, R10), (Diamonds, King), (Spades, R5)]);
        assert_eq!(determine_outcome(&player, &dealer), Outcome::DealerBust);

        let player_bj = hand(&[(Spades, Ace), (Hearts, King)]);
        let dealer = hand(&[(Clubs, R10), (Diamonds, R8)]);
        assert_eq!(
            determine_outcome(&player_bj, &dealer),
            Outcome::PlayerBlackjack
        );

        let dealer_bj = hand(&[(Clubs, Ace), (Diamonds, Queen)]);
        assert_eq!(determine_outcome(&player_bj, &dealer_bj), Outcome::Push);
    }
}
