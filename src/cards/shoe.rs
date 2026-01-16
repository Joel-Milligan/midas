use super::*;
use rand::prelude::SliceRandom;
use rand::rng;

pub struct Shoe {
    cards: Vec<Card>,
}

impl Shoe {
    pub fn new() -> Shoe {
        let mut cards = Vec::new();

        for suit in Suit::VARIANTS {
            for face in Face::VARIANTS {
                cards.push(Card { suit, face });
            }
        }

        Shoe { cards }
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn shuffle(&mut self, discards: &mut Vec<Card>) {
        self.cards.append(discards);
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
            shoe.deal().unwrap(),
            Card {
                suit: Suit::Spade,
                face: Face::King
            }
        );
        assert_eq!(
            shoe.deal().unwrap(),
            Card {
                suit: Suit::Spade,
                face: Face::Queen
            }
        );
        assert_eq!(
            shoe.deal().unwrap(),
            Card {
                suit: Suit::Spade,
                face: Face::Jack
            }
        );
    }

    #[test]
    fn deal_until_empty() {
        let mut shoe = Shoe::new();

        // Empty the shoe
        for _ in 0..52 {
            shoe.deal();
        }

        assert_eq!(shoe.deal(), None);
    }
}
