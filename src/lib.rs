//! # Bridget v2
//!
//! The game of Bridge uses a pack of 52 playing cards (no Jokers)
//!
//! Rust arrays are fixed length so can model the pack optimally.

use crate::game::create_new_game_data_structure;
use crate::term_in_out::{card_as_string_to_tuple, display_game_cmd_line, read_msg, write_msg};
use crate::types::Game;

pub mod constants;
pub mod game;
pub mod random;
pub mod term_in_out;
pub mod types;

pub fn run_game() {
    let (game, mut game_states) = init_game();
    let mut quit = false;
    while quit == false {
        let mut keyboard_string = String::new();
        read_msg(&mut keyboard_string);
        let keypresses = keyboard_string.as_str().trim();
        match keypresses {
            // Quit
            "qu" => {
                println!("Goodbye!");
                quit = true;
            }
            // Seating plan
            "sp" => {
                println!(
                    "Please enter North, South, East and West player names separated by spaces"
                );
                let game_updated = game.clone().update_seating_plan();
                // shadows previous game pointer
                let game = game_updated;
                game_states.push(game.clone());
                display_game_cmd_line(&game);
            }
            // Current game state
            "gs" => {
                println!("{:?}", &game);
            }
            // New Deal
            "nd" => {
                println!("New deal");
            }
            &_ => match keypresses.chars().nth(0) {
                Some('2'..='9' | 't' | 'j' | 'q' | 'k' | 'a') => {
                    //2dprintln!("matched {}", &keypresses);
                    match keypresses.chars().nth(1) {
                        Some('c' | 'd' | 'h' | 's') => {
                            //println!("matched 2 chrs: '{}'", keypresses);
                            // TO DO - update game stae with discard
                            let (rank, suit) = card_as_string_to_tuple(keypresses);
                            let game_updated = game.clone().update_discard(rank, suit);
                            let game = game_updated;
                            game_states.push(game.clone());
                            display_game_cmd_line(&game);
                        }
                        Some(_) => {
                            println!("Did not recognise your 2nd character in '{}'", &keypresses);
                        }
                        None => {
                            println!(
                                "Expected 2 characters, but only got '{}', please re-try",
                                &keypresses
                            );
                        }
                    }
                }
                Some(_) => println!(
                    "Did not recognise your first character in '{}'",
                    &keypresses
                ),
                None => (), // user pressed Enter key only
            },
        }
    }
}

fn init_game() -> (Game, Vec<Game>) {
    let game = create_new_game_data_structure();
    let mut game_states: Vec<Game> = Vec::new();
    // looks like we will need to implement Clone for Game struct
    game_states.push(game.clone());
    display_game_cmd_line(&game);
    //println!("{:?}", game_states[0]);
    write_msg("Bridgetv2 Menu:\nqu = Quit, sp = Seating Plan, nd = New deal, 2s..ac = Card choice");
    (game, game_states)
}
