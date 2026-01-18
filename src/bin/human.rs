use std::collections::HashMap;

use midas::{FlatBettingStrategy, Game, HumanActionStrategy, Player};

fn main() {
    let mut players = HashMap::new();
    players.insert(
        0,
        Player::new(
            0,
            100.0,
            Box::new(FlatBettingStrategy),
            Box::new(HumanActionStrategy),
        ),
    );
    let mut game = Game::new(players);

    loop {
        let round_results = game.round();
        for result in round_results {
            println!("{:?}", result);
        }
    }
}
