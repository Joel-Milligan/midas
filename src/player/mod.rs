mod actions;
mod human;
mod optimal_ai;
mod simple_ai;

pub use actions::Action;
pub use human::Human;
pub use optimal_ai::OptimalAi;
pub use simple_ai::SimpleAi;

use crate::cards::{Card, Hand};

pub trait Player {
    // TODO: Maybe use a builder pattern on Game to prevent needing Box<dyn Player>
    fn new(id: u8, starting_balance: f32) -> Box<dyn Player>
    where
        Self: Sized;
    fn id(&self) -> u8;
    fn balance(&self) -> f32;
    fn bet(&mut self) -> f32;
    fn deduct(&mut self, amount: f32);
    fn win(&mut self, amount: f32);
    fn action(&self, hand: &Hand, dealer_card: &Card) -> Action;
}
