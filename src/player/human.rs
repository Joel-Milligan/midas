use std::io;

use crate::Player;
use crate::cards::{Card, Hand};
use crate::player::Action;

pub struct Human {
    pub balance: f32,
}

impl Player for Human {
    fn new(balance: f32) -> impl Player {
        Self { balance }
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

    fn action(&self, hand: &Hand, dealer_card: &Card) -> Action {
        println!("Hand: {} ({})", hand, hand.value());
        println!("Dealer: {:?}", dealer_card);

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            match input.trim() {
                "hit" => return Action::Hit,
                "stand" => return Action::Stand,
                "split" => return Action::Split,
                "double" => return Action::Double,
                _ => {}
            }
        }
    }
}
