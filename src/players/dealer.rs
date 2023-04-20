use crate::cards::*;

pub struct Dealer {
    pub hand: Vec<Card>,
    shoe: Shoe,
    discards: Vec<Card>,
}

impl Dealer {
    pub fn new() -> Self {
        let hand = vec![];
        let shoe = Shoe::new();
        let discards = Vec::new();

        Self {
            shoe,
            hand,
            discards,
        }
    }

    pub fn shuffle(&mut self) {
        self.shoe.shuffle(&mut self.discards);
    }

    pub fn deal_to(&mut self, hand: &mut Hand) {
        loop {
            if let Some(card) = self.shoe.deal() {
                hand.add_card(card);
                return;
            } else {
                self.shuffle();
            }
        }
    }

    pub fn deal_to_self(&mut self) {
        loop {
            if let Some(card) = self.shoe.deal() {
                self.hand.push(card);
                return;
            } else {
                self.shuffle();
            }
        }
    }

    pub fn discard_hand(&mut self, hand: &mut Hand) {
        self.discards.append(&mut hand.discard_hand());
    }

    pub fn discard(&mut self) {
        self.discards.append(&mut self.hand);
    }

    pub fn value(&self) -> u8 {
        let mut value = 0;
        let mut high_aces = 0;

        for card in &self.hand {
            match card.face {
                Face::Ace => {
                    value += 11;
                    high_aces += 1;
                }
                Face::Two => value += 2,
                Face::Three => value += 3,
                Face::Four => value += 4,
                Face::Five => value += 5,
                Face::Six => value += 6,
                Face::Seven => value += 7,
                Face::Eight => value += 8,
                Face::Nine => value += 9,
                Face::Ten | Face::Jack | Face::Queen | Face::King => value += 10,
            }
        }

        while value > 21 && high_aces > 0 {
            value -= 10;
            high_aces -= 1;
        }

        value
    }
}
