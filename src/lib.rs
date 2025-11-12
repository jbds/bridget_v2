//! # Bridget v2
//!
//! The game of Bridge uses a pack of 52 playing cards (no Jokers)
//!
//! Rust arrays are fixed length so can model the pack optimally.

use crate::game::create_new_game_data_structure;
use crate::random::update_player_seating_plan;
use crate::term_in_out::{display_game_cmd_line, read_msg, write_msg};

pub mod constants;
pub mod game;
pub mod random;
pub mod term_in_out;
pub mod types;

pub fn run_game() {
    let mut game = create_new_game_data_structure();
    println!("{:?}", &game);
    write_msg("Welcome to Bridgetv2");
    let mut quit = false;
    while quit == false {
        let mut keyboard_string = String::new();
        read_msg(&mut keyboard_string);
        match keyboard_string.as_str() {
            "qu\n" => {
                println!("Goodbye!");
                quit = true;
            }
            "sp\n" => {
                println!(
                    "Please enter North, South, East and West player names separated by spaces"
                );
                random::update_player_seating_plan(&mut game);
            }
            "gs\n" => {
                println!("{:?}", &game);
            }
            &_ => (),
        }
        //display_game_cmd_line(&game);
    }
}
