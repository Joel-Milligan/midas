use midas::{Game, Human, Player};

fn main() {
    let player = Human::new(100.0);
    let mut game = Game::new(player);

    loop {
        let round_results = game.round();
        for result in round_results {
            println!("{:?}", result);
        }
    }
}
