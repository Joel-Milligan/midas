use midas::{FlatBettingStrategy, Game, HumanActionStrategy, Player};

fn main() {
    let mut players = Vec::new();
    players.push(Player::new(
        0,
        100.0,
        Box::new(HumanActionStrategy),
        Box::new(FlatBettingStrategy),
    ));
    let mut game = Game::new(players);

    loop {
        let round_results = game.round();
        for result in round_results {
            println!("{:?}", result);
        }
    }
}
