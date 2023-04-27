#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
use ::phf::{phf_map};

static SUITS: phf::Map<u8, &'static str> = phf_map! {
    0u8 => "♣",
    1u8 => "♦",
    2u8 => "♥",
    3u8 => "♠"
};

static PIPS: phf::Map<u8, &'static str> = phf_map! {
    0u8 => "2",
    1u8 => "3",
    2u8 => "4",
    3u8 => "5",
    4u8 => "6",
    5u8 => "7",
    6u8 => "8",
    7u8 => "9",
    8u8 => "10",
    9u8 => "J",
    10u8 => "Q",
    11u8 => "K",
    12u8 => "A"
};

#[derive(Debug)]
struct Card {
    suit: u8,
    pip: u8,
}

impl Card {
    fn new(number: u8) -> Result<Self, String> {
        if number >= 1 && number <= 52 {
            let suit = (number - 1) / 13;
            let pip = (number - 1).rem_euclid(13);
            return Ok(Card { suit, pip });
        } else {
            return Err("`number` value must be between from 1-52".to_string());
        }
    }

    fn get_card_name(&self) -> String {
        // Return a string representation of the card
        // e.g. "Ace of Spades"
        format!("{}{}", PIPS[&self.pip], SUITS[&self.suit])
    }
}

#[derive(Debug)]
struct Deck {
    deck: u64,
    // bit representation of the deck
    draws: Vec<u8>, // card position numbers that have been drawn (1-52) in order
}

impl Deck {
    fn draw_card(&mut self, card: u8) -> Result<Card, String> {
        // Draw card number 1-52. Takes it out of the deck so that it can't be drawn again.
        // Returns a Card struct with the card value.
        // Side Effect adds it to the Deck draws list
        if card >= 1 && card <= 52 {
            let mask = 1 << card - 1;
            //println!("Adding card: {} to deck: {:064b}", *card, *deck);
            self.deck |= mask;
            //println!("Added card : {} current: {:064b}", *card, *deck);
            let out = num_to_card(&card);
            println!("out card = {:?}", out);
            return Ok(out);
        } else {
            return Err("`card` value must be between from 1-52".to_string());
        }
    }
}

struct Hand {
    cards: u64, // bit representation of a hand
}
// make a Deck struct that holds the u64 deck bit string
// number of drawn cards

fn add_card(card: &mut u64, deck: &mut Deck) -> Result<bool, String> {
    if *card >= 1 && *card <= 52 {
        let mask = 1 << *card - 1;
        //println!("Adding card: {} to deck: {:064b}", *card, *deck);
        deck.deck |= mask;
        //println!("Added card : {} current: {:064b}", *card, *deck);
        return Ok(true);
    } else {
        return Err("`card` value must be between from 1-52".to_string());
    }
}

fn get_cards(deck: &mut Deck) -> Vec<i32> {
    // Return a list of all the cards position numbers that have been drawn
    // from the deck
    let mut bit = 52;
    let mut out = Vec::new();
    let mut mask;
    while bit > 0 {
        mask = 1 << bit - 1;

        if deck.deck & mask != 0 {
            out.push(bit)
            //println!("Found card at position: {}", bit);
            //println!("deck: {:064b}", *deck);
        }
        //println!("bit: {}", bit);
        bit = bit - 1;
    }
    return out;
}

fn num_to_card(card: &u8) -> Card {
    // turn a card number(1-52) into a Card struct
    Card {
        suit: (*card - 1) / 13,
        pip: (*card - 1).rem_euclid(13)
    }
}

fn main() {
    let mut deck = Deck {
        deck: 0,
        draws: Vec::new(),
    };
    let mut card: u64 = 1;
    let cards;


    cards = get_cards(&mut deck);

    println!("cards: {:?}", cards);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add_card() {
        let mut deck: Deck = Deck { deck: 0, draws: Vec::new()};
        let out = add_card(&mut 1, &mut deck).expect("Could not insert card.");
        assert_eq!(out, true);
    }

    #[test]
    fn test_translate_card_number() {
        let mut c: Card = Card { suit: 0, pip: 0 };
        c = num_to_card(&mut 52);
        assert_eq!(c.suit, 3);
        assert_eq!(c.pip , 12);
        c = num_to_card(&mut 1);
        assert_eq!(c.suit, 0);
        assert_eq!(c.pip, 0);
        c = num_to_card(&mut 15);
        assert_eq!(c.suit, 1); // diamonds
        assert_eq!(c.pip, 1); // 3 of diamonds
        c = num_to_card(&mut 36);
        assert_eq!(c.suit, 2); // hearts
        assert_eq!(c.pip, 9); // J of hearts
   }

   #[test]
   fn test_get_card_name() {
       let mut c: Card = Card { suit: 0, pip: 0 };
       c = num_to_card(&mut 52);
       assert_eq!(c.get_card_name(), "A♠");
       c = num_to_card(&mut 1);
       assert_eq!(c.get_card_name(), "2♣");
       c = num_to_card(&mut 15);
       assert_eq!(c.get_card_name(), "3♦");
       c = num_to_card(&mut 36);
       assert_eq!(c.get_card_name(), "J♥");
       c = num_to_card(&mut 40);
       assert_eq!(c.get_card_name(), "2♠");
   }

   #[test]
   fn test_new_card() {
         let mut c: Card = Card { suit: 0, pip: 0 };
         c = Card::new(52).expect("Could not create card.");
         assert_eq!(c.suit, 3);
         assert_eq!(c.pip, 12);
         c = Card::new(1).expect("Could not create card.");
         assert_eq!(c.suit, 0);
         assert_eq!(c.pip, 0);
         c = Card::new(15).expect("Could not create card.");
         assert_eq!(c.suit, 1); // diamonds
         assert_eq!(c.pip, 1); // 3 of diamonds
         c = Card::new(36).expect("Could not create card.");
         assert_eq!(c.suit, 2); // hearts
         assert_eq!(c.pip, 9); // J of hearts
   }

}
