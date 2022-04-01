use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub face: u8,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let face = match self.face {
            1 => "Ace".to_string(),
            2..=10 => self.face.to_string(),
            11 => "Jack".to_string(),
            12 => "Queen".to_string(),
            13 => "King".to_string(),
            _ => "Unknown Value".to_string()
        };

        write!(f, "{} of {:?}s", face, self.suit)
    }
}

#[derive(Clone)]
pub struct Hand(Vec<Card>);

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

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
            if card.face == 1 {
                value += 11;
                aces += 1;
            } else if card.face > 10 {
                value += 10;
            } else {
                value += card.face;
            }
        }

        while value > 21 && aces > 0 {
            value -= 10;
        }

        value
    }
}

pub struct Shoe {
    cards: Vec<Card>
}

impl Shoe {
    pub fn new() -> Shoe {
        let mut cards = Vec::new();

        for face in 1..=13 {
            cards.push(Card { suit: Suit::Club, face });
        }

        for face in 1..=13 {
            cards.push(Card { suit: Suit::Diamond, face });
        }

        for face in 1..=13 {
            cards.push(Card { suit: Suit::Heart, face });
        }

        for face in 1..=13 {
            cards.push(Card { suit: Suit::Spade, face });
        }

        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        Shoe { cards }
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_hand_value() {
        let hand = Hand(vec![
            Card { suit: Suit::Club, face: 10 },
            Card { suit: Suit::Heart, face: 7 },
        ]);

        assert_eq!(hand.value(), 17);
    }

    #[test]
    fn royal_hand_value() {
        let hand = Hand(vec![
            Card { suit: Suit::Club, face: 11 },
            Card { suit: Suit::Heart, face: 7 },
        ]);

        assert_eq!(hand.value(), 17);

        let hand = Hand(vec![
            Card { suit: Suit::Club, face: 11 },
            Card { suit: Suit::Heart, face: 12 },
        ]);

        assert_eq!(hand.value(), 20);
    }

    #[test]
    fn ace_hand_value() {
        let hand = Hand(vec![
            Card { suit: Suit::Club, face: 1 },
            Card { suit: Suit::Heart, face: 7 },
        ]);

        assert_eq!(hand.value(), 18);

        let hand = Hand(vec![
            Card { suit: Suit::Heart, face: 10 },
            Card { suit: Suit::Club, face: 1 },
        ]);

        assert_eq!(hand.value(), 21);

        let hand = Hand(vec![
            Card { suit: Suit::Heart, face: 10 },
            Card { suit: Suit::Club, face: 1 },
            Card { suit: Suit::Diamond, face: 13 },
        ]);

        assert_eq!(hand.value(), 21);

        let hand = Hand(vec![
            Card { suit: Suit::Club, face: 1 },
            Card { suit: Suit::Heart, face: 1 },
            Card { suit: Suit::Club, face: 10 },
            Card { suit: Suit::Diamond, face: 1 },
        ]);

        assert_eq!(hand.value(), 13);
    }
}