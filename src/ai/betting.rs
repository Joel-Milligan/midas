use crate::cards::Card;

pub trait BettingStrategy {
    fn bet(&self, remaining: f32) -> f32;
    fn card_dealt(&mut self, card: &Card);
    fn shuffled(&mut self);
}

pub struct FlatBettingStrategy;

impl BettingStrategy for FlatBettingStrategy {
    fn bet(&self, _remaining: f32) -> f32 {
        10.
    }

    fn card_dealt(&mut self, _card: &Card) {}
    fn shuffled(&mut self) {}
}

pub struct HiLoCountingStrategy {
    count: f32,
}

impl HiLoCountingStrategy {
    pub fn new() -> Self {
        Self { count: 0.0 }
    }
}

impl BettingStrategy for HiLoCountingStrategy {
    fn bet(&self, remaining: f32) -> f32 {
        (10.0 + 1.0 * self.count.max(0.0)).min(remaining)
    }

    fn card_dealt(&mut self, card: &Card) {
        match card.face.value() {
            2..=6 => self.count += 1.0,
            10..=11 => self.count -= 1.0,
            _ => {}
        }
    }

    fn shuffled(&mut self) {
        self.count = 0.0;
    }
}
