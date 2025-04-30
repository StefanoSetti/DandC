use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// A card is the central unit of the game,
/// each card can have a suit and a rank.
#[derive(Debug, PartialEq)]
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

struct Deck {
    cards: [Card; 52],
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
            .collect::<Vec<_>>()
            .try_into()
            .expect("It should alway generate a deck of 52 cards");

        Deck { cards }
    }
}

fn main() {
    println!("Hello, world!");
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
}
