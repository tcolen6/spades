mod deck;
mod card;

use crate::deck::Deck;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    println!("{}", deck);
    deck.shuffle();
    println!("{}", deck);
}
