use std::panic;

use crate::constants::{
    PACK_LO, PACK_LO_U8, PACK_Q1, PACK_Q1_U8, PACK_Q2, PACK_Q2_U8, PACK_Q3, PACK_Q3_U8, PACK_SIZE,
    PACK_SIZE_U8,
};
use crate::random::{
    create_player_seating_plan, get_array_u8_ordered, get_array_u8_shuffled, get_random_poc,
    vec_to_array,
};
use crate::term_in_out::write_msg;
use crate::types::{CardState, Deal, Game, Poc, Rank, Suit};

/// populate the Game data structure with a player seating plan, an initial (random) dealer,
/// and a shuffled pack of cards
pub fn create_new_game_data_structure() -> Game {
    write_msg("Welcome to Bridgetv2");
    write_msg("New Game - Enter N,S,E,W players separated by space:");
    let seating_plan = create_player_seating_plan();
    let initial_dealer_poc = get_random_poc();
    let array_u8_ordered = get_array_u8_ordered();
    let array_u8_shuffled = get_array_u8_shuffled(array_u8_ordered);
    // for testing may not want shuffle
    let array_u8_shuffled = array_u8_ordered;
    // can only use collect() here on a Vec, not an array
    let pack_state_as_vec: Vec<_> = array_u8_shuffled
        // iterate array by value
        .into_iter()
        // add an index (i, _) where _ is the original element
        .enumerate()
        // create each card using enumerator i and value x
        .map(|(i, x)| CardState {
            value: x,
            index: i,
            hand: match i {
                PACK_LO..PACK_Q1 => Poc::North,
                PACK_Q1..PACK_Q2 => Poc::East,
                PACK_Q2..PACK_Q3 => Poc::South,
                PACK_Q3..PACK_SIZE => Poc::West,
                _ => panic!("Card enumerator is > 51"),
            },
            suit: match x {
                PACK_LO_U8..PACK_Q1_U8 => Suit::Club,
                PACK_Q1_U8..PACK_Q2_U8 => Suit::Diamond,
                PACK_Q2_U8..PACK_Q3_U8 => Suit::Heart,
                PACK_Q3_U8..PACK_SIZE_U8 => Suit::Spade,
                _ => panic!("Card value is > 51"),
            },
            rank: match x {
                x if x % PACK_Q1_U8 == 0 => Rank::Two,
                x if x % PACK_Q1_U8 == 1 => Rank::Three,
                x if x % PACK_Q1_U8 == 2 => Rank::Four,
                x if x % PACK_Q1_U8 == 3 => Rank::Five,
                x if x % PACK_Q1_U8 == 4 => Rank::Six,
                x if x % PACK_Q1_U8 == 5 => Rank::Seven,
                x if x % PACK_Q1_U8 == 6 => Rank::Eight,
                x if x % PACK_Q1_U8 == 7 => Rank::Nine,
                x if x % PACK_Q1_U8 == 8 => Rank::Ten,
                x if x % PACK_Q1_U8 == 9 => Rank::Jack,
                x if x % PACK_Q1_U8 == 10 => Rank::Queen,
                x if x % PACK_Q1_U8 == 11 => Rank::King,
                x if x % PACK_Q1_U8 == 12 => Rank::Ace,
                _ => panic!("Card value is > 51"),
            },
            discard: None,
        })
        .collect();
    let pack_state = vec_to_array(pack_state_as_vec);
    // we can now instatiate a deal
    let deal = Deal {
        bids: Vec::new(),
        pack_state: pack_state,
    };
    // followed by a new game (may have to move to beginning?)
    let game = Game {
        deals: vec![deal],
        initial_dealer: initial_dealer_poc,
        seating_plan: seating_plan,
    };
    game
}
