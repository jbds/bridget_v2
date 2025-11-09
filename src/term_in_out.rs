use crate::constants::PACK_SIZE;
use crate::types::{CardState, Game};
use std::io;

pub fn read_msg(s: &mut String) {
    io::stdin().read_line(s).expect("Failed to read line");
}

pub fn write_msg(s: &str) {
    println!("{}", s);
}

pub fn write_msg_game(s: &str, g: &Game) {
    println!("{}{:?}", s, g)
}

// display helpers
pub fn filter_by_spades(hand: &[CardState]) -> Vec<&CardState> {
    hand.into_iter()
        // arrays are constant size, so to filter same, the output must be a Vec<>
        // filter predicate needs matching type both sides. usize because of const PACK_SIZE type
        .filter(|&x| x.value as usize >= 3 * PACK_SIZE / 4)
        .collect()
}

pub fn filter_by_hearts(hand: &[CardState]) -> Vec<&CardState> {
    hand.into_iter()
        // arrays are constant size, so to filter same, the output must be a Vec<>
        // filter predicate needs matching type both sides. usize because of const PACK_SIZE type
        .filter(|&x| {
            (x.value as usize >= PACK_SIZE / 2) && ((x.value as usize) < 3 * PACK_SIZE / 4)
        })
        .collect()
}

pub fn filter_by_diamonds(hand: &[CardState]) -> Vec<&CardState> {
    hand.into_iter()
        // arrays are constant size, so to filter same, the output must be a Vec<>
        // filter predicate needs matching type both sides. usize because of const PACK_SIZE type
        .filter(|&x| (x.value as usize >= PACK_SIZE / 4) && ((x.value as usize) < PACK_SIZE / 2))
        .collect()
}

pub fn filter_by_clubs(hand: &[CardState]) -> Vec<&CardState> {
    hand.into_iter()
        // arrays are constant size, so to filter same, the output must be a Vec<>
        // filter predicate needs matching type both sides. usize because of const PACK_SIZE type
        .filter(|&x| (x.value as usize) < PACK_SIZE / 4)
        .collect()
}

/// takes the Game data structure as input and draws the game state on to a
/// terminal using a virtual card table
// pub fn display_game_cmd_line(g: &Game) {
//     //write_msg_game("Game: ", &g);
//     let deal_index_max = &g.deals.len() - 1;
//     //println!("deal_index_max: {}", deal_index_max);
//     let deal_current = &g.deals.get(deal_index_max).unwrap().pack_state;
//     let ranks_padded = get_card_ranks_as_string_array(deal_current);
//     println!("{:?}", &ranks_padded);
//     println!("{}", &ranks_padded[0]);
//     // TO DO - create the virtual table on the terminal display
// }

// we want to return an array of 16 strings
// starting with a north - spade string and ending with a west - club string
// fn get_card_ranks_as_string_array(arr: &[CardState; 52]) -> Vec<String> {
//     let mut v: Vec<String> = Vec::new();
//     for poc in 0..4 {
//         for suit in 0..4 {
//             // needs size hint 13 and also CardState must derive both Clone and Copy fot try_into() to succeed
//             let hand_poc: [CardState; 13] = match poc {
//                 0 => arr[0..PACK_SIZE / 4].try_into().unwrap(),
//                 1 => arr[PACK_SIZE / 4..PACK_SIZE / 2].try_into().unwrap(),
//                 2 => arr[PACK_SIZE / 2..3 * PACK_SIZE / 4].try_into().unwrap(),
//                 3 => arr[3 * PACK_SIZE / 4..PACK_SIZE].try_into().unwrap(),
//                 _ => {
//                     [CardState {
//                         value: 0,
//                         discard: None,
//                     }; 13]
//                 }
//             };
//             let mut hand_poc_suit = match suit {
//                 0 => filter_by_spades(&hand_poc),
//                 1 => filter_by_hearts(&hand_poc),
//                 2 => filter_by_diamonds(&hand_poc),
//                 3 => filter_by_clubs(&hand_poc),
//                 _ => vec![],
//             };
//             hand_poc_suit.sort();
//             hand_poc_suit.reverse();
//             let hand_poc_suit_ranks: Vec<_> =
//                 hand_poc_suit.into_iter().map(|x| x.rank_as_str()).collect();
//             let hand_poc_suit_ranks_concat = hand_poc_suit_ranks.concat();
//             let ranks_padded_13 = format!("{:.<13}", hand_poc_suit_ranks_concat);
//             //println!("poc{}-suit{}: {}", &poc, &suit, &ranks_padded_13);
//             v.push(ranks_padded_13);
//         }
//     }
//     //println!("{:?}", &v);
//     v
// }

fn display_board() {
    let mut lines: Vec<String> = Vec::new();
    let line0 = format!("{:*<65}", "");
    lines.push(line0);
    let line1 = format!("{:*<34}", "") + "S AKQJT98765432" + &format!("{:*<16}", "");
    lines.push(line1);
    let line2 = format!("{:*<34}", "") + "H AKQJT98765432" + &format!("{:*<16}", "");
    lines.push(line2);
    let line3 = format!("{:*<34}", "") + "D AKQJT98765432" + &format!("{:*<16}", "");
    lines.push(line3);
    let line4 = format!("{:*<34}", "") + "C AKQJT98765432" + &format!("{:*<16}", "");
    lines.push(line4);

    for i in 0..5 {
        println!("{}", &lines[i]);
    }
}
