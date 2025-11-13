//! # Bridget v2
//!
//! The game of Bridge uses a pack of 52 playing cards (no Jokers)
//!
//! Rust arrays are fixed length so can model the pack optimally.

use crate::game::create_new_game_data_structure;
use crate::term_in_out::{display_game_cmd_line, read_msg, write_msg};
use crate::types::Game;

pub mod constants;
pub mod game;
pub mod random;
pub mod term_in_out;
pub mod types;

pub fn run_game() {
    let mut game = create_new_game_data_structure();
    let mut game_states: Vec<Game> = Vec::new();
    // looks like we will need to implement Clone for Game struct
    game_states.push(game.clone());
    //println!("{:?}", game_states[0]);
    write_msg("Welcome to Bridgetv2");
    let mut quit = false;
    while quit == false {
        let mut keyboard_string = String::new();
        read_msg(&mut keyboard_string);
        match keyboard_string.as_str() {
            // Quit
            "qu\n" => {
                println!("Goodbye!");
                quit = true;
            }
            // Seating plan
            "sp\n" => {
                println!(
                    "Please enter North, South, East and West player names separated by spaces"
                );
                let game_updated = game.update_seating_plan();
                game = game_updated;
                game_states.push(game.clone());
            }
            // Current game state
            "gs\n" => {
                println!("{:?}", &game);
            }
            // New Deal
            "nd\n" => {
                println!("New deal");
            }
            &_ => (),
        }
        //display_game_cmd_line(&game);
    }
}
