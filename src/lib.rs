mod cards;
pub mod game;
pub mod player;

pub use game::Game;
pub use player::Player;

/// Result of a single round of blackjack
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RoundResult {
    Blackjack,
    Win,
    Bust,
    Lose,
    Push,
}
