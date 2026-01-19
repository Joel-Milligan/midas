use crate::cards::Card;

pub trait BettingStrategy {
    fn bet(&self, remaining: f32) -> f32;
    fn update(&mut self, cards: &Vec<Card>);
}

pub struct FlatBettingStrategy;

impl BettingStrategy for FlatBettingStrategy {
    fn bet(&self, _remaining: f32) -> f32 {
        10.
    }

    fn update(&mut self, _cards: &Vec<Card>) {}
}

pub struct HiLoCountingStrategy {
    count: f32,
    cards_seen_since_shuffle: usize,
}

impl HiLoCountingStrategy {
    pub fn new() -> Self {
        Self {
            count: 0.0,
            cards_seen_since_shuffle: 0,
        }
    }
}

impl BettingStrategy for HiLoCountingStrategy {
    fn bet(&self, remaining: f32) -> f32 {
        let bet = 10.0 + 1.0 * self.count.max(0.0);
        if bet > remaining { remaining } else { bet }
    }

    fn update(&mut self, cards: &Vec<Card>) {
        self.cards_seen_since_shuffle += cards.len();
        if self.cards_seen_since_shuffle >= 52 {
            self.count = 0.0;
            self.cards_seen_since_shuffle = 0;
        } else {
            for card in cards {
                match card.face.value() {
                    2..=6 => self.count += 1.0,
                    10..=11 => self.count -= 1.0,
                    _ => {}
                }
            }
        }
    }
}
