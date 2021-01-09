use std::fmt;
use std::slice::Iter;
use core::cmp::Ord;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades
}

impl Suit {
    pub fn iterator() -> Iter<'static, Suit>{
        static SUITS: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        SUITS.iter()
    }
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

impl PartialEq for Suit {
    fn eq(&self, other: &Self) -> bool {
        (*self as i32) == (*other as i32)
    }
}

impl PartialOrd for Suit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Suit {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}

impl From<Suit> for i32 {
    fn from(value: Suit) -> Self {
        match value {
            Suit::Clubs => 0,
            Suit::Diamonds => 1,
            Suit::Hearts => 2,
            Suit::Spades => 3,
        }
    }
}
