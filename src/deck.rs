use rand;
use rand::prelude::SliceRandom;
use std::fmt;
use crate::card::{Card, Suit};

const CARDS_IN_DECK: i32 = 52;
const HIGHEST_CARD: u8 = 14;
const LOWEST_CARD: u8 = 2;

pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::new();
        for val in LOWEST_CARD..=HIGHEST_CARD {
            for suit in Suit::iterator() {
                cards.push(Card::new(*suit, val));
            }
        }
        Deck{cards}
    }

    pub fn shuffle(&mut self) {
        let mut rng = &mut rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();
        for card in self.cards.iter() {
            res = res + &format!("{} ", &card.to_string());
        }
        write!(f, "{}", res)
    }
}
