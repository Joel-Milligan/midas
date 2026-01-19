use rand::prelude::SliceRandom;
use rand::rng;

use super::card::{Card, Face, Suit};

pub struct Shoe {
    pub cards: Vec<Card>,
    pub discards: Vec<Card>,
}

impl Shoe {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for suit in Suit::VARIANTS {
            for face in Face::VARIANTS {
                cards.push(Card { suit, face });
            }
        }

        Self {
            cards,
            discards: vec![],
        }
    }

    /// Returns dealt card and indicates whether or not the deal resulted in a shuffle
    pub fn deal(&mut self) -> (Card, bool) {
        if let Some(card) = self.cards.pop() {
            (card, false)
        } else {
            self.shuffle();
            (
                self.cards.pop().expect(
                    format!(
                        "Shoe: {}, Discards: {}",
                        self.cards.len(),
                        self.discards.len()
                    )
                    .as_str(),
                ),
                true,
            )
        }
    }

    fn shuffle(&mut self) {
        self.cards.append(&mut self.discards);
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }
}
