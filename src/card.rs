use std::fmt;
use core::cmp::Ord;
use std::cmp::Ordering;
use crate::suit::Suit;

#[derive(Copy, Clone, Eq)]
pub struct Card {
    pub suit: Suit,
    pub value: u8
}

impl Card {
    pub fn new(suit: Suit, value: u8) -> Card {
        Card {suit, value}
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.suit == other.suit && self.value == other.value
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.suit == other.suit {
            self.value.cmp(&other.value)
        }  else {
            self.suit.cmp(&other.suit)
        }
    }
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
