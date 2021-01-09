use rand;
use rand::prelude::SliceRandom;
use std::fmt;
use std::collections::HashMap;
use crate::card::Card;
use crate::suit::Suit;
use crate::hand::Hand;


const CARDS_IN_DECK: i32 = 52;
const HIGHEST_CARD: u8 = 14;
const LOWEST_CARD: u8 = 2;

pub struct Deck {
    cards: Vec<Card>
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

    pub fn draw(&mut self) -> Result<Card, String> {
        match self.cards.pop() {
            None => Err(String::from("Not enough cards in deck")),
            Some(card) => Ok(card)
        }
    }

    pub fn draw_n(&mut self, num_cards:u8) -> Result<Vec<Card>, String> {
        if num_cards > self.cards.len() as u8 {
            return Err(String::from("Not enough cards in deck"));
        }
        Ok(self.cards.split_off(self.cards.len() - num_cards as usize))
    }

    pub fn deal(&mut self, players: u8, num_cards: u8) -> Result<HashMap<u8, Hand>, String>{
        if players * num_cards > self.cards.len() as u8 {
            return Err(String::from("Not enough cards in deck"));
        }
        let mut ret: HashMap<u8, Hand> = HashMap::new();
        for i in 0..players {
            ret.insert(i, Hand::new(self.draw_n(num_cards).unwrap()));
        }
        println!("{}", self);
        Ok(ret)
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
