extern crate rand;
extern crate ansi_term;

use std::io;
use std::fmt;
use rand::Rng;
use ansi_term::Colour::Red;

#[derive(Debug)]
struct Card {
    suit: String,
    value: String,
    points: i8,
}

impl Card {
    fn new(i: &i8) -> Card {
        let suit = match i / 13 {
            // 0:♥, 1:♦, 2:♣, 3:♠
            0 => Red.bold().paint("♥").to_string(),
            1 => Red.bold().paint("♦").to_string(),
            2 => "♣".to_string(),
            3 => "♠".to_string(),
            _ => "X".to_string(),
        };
        let (value, points) = match i % 13 {
            // 0 => A
            // 1-9 => 2-T
            // 10, 11, 12 => J, Q, K
            0 => ("A".to_string(), 1),
            9 => ("T".to_string(), 10),
            10 => ("J".to_string(), 10),
            11 => ("Q".to_string(), 10),
            12 => ("K".to_string(), 10),
            x => ((x + 1).to_string(), x + 1),
        };
        Card {
            suit: suit,
            value: value,
            points: points,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

fn main() {
    // Shuffle the deck
    // @@@ This is all a little messy - surely a standard way to create a 'random iterator'
    let mut cards: Vec<i8> = (0..52).collect();
    rand::thread_rng().shuffle(&mut cards);
    // Convert into 52 Card objects
    let card_objs: Vec<Card> = cards.iter().map(|x| Card::new(x)).collect();
    let deck = card_objs.as_slice();
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
