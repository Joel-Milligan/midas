use crate::RoundResult;
use crate::cards::card::Face;
use crate::cards::{Card, Hand, Shoe};
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
    pub players: Vec<Player>,
    dealer_hand: Hand,
    hands: Vec<ActiveHand>,
}

impl Game {
    pub fn new(players: Vec<Player>) -> Self {
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
                    .iter_mut()
                    .find(|x| x.id == active_hand.player)
                    .expect("All hands should have a valid player");

                let action = player.action(&active_hand.hand, shown);
                match action {
                    Action::Hit => active_hand
                        .hand
                        .add_card(deal(&mut self.shoe, &mut self.players)),
                    Action::Stand => {
                        active_hand.completed = true;
                    }
                    Action::Double => {
                        assert_eq!(active_hand.hand.cards.len(), 2);
                        player.balance -= active_hand.pot;
                        active_hand.pot *= 2.;
                        active_hand
                            .hand
                            .add_card(deal(&mut self.shoe, &mut self.players));
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
                        active_hand
                            .hand
                            .add_card(deal(&mut self.shoe, &mut self.players));
                        new_hand.add_card(deal(&mut self.shoe, &mut self.players));
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
        for i in 0..self.players.len() {
            // Don't deal in players that can't make minimum bet
            if self.players[i].balance < 10.0 {
                continue;
            }

            let bet = self.players[i].bet();
            let mut hand = Hand::new();
            hand.add_card(deal(&mut self.shoe, &mut self.players));
            hand.add_card(deal(&mut self.shoe, &mut self.players));
            let player = self.players[i].id;
            let blackjack = hand.value() == 21;
            self.hands.push(ActiveHand {
                player,
                hand,
                pot: bet,
                blackjack,
                completed: false,
            });
        }

        self.dealer_hand
            .add_card(deal(&mut self.shoe, &mut self.players));
        self.dealer_hand
            .add_card(secret_deal(&mut self.shoe, &mut self.players));
    }

    fn finish_round(&mut self) -> Vec<RoundResult> {
        // Dealer hits until at least 17
        while self.dealer_hand.value() < 17 {
            self.dealer_hand
                .add_card(deal(&mut self.shoe, &mut self.players));
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

            self.players
                .iter_mut()
                .find(|p| p.id == hand.player)
                .unwrap()
                .balance += winnings;

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

fn deal(shoe: &mut Shoe, players: &mut Vec<Player>) -> Card {
    let (card, shuffled) = shoe.deal();

    players.iter_mut().for_each(|p| p.card_dealt(&card));
    if shuffled {
        players.iter_mut().for_each(|p| p.shuffled());
    }

    card
}

fn secret_deal(shoe: &mut Shoe, players: &mut Vec<Player>) -> Card {
    let (card, shuffled) = shoe.deal();

    if shuffled {
        players.iter_mut().for_each(|p| p.shuffled());
    }

    card
}
