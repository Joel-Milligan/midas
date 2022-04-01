use midas::*;

fn main() {
    let mut game = Game::new();
    let round = game.round();
    println!("{round:?}");
}
