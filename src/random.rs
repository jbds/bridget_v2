use crate::constants::PACK_SIZE;
use crate::term_in_out::read_msg;
use crate::types::{Game, Player, Poc};
use rand::prelude::SliceRandom;
use std::collections::HashMap;

// impure
pub fn get_random_poc() -> Poc {
    let mut rng = rand::rng();
    let mut pocs = [Poc::North, Poc::South, Poc::East, Poc::West];
    pocs.shuffle(&mut rng);
    pocs[0].clone()
}

// impure
pub fn get_array_u8_shuffled(arr: [u8; 52usize]) -> [u8; 52usize] {
    let mut rng = rand::rng();
    let mut arr2 = arr.clone();
    arr2.shuffle(&mut rng);
    arr2
}

// helper
pub fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

pub fn get_array_u8_ordered() -> [u8; PACK_SIZE] {
    let v: Vec<u8> = (0..52u8).collect();
    vec_to_array(v)
}

// impure
pub fn create_player_seating_plan() -> HashMap<Poc, Option<Player>> {
    let mut seating_plan = HashMap::new();
    let mut player_names = String::new();
    read_msg(&mut player_names);
    let mut iter = player_names.split_whitespace();
    // don't try to store the borrowed &str, store a real String
    seating_plan.insert(Poc::North, iter.next().map(|s| s.to_string()));
    seating_plan.insert(Poc::South, iter.next().map(|s| s.to_string()));
    seating_plan.insert(Poc::East, iter.next().map(|s| s.to_string()));
    seating_plan.insert(Poc::West, iter.next().map(|s| s.to_string()));
    seating_plan
}

pub fn init_player_seating_plan() -> HashMap<Poc, Option<Player>> {
    let mut seating_plan = HashMap::new();
    seating_plan.insert(Poc::North, None);
    seating_plan.insert(Poc::South, None);
    seating_plan.insert(Poc::East, None);
    seating_plan.insert(Poc::West, None);
    seating_plan
}

// pub fn update_player_seating_plan(game: Game) -> Game {
//     let mut seating_plan = HashMap::new();
//     let mut player_names = String::new();
//     read_msg(&mut player_names);
//     let mut iter = player_names.split_whitespace();
//     // don't try to store the borrowed &str, store a real String
//     seating_plan.insert(Poc::North, iter.next().map(|s| s.to_string()));
//     seating_plan.insert(Poc::South, iter.next().map(|s| s.to_string()));
//     seating_plan.insert(Poc::East, iter.next().map(|s| s.to_string()));
//     seating_plan.insert(Poc::West, iter.next().map(|s| s.to_string()));
//     let game2 = Game {
//         seating_plan: seating_plan,
//         ..game
//     };
//     game2
// }
