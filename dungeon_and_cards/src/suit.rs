//! A module for playing card suits's functionality.
use core::fmt;
use strum_macros::EnumIter;

/// Enum representing the four standard playing card suits.
///
/// # Examples
/// ```
/// use your_crate::Suit;
///
/// let suit = Suit::Hearts;
/// assert_eq!(suit.to_string(), "♥️");
/// ```
#[derive(EnumIter, Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Suit {
    /// ♠️ Spades suit
    Spades,
    /// ♦️ Diamonds suit
    Diamonds,
    /// ♣️ Clubs suit
    Clubs,
    /// ♥️ Hearts suit
    Hearts,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Spades => write!(f, "♠️"),
            Suit::Diamonds => write!(f, "♦️"),
            Suit::Clubs => write!(f, "♣️"),
            Suit::Hearts => write!(f, "♥️"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suit_should_display_an_emoji() {
        assert_eq!(Suit::Spades.to_string(), "♠️");
        assert_eq!(Suit::Diamonds.to_string(), "♦️");
        assert_eq!(Suit::Clubs.to_string(), "♣️");
        assert_eq!(Suit::Hearts.to_string(), "♥️");
    }
}
