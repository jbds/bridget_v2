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
    //println!("{}", &ranks_padded[0]);
    // TO DO - create the virtual table on the terminal display
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

fn display_board(ranks: Vec<String>) {
    let mut lines: Vec<String> = Vec::new();
    let line0 = format!("{:*<65}", "");
    lines.push(line0);
    let line1 = format!("{:*<34}", "") + "S " + &ranks[0] + &format!("{:*<16}", "");
    lines.push(line1);
    let line2 = format!("{:*<34}", "") + "H " + &ranks[1] + &format!("{:*<16}", "");
    lines.push(line2);
    let line3 = format!("{:*<34}", "") + "D " + &ranks[2] + &format!("{:*<16}", "");
    lines.push(line3);
    let line4 = format!("{:*<34}", "") + "C " + &ranks[3] + &format!("{:*<16}", "");
    lines.push(line4);
    let line5 = format!("{:*<65}", "");
    lines.push(line5);
    let line6 =
        format!("{:*<18}", "") + "S " + &ranks[12] + &format!("{:*<16}", "") + " S " + &ranks[4];
    lines.push(line6);
    let line7 =
        format!("{:*<18}", "") + "H " + &ranks[13] + &format!("{:*<16}", "") + " H " + &ranks[5];
    lines.push(line7);
    let line8 =
        format!("{:*<18}", "") + "D " + &ranks[14] + &format!("{:*<16}", "") + " D " + &ranks[6];
    lines.push(line8);
    let line9 =
        format!("{:*<18}", "") + "C " + &ranks[15] + &format!("{:*<16}", "") + " C " + &ranks[7];
    lines.push(line9);
    let line10 = format!("{:*<65}", "");
    lines.push(line10);
    let line11 = format!("{:*<34}", "") + "S " + &ranks[8] + &format!("{:*<16}", "");
    lines.push(line11);
    let line12 = format!("{:*<34}", "") + "H " + &ranks[9] + &format!("{:*<16}", "");
    lines.push(line12);
    let line13 = format!("{:*<34}", "") + "D " + &ranks[10] + &format!("{:*<16}", "");
    lines.push(line13);
    let line14 = format!("{:*<34}", "") + "C " + &ranks[11] + &format!("{:*<16}", "");
    lines.push(line14);
    let line15 = format!("{:*<65}", "");
    lines.push(line15);

    for i in 0..16 {
        println!("{}", &lines[i]);
    }
}
