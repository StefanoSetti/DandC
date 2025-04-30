use strum_macros::EnumIter;

/// A card is the central unit of the game,
/// each card can have a suit and a rank.
#[derive(Debug, PartialEq)]
struct Card {
    seed: Suit,
    rank: Rank,
}

/// The `suit` can be 1 of 4 types.

#[derive(EnumIter, Debug, PartialEq)]
enum Suit {
    Spades,
    Diamonds,
    Clubs,
    Hearts,
}

/// The `rank` can be one of 13 values.
#[derive(EnumIter, Debug, PartialEq)]
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
    fn new() -> Self {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_card() {}

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
                    deck.cards[i].seed != deck.cards[j].seed
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
            *counts.entry(card.seed).or_insert(0) += 1;
        }

        assert_eq!(counts[&Suit::Spades], 13);
        assert_eq!(counts[&Suit::Hearts], 13);
        assert_eq!(counts[&Suit::Diamonds], 13);
        assert_eq!(counts[&Suit::Clubs], 13);
    }
}
