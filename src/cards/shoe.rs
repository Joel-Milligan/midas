use rand::prelude::SliceRandom;
use rand::rng;

use super::card::{Card, Face, Suit};

pub struct Shoe {
    cards: Vec<Card>,
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

        let discards = Vec::new();

        Self { cards, discards }
    }

    pub fn deal(&mut self) -> Card {
        if let Some(card) = self.cards.pop() {
            card
        } else {
            self.shuffle();
            self.cards
                .pop()
                .expect("Previous shuffle ensures there's at least one card")
        }
    }

    pub fn shuffle(&mut self) {
        self.cards.append(&mut self.discards);
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deal() {
        let mut shoe = Shoe::new();

        assert_eq!(
            shoe.deal(),
            Card {
                suit: Suit::Spade,
                face: Face::King
            }
        );
        assert_eq!(
            shoe.deal(),
            Card {
                suit: Suit::Spade,
                face: Face::Queen
            }
        );
        assert_eq!(
            shoe.deal(),
            Card {
                suit: Suit::Spade,
                face: Face::Jack
            }
        );
    }
}
