mod ai;
mod cards;
mod game;
mod player;

use std::collections::HashMap;
use std::fmt::Write;
use std::fs;
use std::str::FromStr;

pub use ai::action::{HumanActionStrategy, OptimalActionStrategy, SimpleActionStrategy};
pub use ai::betting::{FlatBettingStrategy, HiLoCountingStrategy};
pub use game::Game;
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

pub fn print_round_results(results: &HashMap<RoundResult, i32>, num_rounds: i32) {
    let wins =
        *results.get(&RoundResult::Blackjack).unwrap() + *results.get(&RoundResult::Win).unwrap();
    let win_percent = (wins as f32 / num_rounds as f32) * 100.;
    let draws = *results.get(&RoundResult::Push).unwrap();
    let draw_percent = (draws as f32 / num_rounds as f32) * 100.;
    let losses =
        *results.get(&RoundResult::Lose).unwrap() + *results.get(&RoundResult::Bust).unwrap();
    let loss_percent = (losses as f32 / num_rounds as f32) * 100.;
    println!(
        "{num_rounds: <4} rounds: {win_percent: >2.0}% (W) {draw_percent: >2.0}% (D) {loss_percent: >3.0}% (L)",
    );
}

pub fn save_results_to_csv(results: Vec<HashMap<RoundResult, i32>>) {
    let mut contents = String::from_str("Wins,Draws,Losses,Total\n").unwrap();
    for result in results {
        let wins =
            *result.get(&RoundResult::Blackjack).unwrap() + *result.get(&RoundResult::Win).unwrap();
        let draws = *result.get(&RoundResult::Push).unwrap();
        let losses =
            *result.get(&RoundResult::Lose).unwrap() + *result.get(&RoundResult::Bust).unwrap();
        writeln!(
            contents,
            "{},{},{},{}",
            wins,
            draws,
            losses,
            wins + draws + losses
        )
        .expect("infallible");
    }
    fs::write("results.csv", contents).unwrap()
}
