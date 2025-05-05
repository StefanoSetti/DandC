use crate::deck::{Card, Deck, Rank, Suit};

#[derive(PartialEq, Eq, Debug)]
enum GameState {
    InGame,
    Win,
    Lose,
}

struct Scoundrel {
    deck: Deck,
    life_points: u8, // MAX 20 LP
    weapon_equipped: Option<Card>,
    room_visited: usize,
}

impl Scoundrel {
    pub fn new() -> Self {
        let banned_cards = vec![
            Card::new(Suit::Diamonds, Rank::Ace),
            Card::new(Suit::Diamonds, Rank::Jack),
            Card::new(Suit::Diamonds, Rank::Queen),
            Card::new(Suit::Diamonds, Rank::King),
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Hearts, Rank::Jack),
            Card::new(Suit::Hearts, Rank::Queen),
            Card::new(Suit::Hearts, Rank::King),
        ];

        Self {
            deck: Deck::builder().ban_cards(banned_cards).build(),
            life_points: 20,
            weapon_equipped: None,
            room_visited: 0,
        }
    }

    pub fn enter_room(&mut self) -> Option<Vec<Card>> {
        // In case the deck is over, end the game
        if self.room_visited >= 12 {
            return None;
        }

        self.room_visited += 1;
        Some(self.deck.draw(4))
    }

    pub fn play_card(&mut self, card: &Card) -> GameState {
        match card.suit() {
            Suit::Spades | Suit::Clubs => {
                todo!();
            }
            Suit::Diamonds => self.weapon_equipped = Some(*card),
            Suit::Hearts => {
                self.life_points = (self.life_points + card.rank()).min(20);
            }
        }
        GameState::InGame
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_scoundrel_has_44_cards() {
        let game = Scoundrel::new();

        assert_eq!(game.deck.len(), 44);
    }

    #[test]
    fn new_scoundrel_has_20_life_points() {
        let game = Scoundrel::new();

        assert_eq!(game.life_points, 20);
    }

    #[test]
    fn new_scoundrel_has_no_weapons() {
        let game = Scoundrel::new();

        assert_eq!(game.weapon_equipped, None);
    }

    #[test]
    fn new_scoundrel_has_0_room_visited() {
        let game = Scoundrel::new();

        assert_eq!(game.room_visited, 0);
    }

    #[test]
    fn monster_lowers_life_points() {
        let mut game = Scoundrel::new();

        game.play_card(&Card::new(Suit::Spades, Rank::Five));

        assert_eq!(game.life_points, 15);
    }

    #[test]
    fn when_life_points_reach_0_game_over() {
        let mut game = Scoundrel::new();

        let mut game_state = game.play_card(&Card::new(Suit::Spades, Rank::Five));
        assert_eq!(game.life_points, 15);
        assert_eq!(game_state, GameState::InGame);

        game_state = game.play_card(&Card::new(Suit::Clubs, Rank::Five));
        assert_eq!(game.life_points, 10);
        assert_eq!(game_state, GameState::InGame);

        game_state = game.play_card(&Card::new(Suit::Clubs, Rank::Ten));
        assert_eq!(game.life_points, 10);
        assert_eq!(game_state, GameState::Lose);
    }

    #[test]
    fn playing_diamond_cards_equips_a_weapon_starts_with_no_equipment() {
        let mut game = Scoundrel::new();

        let weapon = Card::new(Suit::Diamonds, Rank::Nine);

        game.play_card(&weapon);
        assert_eq!(game.weapon_equipped, Some(weapon));
    }

    #[test]
    fn playing_diamond_cards_equips_a_weapon_and_discard_old() {
        let mut game = Scoundrel::new();

        let weapon = Card::new(Suit::Diamonds, Rank::Nine);

        game.play_card(&weapon);
        assert_eq!(game.weapon_equipped, Some(weapon));

        let new_weapon = Card::new(Suit::Diamonds, Rank::Five);

        game.play_card(&new_weapon);
        assert_eq!(game.weapon_equipped, Some(new_weapon));
        assert_ne!(game.weapon_equipped, Some(weapon));
    }

    #[test]
    fn playing_heart_cards_heals_life_points() {
        let mut game = Scoundrel::new();

        game.life_points = 15;

        game.play_card(&Card::new(Suit::Hearts, Rank::Five));
        assert_eq!(game.life_points, 20);
    }

    #[test]
    fn player_max_life_points_are_20() {
        let mut game = Scoundrel::new();

        assert_eq!(game.life_points, 20);
        game.play_card(&Card::new(Suit::Hearts, Rank::Five));
        assert_eq!(game.life_points, 20);
    }
}
