//! A module for playing card rank's functionality.
//!
//! Provides `Rank` type and operations for standard poker card
//! with support operation with `u8` type

use core::fmt;
use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Sub, SubAssign},
};
use strum_macros::EnumIter;

/// Enum representing the thirteen standard playing card ranks.
///
/// # Examples
/// ```
/// use your_crate::Rank;
///
/// let rank = Rank::Ace;
/// assert_eq!(rank.to_string(), "A");
/// ```
#[derive(EnumIter, Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Rank {
    /// A - Ace
    Ace = 1,
    /// 2 - Two
    Two = 2,
    /// 3 - Three
    Three = 3,
    /// 4 - Four
    Four = 4,
    /// 5 - Five
    Five = 5,
    /// 6 - Six
    Six = 6,
    /// 7 - Seven
    Seven = 7,
    /// 8 - Eight
    Eight = 8,
    /// 9 - Nine
    Nine = 9,
    /// 10 - Ten
    Ten = 10,
    /// J - Jack
    Jack = 11,
    /// Q - Queen
    Queen = 12,
    /// K - King
    King = 13,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rank::Ace => write!(f, "A"),
            Rank::Two => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "10"),
            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
        }
    }
}

impl From<Rank> for u8 {
    fn from(value: Rank) -> Self {
        match value {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
        }
    }
}

// Overload `+` (addition)
impl Add<u8> for Rank {
    type Output = u8;

    fn add(self, rhs: u8) -> u8 {
        u8::from(self) + u8::from(rhs)
    }
}

// Allow `u8 + Rank` (via commutative property)
impl Add<Rank> for u8 {
    type Output = u8;

    fn add(self, rhs: Rank) -> u8 {
        rhs + self // Reuse `Rank + u8` impl
    }
}

// Allow `u8` -= `rank`
impl AddAssign<Rank> for u8 {
    fn add_assign(&mut self, rhs: Rank) {
        *self += u8::from(rhs);
    }
}

// Overload `-` (subtraction)
impl Sub<u8> for Rank {
    type Output = u8;

    fn sub(self, rhs: u8) -> u8 {
        u8::from(self).saturating_sub(u8::from(rhs)) // Prevents underflow
    }
}

// Allow `u8 - Rank`
impl Sub<Rank> for u8 {
    type Output = u8;

    fn sub(self, rhs: Rank) -> u8 {
        self.saturating_sub(u8::from(rhs)) // Prevent underflow
    }
}

// Allow `u8` -= `rank`
impl SubAssign<Rank> for u8 {
    fn sub_assign(&mut self, rhs: Rank) {
        *self -= u8::from(rhs);
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        u8::from(*self).cmp(&u8::from(*other))
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<u8> for Rank {
    fn eq(&self, other: &u8) -> bool {
        u8::from(*self) == *other
    }
}

impl PartialOrd<u8> for Rank {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        Some(u8::from(*self).cmp(other))
    }
}

impl PartialEq<Rank> for u8 {
    fn eq(&self, other: &Rank) -> bool {
        *self == u8::from(*other)
    }
}

impl PartialOrd<Rank> for u8 {
    fn partial_cmp(&self, other: &Rank) -> Option<Ordering> {
        Some(self.cmp(&u8::from(*other)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cast_a_card_rank_into_u8() {
        assert_eq!(Rank::Ace as u8, 1u8);
        assert_eq!(Rank::Two as u8, 2u8);
        assert_eq!(Rank::Three as u8, 3u8);
        assert_eq!(Rank::Four as u8, 4u8);
        assert_eq!(Rank::Five as u8, 5u8);
        assert_eq!(Rank::Six as u8, 6u8);
        assert_eq!(Rank::Seven as u8, 7u8);
        assert_eq!(Rank::Eight as u8, 8u8);
        assert_eq!(Rank::Nine as u8, 9u8);
        assert_eq!(Rank::Ten as u8, 10u8);
        assert_eq!(Rank::Jack as u8, 11u8);
        assert_eq!(Rank::Queen as u8, 12u8);
        assert_eq!(Rank::King as u8, 13u8);
    }

    #[test]
    fn sum_a_card_with_a_u8() {
        let rank = Rank::Queen;
        let number = 10u8;

        assert_eq!(number + rank, 22u8)
    }

    #[test]
    fn sub_a_card_with_a_u8() {
        let rank = Rank::Queen;
        let number = 10u8;

        assert_eq!(rank - number, 2u8)
    }

    #[test]
    fn test_rank_ordering() {
        assert!(Rank::Ace < Rank::Two);
        assert!(Rank::King > Rank::Queen);
        assert_eq!(Rank::Seven.cmp(&Rank::Seven), Ordering::Equal);
    }

    #[test]
    fn test_rank_u8_comparisons() {
        assert!(Rank::Five == 5);
        assert!(Rank::Nine < 10);
        assert!(8 > Rank::Seven);
        assert!(12 == Rank::Queen);
    }

    #[test]
    fn test_edge_cases() {
        assert!(Rank::Ace == 1);
        assert!(Rank::King == 13);
        assert!(1_u8 == Rank::Ace);
        assert!(13_u8 == Rank::King);
    }
}
