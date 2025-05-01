use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// A card is the central unit of the game,
/// each card can have a suit and a rank.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Card {
    suit: Suit,
    rank: Rank,
}

/// The `suit` can be 1 of 4 types.
#[derive(EnumIter, Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Suit {
    Spades,
    Diamonds,
    Clubs,
    Hearts,
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        // Collect iters for `suits` and `ranks`.
        let suits = Suit::iter();
        let ranks = Rank::iter();

        // NOTE:
        // `map` would give you 4 piles (one per suit), each with 13 cards
        // `flat_map` automatically spreads all 52 cards on one table.
        let cards = suits
            .into_iter()
            .flat_map(|suit| {
                ranks
                    .clone() // One rank iterator per suit, so clone needed. (No clone would consume the iterator at the first suit)
                    .into_iter()
                    .map(move |rank| Card { suit, rank })
            })
            .collect::<Vec<_>>();

        Deck { cards }
    }

    // TODO: pub fn len()

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

        let len = self.cards.len();
        if number_of_draws > len {
            panic!("You are drawing too many cards from the deck!");
        }

        // Split the deck into two parts in O(1) time.
        let split_idx = len - number_of_draws;
        let mut drawn = self.cards.split_off(split_idx);

        // Reverse to match "top of deck" semantics (optional).
        drawn.reverse();

        drawn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_deck_has_52_cards() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn all_cards_in_deck_are_unique() {
        let deck = Deck::new();
        for i in 0..deck.cards.len() {
            for j in (i + 1)..deck.cards.len() {
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
        let deck = Deck::new();
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
    fn shuffle_randomize_card_order() {
        let original_deck = Deck::new();
        let mut shuffled_deck = original_deck.clone();

        // Checks if both decks are equal.
        assert_eq!(original_deck, shuffled_deck);

        shuffled_deck.shuffle();

        // Check that all cards are still present (just order changed)
        assert_eq!(original_deck.cards.len(), original_deck.cards.len());
        assert!(original_deck
            .cards
            .iter()
            .all(|card| original_deck.cards.contains(card)));

        // After the shuffle the decks should have different orders.
        assert_ne!(original_deck, shuffled_deck);
    }

    #[test]
    fn draw_1_card_and_remove_it_from_deck() {
        let mut deck = Deck::new();

        // Draw 1 cars.
        let drawn_card = deck
            .draw(1)
            .first()
            .expect("It should be possible to draw a card from a full deck")
            .to_owned();

        // The drawn card should be not contained in the deck anymore.
        assert!(!deck.cards.contains(&drawn_card));
        assert_eq!(deck.cards.len(), 51);
    }

    #[test]
    fn draw_10_card_and_remove_it_from_deck() {
        let mut deck = Deck::new();

        // Draw 10 cars.
        let drawn_card = deck.draw(10);

        // The drawn card should be not contained in the deck anymore.
        assert!(drawn_card.iter().all(|card| !deck.cards.contains(card)));
        assert_eq!(deck.cards.len(), 42);
    }

    #[test]
    fn draw_0_card_and_remove_it_from_deck() {
        let mut deck = Deck::new();

        // Draw 0 cars.
        let drawn_card = deck.draw(0);

        // The array should contain 0 cards.
        assert_eq!(drawn_card.len(), 0);
        assert_eq!(deck.cards.len(), 52);
    }
}
