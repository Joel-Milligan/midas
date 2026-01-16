use crate::cards::Card;
use crate::cards::Hand;
use crate::player::Player;
use crate::player::actions::Action;

#[derive(Clone)]
pub struct SimpleAi {
    pub balance: f32,
    cutoff: u8,
}

impl Player for SimpleAi {
    fn new(cutoff: u8) -> impl Player {
        Self {
            balance: 100.,
            cutoff,
        }
    }

    fn balance(&self) -> f32 {
        self.balance
    }

    fn bet(&mut self) -> f32 {
        self.balance -= 10.;
        10.
    }

    fn deduct(&mut self, amount: f32) {
        self.balance -= amount;
    }

    fn win(&mut self, amount: f32) {
        self.balance += amount;
    }

    fn action(&self, hand: &Hand, _dealer_card: &Card) -> Action {
        if hand.cards.len() == 2 {
            if hand.is_pair() {
                return Action::Split;
            }

            if hand.value() == 11 {
                return Action::Double;
            }
        }

        if hand.value() < self.cutoff {
            Action::Hit
        } else {
            Action::Stand
        }
    }
}
