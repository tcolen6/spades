mod deck;
mod card;
mod suit;
mod hand;

use crate::deck::Deck;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let hands = deck.deal(4, 13).unwrap();
    for (_, hand) in hands.iter() {
        println!("Player:");
        println!("{}", hand);
    }
}
