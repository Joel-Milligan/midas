use super::card::{Card, Face};

#[derive(Clone)]
pub struct Hand(pub Vec<Card>);

impl Hand {
    pub fn new() -> Hand {
        Hand(Vec::new())
    }

    pub fn add_card(&mut self, card: Card) {
        self.0.push(card);
    }

    pub fn value(&self) -> u8 {
        let mut value = 0;
        let mut aces = 0;

        for card in &self.0 {
            value += match card.face {
                Face::Ace => {
                    aces += 1;
                    11
                }
                Face::Two => 2,
                Face::Three => 3,
                Face::Four => 4,
                Face::Five => 5,
                Face::Six => 6,
                Face::Seven => 7,
                Face::Eight => 8,
                Face::Nine => 9,
                Face::Ten | Face::Jack | Face::Queen | Face::King => 10,
            }
        }

        while value > 21 && aces > 0 {
            value -= 10;
            aces -= 1;
        }

        value
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::card::Suit;

    use super::*;

    #[test]
    fn new() {
        let hand = Hand::new();

        assert_eq!(hand.0.len(), 0);
    }

    #[test]
    fn add_card() {
        let mut hand = Hand::new();

        hand.add_card(Card {
            suit: Suit::Spade,
            face: Face::Ace,
        });

        hand.add_card(Card {
            suit: Suit::Diamond,
            face: Face::King,
        });

        assert_eq!(hand.0.len(), 2);
        assert_eq!(
            hand.0,
            vec![
                Card {
                    suit: Suit::Spade,
                    face: Face::Ace,
                },
                Card {
                    suit: Suit::Diamond,
                    face: Face::King,
                }
            ]
        );
    }

    #[test]
    fn value() {
        // Normal
        let hand = Hand(vec![
            Card {
                suit: Suit::Club,
                face: Face::Ten,
            },
            Card {
                suit: Suit::Heart,
                face: Face::Seven,
            },
        ]);

        assert_eq!(hand.value(), 17);

        // Royal
        let hand = Hand(vec![
            Card {
                suit: Suit::Club,
                face: Face::Jack,
            },
            Card {
                suit: Suit::Heart,
                face: Face::Seven,
            },
        ]);

        assert_eq!(hand.value(), 17);

        let hand = Hand(vec![
            Card {
                suit: Suit::Club,
                face: Face::Jack,
            },
            Card {
                suit: Suit::Heart,
                face: Face::Queen,
            },
        ]);

        assert_eq!(hand.value(), 20);

        // Ace
        let hand = Hand(vec![
            Card {
                suit: Suit::Club,
                face: Face::Ace,
            },
            Card {
                suit: Suit::Heart,
                face: Face::Seven,
            },
        ]);

        assert_eq!(hand.value(), 18);

        let hand = Hand(vec![
            Card {
                suit: Suit::Heart,
                face: Face::Ten,
            },
            Card {
                suit: Suit::Club,
                face: Face::Ace,
            },
        ]);

        assert_eq!(hand.value(), 21);

        let hand = Hand(vec![
            Card {
                suit: Suit::Heart,
                face: Face::Ten,
            },
            Card {
                suit: Suit::Club,
                face: Face::Ace,
            },
            Card {
                suit: Suit::Diamond,
                face: Face::King,
            },
        ]);

        assert_eq!(hand.value(), 21);

        let hand = Hand(vec![
            Card {
                suit: Suit::Club,
                face: Face::Ace,
            },
            Card {
                suit: Suit::Heart,
                face: Face::Ace,
            },
            Card {
                suit: Suit::Club,
                face: Face::Ten,
            },
            Card {
                suit: Suit::Diamond,
                face: Face::Ace,
            },
        ]);

        assert_eq!(hand.value(), 13);
    }
}
