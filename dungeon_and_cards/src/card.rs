//! A module for playing card functionality.
//!
//! Provides types and operations for standard poker card.

use core::fmt;

use crate::{rank::Rank, suit::Suit};

/// # Examples
/// ```
/// use your_crate::{Card, Suit, Rank};
///
/// let card = Card::new(Suit::Spades, Rank::Ace);
/// assert_eq!(card.to_string(), "♠️  A");
/// ```
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Card {
    /// The card's suit (Spades, Diamonds, Clubs, Hearts)
    suit: Suit,
    /// The card's rank (Ace through King)
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}  {}", self.suit, self.rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_a_card() {
        let card = Card::new(Suit::Clubs, Rank::Queen);

        assert_eq!(card.to_string(), "♣️  Q")
    }
}
