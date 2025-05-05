//! A module for playing card deck functionality.
//!
//! Provides types and operations for standard 52-card decks with support for
//! banned cards and various deck operations.

use core::fmt;
use rand::seq::SliceRandom;
use std::{
    collections::HashSet,
    ops::{Add, Sub},
};
use strum::IntoEnumIterator;
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

// Overload `-` (subtraction)
impl Sub<u8> for Rank {
    type Output = u8;

    fn sub(self, rhs: u8) -> u8 {
        u8::from(self).saturating_sub(u8::from(rhs)) // Prevents underflow
    }
}

// Allow `u8 + Rank` (via commutative property)
impl Sub<Rank> for u8 {
    type Output = u8;

    fn sub(self, rhs: Rank) -> u8 {
        self.saturating_sub(u8::from(rhs)) // Prevent underflow
    }
}

/// A playing card combining a suit and rank.
///
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

/// A deck of playing cards with optional banned cards.
///
/// # Examples
/// ```
/// use your_crate::{Deck, Card, Suit, Rank};
///
/// // Create a deck with banned cards
/// let banned = Card { suit: Suit::Hearts, rank: Rank::Ace };
/// let deck = Deck::builder().ban_card(banned).build();
///
/// assert_eq!(deck.len(), 51);
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Deck {
    /// The cards currently in the deck
    cards: Vec<Card>,
    /// The original size of the deck (excluding banned cards)
    size: usize,
    /// Cards that are banned from being in the deck
    banned_cards: Option<HashSet<Card>>,
}

impl Deck {
    /// Creates a new deck builder for configuring and constructing a deck.
    ///
    /// This is the preferred way to create a deck as it provides a fluent
    /// interface for configuration.
    pub fn builder() -> DeckBuilder {
        DeckBuilder::new()
    }

    /// Constructs a new deck with optional banned cards.
    ///
    /// # Arguments
    /// * `banned_cards` - Optional set of cards to exclude from the deck
    fn new(banned_cards: Option<HashSet<Card>>) -> Self {
        // Collect iters for `suits` and `ranks`.
        let suits = Suit::iter();
        let ranks = Rank::iter();

        // Pre-allocate vector size.
        #[allow(unused_assignments)]
        let mut cards = Vec::with_capacity(52 - banned_cards.as_ref().map_or(0, |b| b.len()));

        // NOTE:
        // `map` would give you 4 piles (one per suit), each with 13 cards
        // `flat_map` automatically spreads all 52 cards on one table.
        cards = if let Some(ref banned_cards) = banned_cards {
            suits
                .into_iter()
                .flat_map(|suit| {
                    ranks
                        .clone() // One rank iterator per suit, so clone needed. (No clone would consume the iterator at the first suit)
                        .into_iter()
                        .map(move |rank| Card { suit, rank })
                })
                .filter(|card| !banned_cards.contains(card))
                .collect::<Vec<_>>()
        } else {
            suits
                .into_iter()
                .flat_map(|suit| {
                    ranks
                        .clone() // One rank iterator per suit, so clone needed. (No clone would consume the iterator at the first suit)
                        .into_iter()
                        .map(move |rank| Card { suit, rank })
                })
                .collect::<Vec<_>>()
        };

        Deck {
            size: cards.len(),
            cards,
            banned_cards: banned_cards,
        }
    }

    /// Checks if the deck contains a specific card.
    ///
    /// # Arguments
    /// * `card` - The card to check for
    ///
    /// # Returns
    /// `true` if the card is in the deck, `false` otherwise
    pub fn contains(&self, card: &Card) -> bool {
        self.cards.contains(card)
    }

    /// Draws cards from the top of the deck.
    ///
    /// # Arguments
    /// * `number_of_draws` - How many cards to draw
    ///
    /// # Returns
    /// A vector containing the drawn cards
    ///
    /// # Panics
    /// Panics if attempting to draw more cards than are in the deck
    ///
    /// # Examples
    /// ```
    /// use your_crate::Deck;
    ///
    /// let mut deck = Deck::builder().build();
    /// let cards = deck.draw(5);
    /// assert_eq!(cards.len(), 5);
    /// assert_eq!(deck.len(), 47);
    /// ```
    pub fn draw(&mut self, number_of_draws: usize) -> Vec<Card> {
        if number_of_draws == 0 {
            return Vec::new(); // Edge case: avoid unnecessary allocation
        }

        let len = self.len();
        if number_of_draws > len {
            panic!("You are drawing too many cards from the deck!"); // TODO: generate error
        }

        // Collecting cards from the top.
        self.cards.drain(..number_of_draws).collect()
    }

    /// Returns `true` if the deck is empty.
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Returns the number of cards in the deck.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Resets the deck to its original state (excluding banned cards).
    pub fn reset(&mut self) {
        *self = Self::new(self.banned_cards.clone());
    }

    /// Shuffles the deck randomly.
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::rng());
    }
}

/// Builder for configuring and constructing a `Deck`.
///
/// Provides a fluent interface for specifying banned cards before
/// constructing the deck.
///
/// # Examples
/// ```
/// use your_crate::{DeckBuilder, Card, Suit, Rank};
///
/// let deck = DeckBuilder::new()
///     .ban_card(Card { suit: Suit::Spades, rank: Rank::Ace })
///     .ban_card(Card { suit: Suit::Hearts, rank: Rank::King })
///     .build();
/// ```
pub struct DeckBuilder {
    banned_cards: Option<HashSet<Card>>,
}

impl DeckBuilder {
    /// Creates a new deck builder with no banned cards.
    pub fn new() -> Self {
        Self { banned_cards: None }
    }

    /// Bans a single card from appearing in the deck.
    ///
    /// # Arguments
    /// * `card` - The card to ban
    pub fn ban_card(mut self, card: Card) -> Self {
        self.banned_cards
            .get_or_insert_with(HashSet::new)
            .insert(card);
        self
    }

    /// Bans multiple cards from appearing in the deck.
    ///
    /// # Arguments
    /// * `cards` - An iterator of cards to ban
    pub fn ban_cards(mut self, cards: impl IntoIterator<Item = Card>) -> Self {
        self.banned_cards
            .get_or_insert_with(HashSet::new)
            .extend(cards);
        self
    }

    /// Constructs the deck with the configured banned cards.
    pub fn build(self) -> Deck {
        Deck::new(self.banned_cards)
    }
}

#[cfg(test)]
mod tests {
    use core::num;

    use super::*;

    #[test]
    fn new_deck_has_52_cards() {
        let deck = Deck::builder().build();
        assert_eq!(deck.len(), 52);
    }

    #[test]
    fn all_cards_in_deck_are_unique() {
        let deck = Deck::builder().build();
        for i in 0..deck.len() {
            for j in (i + 1)..deck.len() {
                assert!(
                    deck.cards[i].suit != deck.cards[j].suit
                        || deck.cards[i].rank != deck.cards[j].rank,
                    "Duplicate card found: {:?} and {:?}",
                    deck.cards[i],
                    deck.cards[j]
                );
            }
        }
    }

    #[test]
    fn deck_has_13_cards_of_each_suit() {
        let deck = Deck::builder().build();
        let mut counts = std::collections::HashMap::new();

        for card in &deck.cards {
            *counts.entry(card.suit).or_insert(0) += 1;
        }

        assert_eq!(counts[&Suit::Spades], 13);
        assert_eq!(counts[&Suit::Hearts], 13);
        assert_eq!(counts[&Suit::Diamonds], 13);
        assert_eq!(counts[&Suit::Clubs], 13);
    }

    #[test]
    fn new_deck_with_2_banned_cards_has_50_cards() {
        let banned_1 = Card {
            suit: Suit::Clubs,
            rank: Rank::Eight,
        };
        let banned_2 = Card {
            suit: Suit::Diamonds,
            rank: Rank::Ten,
        };

        let deck = Deck::builder().ban_cards(vec![banned_1, banned_2]).build();

        assert_eq!(deck.len(), 50);
        assert!(!deck.cards.contains(&banned_1));
        assert!(!deck.cards.contains(&banned_2))
    }

    #[test]
    fn shuffle_randomize_card_order() {
        let original_deck = Deck::builder().build();
        let mut shuffled_deck = original_deck.clone();

        // Checks if both decks are equal.
        assert_eq!(original_deck, shuffled_deck);

        shuffled_deck.shuffle();

        // Check that all cards are still present (just order changed)
        assert_eq!(original_deck.len(), original_deck.len());
        assert!(original_deck
            .cards
            .iter()
            .all(|card| original_deck.cards.contains(card)));

        // After the shuffle the decks should have different orders.
        assert_ne!(original_deck, shuffled_deck);
    }

    #[test]
    fn draw_1_card_and_remove_it_from_deck() {
        let mut deck = Deck::builder().build();

        let first = deck
            .cards
            .first()
            .expect("It would be possible to get the first card on a fresh deck")
            .clone();

        // Draw 1 cars.
        let drawn_card = deck
            .draw(1)
            .first()
            .expect("It should be possible to draw a card from a full deck")
            .to_owned();

        // The drawn card should be not contained in the deck anymore.
        assert!(!deck.cards.contains(&drawn_card));
        assert_eq!(deck.len(), 51);

        // Drawn card should be the first card of the deck.
        assert_eq!(drawn_card, first)
    }

    #[test]
    fn draw_10_card_and_remove_it_from_deck() {
        let mut deck = Deck::builder().build();

        // Draw 10 cars.
        let drawn_card = deck.draw(10);

        // The drawn card should be not contained in the deck anymore.
        assert!(drawn_card.iter().all(|card| !deck.cards.contains(card)));
        assert_eq!(deck.len(), 42);
    }

    #[test]
    fn draw_0_card_and_remove_it_from_deck() {
        let mut deck = Deck::builder().build();

        // Draw 0 cars.
        let drawn_card = deck.draw(0);

        // The array should contain 0 cards.
        assert_eq!(drawn_card.len(), 0);
        assert_eq!(deck.len(), 52);
    }

    // #[test]
    // fn test_draw_too_many() {
    //     let mut deck = Deck::new(None);
    //     assert!(matches!(
    //         deck.draw(53),
    //         Err(DeckError::NotEnoughCards {
    //             requested: 53,
    //             available: 52
    //         })
    //     ));
    // }

    #[test]
    fn print_a_card() {
        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::Queen,
        };

        assert_eq!(card.to_string(), "♣️  Q")
    }

    #[test]
    fn cast_a_card_rank_into_u8() {
        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::Ace,
        };
        assert_eq!(card.rank() as u8, 1u8);

        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::Two,
        };
        assert_eq!(card.rank() as u8, 2u8);

        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::Three,
        };
        assert_eq!(card.rank() as u8, 3u8);

        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::Four,
        };
        assert_eq!(card.rank() as u8, 4u8);

        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::Five,
        };
        assert_eq!(card.rank() as u8, 5u8);

        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::Six,
        };
        assert_eq!(card.rank() as u8, 6u8);

        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::Seven,
        };
        assert_eq!(card.rank() as u8, 7u8);

        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::Eight,
        };
        assert_eq!(card.rank() as u8, 8u8);

        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::Nine,
        };
        assert_eq!(card.rank() as u8, 9u8);

        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::Ten,
        };
        assert_eq!(card.rank() as u8, 10u8);

        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::Jack,
        };
        assert_eq!(card.rank() as u8, 11u8);

        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::Queen,
        };
        assert_eq!(card.rank() as u8, 12u8);

        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::King,
        };
        assert_eq!(card.rank() as u8, 13u8)
    }

    #[test]
    fn sum_a_card_with_a_u8() {
        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::Queen,
        };

        let number = 10u8;

        assert_eq!(number + card.rank(), 22u8)
    }

    #[test]
    fn sub_a_card_with_a_u8() {
        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::Queen,
        };

        let number = 10u8;

        assert_eq!(card.rank() - number, 2u8)
    }
}
