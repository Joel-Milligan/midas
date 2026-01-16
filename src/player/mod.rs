mod actions;
mod human;
mod simple_ai;

pub use actions::Action;
pub use human::Human;
pub use simple_ai::SimpleAi;

use crate::cards::{Card, Hand};

pub trait Player {
    fn new() -> impl Player;
    fn balance(&self) -> f32;
    fn bet(&mut self) -> f32;
    fn deduct(&mut self, amount: f32);
    fn win(&mut self, amount: f32);
    fn action(&self, hand: &Hand, dealer_card: &Card) -> Action;
}
