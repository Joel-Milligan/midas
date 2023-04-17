use std::collections::HashMap;

use midas::*;

fn main() {
    let num_rounds = 10;
    let mut game = Game::new();

    let mut possible_results = HashMap::new();
    possible_results.insert(RoundResult::Blackjack, 0);
    possible_results.insert(RoundResult::Win, 0);
    possible_results.insert(RoundResult::Push, 0);
    possible_results.insert(RoundResult::Surrender, 0);
    possible_results.insert(RoundResult::Lose, 0);
    possible_results.insert(RoundResult::Bust, 0);

    for round in 0..num_rounds {
        println!("ROUND {}", round + 1);
        let round = game.round();
        *possible_results.get_mut(&round).unwrap() += 1;
    }

    for (result, times) in possible_results {
        let percent = (times as f32 / num_rounds as f32) * 100.0;
        println!("{result:?}: {percent:.0}");
    }
}
