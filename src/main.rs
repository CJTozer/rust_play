extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Let's play!");

    // Shuffle the deck
    // @@@ This is all a little messy - surely a standard way to create a 'random iterator'
    let mut cards: Vec<u32> = (1..53).collect();
    let mut deck = cards.as_mut_slice();
    rand::thread_rng().shuffle(&mut deck);
    println!("{:?}", deck);
    let mut draw_pile = deck.iter();

    let mut hand = [draw_pile.next(), draw_pile.next()];
    // @@@ Dealer cards...
    // @@@ Show one dealer card?

    loop {
        // @@@ Pretty-print hand
        println!("Your hand: {:?}", hand);

        // @@@ Stick/Hit?
        let mut action = String::new();
        io::stdin().read_line(&mut action)
            .expect("failed to read line");

        // @@@ Match action => Stick/Hit
    }

    // @@@ Dealer plays.

    // @@@ Calculate winner.
}
