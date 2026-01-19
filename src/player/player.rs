use crate::ai::action::ActionStrategy;
use crate::ai::betting::BettingStrategy;
use crate::cards::Card;
use crate::cards::Hand;
use crate::player::Action;

pub struct Player {
    pub id: u8,
    pub balance: f32,
    action_strategy: Box<dyn ActionStrategy>,
    betting_strategy: Box<dyn BettingStrategy>,
}

impl Player {
    pub fn new(
        id: u8,
        balance: f32,
        action_strategy: Box<dyn ActionStrategy>,
        betting_strategy: Box<dyn BettingStrategy>,
    ) -> Self {
        Self {
            id,
            balance,
            action_strategy,
            betting_strategy,
        }
    }

    pub fn card_dealt(&mut self, card: &Card) {
        self.betting_strategy.card_dealt(card);
    }

    pub fn shuffled(&mut self) {
        self.betting_strategy.shuffled();
    }

    pub fn bet(&mut self) -> f32 {
        let bet = self.betting_strategy.bet(self.balance);
        self.balance -= bet;
        bet
    }

    pub fn action(&self, hand: &Hand, dealer_card: &Card) -> Action {
        self.action_strategy.action(hand, dealer_card)
    }
}
