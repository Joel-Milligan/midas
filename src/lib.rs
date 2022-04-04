mod cards;
mod players;
use players::*;

/// Result of a single round of blackjack from the perspective of the player.
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
}

impl Game {
    pub fn new() -> Game {
        let dealer = Dealer::new();
        let player = Player::new();

        Game { dealer, player }
    }

    pub fn round(&mut self) -> RoundResult {
        self.initial_deal();

        while self.player.hand.value() < 21 {
            match self.player.action() {
                PlayerAction::Hit => self.dealer.deal_to(&mut self.player),
                PlayerAction::Stand => break,
            }
        }

        self.finish_round()
    }

    fn initial_deal(&mut self) {
        self.dealer.shuffle();
        self.dealer.deal_to(&mut self.player);
        self.dealer.deal_to_self();
        self.dealer.deal_to(&mut self.player);
        self.dealer.deal_to_self();
    }

    fn finish_round(&mut self) -> RoundResult {
        while self.dealer.hand.value() < 17 {
            self.dealer.deal_to_self();
        }

        let dealer_value = self.dealer.hand.value();
        let player_value = self.player.hand.value();

        self.dealer.discard(&mut self.player);

        if player_value == 21 {
            RoundResult::Blackjack
        } else if player_value > 21 {
            RoundResult::Bust
        } else if dealer_value > 21 {
            RoundResult::Win
        } else if dealer_value < player_value {
            RoundResult::Win
        } else if dealer_value > player_value {
            RoundResult::Lose
        } else {
            RoundResult::Push
        }
    }
}
