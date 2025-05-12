//! Scoundrel card game implementation
//!
//! This module contains the core game logic for a card-based adventure game
//! where players navigate rooms, battle monsters, and manage resources.
//!

use crate::{
    card::{Card},
    deck::Deck,
    rank::Rank,
    suit::Suit,
};

/// Maximum life points a player can have
pub const MAX_LIFE_POINTS: u8 = 20;

/// Total rooms in the game
pub const TOTAL_ROOMS: usize = 12;

/// Total number of cards in a room.
pub const ROOM_SIZE: usize = 4;

/// Represents the current state of the game.
#[derive(PartialEq, Eq, Debug)]
enum GameState {
    /// The game is still in progress.
    InGame,
    /// The player has won the game.
    Win,
    /// The player has lost the game.
    Lose,
}

/// The character weapon.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Weapon {
    /// This card represent the weapon equipped.
    weapon: Card,
    /// This card represent the stack of monsters slayed with the weapon.
    /// The weapon can't slay a monster greater or equal to the latest slayed.
    monster_stack: Vec<Card>,
}

impl Weapon {
        /// Creates a new weapon from a card
    fn new(card: Card) -> Self {
        // TODO: it might be possible to size the monster stack.
        Self {
            weapon: card,
            monster_stack: Vec::new(),
        }
    }

    fn monster_stack(&self) -> Vec<Card> {
        self.monster_stack.clone()
    }
}

/// The main game struct representing the player's state
///
/// # Fields
/// - `deck`: The game deck with banned cards removed
/// - `life_points`: Player's health (max 20)
/// - `weapon_equipped`: Currently equipped weapon card
/// - `room_visited`: Number of rooms entered
///
/// # Examples
/// ```
/// use scoundrel::Scoundrel;
///
/// let mut game = Scoundrel::new();
/// assert_eq!(game.life_points, 20);
/// ```
struct Scoundrel {
    /// The deck of cards used in the game. Some cards are banned at initialization.
    deck: Deck,
    /// Current life points of the character. Maximum is 20.
    life_points: u8,
    /// Currently equipped weapon, if any.
    weapon_equipped: Option<Weapon>,
    /// Number of rooms the character has visited.
    room_visited: usize,
    /// Current room visited.
    room: Vec<Card>,
}

impl Scoundrel {
        /// Banned cards that are removed from the deck at game start
    const BANNED_CARDS: &'static [(Suit, Rank)] = &[
        (Suit::Diamonds, Rank::Ace),
        (Suit::Diamonds, Rank::Jack),
        (Suit::Diamonds, Rank::Queen),
        (Suit::Diamonds, Rank::King),
        (Suit::Hearts, Rank::Ace),
        (Suit::Hearts, Rank::Jack),
        (Suit::Hearts, Rank::Queen),
        (Suit::Hearts, Rank::King),
    ];

    /// Creates a new Scoundrel game instance
    ///
    /// Initializes with:
    /// - 44-card deck (standard 52 minus banned cards)
    /// - `MAX_LIFE_POINTS` life points
    /// - No equipped weapon
    /// - Starting room (0)
    pub fn new() -> Self {
        let banned_cards: Vec<_> = Self::BANNED_CARDS.iter()
            .map(|&(suit, rank)| Card::new(suit, rank))
            .collect();

        Self {
            deck: Deck::builder().ban_cards(banned_cards).build(),
            life_points: MAX_LIFE_POINTS,
            weapon_equipped: None,
            room_visited: 0,
            room: Vec::with_capacity(ROOM_SIZE),
        }
    }

    /// Enters a new room, drawing 4 cards
    ///
    /// # Returns
    /// - `Some(Vec<Card>)` with 4 drawn cards if rooms remain
    /// - `None` if all `TOTAL_ROOMS` rooms have been visited
    ///
    /// # Examples
    /// ```
    /// let mut game = Scoundrel::new();
    /// let cards = game.enter_room().expect("First room should succeed");
    /// assert_eq!(cards.len(), 4);
    /// ```
    pub fn enter_room(&mut self) -> GameState {
        // In case the deck is over, end the game
        if self.room_visited >= TOTAL_ROOMS {
            return GameState::Win;
        }

        self.room_visited += 1;
        match self.room.len() {
            0 => {
                // In case new game or ran away from a room, hand is empty.
                self.room.append(&mut self.deck.draw(4));
            }
            1 => {
                // In case exited a room, hand has 1 card
                self.room.append(&mut self.deck.draw(3));
            }
            _ => {
                todo!() // TODO: should throw an error, or the room is empty (beginning or run) or has 1 card (standard case)
            }
        }

        GameState::InGame
    }

    fn fight_barehanded(&mut self, monster: &Card) -> GameState {
        // In case the rank is higher than the `self.life_points`
        // the character dies. GAME OVER
        if monster.rank() >= self.life_points {
            self.life_points = 0;
            return GameState::Lose;
        }

        // In case of barehanded fight, the damage is inflicted directly to the character.
        self.life_points -= monster.rank();
        GameState::InGame
    }

    fn fight_with_weapon(&mut self, monster: &Card, mut weapon: Weapon) -> (GameState, Weapon) {
        // In case the last monster is smaller than the last one fought
        // the weapon can be used to slay it. Otherwise is is a barehanded fight.
        if weapon
            .monster_stack
            .last()
            .is_some_and(|last_monster| last_monster.rank() < monster.rank())
        {
            // The attack dealt by the monster is equal to the monster `rank` - weapon `rank`. If 0 no damage inflicted to the character.
            let attack = (monster.rank() - weapon.weapon.rank().into()).max(0); // TODO: maybe fix the into() with an impl

            // Save the fight as the latest monster fought.
            weapon.monster_stack.push(*monster);

            // In case the rank is higher than the `self.life_points`
            // the character dies. GAME OVER
            if attack >= self.life_points {
                self.life_points = 0;
                return (GameState::Lose, weapon);
            }

            self.life_points -= attack;
            (GameState::InGame, weapon)
        } else {
            (self.fight_barehanded(monster), weapon)
        }
    }

    fn handle_combat(&mut self, card: &Card) -> GameState {
        // Explicitly taking ownership of the weapon.
        // It will be re-equipped after the fight.
        let weapon = self.weapon_equipped.take();

        // In case weapon equipped check if it is possible
        // to use it or if character has to fight barehanded
        let state = match weapon {
            Some(mut weapon) => {
                let (state, weapon_updated) = self.fight_with_weapon(card, weapon);
                self.weapon_equipped = Some(weapon_updated);
                state
            
            },
            None => self.fight_barehanded(card),
        };


        state
    }

    /// Plays a card from hand, modifying game state
    ///
    /// # Arguments
    /// * `card` - The card to play
    ///
    /// # Returns
    /// Updated `GameState` after playing the card
    ///
    /// # Card Effects
    /// - **Spades/Clubs**: {Deals damage equal to rank} TODO
    /// - **Diamonds**: Equips as weapon
    /// - **Hearts**: Heals life points equal to rank
    pub fn play_card(&mut self, card: &Card) -> GameState {
        match card.suit() {
            Suit::Spades | Suit::Clubs => return self.handle_combat(card),
            Suit::Diamonds => self.weapon_equipped = Some(Weapon::new(*card)),
            Suit::Hearts => {
                self.life_points = (self.life_points + card.rank()).min(20);
            }
        }
        GameState::InGame
    }

    /// Returns current life points
    pub fn life_points(&self) -> u8 {
        self.life_points
    }

    /// Returns currently equipped weapon, if any
    pub fn weapon_equipped(&self) -> Option<&Weapon> {
        self.weapon_equipped.as_ref()
    }

    /// Returns number of rooms visited
    pub fn rooms_visited(&self) -> usize {
        self.room_visited
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

        assert_eq!(game.life_points, MAX_LIFE_POINTS);
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
        assert_eq!(game.life_points, 0);
        assert_eq!(game_state, GameState::Lose);
    }

    #[test]
    fn when_life_points_reach_0_game_over_underflow() {
        let mut game = Scoundrel::new();

        let mut game_state = game.play_card(&Card::new(Suit::Spades, Rank::Five));
        assert_eq!(game.life_points, 15);
        assert_eq!(game_state, GameState::InGame);

        game_state = game.play_card(&Card::new(Suit::Clubs, Rank::Five));
        assert_eq!(game.life_points, 10);
        assert_eq!(game_state, GameState::InGame);

        game_state = game.play_card(&Card::new(Suit::Clubs, Rank::King));
        assert_eq!(game.life_points, 0);
        assert_eq!(game_state, GameState::Lose);
    }

    #[test]
    fn playing_diamond_cards_equips_a_weapon_starts_with_no_equipment() {
        let mut game = Scoundrel::new();

        let weapon = Card::new(Suit::Diamonds, Rank::Nine);

        game.play_card(&weapon);
        assert_eq!(
            game.weapon_equipped().expect("Weapon just equipped").weapon,
            weapon
        );
    }

    #[test]
    fn playing_diamond_cards_equips_a_weapon_and_discard_old() {
        let mut game = Scoundrel::new();

        let weapon = Card::new(Suit::Diamonds, Rank::Nine);

        game.play_card(&weapon);
        assert_eq!(
            game.weapon_equipped().expect("Weapon just equipped").weapon,
            weapon
        );

        let new_weapon = Card::new(Suit::Diamonds, Rank::Five);

        game.play_card(&new_weapon);
        assert_eq!(
            game.weapon_equipped().expect("Weapon just equipped").weapon,
            new_weapon
        );
        assert_ne!(
            game.weapon_equipped().expect("Weapon just equipped").weapon,
            weapon
        );
    }

    #[test]
    fn fight_a_monster_with_a_weapon() {
        let mut game = Scoundrel::new();

        let weapon = Card::new(Suit::Diamonds, Rank::Nine);

        game.play_card(&weapon);
        assert_ne!(
            game.weapon_equipped().expect("Weapon just equipped").weapon,
            weapon
        );

        let monster = Card::new(Suit::Clubs, Rank::Eight);
        game.play_card(&monster);
        // Because the monster has lower rank compared to the weapon,
        // no life-points should be removed, but card should be saved on the weapon's
        // monster stack.
        assert_eq!(game.life_points(), 20);
        assert_eq!(
            game.weapon_equipped
                .expect("Weapon has just been equipped")
                .monster_stack(),
            vec![monster]
        );
    }

    #[test]
    fn fight_a_monster_with_a_weapon_but_monster_bigger_than_weapon() {
        let mut game = Scoundrel::new();

        let weapon = Card::new(Suit::Diamonds, Rank::Nine);

        game.play_card(&weapon);
        assert_ne!(
            game.weapon_equipped().expect("Weapon just equipped").weapon,
            weapon
        );

        let monster = Card::new(Suit::Clubs, Rank::Jack);
        game.play_card(&monster);
        // Because the monster has higher rank compared to the weapon,
        // `monster.rank` - `weapon.rank` () life-points should be removed,
        // life_points - (monster - weapon) = 20 - (11 - 9)
        // but card should be saved on the weapon's
        // monster stack.
        assert_eq!(game.life_points(), 18);
        assert_eq!(
            game.weapon_equipped
                .expect("Weapon has just been equipped")
                .monster_stack(),
            vec![monster]
        );
    }

    #[test]
    fn fight_a_monster_with_a_weapon_but_monster_bigger_than_last_monster() {
        let mut game = Scoundrel::new();

        let weapon = Card::new(Suit::Diamonds, Rank::Nine);

        game.play_card(&weapon);
        assert_ne!(
            game.weapon_equipped().expect("Weapon just equipped").weapon,
            weapon
        );

        let monster = Card::new(Suit::Clubs, Rank::Two);
        game.play_card(&monster);
        // Because the monster has lower rank compared to the weapon,
        // no life-points should be removed, but card should be saved on the weapon's
        // monster stack.
        assert_eq!(game.life_points(), 20);
        assert_eq!(
            game.weapon_equipped
                .as_ref()
                .expect("Weapon has just been equipped")
                .monster_stack(),
            vec![monster]
        );

        let new_monster = Card::new(Suit::Clubs, Rank::Ten);
        game.play_card(&new_monster);
        // Because the monster has higher rank compared to the weapon latest monster in the stack,
        // `new_monster` rank life-points should be removed, but card shouldn't be saved on the weapon's
        // monster stack. It is like the monster if fought barehanded.
        assert_eq!(game.life_points(), 10);
        assert_eq!(
            game.weapon_equipped
                .expect("Weapon has just been equipped")
                .monster_stack()
                .as_ref(),
            vec![monster]
        );
    }

    #[test]
    fn playing_heart_cards_heals_life_points() {
        let mut game = Scoundrel::new();

        game.life_points = 15;

        game.play_card(&Card::new(Suit::Hearts, Rank::Five));
        assert_eq!(game.life_points, MAX_LIFE_POINTS);
    }

    #[test]
    fn player_max_life_points_are_20() {
        let mut game = Scoundrel::new();

        assert_eq!(game.life_points, MAX_LIFE_POINTS);
        game.play_card(&Card::new(Suit::Hearts, Rank::Five));
        assert_eq!(game.life_points, MAX_LIFE_POINTS);
    }
}
