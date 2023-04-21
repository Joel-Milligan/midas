/// Result of a single round of blackjack from the perspective of the player.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Result {
    Blackjack,
    Win,
    Bust,
    Lose,
    Push,
    Surrender,
}
