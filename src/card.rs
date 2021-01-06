use std::fmt;

pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Clubs => write!(f, "C"),
            Suit::Diamonds => write!(f, "D"),
            Suit::Hearts => write!(f, "H"),
            Suit::Spades => write!(f, "S"),
        }
    }
}

pub struct Card {
    pub suit: Suit,
    pub value: u8
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            2..=10 => write!(f, "{}{}", self.value, self.suit),
            11 => write!(f, "J{}", self.suit),
            12 => write!(f, "Q{}", self.suit),
            13 => write!(f, "K{}", self.suit),
            14 => write!(f, "A{}", self.suit),
            _ => write!(f, "?{}", self.suit),
        }
    }
}
