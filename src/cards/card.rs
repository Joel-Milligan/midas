use enum_iterator::IntoEnumIterator;

#[derive(Clone, Copy, Debug, IntoEnumIterator, PartialEq)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Clone, Copy, Debug, IntoEnumIterator, PartialEq)]
pub enum Face {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub face: Face,
}
