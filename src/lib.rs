mod cards;
mod players;
use cards::*;
use players::*;

/// Result of a single round of blackjack from the perspective of the player.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RoundResult {
    Blackjack,
    Win,
    Bust,
    Lose,
    Push,
    Surrender,
}

pub struct Game {
    dealer: Dealer,
    player: PlayerRef,
    hands: Vec<Hand>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            dealer: Dealer::new(),
            player: Box::new(Player::new(100)),
            hands: vec![],
        }
    }

    pub fn round(&mut self) {
        // Initial Deal
        let mut hand = Hand::new(self.player.clone());

        self.dealer.shuffle();
        self.dealer.deal_to(&mut hand);
        self.dealer.deal_to_self();
        self.dealer.deal_to(&mut hand);
        self.dealer.deal_to_self();

        self.hands.push(hand);

        // Play out hands
        for mut hand in &mut self.hands {
            while hand.value() < 21 {
                match hand.player.action(&hand) {
                    PlayerAction::Hit => self.dealer.deal_to(&mut hand),
                    PlayerAction::Double => {}
                    PlayerAction::Stand => break,
                    PlayerAction::Surrender => {}
                }
            }
        }

        // Finish round
        while self.dealer.value() < 17 {
            self.dealer.deal_to_self();
        }

        let dealer_value = self.dealer.value();
        self.dealer.discard();

        for mut hand in &mut self.hands {
            let hand_value = hand.value();
            self.dealer.discard_hand(&mut hand);

            if hand_value == 21 && hand.len() == 2 {
                self.player.payout(hand.bet * 3);
            } else if hand_value > 21 {
            } else if dealer_value > 21 || dealer_value < hand_value {
                self.player.payout(hand.bet * 2);
            } else if dealer_value > hand_value {
            } else {
                self.player.payout(hand.bet);
            }

            hand.bet = 0;
        }
    }
}
