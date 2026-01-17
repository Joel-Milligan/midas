use midas::{Game, Human, Player};

fn main() {
    let players = vec![Human::new(100.0)];
    let mut game = Game::new(players);

    loop {
        let round_results = game.round();
        for result in round_results {
            println!("{:?}", result);
        }
    }
}
