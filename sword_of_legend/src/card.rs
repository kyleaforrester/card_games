use std::fmt;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Suit {
    Sharpness,
    Balance,
    Durability,
    Honor,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Card {
    pub suit: Suit,
    pub value: u32,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suit_char = match self.suit {
            Suit::Sharpness => 'S',
            Suit::Balance => 'C',
            Suit::Durability => 'D',
            Suit::Honor => 'H',
        };
        write!(f, "{}{}", self.value, suit_char)
    }
}

