use crate::types::{CardState, Game, Poc, Suit};
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

/// takes the Game data structure as input and draws the game state on to a
/// terminal using a virtual card table
pub fn display_game_cmd_line(g: &Game) {
    //write_msg_game("Game: ", &g);
    let deal_index_max = &g.deals.len() - 1;
    //println!("deal_index_max: {}", deal_index_max);
    let deal_current = &g.deals.get(deal_index_max).unwrap().pack_state;
    let ranks_padded = get_card_ranks_as_string_array(deal_current);
    println!("{:?}", &ranks_padded);
    println!("{}", &ranks_padded[0]);
    // TO DO - create the virtual table on the terminal display
    display_board();
}

// we want to return an array of 16 strings
// starting with a north - spade string and ending with a west - club string
fn get_card_ranks_as_string_array(arr: &[CardState; 52]) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    for poc in [Poc::North, Poc::East, Poc::South, Poc::West] {
        for suit in [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club] {
            let mut hand_suit: Vec<_> = arr
                .iter()
                .filter(|card_state| card_state.hand == poc && card_state.suit == suit)
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
            let ranks_padded_13 = format!("{:.<13}", hand_suit_ranks_concat);
            //println!("hand_{:?}_{:?}: {:?}", poc, suit, ranks_padded_13);
            v.push(ranks_padded_13);
        }
    }
    v
}

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
