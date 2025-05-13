//! A module for playing deck functionality.
//!
//! Provides types and operations for standard 52-card decks with support for
//! banned cards and various deck operations.

use rand::seq::SliceRandom;
use std::collections::HashSet;
use strum::IntoEnumIterator;

use crate::card::Card;
use crate::rank::Rank;
use crate::suit::Suit;

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
                        .map(move |rank| Card::new(suit, rank))
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
                        .map(move |rank| Card::new(suit, rank))
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

    /// This method puts at the bottom of the deck
    /// an array of card in the given order.
    pub fn bottom(&mut self, cards: &mut Vec<Card>) {
        self.cards.append(cards);
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
                    deck.cards[i].suit() != deck.cards[j].suit()
                        || deck.cards[i].rank() != deck.cards[j].rank(),
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
            *counts.entry(card.suit()).or_insert(0) += 1;
        }

        assert_eq!(counts[&Suit::Spades], 13);
        assert_eq!(counts[&Suit::Hearts], 13);
        assert_eq!(counts[&Suit::Diamonds], 13);
        assert_eq!(counts[&Suit::Clubs], 13);
    }

    #[test]
    fn new_deck_with_2_banned_cards_has_50_cards() {
        let banned_1 = Card::new(Suit::Clubs, Rank::Eight);
        let banned_2 = Card::new(Suit::Diamonds, Rank::Ten);

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

    #[test]
    fn bottom_should_insert_card_at_the_end_of_the_deck() {
        let mut deck = Deck::builder().build();
        
        let drawn_card = deck.draw(4);

        deck.bottom(&mut drawn_card.clone());

        let bottom_cards = &deck.cards[(deck.len()-4)..];

        assert_eq!(bottom_cards, drawn_card);
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
}
