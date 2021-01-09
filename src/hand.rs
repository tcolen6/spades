use std::fmt;
use crate::card::Card;
use crate::suit::Suit;

struct CardInHand {
    card: Card,
    position: u8,
}

impl fmt::Display for CardInHand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.card)
    }
}

pub struct Hand {
    cards: Vec<CardInHand>
}

impl Hand {
    pub fn new(mut cards: Vec<Card>) -> Self {
        let positions: Vec<u8> = (0..cards.len() as u8).collect();
        cards.sort();
        Hand {cards: cards.iter().zip(positions.iter()).map(|(x, y)| CardInHand{card: *x, position: *y}).collect() }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res: String = String::new();
        for suit in Suit::iterator() {
            res = res + &format!("{} |", suit);
            for card in self.cards.iter().filter(|x| x.card.suit == *suit) {
                res = res + &format!(" {}", card);
            }
            res = res + "\n";
        }
        write!(f, "{}", res)
    }
}
