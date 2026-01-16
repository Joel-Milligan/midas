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

        let shown = self.dealer_hand.0.first().unwrap();

        while self.player.hand.value() < 21 {
            match self.player.action(shown) {
                PlayerAction::Hit => self.player.hand.add_card(self.shoe.deal()),
                PlayerAction::Stand => break,
                PlayerAction::Double => {
                    assert_eq!(self.player.hand.len(), 2);

                    // Double bet
                    self.player.balance -= self.pot;
                    self.pot *= 2.;

                    // Deal final card
                    self.player.hand.add_card(self.shoe.deal());
                    break;
                }
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
        // Dealer hits until at least 17
        while self.dealer_hand.value() < 17 {
            self.dealer_hand.add_card(self.shoe.deal());
        }

        // Calculate round result
        let dealer_value = self.dealer_hand.value();
        let player_value = self.player.hand.value();
        let result = if player_value == 21 && self.player.hand.len() == 2 {
            RoundResult::Blackjack
        } else if player_value > 21 {
            RoundResult::Bust
        } else if dealer_value > 21 || dealer_value < player_value {
            RoundResult::Win
        } else if dealer_value > player_value {
            RoundResult::Lose
        } else {
            RoundResult::Push
        };

        // Update player's cash stack with any winnings
        self.player.balance += match result {
            RoundResult::Blackjack => self.pot * 2.5,
            RoundResult::Win => self.pot * 2.,
            RoundResult::Push => self.pot,
            _ => 0.,
        };

        // Clean up game state
        self.pot = 0.;
        self.shoe.discards.append(&mut self.dealer_hand.0);
        self.shoe.discards.append(&mut self.player.hand.0);

        result
    }
}
