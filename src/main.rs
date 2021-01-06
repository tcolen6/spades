mod card;

use crate::card::{Card, Suit};

fn main() {
    let card = Card {
        suit: Suit::Clubs,
        value: 12,
    };
    println!("Your card is: {}", card);
}
