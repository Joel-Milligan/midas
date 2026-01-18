use crate::ai::action::ActionStrategy;
use crate::ai::betting::BettingStrategy;
use crate::cards::Card;
use crate::cards::Hand;
use crate::player::Action;

pub struct Player {
    pub id: u8,
    pub balance: f32,
    betting_strategy: Box<dyn BettingStrategy>,
    action_strategy: Box<dyn ActionStrategy>,
}

impl Player {
    pub fn new(
        id: u8,
        balance: f32,
        betting_strategy: Box<dyn BettingStrategy>,
        action_strategy: Box<dyn ActionStrategy>,
    ) -> Self {
        Self {
            id,
            balance,
            betting_strategy,
            action_strategy,
        }
    }

    pub fn bet(&mut self) -> f32 {
        let bet = self.betting_strategy.bet();
        self.balance -= bet;
        bet
    }

    pub fn action(&self, hand: &Hand, dealer_card: &Card) -> Action {
        self.action_strategy.action(hand, dealer_card)
    }
}
