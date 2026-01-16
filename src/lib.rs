use crate::cards::{Hand, Shoe};
use crate::player::PlayerAction;

mod cards;
pub mod player;

pub use player::Player;

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
    dealer_hand: Hand,
    pub player: Player,
    shoe: Shoe,
    pot: f32,
}

impl Game {
    pub fn new(player: Player) -> Self {
        let dealer_hand = Hand::new();
        let shoe = Shoe::new();

        Self {
            dealer_hand,
            player,
            shoe,
            pot: 0.,
        }
    }

    pub fn round(&mut self) -> RoundResult {
        self.pot = self.player.bet();

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
        self.player.hand.add_card(self.shoe.deal());
        self.dealer_hand.add_card(self.shoe.deal());
        self.player.hand.add_card(self.shoe.deal());
        self.dealer_hand.add_card(self.shoe.deal());
    }

    fn finish_round(&mut self) -> RoundResult {
        while self.dealer_hand.value() < 17 {
            self.dealer_hand.add_card(self.shoe.deal());
        }

        let dealer_value = self.dealer_hand.value();
        let player_value = self.player.hand.value();

        let potential_winnings = self.pot;
        self.pot = 0.;

        self.discard_all_hands();

        if player_value == 21 && self.player.hand.len() == 2 {
            self.player.balance += potential_winnings * 2.5;
            RoundResult::Blackjack
        } else if player_value > 21 {
            RoundResult::Bust
        } else if dealer_value > 21 || dealer_value < player_value {
            self.player.balance += potential_winnings * 2.;
            RoundResult::Win
        } else if dealer_value > player_value {
            RoundResult::Lose
        } else {
            self.player.balance += self.pot;
            RoundResult::Push
        }
    }

    pub fn discard_all_hands(&mut self) {
        self.shoe.discards.append(&mut self.dealer_hand.0);
        self.shoe.discards.append(&mut self.player.hand.0);
    }
}
