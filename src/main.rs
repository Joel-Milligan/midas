use std::collections::HashMap;

use midas::{Game, RoundResult};

fn main() {
    let num_rounds = 10_000;
    let mut game = Game::new();
    let mut results = HashMap::new();

    results.insert(RoundResult::Blackjack, 0);
    results.insert(RoundResult::Bust, 0);
    results.insert(RoundResult::Lose, 0);
    results.insert(RoundResult::Push, 0);
    results.insert(RoundResult::Win, 0);

    for _ in 0..num_rounds {
        let round = game.round();
        *results.get_mut(&round).unwrap() += 1;
    }

    let mut results = results.into_iter().collect::<Vec<_>>();
    results.sort_by_key(|x| x.1);

    for (result, times) in results {
        let percent = (times as f32 / num_rounds as f32) * 100.0;
        println!("{result:?}:    \t{percent:.0}%");
    }
}
