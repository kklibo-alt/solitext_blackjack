use rand::rng;
use rand::seq::SliceRandom;
use std::fmt::{Display, Formatter};
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Rank {
    Ace = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

impl Rank {
    /// Base blackjack value. Aces count as 11 here; a `Hand` demotes them to 1
    /// as needed to avoid busting.
    pub fn blackjack_value(self) -> u8 {
        match self {
            Rank::Ace => 11,
            Rank::Jack | Rank::Queen | Rank::King => 10,
            other => other as u8,
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Rank::Ace => "A",
            Rank::R2 => "2",
            Rank::R3 => "3",
            Rank::R4 => "4",
            Rank::R5 => "5",
            Rank::R6 => "6",
            Rank::R7 => "7",
            Rank::R8 => "8",
            Rank::R9 => "9",
            Rank::R10 => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        };
        write!(f, "{v}")
    }
}

#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Suit {
    Hearts = 0,
    Spades = 1,
    Diamonds = 2,
    Clubs = 3,
}

impl Suit {
    pub fn is_red(self) -> bool {
        match self {
            Self::Hearts | Self::Diamonds => true,
            Self::Spades | Self::Clubs => false,
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Self::Hearts => "♥",
            Self::Spades => "♠",
            Self::Diamonds => "♦",
            Self::Clubs => "♣",
        };
        write!(f, "{v}")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl Card {
    #[cfg(test)]
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    pub fn ordered_deck() -> Vec<Self> {
        let mut cards = vec![];
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card { suit, rank });
            }
        }
        cards
    }

    /// A shuffled shoe made of `decks` standard 52-card decks.
    pub fn shuffled_shoe(decks: usize) -> Vec<Self> {
        let mut rng = rng();
        let mut shoe = vec![];
        for _ in 0..decks.max(1) {
            shoe.extend(Self::ordered_deck());
        }
        shoe.shuffle(&mut rng);
        shoe
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordered_deck() {
        assert_eq!(Card::ordered_deck().len(), 52);
    }

    #[test]
    fn test_shuffled_shoe() {
        assert_eq!(Card::shuffled_shoe(1).len(), 52);
        assert_eq!(Card::shuffled_shoe(6).len(), 312);
    }

    #[test]
    fn test_blackjack_values() {
        assert_eq!(Rank::Ace.blackjack_value(), 11);
        assert_eq!(Rank::R7.blackjack_value(), 7);
        assert_eq!(Rank::R10.blackjack_value(), 10);
        assert_eq!(Rank::Jack.blackjack_value(), 10);
        assert_eq!(Rank::Queen.blackjack_value(), 10);
        assert_eq!(Rank::King.blackjack_value(), 10);
    }
}
