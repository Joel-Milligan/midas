use super::*;

#[derive(Clone)]
pub struct Hand(Vec<Card>);

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
            match card.face {
                Face::Ace => {
                    value += 11;
                    aces += 1;
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

        while value > 21 && aces > 0 {
            value -= 10;
        }

        value
    }

    pub fn discard_hand(&mut self) -> Vec<Card> {
        let ret = self.0.clone();
        self.0.clear();
        ret
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn discard_hand() {
        let mut hand = Hand::new();

        hand.add_card(Card {
            suit: Suit::Diamond,
            face: Face::Eight,
        });

        let discard = hand.discard_hand();

        assert_eq!(hand.0.len(), 0);
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

        assert_eq!(hand.0.len(), 0);
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
