//! # Bridget v2
//!
//! The game of Bridge uses a pack of 52 playing cards (no Jokers)
//!
//! Rust arrays are fixed length so can model the pack optimally.

use crate::game::create_new_game_data_structure;
use crate::term_in_out::display_game_cmd_line;

pub mod constants;
pub mod game;
pub mod random;
pub mod term_in_out;
pub mod types;

pub fn run_game() {
    let game = create_new_game_data_structure();
    display_game_cmd_line(&game);
}
