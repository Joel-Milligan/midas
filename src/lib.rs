mod cards;
pub mod game;
mod player;

pub use game::Game;
pub use player::Player;
pub use player::SimpleAi;

/// Result of a single round of blackjack
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RoundResult {
    Blackjack,
    Win,
    Bust,
    Lose,
    Push,
}
