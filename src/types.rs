use crate::term_in_out::read_msg;
use std::collections::HashMap;

/// At start of game, we need to ask for and store the 4 player names
pub type Player = String;

pub type KeyboardString = String;

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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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

impl CardState {
    pub fn rank_as_str(&self) -> String {
        match self.rank {
            Rank::Ace => "A".to_string(),
            Rank::King => "K".to_string(),
            Rank::Queen => "Q".to_string(),
            Rank::Jack => "J".to_string(),
            Rank::Ten => "T".to_string(),
            Rank::Nine => "9".to_string(),
            Rank::Eight => "8".to_string(),
            Rank::Seven => "7".to_string(),
            Rank::Six => "6".to_string(),
            Rank::Five => "5".to_string(),
            Rank::Four => "4".to_string(),
            Rank::Three => "3".to_string(),
            Rank::Two => "2".to_string(),
        }
    }
}

/// A deal consists of the state of the 52 cards dealt together with
/// the list of bids made at the auction
#[derive(Debug, Clone)]
pub struct Deal {
    pub bids: Vec<Bid>,
    pub pack_state: [CardState; 52],
}

/// An entire game of Chicago bridge consists of an arbitrary number of modulo 4
/// deals together with the initial dealer and the seating plan
#[derive(Debug, Clone)]
pub struct Game {
    pub deals: Vec<Deal>,
    pub initial_dealer: Poc,
    pub seating_plan: HashMap<Poc, Option<Player>>,
}

impl Game {
    pub fn update_seating_plan(self) -> Self {
        let mut seating_plan = HashMap::new();
        let mut player_names = String::new();
        read_msg(&mut player_names);
        let mut iter = player_names.split_whitespace();
        // don't try to store the borrowed &str, store a real String
        seating_plan.insert(Poc::North, iter.next().map(|s| s.to_string()));
        seating_plan.insert(Poc::South, iter.next().map(|s| s.to_string()));
        seating_plan.insert(Poc::East, iter.next().map(|s| s.to_string()));
        seating_plan.insert(Poc::West, iter.next().map(|s| s.to_string()));
        let game_updated = Game {
            seating_plan: seating_plan,
            ..self
        };
        game_updated
    }

    pub fn update_discard(self, rank: Rank, suit: Suit) -> Self {
        println!("rank: {:?} suit: {:?}", &rank, &suit);
        let index = self.deals.len() - 1;
        println!("deal_index: {:?}", index);
        let mut pack_state = self.deals[index].pack_state.clone();
        let card: Vec<_> = pack_state
            .into_iter()
            .filter(|x| x.rank == rank && x.suit == suit)
            .collect();
        println!("{:?}", &card);
        // how many discards already?
        //let temp = pack_state.into_iter().fold(0, |acc, x.hand| acc + x);
        //pack_state[0].discard =
        //let game_updated = Game { deals: ..self };
        //game_updated
        self
    }
}
