use crate::cards::*;

#[derive(Debug)]
pub enum PlayerAction {
    Hit,
    Stand,
}

#[derive(Clone)]
pub struct Player {
    pub hand: Hand
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

pub struct Dealer {
    pub hand: Hand,
    shoe: Shoe,
}

impl Dealer {
    pub fn new() -> Dealer {
        let hand = Hand::new();
        let shoe = Shoe::new();
        
        Dealer { shoe, hand }
    }

    pub fn shuffle(&mut self) {
        self.shoe.shuffle();
    }

    pub fn deal_to(&mut self, player: &mut Player) {
        if let Some(card) = self.shoe.deal() {
            player.hand.add_card(card);
        }
    }

    pub fn deal_to_self(&mut self) {
        if let Some(card) = self.shoe.deal() {
            self.hand.add_card(card);
        }
    }
}
