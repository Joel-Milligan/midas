use crate::cards::Hand;

pub enum PlayerAction {
    Hit,
    Stand,
}

#[derive(Clone)]
pub struct Player {
    pub hand: Hand,
}

impl Player {
    pub fn new() -> Player {
        Player { hand: Hand::new() }
    }

    pub fn action(&self) -> PlayerAction {
        if rand::random() {
            PlayerAction::Hit
        } else {
            PlayerAction::Stand
        }
    }
}
