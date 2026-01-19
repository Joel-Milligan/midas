use std::collections::HashMap;

use crate::RoundResult;
use crate::cards::card::Face;
use crate::cards::{Hand, Shoe};
use crate::player::{Action, Player};

#[derive(Debug)]
pub struct ActiveHand {
    pub player: u8,
    pub hand: Hand,
    pub pot: f32,
    pub blackjack: bool,
    pub completed: bool,
}

pub struct Game {
    shoe: Shoe,
    pub players: HashMap<u8, Player>,
    dealer_hand: Hand,
    hands: Vec<ActiveHand>,
}

impl Game {
    pub fn new(players: HashMap<u8, Player>) -> Self {
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

        let mut splits = vec![];
        while self.hands.iter().any(|h| !h.completed) {
            for active_hand in self.hands.iter_mut().filter(|h| !h.completed) {
                if active_hand.hand.value() >= 21 {
                    active_hand.completed = true;
                    continue;
                }

                let player = self
                    .players
                    .get_mut(&active_hand.player)
                    .expect("All hands should have a valid player");

                let action = player.action(&active_hand.hand, shown);
                match action {
                    Action::Hit => active_hand.hand.add_card(self.shoe.deal()),
                    Action::Stand => {
                        active_hand.completed = true;
                    }
                    Action::Double => {
                        assert_eq!(active_hand.hand.cards.len(), 2);
                        player.balance -= active_hand.pot;
                        active_hand.pot *= 2.;
                        active_hand.hand.add_card(self.shoe.deal());
                        active_hand.completed = true;
                    }
                    Action::Split => {
                        assert!(active_hand.hand.is_pair());
                        player.balance -= active_hand.pot;
                        let second_card = active_hand
                            .hand
                            .cards
                            .pop()
                            .expect("Hand guaranteed to be 2 cards");
                        let mut new_hand = Hand::new();
                        new_hand.cards.push(second_card);
                        active_hand.hand.add_card(self.shoe.deal());
                        new_hand.add_card(self.shoe.deal());
                        splits.push(ActiveHand {
                            player: active_hand.player,
                            hand: new_hand,
                            pot: active_hand.pot,
                            blackjack: false, // Splits can never be a real blackjack
                            completed: second_card.face == Face::Ace,
                        });
                        active_hand.completed = second_card.face == Face::Ace; // Can only split aces once
                    }
                }
            }
            self.hands.append(&mut splits);
            splits.clear();
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
            let blackjack = player_hand.value() == 21;
            self.hands.push(ActiveHand {
                player: *id,
                hand: player_hand,
                pot: bet,
                blackjack,
                completed: false,
            });
        }
    }

    fn finish_round(&mut self) -> Vec<RoundResult> {
        // Dealer hits until at least 17
        while self.dealer_hand.value() < 17 {
            self.dealer_hand.add_card(self.shoe.deal());
        }

        let dealer_value = self.dealer_hand.value();
        let dealer_bust = dealer_value > 21;
        let dealer_blackjack = dealer_value == 21 && self.dealer_hand.cards.len() == 2;

        // Calculate round results for each hand
        let mut results = vec![];
        for hand in &self.hands {
            let player_value = hand.hand.value();
            let player_bust = player_value > 21;

            let result = if hand.blackjack && !dealer_blackjack {
                RoundResult::Blackjack
            } else if player_bust
                || (dealer_blackjack && !hand.blackjack)
                || (dealer_value > player_value && !dealer_bust)
            {
                RoundResult::Lose
            } else if dealer_bust || dealer_value < player_value {
                RoundResult::Win
            } else {
                RoundResult::Push
            };

            // Update player's cash stack with any winnings
            let winnings = match result {
                RoundResult::Blackjack => hand.pot * 2.5,
                RoundResult::Win => hand.pot * 2.0,
                RoundResult::Push => hand.pot,
                _ => 0.,
            };
            let player = self.players.get_mut(&hand.player).unwrap();

            player.balance += winnings;

            results.push(result);
        }

        let mut to_discard = vec![];
        to_discard.append(&mut self.dealer_hand.cards);
        while let Some(mut hand) = self.hands.pop() {
            to_discard.append(&mut hand.hand.cards);
        }

        // Notify players of hands about to be discarded
        for player in self.players.values_mut() {
            player.notify(&to_discard);
        }

        // Clean up game state
        self.shoe.discards.append(&mut to_discard);
        self.hands.clear();

        results
    }
}
