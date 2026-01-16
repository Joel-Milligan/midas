#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl Suit {
    pub const VARIANTS: [Self; 4] = [Self::Club, Self::Diamond, Self::Heart, Self::Spade];
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

impl Face {
    pub const VARIANTS: [Self; 13] = [
        Self::Ace,
        Self::Two,
        Self::Three,
        Self::Four,
        Self::Five,
        Self::Six,
        Self::Seven,
        Self::Eight,
        Self::Nine,
        Self::Ten,
        Self::Jack,
        Self::Queen,
        Self::King,
    ];
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub face: Face,
}
