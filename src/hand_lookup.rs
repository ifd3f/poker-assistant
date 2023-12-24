use std::{collections::HashMap, ops::Index};

use compact_poker::shrink_hand;
use poker::Card;
use poker_assistant_codegen::{N_HANDS, ORDERED_HANDS_RAW};

pub struct HandLookup {
    pub map: HashMap<u32, u32, fasthash::t1ha::t1ha0::Hash64>,
}

impl HandLookup {
    pub fn new() -> Self {
        let mut map = HashMap::with_hasher(fasthash::t1ha::t1ha0::Hash64);
        map.reserve(N_HANDS);
        for i in 0..N_HANDS {
            let bytes = &ORDERED_HANDS_RAW[i..i + 4];
            let hand = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            map.insert(hand, i as u32);
        }
        Self { map }
    }

    pub fn get(&self, i: impl Idx) -> Option<&u32> {
        self.map.get(&i.to_shrunken_hand())
    }
}

impl<I: Idx> Index<I> for HandLookup {
    type Output = u32;

    fn index(&self, index: I) -> &Self::Output {
        HandLookup::get(self, index).expect("could not find value matching index")
    }
}

trait Idx {
    fn to_shrunken_hand(self) -> u32;
}

impl Idx for &[Card] {
    fn to_shrunken_hand(self) -> u32 {
        shrink_hand(self)
    }
}

impl Idx for u32 {
    fn to_shrunken_hand(self) -> u32 {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_reverse_lookup_works() {
        HandLookup::new();
    }
}
