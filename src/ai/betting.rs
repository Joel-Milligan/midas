pub trait BettingStrategy {
    fn bet(&self) -> f32;
}

pub struct FlatBettingStrategy;

impl BettingStrategy for FlatBettingStrategy {
    fn bet(&self) -> f32 {
        10.
    }
}
