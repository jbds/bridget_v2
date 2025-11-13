use std::panic;

use crate::constants::{
    PACK_LO, PACK_LO_U8, PACK_Q1, PACK_Q1_U8, PACK_Q2, PACK_Q2_U8, PACK_Q3, PACK_Q3_U8, PACK_SIZE,
    PACK_SIZE_U8,
};
use crate::random::{
    get_array_u8_ordered, get_array_u8_shuffled, get_random_poc, init_player_seating_plan,
    vec_to_array,
};
use crate::types::{CardState, Deal, Game, Poc, Rank, Suit};

/// populate the Game data structure with a player seating plan, an initial (random) dealer,
/// and a shuffled pack of cards
pub fn create_new_game_data_structure() -> Game {
    let seating_plan = init_player_seating_plan();
    let initial_dealer_poc = get_random_poc();
    let array_u8_ordered = get_array_u8_ordered();
    let array_u8_shuffled = get_array_u8_shuffled(array_u8_ordered);
    // for testing may not want shuffle
    //let array_u8_shuffled = array_u8_ordered;
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
                _ => panic!("Card enumerator is >= PACK_SIZE"),
            },
            suit: match x {
                PACK_LO_U8..PACK_Q1_U8 => Suit::Club,
                PACK_Q1_U8..PACK_Q2_U8 => Suit::Diamond,
                PACK_Q2_U8..PACK_Q3_U8 => Suit::Heart,
                PACK_Q3_U8..PACK_SIZE_U8 => Suit::Spade,
                _ => panic!("Card value is >= PACK_SIZE_U8"),
            },
            rank: match x % PACK_Q1_U8 {
                0 => Rank::Two,
                1 => Rank::Three,
                2 => Rank::Four,
                3 => Rank::Five,
                4 => Rank::Six,
                5 => Rank::Seven,
                6 => Rank::Eight,
                7 => Rank::Nine,
                8 => Rank::Ten,
                9 => Rank::Jack,
                10 => Rank::Queen,
                11 => Rank::King,
                12 => Rank::Ace,
                _ => panic!("Card rank is >= PACK_Q1_U8"),
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
    // followed by a new game instance
    let game = Game {
        deals: vec![deal],
        initial_dealer: initial_dealer_poc,
        seating_plan: seating_plan,
    };
    game
}

/// for a new deal, we need to shuffle the pack
pub fn shuffle_pack() -> Vec<CardState> {
    Vec::new()
}
