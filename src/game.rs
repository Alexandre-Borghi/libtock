use rand::seq::SliceRandom;

use crate::{card::Card, color::Color, player::Player};

pub struct Game {
    deck: [Card; 52],
    players: [Player; 4],
}

impl Game {
    pub fn new() -> Self {
        Self {
            deck: Self::ordered_deck(),
            players: [Default::default(); 4],
        }
    }

    pub fn deck_shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.deck.shuffle(&mut rng);
    }

    const fn ordered_deck() -> [Card; 52] {
        [
            Card::new(1, Color::Clubs),
            Card::new(1, Color::Diamonds),
            Card::new(1, Color::Hearts),
            Card::new(1, Color::Spades),
            Card::new(2, Color::Clubs),
            Card::new(2, Color::Diamonds),
            Card::new(2, Color::Hearts),
            Card::new(2, Color::Spades),
            Card::new(3, Color::Clubs),
            Card::new(3, Color::Diamonds),
            Card::new(3, Color::Hearts),
            Card::new(3, Color::Spades),
            Card::new(4, Color::Clubs),
            Card::new(4, Color::Diamonds),
            Card::new(4, Color::Hearts),
            Card::new(4, Color::Spades),
            Card::new(5, Color::Clubs),
            Card::new(5, Color::Diamonds),
            Card::new(5, Color::Hearts),
            Card::new(5, Color::Spades),
            Card::new(6, Color::Clubs),
            Card::new(6, Color::Diamonds),
            Card::new(6, Color::Hearts),
            Card::new(6, Color::Spades),
            Card::new(7, Color::Clubs),
            Card::new(7, Color::Diamonds),
            Card::new(7, Color::Hearts),
            Card::new(7, Color::Spades),
            Card::new(8, Color::Clubs),
            Card::new(8, Color::Diamonds),
            Card::new(8, Color::Hearts),
            Card::new(8, Color::Spades),
            Card::new(9, Color::Clubs),
            Card::new(9, Color::Diamonds),
            Card::new(9, Color::Hearts),
            Card::new(9, Color::Spades),
            Card::new(10, Color::Clubs),
            Card::new(10, Color::Diamonds),
            Card::new(10, Color::Hearts),
            Card::new(10, Color::Spades),
            Card::new(11, Color::Clubs),
            Card::new(11, Color::Diamonds),
            Card::new(11, Color::Hearts),
            Card::new(11, Color::Spades),
            Card::new(12, Color::Clubs),
            Card::new(12, Color::Diamonds),
            Card::new(12, Color::Hearts),
            Card::new(12, Color::Spades),
            Card::new(13, Color::Clubs),
            Card::new(13, Color::Diamonds),
            Card::new(13, Color::Hearts),
            Card::new(13, Color::Spades),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_shuffle() {
        let mut game = Game::new();
        game.deck_shuffle();
        assert_ne!(Game::ordered_deck(), game.deck);
    }
}
