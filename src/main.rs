use ::phf::{phf_map};

static SUITS: phf::Map<u8, &'static str> = phf_map! {
    0u8 => "Clubs",
    1u8 => "Diamonds",
    2u8 => "Hearts",
    3u8 => "Spades"
};

static PIPS: phf::Map<u8, &'static str> = phf_map! {
    0u8 => "2 ",
    1u8 => "3 ",
    2u8 => "4 ",
    3u8 => "5 ",
    4u8 => "6 ",
    5u8 => "7 ",
    6u8 => "8 ",
    7u8 => "9 ",
    8u8 => "10",
    9u8 => "J ",
    10u8 => "Q ",
    11u8 => "K ",
    12u8 => "A "
};

#[derive(Debug)]
struct Card {
    suit: u8,
    pip: u8,
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
            let out = translate_card_number(&card);
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

fn translate_card_number(card: &u8) -> Card {
    // turn a card number(1-52) into a pip/suit name
    let c = Card {
        suit: (*card - 1) / 13,
        pip: (*card - 1).rem_euclid(13)
    };
    c
}

fn main() {
    let mut deck = Deck {
        deck: 0,
        draws: Vec::new(),
    };
    let mut card: u64 = 1;
    let cards;
    add_card(&mut card, &mut deck).expect("foo");
    add_card(&mut 4, &mut deck).expect("bar");
    add_card(&mut 52, &mut deck).expect("baz");

    translate_card_number(&mut 52);
    translate_card_number(&mut 1);

    cards = get_cards(&mut deck);

    println!("cards: {:?}", cards);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add_card() {
        let mut deck: u64 = 0;
        let out = add_card(&mut 1, &mut deck).expect("Could not insert card.");
        assert_eq!(out, true);
    }
}