use crate::cards::Shoe;
use crate::players::{Dealer, Player, PlayerAction};

mod cards;
mod players;

/// Result of a single round of blackjack
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RoundResult {
    Blackjack,
    Win,
    Bust,
    Lose,
    Push,
}

pub struct Game {
    dealer: Dealer,
    player: Player,
    shoe: Shoe,
}

impl Game {
    pub fn new() -> Self {
        let dealer = Dealer::new();
        let player = Player::new();
        let shoe = Shoe::new();

        Self {
            dealer,
            player,
            shoe,
        }
    }

    pub fn round(&mut self) -> RoundResult {
        self.initial_deal();

        while self.player.hand.value() < 21 {
            match self.player.action() {
                PlayerAction::Hit => self.player.hand.add_card(self.shoe.deal()),
                PlayerAction::Stand => break,
            }
        }

        self.finish_round()
    }

    fn initial_deal(&mut self) {
        self.shoe.shuffle();
        self.dealer.hand.add_card(self.shoe.deal());
        self.player.hand.add_card(self.shoe.deal());
        self.dealer.hand.add_card(self.shoe.deal());
        self.player.hand.add_card(self.shoe.deal());
    }

    fn finish_round(&mut self) -> RoundResult {
        while self.dealer.hand.value() < 17 {
            let card = self.shoe.deal();
            self.dealer.hand.add_card(card);
        }

        let minimum_hand = self.player.hand.len() == 2;
        let dealer_value = self.dealer.hand.value();
        let player_value = self.player.hand.value();

        self.discard_all_hands();

        if player_value == 21 && minimum_hand {
            RoundResult::Blackjack
        } else if player_value > 21 {
            RoundResult::Bust
        } else if dealer_value > 21 || dealer_value < player_value {
            RoundResult::Win
        } else if dealer_value > player_value {
            RoundResult::Lose
        } else {
            RoundResult::Push
        }
    }

    pub fn discard_all_hands(&mut self) {
        self.shoe.discards.append(&mut self.dealer.hand.0);
        self.shoe.discards.append(&mut self.player.hand.0);
    }
}
