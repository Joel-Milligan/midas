use crate::cards::Hand;

pub enum PlayerAction {
    Hit,
    Stand,
}

#[derive(Clone)]
pub struct Player {
    pub hand: Hand,
    pub balance: f32,
    cutoff: u8,
}

impl Player {
    pub fn new(cutoff: u8) -> Player {
        Player {
            hand: Hand::new(),
            balance: 100.,
            cutoff,
        }
    }

    pub fn bet(&mut self) -> f32 {
        self.balance -= 10.;
        10.
    }

    pub fn action(&self) -> PlayerAction {
        if self.hand.value() < self.cutoff {
            PlayerAction::Hit
        } else {
            PlayerAction::Stand
        }
    }
}
