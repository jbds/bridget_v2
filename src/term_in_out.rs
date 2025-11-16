use crate::types::{CardState, Game, Poc, Suit, Rank};
use std::io;

// legacy VT100 style dumb terminal constants
const RST: &str = "\x1B[0m";
const BOW: &str = "\x1B[30;107m";
const BOLG: &str = "\x1B[30;100m";
const ROW: &str = "\x1B[31;107m";
const ROLG: &str = "\x1B[31;47m";
const BODG: &str = "\x1B[30;47m";

// unicode points for card suits
const SPADE: &str = "\u{2660}";
const HEART: &str = "\u{2665}";
const DIAMOND: &str = "\u{2666}";
const CLUB: &str = "\u{2663}";

// unicode points for box drawing characters
const HZ: &str = "\u{2550}";
const VT: &str = "\u{2551}";
const TL: &str = "\u{2554}";
const TR: &str = "\u{2557}";
const BL: &str = "\u{255A}";
const BR: &str = "\u{255D}";

pub fn read_msg(s: &mut String) {
    io::stdin().read_line(s).expect("Failed to read line");
}

pub fn write_msg(s: &str) {
    println!("{}", s);
}

pub fn write_msg_game(s: &str, g: &Game) {
    println!("{}{:?}", s, g)
}

/// takes the Game data structure as input and draws the game state on to a
/// terminal using a virtual card table
pub fn display_game_cmd_line(g: &Game) {
    //write_msg_game("Game: ", &g);
    let deal_index_max = &g.deals.len() - 1;
    //println!("deal_index_max: {}", deal_index_max);
    let deal_current = &g.deals.get(deal_index_max).unwrap().pack_state;
    let ranks_padded = get_card_ranks_as_string_array(deal_current);
    //println!("{:?}", &ranks_padded);
    display_board(ranks_padded);
}

// we want to return an array of 16 strings
// starting with a north - spade string and ending with a west - club string
fn get_card_ranks_as_string_array(arr: &[CardState; 52]) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    for poc in [Poc::North, Poc::East, Poc::South, Poc::West] {
        for suit in [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club] {
            let mut hand_suit: Vec<_> = arr
                .iter()
                .filter(|card_state| card_state.hand == poc && card_state.suit == suit && card_state.discard == None)
                .collect();
            hand_suit.sort();
            hand_suit.reverse();
            //println!("hand_{:?}_{:?}: {:?}", poc, suit, hand_suit);
            let hand_suit_ranks: Vec<_> = hand_suit
                .into_iter()
                .map(|card_state| card_state.rank_as_str())
                .collect();
            //println!("hand_{:?}_{:?}: {:?}", poc, suit, hand_suit_ranks);
            let hand_suit_ranks_concat = hand_suit_ranks.concat();
            //println!("hand_{:?}_{:?}: {:?}", poc, suit, hand_suit_ranks_concat);
            let ranks_padded_13 = format!("{: <13}", hand_suit_ranks_concat);
            //println!("hand_{:?}_{:?}: {:?}", poc, suit, ranks_padded_13);
            v.push(ranks_padded_13);
        }
    }
    v
}

fn display_board(ranks: Vec<String>) {
    let mut lines: Vec<String> = Vec::new();
    let line0 = format!("{}{: <65}{}", BOW, "", RST);
    lines.push(line0);
    let line1 = 
        format!("{}{: <34}{}", BOW, "", RST)
        + &format!("{}{}{}{}{}", BOW, SPADE, " ", &ranks[0], RST)
        + &format!("{}{: <16}{}", BOW, "", RST);
    lines.push(line1);
    let line2 = format!("{}{: <34}{}", BOW, "", RST)
        + &format!("{}{}{}{}{}{}", ROW, HEART, BOW, " ", &ranks[1], RST)
        + &format!("{}{: <16}{}", BOW, "", RST);
    lines.push(line2);
    let line3 = format!("{}{: <34}{}", BOW, "", RST)
        + &format!("{}{}{}{}{}{}", ROW, DIAMOND, BOW," ", &ranks[2], RST)
        + &format!("{}{: <16}{}", BOW, "", RST);
    lines.push(line3);
    let line4 = format!("{}{: <34}{}", BOW, "", RST)
        + &format!("{}{}{}{}{}", BOW, CLUB, " ", &ranks[3], RST)
        + &format!("{}{: <16}{}", BOW, "", RST);
    lines.push(line4);
    let line5 = 
        format!("{}{: <33}{}", BOW, "", RST)
        + &format!("{}{: <5}{}", BOLG, "", RST)
        + &format!("{}{}{}{}{}{}{}{}", BODG, TL, HZ, HZ, HZ, HZ, TR, RST)
        + &format!("{}{: <5}{}", BOLG, "", RST)
        + &format!("{}{: <16}{}", BOW, "", RST);
    lines.push(line5);
        let line6 = 
        format!("{}{: <33}{}", BOW, "", RST)
        + &format!("{}{: <5}{}", BOLG, "", RST)
        //+ &format!("{}{}{}{}{}{}{}{}{}{}", BODG, VT, " ", "2", ROLG, HEART, BODG, " ", VT, RST)
        + &format!("{}{}{}{}{}{}{}{}{}{}", BODG, VT, " ", "2", "", "H", "", " ", VT, RST)
        + &format!("{}{: <5}{}", BOLG, "", RST)
        + &format!("{}{: <16}{}", BOW, "", RST);
    lines.push(line6);
    let line7 = format!("{}{: <18}{}", BOW, "", RST)
        + &format!("{}{}{}{}{}", BOW, SPADE, " ", &ranks[12], RST)
        + &format!("{}{: <1}{}", BOLG, "", RST)
        + &format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", BODG, TL, HZ, HZ, HZ, BL, HZ, HZ, HZ, TL, HZ, HZ, HZ, HZ, TR, RST)
        + &format!("{}{: <1}{}", BOLG, "", RST)
        + &format!("{}{}{}{}{}{}", BOW, " ", SPADE, " ", &ranks[4], RST);
    lines.push(line7);
    let line8 =
        format!("{}{: <18}{}", BOW, "", RST)
        + &format!("{}{}{}{}{}{}", ROW, HEART, BOW, " ", &ranks[13], RST)
        + &format!("{}{: <1}{}", BOLG, "", RST)
        //+ &format!("{}{}{}{}{}{}{}{}{}{}", BODG, VT, " ", "3", ROLG, DIAMOND, BODG, " ", VT, RST)
        + &format!("{}{}{}{}{}{}{}{}{}{}", BODG, VT, " ", "3", "", "D", "", " ", VT, RST)
        + &format!("{}{: <2}{}", BOLG, "", RST)
        //+ &format!("{}{}{}{}{}{}{}{}{}{}", BODG, VT, " ", "4", ROLG, DIAMOND, BODG, " ", VT, RST)
        + &format!("{}{}{}{}{}{}{}{}{}{}", BODG, VT, " ", "4", "", "D", "", " ", VT, RST)
        + &format!("{}{: <1}{}", BOLG, "", RST)
        + &format!("{}{}{}{}{}{}{}{}", BOW, " ", ROW, HEART, BOW," ", &ranks[5], RST);
    lines.push(line8);
    let line9 = format!("{}{: <18}{}", BOW, "", RST)
        + &format!("{}{}{}{}{}{}", ROW, DIAMOND, BOW, " ", &ranks[14], RST)
        + &format!("{}{: <1}{}", BOLG, "", RST)
        + &format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", BODG, BL, HZ, HZ, HZ, TL, HZ, HZ, HZ, HZ, TR, HZ, HZ, HZ, BR, RST)
        + &format!("{}{: <1}{}", BOLG, "", RST)
        + &format!("{}{}{}{}{}{}{}{}", BOW, " ", ROW, DIAMOND, BOW, " ", &ranks[6], RST);
    lines.push(line9);
    let line10 =
        format!("{}{: <18}{}", BOW, "", RST)
        + &format!("{}{}{}{}{}", BOW, CLUB, " ", &ranks[15], RST)
        + &format!("{}{: <5}{}", BOLG, "", RST)
        //+ &format!("{}{}{}{}{}{}{}{}{}{}", BODG, VT, " ", "5", ROLG, HEART, BODG, " ", VT, RST)
        + &format!("{}{}{}{}{}{}{}{}{}{}", BODG, VT, " ", "5", "", "H", "", " ", VT, RST)
        + &format!("{}{: <5}{}", BOLG, "", RST)
        + &format!("{}{}{}{}{}{}", BOW, " ", CLUB, " ", &ranks[7], RST);
    lines.push(line10);
    let line11 = 
        //format!("{}{: <65}{}", BOW, "", RST);
        format!("{}{: <33}{}", BOW, "", RST)
        + &format!("{}{: <5}{}", BOLG, "", RST)
        + &format!("{}{}{}{}{}{}{}{}", BODG, BL, HZ, HZ, HZ, HZ, BR, RST)
        + &format!("{}{: <5}{}", BOLG, "", RST)
        + &format!("{}{: <16}{}", BOW, "", RST);
    lines.push(line11);
    let line12 = 
        //format!("{:*<34}", "") + "S " + &ranks[8] + &format!("{:*<16}", "");
        format!("{}{: <34}{}", BOW, "", RST)
        + &format!("{}{}{}{}{}", BOW, SPADE, " ", &ranks[8], RST)
        + &format!("{}{: <16}{}", BOW, "", RST);
    lines.push(line12);
    let line13 = 
        //format!("{:*<34}", "") + "H " + &ranks[9] + &format!("{:*<16}", "");
        format!("{}{: <34}{}", BOW, "", RST)
        + &format!("{}{}{}{}{}{}", ROW, HEART, BOW, " ", &ranks[9], RST)
        + &format!("{}{: <16}{}", BOW, "", RST);
    lines.push(line13);
    let line14 = 
        //format!("{:*<34}", "") + "D " + &ranks[10] + &format!("{:*<16}", "");
        format!("{}{: <34}{}", BOW, "", RST)
        + &format!("{}{}{}{}{}{}", ROW, DIAMOND, BOW, " ", &ranks[10], RST)
        + &format!("{}{: <16}{}", BOW, "", RST);
    lines.push(line14);
    let line15 = 
        //format!("{:*<34}", "") + "C " + &ranks[11] + &format!("{:*<16}", "");
        format!("{}{: <34}{}", BOW, "", RST)
        + &format!("{}{}{}{}{}", BOW, CLUB, " ", &ranks[11], RST)
        + &format!("{}{: <16}{}", BOW, "", RST);
    lines.push(line15);
    let line16 = format!("{}{: <65}{}", BOW, "", RST);
    lines.push(line16);

    for i in 0..17 {
        println!("{}", &lines[i]);
    }
}

/// helper function to convert keypresses eg "2c" to tuple (TWO, CLUB)
pub fn card_as_string_to_tuple(s: &str) -> (Rank, Suit) {
    if s.len() != 2 {
        panic!("card_as_string_to_tuple received unexpected string: '{}'", &s)
    } else {
        //println!("s[0]: {:?}", &s.chars().nth(0));
        //println!("s[1]: {:?}", &s.chars().nth(1));
        let rank = match &s.chars().nth(0) {
            None => panic!("card_as_string_to_tuple missing rank"),
            Some('2') => Rank::Two,
            Some('3') => Rank::Three,
            Some('4') => Rank::Four,
            Some('5') => Rank::Five,
            Some('6') => Rank::Six,
            Some('7') => Rank::Seven,
            Some('8') => Rank::Eight,
            Some('9') => Rank::Nine,
            Some('t') => Rank::Ten,
            Some('j') => Rank::Jack,
            Some('q') => Rank::Queen,
            Some('k') => Rank::King,
            Some('a') => Rank::Ace,
            Some(_) => panic!("card_as_string_to_tuple unexpected first character"),
        };
        let suit = match &s.chars().nth(1) {
            None => panic!("card_as_string_to_tuple missing suit"),
            Some('c') => Suit::Club,
            Some('d') => Suit::Diamond,
            Some('h') => Suit::Heart,
            Some('s') => Suit::Spade,
            Some(_) => panic!("card_as_string_to_tuple unexpected second character"),
        };
        (rank, suit)
    }
}
