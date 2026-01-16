use crate::cards::Card;
use crate::cards::Hand;

pub enum PlayerAction {
    Hit,
    Stand,
    Double,
    Split,
}

#[derive(Clone)]
pub struct Player {
    pub balance: f32,
    cutoff: u8,
}

impl Player {
    pub fn new(cutoff: u8) -> Player {
        Player {
            balance: 100.,
            cutoff,
        }
    }

    pub fn bet(&mut self) -> f32 {
        self.balance -= 10.;
        10.
    }

    pub fn action(&self, hand: &Hand, _dealer_card: &Card) -> PlayerAction {
        if hand.cards.len() == 2 {
            if hand.is_pair() {
                return PlayerAction::Split;
            }

            if hand.value() == 11 {
                return PlayerAction::Double;
            }
        }

        if hand.value() < self.cutoff {
            PlayerAction::Hit
        } else {
            PlayerAction::Stand
        }
    }
}
