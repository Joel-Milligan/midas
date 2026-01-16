use crate::RoundResult;
use crate::cards::{Hand, Shoe};
use crate::player::{Player, PlayerAction};

pub struct Game {
    dealer_hand: Hand,
    pub player: Player,
    hands: Vec<Hand>,
    shoe: Shoe,
    pots: Vec<f32>, // TODO: Pots are tied by index to hands
}

impl Game {
    pub fn new(player: Player) -> Self {
        let dealer_hand = Hand::new();
        let shoe = Shoe::new();

        Self {
            dealer_hand,
            player,
            hands: vec![],
            shoe,
            pots: vec![0.],
        }
    }

    pub fn round(&mut self) -> Vec<RoundResult> {
        self.pots[0] = self.player.bet();

        self.initial_deal();

        let shown = self.dealer_hand.cards.first().unwrap();

        // TODO: Gross hack to add split hands to the end of hands while still iterating over it
        let mut i = 0;
        while i < self.hands.len() {
            while self.hands[i].value() < 21 {
                match self.player.action(&self.hands[i], shown) {
                    PlayerAction::Hit => self.hands[i].add_card(self.shoe.deal()),
                    PlayerAction::Stand => break,
                    PlayerAction::Double => {
                        assert_eq!(self.hands[i].cards.len(), 2);
                        self.player.balance -= self.pots[i];
                        self.pots[i] *= 2.;
                        self.hands[i].add_card(self.shoe.deal());
                        break;
                    }
                    PlayerAction::Split => {
                        assert!(self.hands[i].is_pair());
                        self.pots.push(self.player.bet());
                        let second_card = self.hands[i]
                            .cards
                            .pop()
                            .expect("Hand guaranteed to be 2 cards");
                        let mut new_hand = Hand::new();
                        new_hand.cards.push(second_card);
                        self.hands[i].add_card(self.shoe.deal());
                        new_hand.add_card(self.shoe.deal());
                        self.hands.push(new_hand);
                    }
                }
            }
            i += 1;
        }

        self.finish_round()
    }

    fn initial_deal(&mut self) {
        self.shoe.shuffle();
        let mut player_hand = Hand::new();
        player_hand.add_card(self.shoe.deal());
        self.dealer_hand.add_card(self.shoe.deal());
        player_hand.add_card(self.shoe.deal());
        self.dealer_hand.add_card(self.shoe.deal());
        self.hands.push(player_hand);
    }

    fn finish_round(&mut self) -> Vec<RoundResult> {
        // Dealer hits until at least 17
        while self.dealer_hand.value() < 17 {
            self.dealer_hand.add_card(self.shoe.deal());
        }
        let dealer_value = self.dealer_hand.value();

        // Calculate round results for each hand
        let mut results = vec![];

        for (i, hand) in self.hands.iter().enumerate() {
            let player_value = hand.value();
            let result = if player_value == 21 && hand.cards.len() == 2 {
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
                RoundResult::Blackjack => self.pots[i] * 2.5,
                RoundResult::Win => self.pots[i] * 2.,
                RoundResult::Push => self.pots[i],
                _ => 0.,
            };

            results.push(result);
        }

        // Clean up game state
        self.pots = vec![0.];
        self.shoe.discards.append(&mut self.dealer_hand.cards);
        while let Some(mut hand) = self.hands.pop() {
            self.shoe.discards.append(&mut hand.cards);
        }
        self.hands.clear();

        results
    }
}
