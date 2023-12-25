use std::{collections::HashMap, ops::Index};
use lazy_static::lazy_static;

use compact_poker::SHand;
use poker::Card;

const ORDERED_HANDS_RAW: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ordered_hands.bin"));

pub const N_HANDS: usize = ORDERED_HANDS_RAW.len() / 4;

lazy_static! {
    pub static ref LOOKUP: HandLookup = HandLookup::new();
}

pub struct HandLookup {
    pub map: HashMap<SHand, u32, fasthash::t1ha::t1ha0::Hash64>,
}

impl HandLookup {
    pub fn new() -> Self {
        let mut map = HashMap::with_hasher(fasthash::t1ha::t1ha0::Hash64);
        map.reserve(N_HANDS);
        for i in 0..N_HANDS {
            let offset = i * 4;
            let bytes = &ORDERED_HANDS_RAW[offset..offset + 4];
            let hand = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

            // This is safe because it is guaranteed by ORDERED_HANDS_RAW.
            unsafe {
                map.insert(SHand::unsafe_from_raw(hand), i as u32);
            }
        }
        Self { map }
    }

    pub fn get(&self, i: impl Into<SHand>) -> Option<&u32> {
        self.map.get(&i.into())
    }
}

impl<I: Into<SHand>> Index<I> for HandLookup {
    type Output = u32;

    fn index(&self, index: I) -> &Self::Output {
        HandLookup::get(self, index).expect("could not find value matching index")
    }
}

#[cfg(test)]
mod tests {
    use compact_poker::SCard;

    use poker::{Rank, Suit};
    use super::*;

    #[test]
    fn build_reverse_lookup_works() {
        HandLookup::new();
    }

    #[test]
    fn correct_number_of_hands() {
assert_eq!(N_HANDS, 2598960);
    }

    #[test]
    fn correct_number_of_bytes() {
assert_eq!(ORDERED_HANDS_RAW.len(), 2598960 * 4);
    }

    #[test]
    fn lookup_royal_flush() {
        let hand: &[SCard] = &[
            SCard::new(Rank::Ace, Suit::Spades),
            SCard::new(Rank::King, Suit::Spades),
            SCard::new(Rank::Queen, Suit::Spades),
            SCard::new(Rank::Jack, Suit::Spades),
            SCard::new(Rank::Ten, Suit::Spades)
        ];

        assert!(LOOKUP[hand] > 2598960 - 4);
    }

    #[test]
    fn run_lookup() {
        let hand: &[SCard] = &[
            SCard::new(Rank::Eight, Suit::Clubs),
            SCard::new(Rank::Queen, Suit::Hearts),
            SCard::new(Rank::Two, Suit::Diamonds),
            SCard::new(Rank::King, Suit::Spades),
            SCard::new(Rank::Two, Suit::Hearts),
        ];

        assert_eq!(LOOKUP[hand], LOOKUP[SHand::from(hand)]);
    }
}
