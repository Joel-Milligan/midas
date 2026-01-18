use std::io;

use crate::cards::card::Face;
use crate::cards::{Card, Hand};
use crate::player::Action;

pub trait ActionStrategy {
    fn action(&self, hand: &Hand, dealer_card: &Card) -> Action;
}

pub struct HumanActionStrategy;

impl ActionStrategy for HumanActionStrategy {
    fn action(&self, hand: &Hand, dealer_card: &Card) -> Action {
        println!("Hand: {} ({})", hand, hand.value());
        println!("Dealer: {:?}", dealer_card);

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            match input.trim() {
                "hit" => return Action::Hit,
                "stand" => return Action::Stand,
                "split" => return Action::Split,
                "double" => return Action::Double,
                _ => {}
            }
        }
    }
}

pub struct SimpleActionStrategy;

impl ActionStrategy for SimpleActionStrategy {
    fn action(&self, hand: &Hand, _dealer_card: &Card) -> Action {
        if hand.cards.len() == 2 {
            if hand.is_pair() {
                return Action::Split;
            }

            if hand.value() == 11 {
                return Action::Double;
            }
        }

        if hand.value() < 15 {
            Action::Hit
        } else {
            Action::Stand
        }
    }
}

pub struct OptimalActionStrategy;

impl ActionStrategy for OptimalActionStrategy {
    fn action(&self, hand: &Hand, dealer_card: &Card) -> Action {
        let initial_cards = hand.cards.len() == 2;

        if hand.is_pair() && should_split(hand.cards[0].face, dealer_card.face) {
            return Action::Split;
        }

        if hand.is_soft() {
            get_soft_total_action(hand.value(), initial_cards, dealer_card.face)
        } else {
            get_hard_total_action(hand.value(), initial_cards, dealer_card.face)
        }
    }
}

fn should_split(pair_face: Face, dealer_face: Face) -> bool {
    match pair_face {
        Face::Ace | Face::Eight => true,
        Face::Ten | Face::King | Face::Queen | Face::Jack | Face::Five => false,
        Face::Nine => matches!(
            dealer_face,
            Face::Nine
                | Face::Eight
                | Face::Six
                | Face::Five
                | Face::Four
                | Face::Three
                | Face::Two
        ),
        Face::Seven => matches!(
            dealer_face,
            Face::Seven | Face::Six | Face::Five | Face::Four | Face::Three | Face::Two
        ),
        Face::Six | Face::Three | Face::Two => matches!(
            dealer_face,
            Face::Six | Face::Five | Face::Four | Face::Three | Face::Two
        ),
        Face::Four => matches!(dealer_face, Face::Five | Face::Four),
    }
}

fn get_soft_total_action(hand_value: u8, initial_cards: bool, dealer_face: Face) -> Action {
    match hand_value {
        20 => Action::Stand,
        19 => match dealer_face {
            Face::Six => {
                if initial_cards {
                    Action::Double
                } else {
                    Action::Stand
                }
            }
            _ => Action::Stand,
        },
        18 => match dealer_face {
            Face::Ace | Face::King | Face::Queen | Face::Jack | Face::Ten | Face::Nine => {
                Action::Hit
            }
            Face::Eight | Face::Seven => Action::Stand,
            _ => {
                if initial_cards {
                    Action::Double
                } else {
                    Action::Stand
                }
            }
        },
        17 => match dealer_face {
            Face::Six | Face::Five | Face::Four | Face::Three => {
                if initial_cards {
                    Action::Double
                } else {
                    Action::Hit
                }
            }
            _ => Action::Hit,
        },
        16 | 15 => match dealer_face {
            Face::Six | Face::Five | Face::Four => {
                if initial_cards {
                    Action::Double
                } else {
                    Action::Hit
                }
            }
            _ => Action::Hit,
        },
        14 | 13 => match dealer_face {
            Face::Six | Face::Five => {
                if initial_cards {
                    Action::Double
                } else {
                    Action::Hit
                }
            }
            _ => Action::Hit,
        },
        score => unreachable!("Not a possible soft hand score: {score}"),
    }
}

fn get_hard_total_action(hand_value: u8, initial_cards: bool, dealer_face: Face) -> Action {
    match hand_value {
        17..=20 => Action::Stand,
        13..=16 => match dealer_face {
            Face::Six | Face::Five | Face::Four | Face::Three | Face::Two => Action::Stand,
            _ => Action::Hit,
        },
        12 => match dealer_face {
            Face::Six | Face::Five | Face::Four => Action::Stand,
            _ => Action::Hit,
        },
        11 => {
            if initial_cards {
                Action::Double
            } else {
                Action::Hit
            }
        }
        10 => {
            if initial_cards {
                match dealer_face {
                    Face::Ace | Face::King | Face::Queen | Face::Jack | Face::Ten => Action::Hit,
                    _ => Action::Double,
                }
            } else {
                Action::Hit
            }
        }
        9 => {
            if initial_cards {
                match dealer_face {
                    Face::Six | Face::Five | Face::Four | Face::Three => Action::Double,
                    _ => Action::Hit,
                }
            } else {
                Action::Hit
            }
        }
        _ => Action::Hit,
    }
}
