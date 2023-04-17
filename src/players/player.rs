use rand::seq::SliceRandom;

use crate::cards::*;

#[derive(Clone)]
pub enum PlayerAction {
    Hit,
    Stand,
    Double,
    Surrender,
}

#[derive(Clone)]
pub struct Player {
    pub hand: Hand,
    pub wallet: i32,
    pub current_bet: i32,
}

impl Player {
    pub fn new(starting_funds: i32) -> Player {
        Player {
            hand: Hand::new(),
            wallet: starting_funds,
            current_bet: 0,
        }
    }

    pub fn action(&self) -> PlayerAction {
        let actions = [
            PlayerAction::Hit,
            PlayerAction::Stand,
            PlayerAction::Double,
            PlayerAction::Surrender,
        ];

        let mut rng = rand::thread_rng();
        actions.choose(&mut rng).unwrap().clone()
    }

    pub fn bet(&mut self) {
        println!("WALLET: {}", self.wallet);

        let bet = 10;
        self.current_bet = bet;
        self.wallet -= self.current_bet;
    }

    pub fn double_bet(&mut self) {
        self.current_bet = self.current_bet * 2;
        self.wallet -= self.current_bet;
    }

    pub fn payout(&mut self, winnings: i32) {
        println!("PAYOUT: {winnings}");

        self.wallet += winnings;
        println!("WALLET: {}", self.wallet);
        self.current_bet = 0;
    }
}
