use ::phf::{phf_map};

static SUITS: phf::Map<i32, &'static str> = phf_map! {
    0i32 => "Clubs",
    1i32 => "Diamonds",
    2i32 => "Hearts",
    3i32 => "Spades"
};

static PIPS: phf::Map<i32, &'static str> = phf_map! {
    0i32 => "2 ",
    1i32 => "3 ",
    2i32 => "4 ",
    3i32 => "5 ",
    4i32 => "6 ",
    5i32 => "7 ",
    6i32 => "8 ",
    7i32 => "9 ",
    8i32 => "10",
    9i32 => "J ",
    10i32 => "Q ",
    11i32 => "K ",
    12i32 => "A "
};

fn add_card(card: &mut u64, deck: &mut u64) -> Result<bool, String> {
    if *card >= 1 && *card <= 52 {
        let mask = 1 << *card - 1;
        println!("Adding card: {} to deck: {:064b}", *card, *deck);
        *deck |= mask;
        println!("Added card : {} current: {:064b}", *card, *deck);
        return Ok(true);
    } else {
        return Err("`card` value must be between from 1-52".to_string());
    }
}

fn get_cards(deck: &u64) -> Vec<i32> {
    let mut bit = 52;
    let mut out = Vec::new();
    let mut mask;
    while bit > 0 {
        mask = 1 << bit - 1;

        if *deck & mask != 0 {
            out.push(bit)
            //println!("Found card at position: {}", bit);
            //println!("deck: {:064b}", *deck);
        }
        //println!("bit: {}", bit);
        bit = bit - 1;
    }
    return out;
}

fn translate_card_number(card: &i32) {
    // turn a card number(1-52) into a pip/suit name
    let suit = (*card - 1) / 13;
    let pip = (*card - 1).rem_euclid(13);
    println!("card: {} suit: {:?} pip: {}", *card, SUITS[&suit], PIPS[&pip]);
}

fn main() {
    let mut deck: u64 = 0;
    let mut card: u64 = 1;
    let cards;
    add_card(&mut card, &mut deck).expect("foo");
    add_card(&mut 4, &mut deck).expect("bar");
    add_card(&mut 52, &mut deck).expect("baz");

    translate_card_number(&mut 52);
    translate_card_number(&mut 1);

    cards = get_cards(&deck);

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