use std::collections::HashMap;

/// At start of game, we need to ask for and store the 4 player names
pub type Player = String;

/// track the order in which the cards are discarded
pub type Discard = Option<u8>;

/// The cards are dealt out to 4 locations described by 4 points of the compass (Poc)
#[derive(Eq, PartialEq, Hash, Debug, Clone, Ord, PartialOrd)]
pub enum Poc {
    North = 0,
    East,
    South,
    West,
}

/// A bid level for suit or no trumps can range from 1 to 7
#[derive(Debug)]
pub enum BidLevel {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

/// A bid can be one of Level-Suit, Level-NoTrumps, Pass, Double, or Redouble
#[repr(u8)]
#[derive(Debug)]
pub enum Bid {
    Club(BidLevel) = 7,
    Diamond(BidLevel) = 14,
    Heart(BidLevel) = 21,
    Spade(BidLevel) = 28,
    NoTrumps(BidLevel) = 35,
    Pass = 255,
    Double = 254,
    Redouble = 253,
}

/// convention is to show suits in decreasing order?
#[derive(Eq, PartialEq, Clone, Debug, Ord, PartialOrd)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

/// convention is to show rank in decreasing order
#[derive(Debug, Ord, Eq, PartialOrd, PartialEq, Clone)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

/// Each playing card has an explicit card state defined by hand, suit, rank and discard.
/// Cards are either in a player's hand (when discard = None)
/// or have been discarded in the order indicated by Some(u8)
#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Clone)]
pub struct CardState {
    pub value: u8,
    pub index: usize,
    pub hand: Poc,
    pub suit: Suit,
    pub rank: Rank,
    pub discard: Discard,
}

// impl CardState {
//     pub fn rank_as_str(&self) -> String {
//         match self.value {
//             51 | 38 | 25 | 12 => "A".to_string(),
//             50 | 37 | 24 | 11 => "K".to_string(),
//             49 | 36 | 23 | 10 => "Q".to_string(),
//             48 | 35 | 22 | 9 => "J".to_string(),
//             47 | 34 | 21 | 8 => "T".to_string(),
//             46 | 33 | 20 | 7 => "9".to_string(),
//             45 | 32 | 19 | 6 => "8".to_string(),
//             44 | 31 | 18 | 5 => "7".to_string(),
//             43 | 30 | 17 | 4 => "6".to_string(),
//             42 | 29 | 16 | 3 => "5".to_string(),
//             41 | 28 | 15 | 2 => "4".to_string(),
//             40 | 27 | 14 | 1 => "3".to_string(),
//             39 | 26 | 13 | 0 => "2".to_string(),
//             _ => "?".to_string(),
//         }
//     }
// }

/// A deal consists of the state of the 52 cards dealt together with
/// the list of bids made at the auction
#[derive(Debug)]
pub struct Deal {
    pub bids: Vec<Bid>,
    pub pack_state: [CardState; 52],
}

/// An entire game of Chicago bridge consists of an arbitrary number of modulo 4
/// deals together with the initial dealer and the seating plan
#[derive(Debug)]
pub struct Game {
    pub deals: Vec<Deal>,
    pub initial_dealer: Poc,
    pub seating_plan: HashMap<Poc, Option<Player>>,
}
