use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::{Card, Face, Suit};

#[derive(Default)]
pub struct Shoe {
    cards: Vec<Card>,
}

impl Shoe {
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn new() -> Self {
        Self {
            cards: vec![
                Card {
                    suit: Suit::Club,
                    face: Face::Ace,
                },
                Card {
                    suit: Suit::Club,
                    face: Face::Two,
                },
                Card {
                    suit: Suit::Club,
                    face: Face::Three,
                },
                Card {
                    suit: Suit::Club,
                    face: Face::Four,
                },
                Card {
                    suit: Suit::Club,
                    face: Face::Five,
                },
                Card {
                    suit: Suit::Club,
                    face: Face::Six,
                },
                Card {
                    suit: Suit::Club,
                    face: Face::Seven,
                },
                Card {
                    suit: Suit::Club,
                    face: Face::Eight,
                },
                Card {
                    suit: Suit::Club,
                    face: Face::Nine,
                },
                Card {
                    suit: Suit::Club,
                    face: Face::Ten,
                },
                Card {
                    suit: Suit::Club,
                    face: Face::Jack,
                },
                Card {
                    suit: Suit::Club,
                    face: Face::Queen,
                },
                Card {
                    suit: Suit::Club,
                    face: Face::King,
                },
                Card {
                    suit: Suit::Diamond,
                    face: Face::Ace,
                },
                Card {
                    suit: Suit::Diamond,
                    face: Face::Two,
                },
                Card {
                    suit: Suit::Diamond,
                    face: Face::Three,
                },
                Card {
                    suit: Suit::Diamond,
                    face: Face::Four,
                },
                Card {
                    suit: Suit::Diamond,
                    face: Face::Five,
                },
                Card {
                    suit: Suit::Diamond,
                    face: Face::Six,
                },
                Card {
                    suit: Suit::Diamond,
                    face: Face::Seven,
                },
                Card {
                    suit: Suit::Diamond,
                    face: Face::Eight,
                },
                Card {
                    suit: Suit::Diamond,
                    face: Face::Nine,
                },
                Card {
                    suit: Suit::Diamond,
                    face: Face::Ten,
                },
                Card {
                    suit: Suit::Diamond,
                    face: Face::Jack,
                },
                Card {
                    suit: Suit::Diamond,
                    face: Face::Queen,
                },
                Card {
                    suit: Suit::Diamond,
                    face: Face::King,
                },
                Card {
                    suit: Suit::Heart,
                    face: Face::Ace,
                },
                Card {
                    suit: Suit::Heart,
                    face: Face::Two,
                },
                Card {
                    suit: Suit::Heart,
                    face: Face::Three,
                },
                Card {
                    suit: Suit::Heart,
                    face: Face::Four,
                },
                Card {
                    suit: Suit::Heart,
                    face: Face::Five,
                },
                Card {
                    suit: Suit::Heart,
                    face: Face::Six,
                },
                Card {
                    suit: Suit::Heart,
                    face: Face::Seven,
                },
                Card {
                    suit: Suit::Heart,
                    face: Face::Eight,
                },
                Card {
                    suit: Suit::Heart,
                    face: Face::Nine,
                },
                Card {
                    suit: Suit::Heart,
                    face: Face::Ten,
                },
                Card {
                    suit: Suit::Heart,
                    face: Face::Jack,
                },
                Card {
                    suit: Suit::Heart,
                    face: Face::Queen,
                },
                Card {
                    suit: Suit::Heart,
                    face: Face::King,
                },
                Card {
                    suit: Suit::Spade,
                    face: Face::Ace,
                },
                Card {
                    suit: Suit::Spade,
                    face: Face::Two,
                },
                Card {
                    suit: Suit::Spade,
                    face: Face::Three,
                },
                Card {
                    suit: Suit::Spade,
                    face: Face::Four,
                },
                Card {
                    suit: Suit::Spade,
                    face: Face::Five,
                },
                Card {
                    suit: Suit::Spade,
                    face: Face::Six,
                },
                Card {
                    suit: Suit::Spade,
                    face: Face::Seven,
                },
                Card {
                    suit: Suit::Spade,
                    face: Face::Eight,
                },
                Card {
                    suit: Suit::Spade,
                    face: Face::Nine,
                },
                Card {
                    suit: Suit::Spade,
                    face: Face::Ten,
                },
                Card {
                    suit: Suit::Spade,
                    face: Face::Jack,
                },
                Card {
                    suit: Suit::Spade,
                    face: Face::Queen,
                },
                Card {
                    suit: Suit::Spade,
                    face: Face::King,
                },
            ],
        }
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn shuffle(&mut self, discards: &mut Vec<Card>) {
        self.cards.append(discards);
        let mut rng = thread_rng();
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

        for _ in 0..52 {
            shoe.deal();
        }

        assert_eq!(shoe.deal(), None);
    }
}
