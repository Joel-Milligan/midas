use std::collections::HashMap;

use midas::{Game, Player, RoundResult};

fn main() {
    for cutoff in 0..=21 {
        let mut total_lasted = 0;
        for _ in 0..1000 {
            let mut num_rounds = 0;
            let player = Player::new(cutoff);
            let mut game = Game::new(player);

            let mut results = HashMap::new();
            results.insert(RoundResult::Bust, 0);
            results.insert(RoundResult::Lose, 0);
            results.insert(RoundResult::Push, 0);
            results.insert(RoundResult::Win, 0);
            results.insert(RoundResult::Blackjack, 0);

            while game.player.balance >= 10. {
                let round = game.round();
                *results.get_mut(&round).unwrap() += 1;
                num_rounds += 1;
            }

            total_lasted += num_rounds;

            let wins = *results.get(&RoundResult::Blackjack).unwrap()
                + *results.get(&RoundResult::Win).unwrap();
            let win_percent = (wins as f32 / num_rounds as f32) * 100.;
            let draws = *results.get(&RoundResult::Push).unwrap();
            let draw_percent = (draws as f32 / num_rounds as f32) * 100.;
            let losses = *results.get(&RoundResult::Lose).unwrap()
                + *results.get(&RoundResult::Bust).unwrap();
            let loss_percent = (losses as f32 / num_rounds as f32) * 100.;
            println!(
                "\t{num_rounds} rounds:\t{win_percent:.0}(W)\t{draw_percent:.0}(D)\t{loss_percent:.0}(L)",
            );
        }
        println!(
            "Cutoff of {:0>2} lasted an average of {} rounds.",
            cutoff,
            total_lasted / 1000
        );
    }
}
