use crate::cards::Hand;

pub struct Dealer {
    pub hand: Hand,
}

impl Dealer {
    pub fn new() -> Self {
        let hand = Hand::new();
        Self { hand }
    }
}
