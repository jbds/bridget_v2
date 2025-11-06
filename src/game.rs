use crate::random::{
    create_player_seating_plan, get_pack_ordered, get_pack_shuffled, get_random_poc,
};
use crate::term_in_out::write_msg;
use crate::types::{CardState, Deal, Game};

/// populate the Game data structure with a player seating plan, an initial (random) dealer,
/// and a shuffled pack of cards
pub fn create_new_game_data_structure() -> Game {
    write_msg("Welcome to Bridgetv2");
    write_msg("New Game - Enter N,S,E,W players separated by space:");
    let seating_plan = create_player_seating_plan();
    let initial_dealer_poc = get_random_poc();
    let pack_ordered = get_pack_ordered();
    let pack_shuffled = get_pack_shuffled(pack_ordered);
    // for testing may not want shuffle
    //let pack_shuffled = pack_ordered;
    let pack_state = pack_shuffled.map(|x| CardState {
        value: x,
        discard: None,
    });
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
