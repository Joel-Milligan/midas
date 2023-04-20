use rand::seq::SliceRandom;

use crate::cards::*;

#[derive(Clone)]
pub enum PlayerAction {
    Hit,
    Stand,
    Double,
    Surrender,
}

pub type PlayerRef = Box<Player>;

#[derive(Default, Clone)]
pub struct Player {
    pub wallet: i32,
}

impl Player {
    pub fn new(starting_funds: i32) -> Player {
        Player {
            wallet: starting_funds,
        }
    }

    pub fn action(&self, _hand: &Hand) -> PlayerAction {
        let actions = [
            PlayerAction::Hit,
            PlayerAction::Stand,
            PlayerAction::Double,
            PlayerAction::Surrender,
        ];

        let mut rng = rand::thread_rng();
        actions.choose(&mut rng).unwrap().clone()
    }

    pub fn payout(&mut self, payout: i32) {
        self.wallet += payout;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let player = Player::default();
        assert_eq!(player.wallet, 0);
    }

    #[test]
    fn new() {
        let player = Player::new(100);
        assert_eq!(player.wallet, 100);
    }

    #[test]
    fn payout() {
        let mut player = Player::default();
        player.payout(100);
        assert_eq!(player.wallet, 100);
    }
}
