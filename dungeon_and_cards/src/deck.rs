use core::fmt;

use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// The `suit` can be 1 of 4 types.
#[derive(EnumIter, Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Suit {
    Spades,
    Diamonds,
    Clubs,
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

/// The `rank` can be one of 13 values.
#[derive(EnumIter, Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
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

/// A card is the central unit of the game,
/// each card can have a suit and a rank.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Card {
    suit: Suit,
    rank: Rank,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}  {}", self.suit, self.rank)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Deck {
    /// The array of contained card.
    cards: Vec<Card>,
    /// The number of cards contained in the
    /// deck at creation (banned card not counted).
    size: usize,
    /// The array of banned cards (can't be drawn).
    banned_cards: Option<Vec<Card>>,
}

impl Deck {
    pub fn new(banned_cards: Option<Vec<Card>>) -> Self {
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

    /// Returns the number of cards in the deck.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Shuffles the deck randomly.
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::rng());
    }

    /// Draws `k` card from the top of the deck.
    /// Once drawn the cards are removed from the deck.
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_deck_has_52_cards() {
        let deck = Deck::new(None);
        assert_eq!(deck.len(), 52);
    }

    #[test]
    fn all_cards_in_deck_are_unique() {
        let deck = Deck::new(None);
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
        let deck = Deck::new(None);
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

        let deck = Deck::new(Some(vec![banned_1, banned_2]));
        assert_eq!(deck.len(), 50);
        assert!(!deck.cards.contains(&banned_1));
        assert!(!deck.cards.contains(&banned_2))
    }

    #[test]
    fn shuffle_randomize_card_order() {
        let original_deck = Deck::new(None);
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
        let mut deck = Deck::new(None);

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
        let mut deck = Deck::new(None);

        // Draw 10 cars.
        let drawn_card = deck.draw(10);

        // The drawn card should be not contained in the deck anymore.
        assert!(drawn_card.iter().all(|card| !deck.cards.contains(card)));
        assert_eq!(deck.len(), 42);
    }

    #[test]
    fn draw_0_card_and_remove_it_from_deck() {
        let mut deck = Deck::new(None);

        // Draw 0 cars.
        let drawn_card = deck.draw(0);

        // The array should contain 0 cards.
        assert_eq!(drawn_card.len(), 0);
        assert_eq!(deck.len(), 52);
    }

    #[test]
    fn print_a_card() {
        let card = Card {
            suit: Suit::Clubs,
            rank: Rank::Queen,
        };

        assert_eq!(card.to_string(), "♣️  Q")
    }
}
