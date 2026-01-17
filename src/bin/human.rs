use std::collections::HashMap;

use midas::{Game, Human, Player};

fn main() {
    let mut players = HashMap::new();
    players.insert(0, Human::new(0, 100.0));
    let mut game = Game::new(players);

    loop {
        let round_results = game.round();
        for result in round_results {
            println!("{:?}", result);
        }
    }
}
