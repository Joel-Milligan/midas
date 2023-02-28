use super::Player;
use crate::cards::*;

pub struct Dealer {
    pub hand: Hand,
    shoe: Shoe,
    discards: Vec<Card>,
}

impl Dealer {
    pub fn new() -> Dealer {
        let hand = Hand::new();
        let shoe = Shoe::new();
        let discards = Vec::new();

        Dealer {
            shoe,
            hand,
            discards,
        }
    }

    pub fn shuffle(&mut self) {
        self.shoe.shuffle(&mut self.discards);
    }

    pub fn deal_to(&mut self, player: &mut Player) {
        loop {
            if let Some(card) = self.shoe.deal() {
                player.hand.add_card(card);
                return;
            } else {
                self.shuffle();
            }
        }
    }

    pub fn deal_to_self(&mut self) {
        loop {
            if let Some(card) = self.shoe.deal() {
                self.hand.add_card(card);
                return;
            } else {
                self.shuffle();
            }
        }
    }

    pub fn discard_all_hands(&mut self, player: &mut Player) {
        self.discards.append(&mut self.hand.discard_hand());
        self.discards.append(&mut player.hand.discard_hand());
    }
}
