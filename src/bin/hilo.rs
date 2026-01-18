use std::collections::HashMap;

use midas::{
    Game, HiLoCountingStrategy, OptimalActionStrategy, Player, RoundResult, save_results_to_csv,
};

fn main() {
    let mut all_results = vec![];
    let mut total_rounds = 0;
    for _ in 0..100_000 {
        let mut num_rounds = 0;
        let mut players = HashMap::new();
        let optimal_ai = Player::new(
            0,
            100.0,
            Box::new(HiLoCountingStrategy::new()),
            Box::new(OptimalActionStrategy),
        );
        players.insert(0, optimal_ai);
        let mut game = Game::new(players);

        let mut results = HashMap::new();
        results.insert(RoundResult::Bust, 0);
        results.insert(RoundResult::Lose, 0);
        results.insert(RoundResult::Push, 0);
        results.insert(RoundResult::Win, 0);
        results.insert(RoundResult::Blackjack, 0);

        while game.players.get(&0).unwrap().balance >= 10. {
            let round_results = game.round();
            for result in round_results {
                *results.get_mut(&result).unwrap() += 1;
                num_rounds += 1;
            }
        }

        total_rounds += num_rounds;
        all_results.push(results);
    }

    save_results_to_csv(all_results);
    println!("Average of {} rounds.", total_rounds / 100_000);
}
