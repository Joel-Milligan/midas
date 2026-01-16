use std::collections::HashMap;

use midas::{Game, Player, RoundResult, SimpleAi};

fn main() {
    let mut total_rounds = 0;
    for _ in 0..100_000 {
        let mut num_rounds = 0;
        let player = SimpleAi::new();
        let mut game = Game::new(player);

        let mut results = HashMap::new();
        results.insert(RoundResult::Bust, 0);
        results.insert(RoundResult::Lose, 0);
        results.insert(RoundResult::Push, 0);
        results.insert(RoundResult::Win, 0);
        results.insert(RoundResult::Blackjack, 0);

        while game.player.balance() >= 10. {
            let round_results = game.round();
            for result in round_results {
                *results.get_mut(&result).unwrap() += 1;
                num_rounds += 1;
            }
        }

        total_rounds += num_rounds;
        print_round_results(results, num_rounds);
    }

    println!("Average of {} rounds.", total_rounds / 100_000);
}

fn print_round_results(results: HashMap<RoundResult, i32>, num_rounds: i32) {
    let wins =
        *results.get(&RoundResult::Blackjack).unwrap() + *results.get(&RoundResult::Win).unwrap();
    let win_percent = (wins as f32 / num_rounds as f32) * 100.;
    let draws = *results.get(&RoundResult::Push).unwrap();
    let draw_percent = (draws as f32 / num_rounds as f32) * 100.;
    let losses =
        *results.get(&RoundResult::Lose).unwrap() + *results.get(&RoundResult::Bust).unwrap();
    let loss_percent = (losses as f32 / num_rounds as f32) * 100.;
    println!(
        "{num_rounds: <4} rounds: {win_percent: >2.0}(W) {draw_percent: >2.0}(D) {loss_percent: >3.0}(L)",
    );
}
