use crate::ai::ActionStrategy;
use crate::cards::Card;
use crate::cards::Hand;
use crate::player::Action;

pub struct Player {
    pub id: u8,
    pub balance: f32,
    action_strategy: Box<dyn ActionStrategy>,
}

impl Player {
    pub fn new(id: u8, balance: f32, action_strategy: Box<dyn ActionStrategy>) -> Self {
        Self {
            id,
            balance,
            action_strategy,
        }
    }

    pub fn bet(&mut self) -> f32 {
        self.balance -= 10.;
        10.
    }

    pub fn action(&self, hand: &Hand, dealer_card: &Card) -> Action {
        self.action_strategy.action(hand, dealer_card)
    }
}
