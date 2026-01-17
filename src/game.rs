use std::collections::HashMap;

use crate::RoundResult;
use crate::cards::{Hand, Shoe};
use crate::player::{Action, Player};

pub struct ActiveHand {
    pub player: u8,
    pub hand: Hand,
    pub pot: f32,
}

pub struct Game {
    shoe: Shoe,
    pub players: HashMap<u8, Box<dyn Player>>,
    dealer_hand: Hand,
    hands: Vec<ActiveHand>,
}

impl Game {
    pub fn new(players: HashMap<u8, Box<dyn Player>>) -> Self {
        let dealer_hand = Hand::new();
        let shoe = Shoe::new();

        Self {
            dealer_hand,
            players,
            hands: vec![],
            shoe,
        }
    }

    pub fn round(&mut self) -> Vec<RoundResult> {
        self.initial_deal();

        let shown = self.dealer_hand.cards.first().unwrap();

        // TODO: Gross hack to add split hands to the end of hands while still iterating over it
        let mut i = 0;
        while i < self.hands.len() {
            while self.hands[i].hand.value() < 21 {
                let player = self
                    .players
                    .get_mut(&self.hands[i].player)
                    .expect("All hands should have a valid player");

                let action = player.action(&self.hands[i].hand, shown);

                match action {
                    Action::Hit => self.hands[i].hand.add_card(self.shoe.deal()),
                    Action::Stand => break,
                    Action::Double => {
                        assert_eq!(self.hands[i].hand.cards.len(), 2);
                        player.deduct(self.hands[i].pot);
                        self.hands[i].pot *= 2.;
                        self.hands[i].hand.add_card(self.shoe.deal());
                        break;
                    }
                    Action::Split => {
                        assert!(self.hands[i].hand.is_pair());
                        let second_card = self.hands[i]
                            .hand
                            .cards
                            .pop()
                            .expect("Hand guaranteed to be 2 cards");
                        let mut new_hand = Hand::new();
                        new_hand.cards.push(second_card);
                        self.hands[i].hand.add_card(self.shoe.deal());
                        new_hand.add_card(self.shoe.deal());
                        self.hands.push(ActiveHand {
                            player: self.hands[i].player,
                            hand: new_hand,
                            pot: player.bet(),
                        });
                    }
                }
            }
            i += 1;
        }

        self.finish_round()
    }

    fn initial_deal(&mut self) {
        self.shoe.shuffle();

        self.dealer_hand.add_card(self.shoe.deal());
        self.dealer_hand.add_card(self.shoe.deal());

        for (id, player) in &mut self.players {
            let bet = player.bet();
            let mut player_hand = Hand::new();
            player_hand.add_card(self.shoe.deal());
            player_hand.add_card(self.shoe.deal());
            self.hands.push(ActiveHand {
                player: *id,
                hand: player_hand,
                pot: bet,
            });
        }
    }

    fn finish_round(&mut self) -> Vec<RoundResult> {
        // Dealer hits until at least 17
        while self.dealer_hand.value() < 17 {
            self.dealer_hand.add_card(self.shoe.deal());
        }
        let dealer_value = self.dealer_hand.value();

        // Calculate round results for each hand
        let mut results = vec![];
        for hand in &self.hands {
            let player_value = hand.hand.value();
            let result = if player_value == 21 && hand.hand.cards.len() == 2 {
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
            let winnings = match result {
                RoundResult::Blackjack => hand.pot * 2.5,
                RoundResult::Win => hand.pot * 2.,
                RoundResult::Push => hand.pot,
                _ => 0.,
            };
            self.players.get_mut(&hand.player).unwrap().win(winnings);

            results.push(result);
        }

        // Clean up game state
        self.shoe.discards.append(&mut self.dealer_hand.cards);
        while let Some(mut hand) = self.hands.pop() {
            self.shoe.discards.append(&mut hand.hand.cards);
        }
        self.hands.clear();

        results
    }
}
