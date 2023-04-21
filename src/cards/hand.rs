use crate::{players::player::Ref, Card, Face};

#[derive(Default, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub bet: i32,
    pub player: Ref,
}

impl Hand {
    #[must_use]
    pub fn new(player: Ref) -> Self {
        Self {
            cards: vec![],
            bet: 0,
            player,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    #[must_use]
    pub fn value(&self) -> u8 {
        let mut value = 0;
        let mut high_aces = 0;

        for card in &self.cards {
            match card.face {
                Face::Ace => {
                    value += 11;
                    high_aces += 1;
                }
                Face::Two => value += 2,
                Face::Three => value += 3,
                Face::Four => value += 4,
                Face::Five => value += 5,
                Face::Six => value += 6,
                Face::Seven => value += 7,
                Face::Eight => value += 8,
                Face::Nine => value += 9,
                Face::Ten | Face::Jack | Face::Queen | Face::King => value += 10,
            }
        }

        while value > 21 && high_aces > 0 {
            value -= 10;
            high_aces -= 1;
        }

        value
    }

    pub fn discard_hand(&mut self) -> Vec<Card> {
        let ret = self.cards.clone();
        self.cards.clear();
        ret
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn bet(&mut self, bet: i32) {
        self.bet = bet;
    }
}

#[cfg(test)]
mod tests {
    use crate::Suit;

    use super::*;

    #[test]
    fn new() {
        let hand = Hand::new(Ref::default());
        assert_eq!(hand.len(), 0);
    }

    #[test]
    fn default() {
        let hand = Hand::default();
        assert_eq!(hand.len(), 0);
    }

    #[test]
    fn add_card() {
        let mut hand = Hand::default();

        hand.add_card(Card {
            suit: Suit::Spade,
            face: Face::Ace,
        });

        hand.add_card(Card {
            suit: Suit::Diamond,
            face: Face::King,
        });

        assert_eq!(hand.len(), 2);
        assert_eq!(
            hand.cards,
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
        let hand = Hand {
            cards: vec![
                Card {
                    suit: Suit::Club,
                    face: Face::Ten,
                },
                Card {
                    suit: Suit::Heart,
                    face: Face::Seven,
                },
            ],
            bet: 0,
            player: Ref::default(),
        };

        assert_eq!(hand.value(), 17);

        // Royal
        let hand = Hand {
            cards: vec![
                Card {
                    suit: Suit::Club,
                    face: Face::Jack,
                },
                Card {
                    suit: Suit::Heart,
                    face: Face::Seven,
                },
            ],
            bet: 0,
            player: Ref::default(),
        };

        assert_eq!(hand.value(), 17);

        let hand = Hand {
            cards: vec![
                Card {
                    suit: Suit::Club,
                    face: Face::Jack,
                },
                Card {
                    suit: Suit::Heart,
                    face: Face::Queen,
                },
            ],
            bet: 0,
            player: Ref::default(),
        };

        assert_eq!(hand.value(), 20);

        // Ace
        let hand = Hand {
            cards: vec![
                Card {
                    suit: Suit::Club,
                    face: Face::Ace,
                },
                Card {
                    suit: Suit::Heart,
                    face: Face::Seven,
                },
            ],
            bet: 0,
            player: Ref::default(),
        };

        assert_eq!(hand.value(), 18);

        let hand = Hand {
            cards: vec![
                Card {
                    suit: Suit::Heart,
                    face: Face::Ten,
                },
                Card {
                    suit: Suit::Club,
                    face: Face::Ace,
                },
            ],
            bet: 0,
            player: Ref::default(),
        };

        assert_eq!(hand.value(), 21);

        let hand = Hand {
            cards: vec![
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
            ],
            bet: 0,
            player: Ref::default(),
        };

        assert_eq!(hand.value(), 21);

        let hand = Hand {
            cards: vec![
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
            ],
            bet: 0,
            player: Ref::default(),
        };

        assert_eq!(hand.value(), 13);
    }

    #[test]
    fn discard_hand() {
        let mut hand = Hand::default();

        hand.add_card(Card {
            suit: Suit::Diamond,
            face: Face::Eight,
        });

        let discard = hand.discard_hand();

        assert_eq!(hand.len(), 0);
        assert_eq!(discard.len(), 1);
        assert_eq!(
            discard,
            vec![Card {
                suit: Suit::Diamond,
                face: Face::Eight
            }]
        );

        hand.add_card(Card {
            suit: Suit::Club,
            face: Face::Jack,
        });
        hand.add_card(Card {
            suit: Suit::Spade,
            face: Face::Three,
        });
        hand.add_card(Card {
            suit: Suit::Club,
            face: Face::Ace,
        });

        let discard = hand.discard_hand();

        assert_eq!(hand.len(), 0);
        assert_eq!(discard.len(), 3);
        assert_eq!(
            discard,
            vec![
                Card {
                    suit: Suit::Club,
                    face: Face::Jack
                },
                Card {
                    suit: Suit::Spade,
                    face: Face::Three
                },
                Card {
                    suit: Suit::Club,
                    face: Face::Ace
                },
            ]
        );
    }
}
